use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    env,
    error::Error,
    io,
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Terminal,
};

#[cfg(windows)]
use winapi::um::utilapiset::Beep;

#[cfg(not(windows))]
use std::{io::Write, thread};

#[cfg(unix)]
use signal_hook::{
    consts::signal::{SIGHUP, SIGINT, SIGTERM},
    flag as signal_flag,
};

#[derive(Debug, PartialEq, Eq)]
enum TimerStatus {
    Completed,
    Canceled,
}

#[derive(Debug, PartialEq, Eq)]
struct TimerConfig {
    command: TimerCommand,
    color: Color,
    mode: TimerMode,
    quiet: bool,
    tmux_target: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum TimerCommand {
    Countdown { duration: Duration },
    Stopwatch,
}

#[derive(Debug, PartialEq, Eq)]
enum TimerMode {
    Terminal,
    Tmux,
}

#[derive(Debug, PartialEq, Eq)]
enum TmuxControl {
    Pause,
    Resume,
    Restart,
}

const COLOR_NAMES: &str = "black, red, green, yellow, blue, magenta, cyan, gray, dark-gray, white, light-red, light-green, light-yellow, light-blue, light-magenta, light-cyan";
const TMUX_TIMER_PID_OPTION: &str = "@timer_pid";
const TMUX_TIMER_OPTION: &str = "@timer_status";
const TMUX_TIMER_CONTROL_OPTION: &str = "@timer_control";
const TMUX_TIMER_PAUSED_OPTION: &str = "@timer_paused";

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let binary = args.first().map(String::as_str).unwrap_or("timer");
    let cli_args: Vec<&str> = args.iter().skip(1).map(String::as_str).collect();

    if cli_args.is_empty() {
        print_usage(binary);
        return Ok(());
    }

    if cli_args.iter().any(|arg| *arg == "--help" || *arg == "-h") {
        print_usage(binary);
        return Ok(());
    }

    let config =
        parse_timer_config(&cli_args).map_err(|err| format!("{err}\n\n{}", usage(binary)))?;

    let TimerConfig {
        command,
        color,
        mode,
        quiet,
        tmux_target,
    } = config;

    match command {
        TimerCommand::Countdown { duration } => {
            let status = match mode {
                TimerMode::Terminal => run_timer(duration, color)?,
                TimerMode::Tmux => run_tmux_timer(duration, color, quiet, tmux_target.as_deref())?,
            };

            match status {
                TimerStatus::Completed => {
                    play_alarm()?;
                    if !quiet {
                        println!("\nTimer ended");
                    }
                }
                TimerStatus::Canceled => {
                    if !quiet {
                        println!("\nTimer canceled");
                    }
                }
            }
        }
        TimerCommand::Stopwatch => {
            let elapsed = match mode {
                TimerMode::Terminal => run_stopwatch(color)?,
                TimerMode::Tmux => run_tmux_stopwatch(color, quiet, tmux_target.as_deref())?,
            };

            if !quiet {
                println!("\nStopwatch stopped at {}", format_hms(elapsed.as_secs()));
            }
        }
    }

    Ok(())
}

fn usage(binary: &str) -> String {
    format!(
        "Usage: {binary} [options] <duration>\n       {binary} [options] stopwatch\n\nOptions:\n  -c, --color <color>  Set the display color (default: red)\n      --stopwatch      Count up until stopped instead of requiring a duration\n      --tmux           Show the timer/stopwatch in tmux status-right instead of full-screen UI\n      --quiet          Suppress terminal messages for background launches\n      --tmux-target <target>\n                       Scope tmux updates to a specific session target\n\nColors:\n  {COLOR_NAMES}\n\nExamples:\n  {binary} 25m\n  {binary} 1h30m --color green\n  {binary} --color blue 90s\n  {binary} -c cyan 1:30\n  {binary} stopwatch\n  {binary} --stopwatch --color green\n  {binary} --tmux --quiet --color green 25m\n  {binary} --tmux --quiet stopwatch"
    )
}

fn print_usage(binary: &str) {
    println!("{}", usage(binary));
}

