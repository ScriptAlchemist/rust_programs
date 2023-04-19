use clipboard_win::{formats,get_clipboard};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    match get_clipboard::<String, formats::Unicode>(formats::Unicode) {
        Ok(data) => {
            // Use the clipboard data here
            let mut file = File::create(r"\\wsl$\Ubuntu\home\bender\tmp\clipboard.txt")?;

            file.write_all(data.as_bytes())?;
            println!("{}", data);
        },
        Err(e) => {
            eprint!("Failed to get clipboard data: {}", e);
            return Ok(());
        }
    };
    Ok(())
}

