use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let trimmed = line.trim_start();
                let leading_whitespace = &line[..line.len() - trimmed.len()];
                if trimmed.starts_with("//") {
                    let without_comment = &trimmed[2..];
                    println!("{}{}", leading_whitespace, without_comment.trim_start());
                } else {
                    println!("{}{}", leading_whitespace, trimmed);
                }
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
