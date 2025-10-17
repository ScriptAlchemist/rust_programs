use anyhow::Result;
use lopdf::Document;
use std::fs;

mod cli;
mod format;

fn main() -> Result<()> {
    let args = cli::parse()?;

    let doc = Document::load(&args.input)?;
    let pages = doc.get_pages();
    let page_numbers: Vec<u32> = pages.keys().cloned().collect();

    let start_page = args.start.unwrap_or(1);
    let end_page = args
        .end
        .unwrap_or(*page_numbers.last().unwrap_or(&u32::MAX));

    println!("Starting extraction from page {}", start_page);
    if let Some(end) = args.end {
        println!("Ending extraction at page {}", end);
    }

    let page_numbers_to_extract: Vec<u32> = page_numbers
        .into_iter()
        .filter(|&p| p >= start_page && p <= end_page)
        .collect();

    if page_numbers_to_extract.is_empty() {
        println!("No pages to extract.");
        return Ok(());
    }

    let all_text = doc.extract_text(&page_numbers_to_extract)?;
    let cleaned_text = format::clean_text(&all_text);

    fs::write(&args.output, cleaned_text)?;
    println!("Wrote {}", args.output.to_string_lossy());
    Ok(())
}