fn parse_timer_config(args: &[&str]) -> Result<TimerConfig, String> {
    let mut command_arg = None;
    let mut stopwatch = false;
    let mut color = Color::Red;
    let mut mode = TimerMode::Terminal;
    let mut quiet = false;
    let mut tmux_target = None;
    let mut index = 0;

    while index < args.len() {
        match args[index] {
            "--tmux" => {
                mode = TimerMode::Tmux;
            }
            "--quiet" => {
                quiet = true;
            }
            "--stopwatch" => {
                stopwatch = true;
            }
            "--tmux-target" => {
                index += 1;
                let target = args
                    .get(index)
                    .ok_or_else(|| "missing target after --tmux-target".to_string())?;
                tmux_target = Some(parse_tmux_target(target)?);
            }
            arg if arg.starts_with("--tmux-target=") => {
                let target = arg
                    .strip_prefix("--tmux-target=")
                    .expect("prefix checked before stripping");
                tmux_target = Some(parse_tmux_target(target)?);
            }
            "--color" | "-c" => {
                index += 1;
                let color_arg = args
                    .get(index)
                    .ok_or_else(|| "missing color after --color".to_string())?;
                color = parse_color(color_arg)?;
            }
            arg if arg.starts_with("--color=") => {
                let color_arg = arg
                    .strip_prefix("--color=")
                    .expect("prefix checked before stripping");
                color = parse_color(color_arg)?;
            }
            arg if arg.starts_with('-') => {
                return Err(format!("unknown option '{arg}'"));
            }
            arg => {
                if command_arg.is_some() {
                    return Err(format!("unexpected extra argument '{arg}'"));
                }

                command_arg = Some(arg);
            }
        }

        index += 1;
    }

    let command = match (stopwatch, command_arg) {
        (true, None) => TimerCommand::Stopwatch,
        (true, Some("stopwatch")) => TimerCommand::Stopwatch,
        (true, Some(_)) => {
            return Err("--stopwatch cannot be used with a duration".to_string());
        }
        (false, Some("stopwatch")) => TimerCommand::Stopwatch,
        (false, Some(duration_arg)) => TimerCommand::Countdown {
            duration: parse_duration(duration_arg)?,
        },
        (false, None) => {
            return Err("missing duration or stopwatch command".to_string());
        }
    };

    Ok(TimerConfig {
        command,
        color,
        mode,
        quiet,
        tmux_target,
    })
}

fn parse_tmux_target(input: &str) -> Result<String, String> {
    let target = input.trim();

    if target.is_empty() {
        Err("tmux target cannot be empty".to_string())
    } else {
        Ok(target.to_string())
    }
}

fn parse_color(input: &str) -> Result<Color, String> {
    let normalized: String = input
        .trim()
        .chars()
        .filter(|c| *c != '-' && *c != '_' && !c.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect();

    match normalized.as_str() {
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" | "purple" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "gray" | "grey" => Ok(Color::Gray),
        "darkgray" | "darkgrey" => Ok(Color::DarkGray),
        "white" => Ok(Color::White),
        "lightred" | "brightred" => Ok(Color::LightRed),
        "lightgreen" | "brightgreen" => Ok(Color::LightGreen),
        "lightyellow" | "brightyellow" => Ok(Color::LightYellow),
        "lightblue" | "brightblue" => Ok(Color::LightBlue),
        "lightmagenta" | "brightmagenta" | "lightpurple" | "brightpurple" => {
            Ok(Color::LightMagenta)
        }
        "lightcyan" | "brightcyan" => Ok(Color::LightCyan),
        "" => Err(format!("color cannot be empty; use one of: {COLOR_NAMES}")),
        _ => Err(format!(
            "unsupported color '{input}'; use one of: {COLOR_NAMES}"
        )),
    }
}

fn foreground_for_background(background: Color) -> Color {
    match background {
        Color::Yellow
        | Color::Green
        | Color::Cyan
        | Color::Gray
        | Color::White
        | Color::LightYellow
        | Color::LightGreen
        | Color::LightCyan => Color::Black,
        _ => Color::White,
    }
}

fn parse_duration(input: &str) -> Result<Duration, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("duration cannot be empty".to_string());
    }

    if input.contains(':') {
        return parse_colon_duration(input);
    }

    if input.chars().all(|c| c.is_ascii_digit()) {
        return duration_from_secs(parse_u64(input)?);
    }

    let mut total_secs = 0_u64;
    let mut number = String::new();
    let mut saw_unit = false;

    for c in input.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }

        if c.is_ascii_digit() {
            number.push(c);
            continue;
        }

        if number.is_empty() {
            return Err(format!("missing number before '{c}'"));
        }

        let amount = parse_u64(&number)?;
        number.clear();

        let unit_seconds = match c.to_ascii_lowercase() {
            'h' => checked_mul(amount, 3_600)?,
            'm' => checked_mul(amount, 60)?,
            's' => amount,
            _ => return Err(format!("invalid duration unit '{c}'; use h, m, or s")),
        };

        total_secs = checked_add(total_secs, unit_seconds)?;
        saw_unit = true;
    }

    if !number.is_empty() {
        return Err("duration value is missing a unit; use h, m, s, or plain seconds".to_string());
    }

    if !saw_unit {
        return Err("duration must contain a number and a unit".to_string());
    }

    duration_from_secs(total_secs)
}

