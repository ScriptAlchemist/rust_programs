use anyhow::Result;
use lopdf::Document;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pdf-to-txt <file.pdf>");
        std::process::exit(1);
    }
    let path = &args[1];

    let doc = Document::load(path)?;

    let pages = doc.get_pages();
    let mut text = String::new();

    for (i, _) in pages.iter() {
        let page_number = *i;
        let page_text = doc.extract_text(&[page_number]);
        if let Ok(page_text) = page_text {
            text.push_str(&page_text);
        }
    }

    if text.trim().is_empty() {
        eprintln!("PDF loaded but no text could be extracted");
        eprintln!("This PDF may contain images, complex formatting, or encrypted text");
        std::process::exit(1);
    }
    print!("{}", text);

    Ok(())
}
