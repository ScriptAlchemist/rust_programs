use std::io::{self, BufRead};
use chrono::Utc;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let now = Utc::now();
                let trimmed_start = line.trim_start();
                let leading_whitespace = &line[..line.len() - trimmed_start.len()];
                println!("{}{}'{time}'", leading_whitespace, trimmed_start, time = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true));
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