fn parse_colon_duration(input: &str) -> Result<Duration, String> {
    let parts: Vec<&str> = input.split(':').collect();

    if !(2..=3).contains(&parts.len()) {
        return Err("colon duration must be mm:ss or hh:mm:ss".to_string());
    }

    if parts.iter().any(|part| part.is_empty()) {
        return Err("colon duration cannot contain empty segments".to_string());
    }

    if !parts
        .iter()
        .all(|part| part.chars().all(|c| c.is_ascii_digit()))
    {
        return Err("colon duration segments must be numeric".to_string());
    }

    let values = parts
        .iter()
        .map(|part| parse_u64(part))
        .collect::<Result<Vec<_>, _>>()?;

    let total_secs = match values.as_slice() {
        [minutes, seconds] => {
            if *seconds >= 60 {
                return Err("seconds must be less than 60 in mm:ss format".to_string());
            }

            checked_add(checked_mul(*minutes, 60)?, *seconds)?
        }
        [hours, minutes, seconds] => {
            if *minutes >= 60 || *seconds >= 60 {
                return Err(
                    "minutes and seconds must be less than 60 in hh:mm:ss format".to_string(),
                );
            }

            checked_add(
                checked_add(checked_mul(*hours, 3_600)?, checked_mul(*minutes, 60)?)?,
                *seconds,
            )?
        }
        _ => unreachable!("duration part count is already validated"),
    };

    duration_from_secs(total_secs)
}

fn parse_u64(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| format!("duration component '{input}' is too large"))
}

fn duration_from_secs(seconds: u64) -> Result<Duration, String> {
    if seconds == 0 {
        return Err("duration must be greater than zero".to_string());
    }

    Ok(Duration::from_secs(seconds))
}

fn checked_mul(value: u64, multiplier: u64) -> Result<u64, String> {
    value
        .checked_mul(multiplier)
        .ok_or_else(|| "duration is too large".to_string())
}

fn checked_add(left: u64, right: u64) -> Result<u64, String> {
    left.checked_add(right)
        .ok_or_else(|| "duration is too large".to_string())
}

struct PausableClock {
    accumulated: Duration,
    current_run_started_at: Instant,
    paused: bool,
}

impl PausableClock {
    fn start() -> Self {
        Self {
            accumulated: Duration::ZERO,
            current_run_started_at: Instant::now(),
            paused: false,
        }
    }

    fn elapsed(&self) -> Duration {
        if self.paused {
            self.accumulated
        } else {
            self.accumulated
                .saturating_add(self.current_run_started_at.elapsed())
        }
    }

    fn pause(&mut self) {
        if !self.paused {
            self.accumulated = self.elapsed();
            self.paused = true;
        }
    }

    fn resume(&mut self) {
        if self.paused {
            self.current_run_started_at = Instant::now();
            self.paused = false;
        }
    }

    fn restart(&mut self) {
        self.accumulated = Duration::ZERO;
        self.current_run_started_at = Instant::now();
        self.paused = false;
    }

    fn paused(&self) -> bool {
        self.paused
    }
}

fn run_timer(total_duration: Duration, color: Color) -> Result<TimerStatus, Box<dyn Error>> {
    let signals = ShutdownSignals::start()?;
    let mut session = TerminalSession::start()?;
    let start = Instant::now();

    let status = loop {
        let elapsed = start.elapsed();
        let remaining = total_duration.saturating_sub(elapsed);

        session.draw_timer(total_duration, remaining, color)?;

        if remaining.is_zero() {
            break TimerStatus::Completed;
        }

        if signals.received() {
            break TimerStatus::Canceled;
        }

        if event::poll(Duration::from_millis(100))? && should_quit(event::read()?) {
            break TimerStatus::Canceled;
        }
    };

    session.restore()?;
    Ok(status)
}

fn run_stopwatch(color: Color) -> Result<Duration, Box<dyn Error>> {
    let signals = ShutdownSignals::start()?;
    let mut session = TerminalSession::start()?;
    let start = Instant::now();

    let elapsed = loop {
        session.draw_stopwatch(start.elapsed(), color)?;

        if signals.received() {
            break start.elapsed();
        }

        if event::poll(Duration::from_millis(100))? && should_quit(event::read()?) {
            break start.elapsed();
        }
    };

    session.restore()?;
    Ok(elapsed)
}

