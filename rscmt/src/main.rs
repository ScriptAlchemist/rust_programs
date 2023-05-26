use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let trimmed_start = line.trim_start();
                let leading_whitespace = &line[..line.len() - trimmed_start.len()];
                println!("{}// {}", leading_whitespace, trimmed_start);
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
