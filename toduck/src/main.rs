use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::env;

fn toduck(file_path: &str) -> io::Result<()> {
    let duck = "https://lite.duckduckgo.com/lite?kd=-1&kp=-1&q=";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut output = String::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(capture) = line.find("[") {
            let end = line[capture..].find("]()").unwrap() + capture;
            let text = &line[capture + 1..end];
            let url_text = text.replace(" ", "%20");
            output.push_str(&format!("{}[{}]({}{})", &line[..capture], text, duck, url_text));
        } else {
            output.push_str(&line);
        }
        output.push_str("\n");
    }

    let mut file = File::create(file_path)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        if let Err(err) = toduck(&file_path) {
            eprintln!("Error: {}", err);
        }
    } else {
        eprintln!("Usage: toduck <file>");
    }
}