fn run_tmux_timer(
    total_duration: Duration,
    color: Color,
    quiet: bool,
    tmux_target: Option<&str>,
) -> Result<TimerStatus, Box<dyn Error>> {
    let signals = ShutdownSignals::start()?;
    let mut tmux = TmuxStatusSession::start("timer", color, tmux_target)?;
    let mut raw_mode = match RawModeGuard::start() {
        Ok(raw_mode) => Some(raw_mode),
        Err(err) => {
            if !quiet {
                eprintln!("Keyboard cancel is unavailable in this terminal: {err}");
            }
            None
        }
    };

    if !quiet {
        if raw_mode.is_some() {
            println!("Timer running in tmux status-right. Press q, Esc, or Ctrl-C to cancel.");
        } else {
            println!("Timer running in tmux status-right.");
        }
    }

    let mut clock = PausableClock::start();
    let mut last_display = None;

    let status = loop {
        if apply_tmux_control(&tmux, &mut clock)? {
            last_display = None;
        }

        let elapsed = clock.elapsed();
        let remaining = total_duration.saturating_sub(elapsed);
        let display_secs = display_seconds(remaining);
        let paused = clock.paused();

        if last_display != Some((display_secs, paused)) {
            tmux.set_timer_text(&format_tmux_display(display_secs, paused))?;
            last_display = Some((display_secs, paused));
        }

        if remaining.is_zero() && !paused {
            break TimerStatus::Completed;
        }

        if signals.received() {
            break TimerStatus::Canceled;
        }

        if raw_mode.is_some() {
            if event::poll(Duration::from_millis(100))? && should_quit(event::read()?) {
                break TimerStatus::Canceled;
            }
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    };

    if let Some(raw_mode) = &mut raw_mode {
        raw_mode.restore()?;
    }
    tmux.restore()?;

    Ok(status)
}

fn run_tmux_stopwatch(
    color: Color,
    quiet: bool,
    tmux_target: Option<&str>,
) -> Result<Duration, Box<dyn Error>> {
    let signals = ShutdownSignals::start()?;
    let mut tmux = TmuxStatusSession::start("stopwatch", color, tmux_target)?;
    let mut raw_mode = match RawModeGuard::start() {
        Ok(raw_mode) => Some(raw_mode),
        Err(err) => {
            if !quiet {
                eprintln!("Keyboard stop is unavailable in this terminal: {err}");
            }
            None
        }
    };

    if !quiet {
        if raw_mode.is_some() {
            println!("Stopwatch running in tmux status-right. Press q, Esc, or Ctrl-C to stop.");
        } else {
            println!("Stopwatch running in tmux status-right.");
        }
    }

    let mut clock = PausableClock::start();
    let mut last_display = None;

    let elapsed = loop {
        if apply_tmux_control(&tmux, &mut clock)? {
            last_display = None;
        }

        let elapsed = clock.elapsed();
        let display_secs = elapsed.as_secs();
        let paused = clock.paused();

        if last_display != Some((display_secs, paused)) {
            tmux.set_timer_text(&format_tmux_display(display_secs, paused))?;
            last_display = Some((display_secs, paused));
        }

        if signals.received() {
            break clock.elapsed();
        }

        if raw_mode.is_some() {
            if event::poll(Duration::from_millis(100))? && should_quit(event::read()?) {
                break clock.elapsed();
            }
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    };

    if let Some(raw_mode) = &mut raw_mode {
        raw_mode.restore()?;
    }
    tmux.restore()?;

    Ok(elapsed)
}

fn apply_tmux_control(
    tmux: &TmuxStatusSession,
    clock: &mut PausableClock,
) -> Result<bool, Box<dyn Error>> {
    let Some(control) = tmux.take_control()? else {
        return Ok(false);
    };

    match control {
        TmuxControl::Pause => {
            clock.pause();
            tmux.set_paused(true)?;
        }
        TmuxControl::Resume => {
            clock.resume();
            tmux.set_paused(false)?;
        }
        TmuxControl::Restart => {
            clock.restart();
            tmux.set_paused(false)?;
        }
    }

    Ok(true)
}

fn should_quit(event: Event) -> bool {
    match event {
        Event::Key(key) if key.code == KeyCode::Char('q') => true,
        Event::Key(key) if key.code == KeyCode::Esc => true,
        Event::Key(key)
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            true
        }
        _ => false,
    }
}

struct ShutdownSignals {
    received: Arc<AtomicBool>,
}

impl ShutdownSignals {
    #[cfg(unix)]
    fn start() -> Result<Self, Box<dyn Error>> {
        let received = Arc::new(AtomicBool::new(false));

        for signal in [SIGHUP, SIGINT, SIGTERM] {
            signal_flag::register(signal, Arc::clone(&received))?;
        }

        Ok(Self { received })
    }

    #[cfg(not(unix))]
    fn start() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            received: Arc::new(AtomicBool::new(false)),
        })
    }

    fn received(&self) -> bool {
        self.received.load(Ordering::SeqCst)
    }
}

