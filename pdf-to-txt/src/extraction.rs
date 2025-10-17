use anyhow::{Context, Result};
use lopdf::Document;

use crate::cli::ParsedArgs;

pub fn extract(args: &ParsedArgs) -> Result<String> {
    let doc = Document::load(&args.input)?;

    let pages = doc.get_pages();
    let mut sorted_pages: Vec<u32> = pages.keys().cloned().collect();
    sorted_pages.sort();

    let start_page = args.start.unwrap_or(1);
    let end_page = args
        .end
        .unwrap_or(*sorted_pages.last().unwrap_or(&u32::MAX));

    println!("Starting extraction from page {}", start_page);
    if let Some(end) = args.end {
        println!("Ending extraction at page {}", end);
    }

    let page_numbers_to_extract: Vec<u32> = sorted_pages
        .into_iter()
        .filter(|&p| p >= start_page && p <= end_page)
        .collect();

    if page_numbers_to_extract.is_empty() {
        println!("No pages to extract.");
        return Ok(String::new());
    }

    let text = doc
        .extract_text(&page_numbers_to_extract)
        .context("Failed to extract text from PDF")?;

    Ok(text)
}
