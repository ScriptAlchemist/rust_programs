use anyhow::Result;
use lopdf::Document;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} INPUT.pdf [OUTPUT.txt]", args[0]);
        std::process::exit(1);
    }
    let inpdf = &args[1];
    let out_path = if args.len() > 2 {
        args[2].clone()
    } else {
        "gsOut.txt".to_string()
    };

    let doc = Document::load(inpdf)?;
    let pages = doc.get_pages();

    // Extract text from all pages
    let page_numbers: Vec<u32> = pages.keys().cloned().collect();
    let all_text = doc.extract_text(&page_numbers)?;

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

    // Handle multi-character replacements first
    let text = text.replace("…", "...");

    // Single-character replacements and filtering
    let processed_text: String = text
        .chars()
        .map(|c| replacements.get(&c).cloned().unwrap_or(c))
        // Filter for printable ASCII and basic whitespace, similar to the script
        .filter(|&c| (c.is_ascii_graphic() || c.is_ascii_whitespace()) && c != '\u{000C}')
        .collect();

    // Replicate the tokenization logic: split by whitespace and join with newlines
    let tokens: Vec<&str> = processed_text.split_whitespace().collect();
    tokens.join("\n")
}
