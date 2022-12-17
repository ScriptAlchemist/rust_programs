use std::io;
use std::io::BufRead;
use std::io::Write;

fn main() -> io::Result<()> {
    // Open the /tmp/buf file in write mode
    let mut buf = std::fs::File::create("/tmp/buf")?;

    // Read lines from standard input
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        // Write the line to the /tmp/buf file
        writeln!(buf, "{}", line)?;

        // Print the line to the terminal
        println!("{}", line);
    }

    Ok(())
}

