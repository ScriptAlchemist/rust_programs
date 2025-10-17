use anyhow::Result;
use lopdf::Document;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut start_page_arg: Option<u32> = None;
    let mut end_page_arg: Option<u32> = None;
    let mut file_args: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--start" => {
                i += 1;
                if i < args.len() {
                    start_page_arg = args[i].parse().ok();
                }
            }
            "--end" => {
                i += 1;
                if i < args.len() {
                    end_page_arg = args[i].parse().ok();
                }
            }
            _ => {
                if !args[i].starts_with('-') {
                    file_args.push(args[i].clone());
                }
            }
        }
        i += 1;
    }

    if file_args.is_empty() {
        eprintln!(
            "Usage: {} [--start <page>] [--end <page>] INPUT.pdf [OUTPUT.txt]",
            args[0]
        );
        std::process::exit(1);
    }

    let inpdf = &file_args[0];
    let out_path = if file_args.len() > 1 {
        file_args[1].clone()
    } else {
        "gsOut.txt".to_string()
    };

    let doc = Document::load(inpdf)?;
    let pages = doc.get_pages();
    let mut sorted_pages: Vec<u32> = pages.keys().cloned().collect();
    sorted_pages.sort();

    let start_page = start_page_arg.unwrap_or_else(|| 1);
    let end_page = end_page_arg.unwrap_or(*sorted_pages.last().unwrap());

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
