#[allow(dead_code)]
pub fn runner() {
    for code in 30..=37 {
        println!("Text color {code}: {}this text is colored.{}",
            format!("\x1b[{}m", code),
            "\x1b[0m",
            code=code);
    }

    for code in 40..=47 {
        println!("Background color {code}: {}             {}",
            format!("\x1b[{}m", code),
            "\x1b[0m",
            code=code);
    }
}
