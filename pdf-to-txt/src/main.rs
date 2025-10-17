use anyhow::Result;
use std::fs;

mod cli;
mod extraction;
mod format;

fn main() -> Result<()> {
    let args = cli::parse()?;
    let all_text = extraction::extract(&args)?;
    let cleaned_text = format::clean_text(&all_text);
    fs::write(&args.output, cleaned_text)?;
    println!("Wrote {}", args.output.to_string_lossy());
    Ok(())
}
