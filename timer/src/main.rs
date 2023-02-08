use std::{env, time::{Duration, Instant}};
use winapi::um::consoleapi::Beep;

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

    println!("Timer started for {:?}", total_duration);
    while Instant::now() < end {
        let remaining = end - Instant::now();
        let remaining_secs = remaining.as_secs();
        let remaining_hours = remaining_secs / 3600;
        let remaining_minutes = (remaining_secs % 3600) / 60;
        let remaining_seconds = remaining_secs % 60;
        println!("\r{:02}h:{:02}m:{:02}s remaining", remaining_hours, remaining_minutes, remaining_seconds);
        std::thread::sleep(Duration::from_secs(1));
    }
    println!("\nTimer ended");

    unsafe {
        Beep(1000, 500);
    }
}
