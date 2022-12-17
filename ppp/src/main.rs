use std::io;

fn main() -> io::Result<()> {
    // Read the contents of the /tmp/buf file
    let buf = std::fs::read_to_string("/tmp/buf")?;

    // Print the contents of the file
    println!("{}", buf);

    Ok(())
}

