use anyhow::anyhow;
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub input_file: String,
    pub output_file: Option<String>,
    #[arg(short, long)]
    pub start: Option<u32>,
    #[arg(short, long)]
    pub end: Option<u32>,
}

pub struct ParsedArgs {
    pub input: std::path::PathBuf,
    pub output: std::path::PathBuf,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

pub fn parse() -> Result<ParsedArgs, anyhow::Error> {
    let input = Cli::parse();
    let in_path = Path::new(&input.input_file);
    if in_path.extension().and_then(std::ffi::OsStr::to_str) != Some("pdf") {
        return Err(anyhow!("Input file must be a PDF with a .pdf extension."));
    }

    let inpdf = std::path::PathBuf::from(&input.input_file);
    let out_path = input
        .output_file
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| in_path.with_extension("txt"));
    let start_page_arg = input.start;
    let end_page_arg = input.end;

    Ok(ParsedArgs {
        input: inpdf,
        output: out_path,
        start: start_page_arg,
        end: end_page_arg,
    })
}
