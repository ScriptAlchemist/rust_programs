use anyhow::Result;
use clap::Parser;
use lopdf::Document;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The PDF file to extract text from
    input_file: String,

    /// The file to write the extracted text to
    output_file: Option<String>,

    /// The page to start extraction from
    #[arg(short, long)]
    start: Option<u32>,

    /// The page to end extraction at
    #[arg(short, long)]
    end: Option<u32>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let inpdf = &cli.input_file;
    let out_path = cli.output_file.unwrap_or_else(|| "out.txt".to_string());
    let start_page_arg = cli.start;
    let end_page_arg = cli.end;

    let doc = Document::load(inpdf)?;
    let pages = doc.get_pages();
    let mut sorted_pages: Vec<u32> = pages.keys().cloned().collect();
    sorted_pages.sort();

    let start_page = start_page_arg.unwrap_or(1);
    let end_page = end_page_arg.unwrap_or(*sorted_pages.last().unwrap_or(&u32::MAX));

    println!("Starting extraction from page {}", start_page);
    if let Some(end) = end_page_arg {
        println!("Ending extraction at page {}", end);
    }

    let page_numbers_to_extract: Vec<u32> = sorted_pages
        .into_iter()
        .filter(|&p| p >= start_page && p <= end_page)
        .collect();

    if page_numbers_to_extract.is_empty() {
        println!("No pages to extract.");
        return Ok(());
    }

    let all_text = doc.extract_text(&page_numbers_to_extract)?;
    let cleaned_text = clean_text(&all_text);

    fs::write(&out_path, cleaned_text)?;
    println!("Wrote {}", out_path);

    Ok(())
}

fn clean_text(text: &str) -> String {
    let mut replacements = HashMap::new();
    replacements.insert('‘', '\'');
    replacements.insert('’', '\'');
    replacements.insert('‚', '\'');
    replacements.insert('‛', '\'');
    replacements.insert('ʼ', '\'');
    replacements.insert('ʹ', '\'');
    replacements.insert('ʻ', '\'');
    replacements.insert('“', '"');
    replacements.insert('”', '"');
    replacements.insert('„', '"');
    replacements.insert('‟', '"');
    replacements.insert('ʺ', '"');
    replacements.insert('«', '"');
    replacements.insert('»', '"');
    replacements.insert('–', '-');
    replacements.insert('—', '-');
    replacements.insert('―', '-');
    replacements.insert('﹣', '-');
    replacements.insert('－', '-');
    replacements.insert('∕', '/');
    replacements.insert('／', '/');
    replacements.insert('⧸', '/');
    replacements.insert('＼', '\\');

    let text = text.replace("…", "...");

    let processed_text: String = text
        .chars()
        .map(|c| replacements.get(&c).cloned().unwrap_or(c))
        .filter(|&c| (c.is_ascii_graphic() || c.is_ascii_whitespace()) && c != '\u{000C}')
        .collect();

    let tokens: Vec<&str> = processed_text.split_whitespace().collect();
    tokens.join("\n")
}