struct RawModeGuard {
    restored: bool,
}

impl RawModeGuard {
    fn start() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        Ok(Self { restored: false })
    }

    fn restore(&mut self) -> Result<(), Box<dyn Error>> {
        if self.restored {
            return Ok(());
        }

        disable_raw_mode()?;
        self.restored = true;

        Ok(())
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    restored: bool,
}

impl TerminalSession {
    fn start() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
            let _ = disable_raw_mode();
            return Err(err.into());
        }

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = match Terminal::new(backend) {
            Ok(terminal) => terminal,
            Err(err) => {
                let _ = disable_raw_mode();
                return Err(err.into());
            }
        };

        terminal.clear()?;

        Ok(Self {
            terminal,
            restored: false,
        })
    }

    fn draw_timer(
        &mut self,
        total_duration: Duration,
        remaining: Duration,
        color: Color,
    ) -> Result<(), Box<dyn Error>> {
        let remaining_secs = display_seconds(remaining);
        let elapsed = total_duration.saturating_sub(remaining);
        let progress = (elapsed.as_secs_f64() / total_duration.as_secs_f64()).clamp(0.0, 1.0);

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("timer - q: quit")
                    .borders(Borders::ALL),
            )
            .style(
                Style::default()
                    .fg(foreground_for_background(color))
                    .bg(color),
            )
            .ratio(progress)
            .label(format_hms(remaining_secs));

        self.terminal.draw(|frame| {
            let size = frame.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            frame.render_widget(gauge, layout[0]);
        })?;

        Ok(())
    }

    fn draw_stopwatch(&mut self, elapsed: Duration, color: Color) -> Result<(), Box<dyn Error>> {
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("stopwatch - q: stop")
                    .borders(Borders::ALL),
            )
            .style(
                Style::default()
                    .fg(foreground_for_background(color))
                    .bg(color),
            )
            .ratio(1.0)
            .label(format_hms(elapsed.as_secs()));

        self.terminal.draw(|frame| {
            let size = frame.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            frame.render_widget(gauge, layout[0]);
        })?;

        Ok(())
    }

    fn restore(&mut self) -> Result<(), Box<dyn Error>> {
        if self.restored {
            return Ok(());
        }

        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        self.restored = true;

        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

struct TmuxStatusSession {
    target_session: String,
    original_status_right: String,
    original_timer_option: Option<String>,
    original_timer_control_option: Option<String>,
    original_timer_paused_option: Option<String>,
    restored: bool,
}

impl TmuxStatusSession {
    fn start(label: &str, color: Color, target: Option<&str>) -> Result<Self, Box<dyn Error>> {
        if env::var_os("TMUX").is_none() {
            return Err("--tmux can only be used inside a tmux session".into());
        }

        let target_session = resolve_tmux_target(target)?;

        if let Some(pid) = tmux_capture_optional(&[
            "show-option",
            "-qv",
            "-t",
            &target_session,
            TMUX_TIMER_PID_OPTION,
        ])? {
            if process_exists(&pid) {
                return Err(format!(
                    "a tmux timer or stopwatch is already running in {target_session} with pid {pid}"
                )
                .into());
            }
        }

        let original_status_right =
            tmux_capture(&["show-option", "-qv", "-t", &target_session, "status-right"])?;
        let original_timer_option = tmux_capture_optional(&[
            "show-option",
            "-qv",
            "-t",
            &target_session,
            TMUX_TIMER_OPTION,
        ])?;
        let original_timer_control_option = tmux_capture_optional(&[
            "show-option",
            "-qv",
            "-t",
            &target_session,
            TMUX_TIMER_CONTROL_OPTION,
        ])?;
        let original_timer_paused_option = tmux_capture_optional(&[
            "show-option",
            "-qv",
            "-t",
            &target_session,
            TMUX_TIMER_PAUSED_OPTION,
        ])?;
        let status_right =
            append_tmux_status_segment(&original_status_right, &tmux_status_segment(label, color));

        tmux_run(&[
            "set-option",
            "-q",
            "-t",
            &target_session,
            "status-right",
            &status_right,
        ])?;

        let session = Self {
            target_session,
            original_status_right,
            original_timer_option,
            original_timer_control_option,
            original_timer_paused_option,
            restored: false,
        };
        session.set_timer_text("starting")?;
        session.clear_control()?;
        session.set_paused(false)?;
        session.set_timer_pid(std::process::id())?;

        Ok(session)
    }

    fn set_timer_text(&self, text: &str) -> Result<(), Box<dyn Error>> {
        tmux_run(&[
            "set-option",
            "-q",
            "-t",
            &self.target_session,
            TMUX_TIMER_OPTION,
            text,
        ])
    }

    fn set_timer_pid(&self, pid: u32) -> Result<(), Box<dyn Error>> {
        tmux_run(&[
            "set-option",
            "-q",
            "-t",
            &self.target_session,
            TMUX_TIMER_PID_OPTION,
            &pid.to_string(),
        ])
    }

    fn set_paused(&self, paused: bool) -> Result<(), Box<dyn Error>> {
        tmux_run(&[
            "set-option",
            "-q",
            "-t",
            &self.target_session,
            TMUX_TIMER_PAUSED_OPTION,
            if paused { "1" } else { "0" },
        ])
    }

    fn take_control(&self) -> Result<Option<TmuxControl>, Box<dyn Error>> {
        let Some(control) = tmux_capture_optional(&[
            "show-option",
            "-qv",
            "-t",
            &self.target_session,
            TMUX_TIMER_CONTROL_OPTION,
        ])?
        else {
            return Ok(None);
        };

        self.clear_control()?;
        Ok(parse_tmux_control(&control))
    }

    fn clear_control(&self) -> Result<(), Box<dyn Error>> {
        tmux_run(&[
            "set-option",
            "-qu",
            "-t",
            &self.target_session,
            TMUX_TIMER_CONTROL_OPTION,
        ])
    }

    fn restore(&mut self) -> Result<(), Box<dyn Error>> {
        if self.restored {
            return Ok(());
        }

        tmux_run(&[
            "set-option",
            "-q",
            "-t",
            &self.target_session,
            "status-right",
            &self.original_status_right,
        ])?;

        restore_tmux_option(
            &self.target_session,
            TMUX_TIMER_OPTION,
            self.original_timer_option.as_deref(),
        )?;
        restore_tmux_option(
            &self.target_session,
            TMUX_TIMER_CONTROL_OPTION,
            self.original_timer_control_option.as_deref(),
        )?;
        restore_tmux_option(
            &self.target_session,
            TMUX_TIMER_PAUSED_OPTION,
            self.original_timer_paused_option.as_deref(),
        )?;

        let _ = tmux_run(&[
            "set-option",
            "-qu",
            "-t",
            &self.target_session,
            TMUX_TIMER_PID_OPTION,
        ]);

        self.restored = true;

        Ok(())
    }
}

impl Drop for TmuxStatusSession {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

fn restore_tmux_option(
    target_session: &str,
    option: &str,
    original_value: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    match original_value {
        Some(value) => tmux_run(&["set-option", "-q", "-t", target_session, option, value]),
        None => tmux_run(&["set-option", "-qu", "-t", target_session, option]),
    }
}

fn resolve_tmux_target(target: Option<&str>) -> Result<String, Box<dyn Error>> {
    if let Some(target) = target {
        return Ok(target.to_string());
    }

    if let Ok(target) = env::var("TIMER_TMUX_TARGET") {
        let target = target.trim();
        if !target.is_empty() {
            return Ok(target.to_string());
        }
    }

    if let Ok(pane) = env::var("TMUX_PANE") {
        return tmux_capture(&["display-message", "-p", "-t", &pane, "#{session_id}"]);
    }

    tmux_capture(&["display-message", "-p", "#{session_id}"]).map_err(|err| {
        format!(
            "could not resolve tmux session target; launch from a tmux pane or pass --tmux-target: {err}"
        )
        .into()
    })
}

fn append_tmux_status_segment(status_right: &str, segment: &str) -> String {
    if status_right.trim().is_empty() {
        segment.to_string()
    } else {
        format!("{status_right} {segment}")
    }
}

fn tmux_status_segment(label: &str, color: Color) -> String {
    let background = tmux_color_name(color);
    let foreground = tmux_color_name(foreground_for_background(color));

    format!("#[fg={foreground},bg={background},bold] {label} #{{@timer_status}} #[default]")
}

fn parse_tmux_control(input: &str) -> Option<TmuxControl> {
    let normalized = input.trim().to_ascii_lowercase();

    match normalized.as_str() {
        "pause" => Some(TmuxControl::Pause),
        "resume" => Some(TmuxControl::Resume),
        "restart" => Some(TmuxControl::Restart),
        _ => None,
    }
}

fn format_tmux_display(total_seconds: u64, paused: bool) -> String {
    let time = format_hms(total_seconds);

    if paused {
        format!("paused {time}")
    } else {
        time
    }
}

fn tmux_color_name(color: Color) -> &'static str {
    match color {
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
        Color::Magenta => "magenta",
        Color::Cyan => "cyan",
        Color::Gray => "colour245",
        Color::DarkGray => "colour8",
        Color::White => "white",
        Color::LightRed => "brightred",
        Color::LightGreen => "brightgreen",
        Color::LightYellow => "brightyellow",
        Color::LightBlue => "brightblue",
        Color::LightMagenta => "brightmagenta",
        Color::LightCyan => "brightcyan",
        Color::Rgb(_, _, _) | Color::Indexed(_) | Color::Reset => "default",
    }
}

fn tmux_capture(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .map_err(|err| format!("failed to run tmux: {err}"))?;

    if !output.status.success() {
        return Err(format!("tmux command failed: {}", command_error(&output.stderr)).into());
    }

    Ok(trim_command_output(output.stdout))
}

fn process_exists(pid: &str) -> bool {
    if pid.parse::<u32>().is_err() {
        return false;
    }

    Command::new("kill")
        .args(["-0", pid])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn tmux_capture_optional(args: &[&str]) -> Result<Option<String>, Box<dyn Error>> {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .map_err(|err| format!("failed to run tmux: {err}"))?;

    if output.status.success() {
        Ok(Some(trim_command_output(output.stdout)))
    } else {
        Ok(None)
    }
}

fn tmux_run(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .map_err(|err| format!("failed to run tmux: {err}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("tmux command failed: {}", command_error(&output.stderr)).into())
    }
}

fn trim_command_output(output: Vec<u8>) -> String {
    String::from_utf8_lossy(&output)
        .trim_end_matches(['\r', '\n'])
        .to_string()
}

fn command_error(stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();

    if stderr.is_empty() {
        "no error output".to_string()
    } else {
        stderr.to_string()
    }
}

fn format_hms(total_seconds: u64) -> String {
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours:02}h:{minutes:02}m:{seconds:02}s")
}

fn display_seconds(duration: Duration) -> u64 {
    let millis = duration.as_millis();

    if millis == 0 {
        return 0;
    }

    ((millis + 999) / 1_000).min(u64::MAX as u128) as u64
}

#[cfg(windows)]
fn play_alarm() -> Result<(), Box<dyn Error>> {
    unsafe {
        Beep(440, 500);
        Beep(400, 800);
        Beep(440, 500);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn play_alarm() -> Result<(), Box<dyn Error>> {
    let beep_worked = std::process::Command::new("osascript")
        .args(["-e", "beep 3"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if beep_worked {
        return Ok(());
    }

    ring_terminal_bell(3)
}

#[cfg(all(not(windows), not(target_os = "macos")))]
fn play_alarm() -> Result<(), Box<dyn Error>> {
    ring_terminal_bell(3)
}

#[cfg(not(windows))]
fn ring_terminal_bell(times: usize) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    for _ in 0..times {
        write!(stdout, "\x07")?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(300));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_timer_config_with_default_color() {
        assert_eq!(
            parse_timer_config(&["25m"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Countdown {
                    duration: Duration::from_secs(1_500),
                },
                color: Color::Red,
                mode: TimerMode::Terminal,
                quiet: false,
                tmux_target: None,
            }
        );
    }

    #[test]
    fn parses_timer_config_with_color_option() {
        assert_eq!(
            parse_timer_config(&["25m", "--color", "green"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Countdown {
                    duration: Duration::from_secs(1_500),
                },
                color: Color::Green,
                mode: TimerMode::Terminal,
                quiet: false,
                tmux_target: None,
            }
        );
        assert_eq!(
            parse_timer_config(&["--color=light-blue", "90s"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Countdown {
                    duration: Duration::from_secs(90),
                },
                color: Color::LightBlue,
                mode: TimerMode::Terminal,
                quiet: false,
                tmux_target: None,
            }
        );
        assert_eq!(
            parse_timer_config(&["-c", "purple", "1:30"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Countdown {
                    duration: Duration::from_secs(90),
                },
                color: Color::Magenta,
                mode: TimerMode::Terminal,
                quiet: false,
                tmux_target: None,
            }
        );
    }

    #[test]
    fn parses_timer_config_with_tmux_mode() {
        assert_eq!(
            parse_timer_config(&["--tmux", "--color", "green", "25m"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Countdown {
                    duration: Duration::from_secs(1_500),
                },
                color: Color::Green,
                mode: TimerMode::Tmux,
                quiet: false,
                tmux_target: None,
            }
        );
    }

    #[test]
    fn parses_timer_config_with_quiet_mode() {
        assert_eq!(
            parse_timer_config(&["--tmux", "--quiet", "--tmux-target", "work", "25m"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Countdown {
                    duration: Duration::from_secs(1_500),
                },
                color: Color::Red,
                mode: TimerMode::Tmux,
                quiet: true,
                tmux_target: Some("work".to_string()),
            }
        );
    }

    #[test]
    fn parses_stopwatch_config() {
        assert_eq!(
            parse_timer_config(&["stopwatch"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Stopwatch,
                color: Color::Red,
                mode: TimerMode::Terminal,
                quiet: false,
                tmux_target: None,
            }
        );
        assert_eq!(
            parse_timer_config(&["--stopwatch", "--color", "green"]).unwrap(),
            TimerConfig {
                command: TimerCommand::Stopwatch,
                color: Color::Green,
                mode: TimerMode::Terminal,
                quiet: false,
                tmux_target: None,
            }
        );
        assert_eq!(
            parse_timer_config(&["--tmux", "--quiet", "--tmux-target", "work", "stopwatch"])
                .unwrap(),
            TimerConfig {
                command: TimerCommand::Stopwatch,
                color: Color::Red,
                mode: TimerMode::Tmux,
                quiet: true,
                tmux_target: Some("work".to_string()),
            }
        );
    }

    #[test]
    fn rejects_invalid_timer_config() {
        assert!(parse_timer_config(&[]).is_err());
        assert!(parse_timer_config(&["25m", "30m"]).is_err());
        assert!(parse_timer_config(&["25m", "--color"]).is_err());
        assert!(parse_timer_config(&["25m", "--wat"]).is_err());
        assert!(parse_timer_config(&["25m", "--color", "beige"]).is_err());
        assert!(parse_timer_config(&["--tmux-target", "", "25m"]).is_err());
        assert!(parse_timer_config(&["--stopwatch", "25m"]).is_err());
    }

    #[test]
    fn builds_tmux_status_segment() {
        assert_eq!(
            tmux_status_segment("timer", Color::Green),
            "#[fg=black,bg=green,bold] timer #{@timer_status} #[default]"
        );
        assert_eq!(
            tmux_status_segment("stopwatch", Color::Blue),
            "#[fg=white,bg=blue,bold] stopwatch #{@timer_status} #[default]"
        );
        assert_eq!(
            append_tmux_status_segment("existing", &tmux_status_segment("timer", Color::Blue)),
            "existing #[fg=white,bg=blue,bold] timer #{@timer_status} #[default]"
        );
    }

    #[test]
    fn parses_tmux_controls() {
        assert_eq!(parse_tmux_control("pause"), Some(TmuxControl::Pause));
        assert_eq!(parse_tmux_control(" resume "), Some(TmuxControl::Resume));
        assert_eq!(parse_tmux_control("RESTART"), Some(TmuxControl::Restart));
        assert_eq!(parse_tmux_control("wat"), None);
    }

    #[test]
    fn formats_tmux_display_with_paused_state() {
        assert_eq!(format_tmux_display(90, false), "00h:01m:30s");
        assert_eq!(format_tmux_display(90, true), "paused 00h:01m:30s");
    }

    #[test]
    fn rejects_invalid_timer_pid() {
        assert!(!process_exists("abc"));
    }

    #[test]
    fn parses_unit_durations() {
        assert_eq!(parse_duration("25m").unwrap(), Duration::from_secs(1_500));
        assert_eq!(
            parse_duration("1h30m5s").unwrap(),
            Duration::from_secs(5_405)
        );
        assert_eq!(parse_duration("90s").unwrap(), Duration::from_secs(90));
    }

    #[test]
    fn parses_plain_seconds() {
        assert_eq!(parse_duration("90").unwrap(), Duration::from_secs(90));
    }

    #[test]
    fn parses_colon_durations() {
        assert_eq!(parse_duration("1:30").unwrap(), Duration::from_secs(90));
        assert_eq!(
            parse_duration("1:02:03").unwrap(),
            Duration::from_secs(3_723)
        );
    }

    #[test]
    fn rejects_invalid_durations() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("0s").is_err());
        assert!(parse_duration("1x").is_err());
        assert!(parse_duration("1m30").is_err());
        assert!(parse_duration("1:60").is_err());
    }

    #[test]
    fn rounds_display_seconds_up() {
        assert_eq!(display_seconds(Duration::from_millis(1)), 1);
        assert_eq!(display_seconds(Duration::from_millis(999)), 1);
        assert_eq!(display_seconds(Duration::from_millis(1_001)), 2);
        assert_eq!(display_seconds(Duration::ZERO), 0);
    }
}
