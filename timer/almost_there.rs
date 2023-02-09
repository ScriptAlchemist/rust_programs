use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, time::{Duration, Instant}, io};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Gauge},
    Terminal
};
use winapi::um::utilapiset::Beep;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: timer [time in format hhmmss]");
        return;
    }

    let timer_duration = args[1].clone();

    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    let timer_duration_parts = timer_duration.chars();
    let mut digit_str = String::new();
    for c in timer_duration_parts {
        if c.is_numeric() {
            digit_str.push(c);
            continue;
        }
        let digit = digit_str.parse::<u64>().unwrap_or(0);
        digit_str = String::new();
        match c {
            'h' => hours = digit,
            'm' => minutes = digit,
            's' => seconds = digit,
            _ => {
                println!("Invalid character in duration string");
                return;
            }
        }
    }

    let total_duration = Duration::from_secs(hours * 3600 + minutes * 60 + seconds);

    let start = Instant::now();
    let end = start + total_duration;

    // setup terminal
    enable_raw_mode();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    println!("Timer started for {:?}", total_duration);
    while Instant::now() < end {
        let remaining = end - Instant::now();
        let remaining_secs = remaining.as_secs();
        let remaining_hours = remaining_secs / 3600;
        let remaining_minutes = (remaining_secs % 3600) / 60;
        let remaining_seconds = remaining_secs % 60;
        let total_time_secs = total_duration.as_secs();
        let elapsed_time_secs = total_time_secs - remaining_secs;
        let elapsed_time_fraction = elapsed_time_secs as f64 / total_time_secs as f64;

        let gauge = Gauge::default()
            .block(Block::default().title("Elapsed Time"))
            .style(Style::default().fg(Color::Yellow))
            .ratio(elapsed_time_fraction)
            .label(format!("{:02}h:{:02}m:{:02}s", remaining_hours, remaining_minutes, remaining_seconds));
				terminal.draw(|f| {
								let size = f.size();
								let layout = Layout::default()
										.direction(Direction::Vertical)
										.constraints([Constraint::Percentage(100)].as_ref())
										.split(size);

								f.render_widget(gauge, layout[0]);
						}).unwrap();
						std::thread::sleep(Duration::from_secs(1));
				}

        if event::poll(Duration::from_millis(100)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        disable_raw_mode().unwrap();
                        break;
                    }
                }
                _ => {}
            }
        }
				unsafe {
						Beep(440, 500);
						Beep(400, 800);
						Beep(440, 500);
				}
        disable_raw_mode();
				println!("\nTimer ended");
}

