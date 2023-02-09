use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{env, time::{Duration, Instant}, io};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Gauge, Paragraph, Borders},
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
    enable_raw_mode().unwrap();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

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
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(elapsed_time_fraction);

        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let text = [                Span::styled(                    format!("{:02}h:{:02}m:{:02}s remaining", remaining_hours, remaining_minutes, remaining_seconds),                    Style::default().fg(Color::White),                ),            ];

            let text_vec: Vec<Spans<'_>> = text.iter().cloned().collect();
            f.render_widget(gauge, chunks[0]);
            f.render_widget(
                Paragraph::new(text_vec.iter())
                    .block(Block::default().title("Time Remaining").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center),
                chunks[1],
            );
        });

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
    }

    disable_raw_mode().unwrap();
    unsafe {
            Beep(440, 500);
            Beep(400, 800);
            Beep(440, 500);
    }
    println!("\nTimer ended");
}
