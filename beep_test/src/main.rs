use std::io::{Error, Write};

#[cfg(target_os = "windows")]
fn beep() -> Result<(), Error> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "\x07")?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn beep() -> Result<(), Error> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "\u{0007}")?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn beep() -> Result<(), Error> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "\u{0007}")?;
    Ok(())
}

fn main() -> Result<(), Error> {
    beep()
}
