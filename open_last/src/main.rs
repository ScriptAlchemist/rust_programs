use std::fs;
use std::process::Command;
use std::time::SystemTime;

#[cfg(windows)]
use std::os::windows::prelude::*;

fn main() -> std::io::Result<()> {

    let dir = ".";

    let entry = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .filter_map(|entry| fs::metadata(&entry.path()).map(|meta| (entry.path(), meta)).ok())
        .max_by_key(|(_, meta)| get_last_modified_time(meta.clone()).unwrap_or(SystemTime::UNIX_EPOCH))
        .map(|(path, _)| path);

    if let Some(newest_file_path) = entry {
        println!("File path: {:?}", newest_file_path);

        #[cfg(windows)]
        let mut child = Command::new("vim.bat")
            .arg(newest_file_path)
            .spawn()
            .expect("Failed to open file in Vim");

        #[cfg(unix)]
        let mut child = Command::new("nvim")
            .arg(newest_file_path)
            .spawn()
            .expect("Failed to open file in Vim");

        let _ = child.wait();
    } else {
        println!("No files found in directory {}", dir);
    }
    Ok(())
}

#[cfg(unix)]
fn get_last_modified_time(meta: fs::Metadata) -> std::io::Result<SystemTime> {
    meta.modified()
}

#[cfg(windows)]
fn get_last_modified_time(meta: fs::Metadata) -> std::io::Result<SystemTime> {
    Ok(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs((meta.last_write_time() / 10_000_000) as u64))
}
