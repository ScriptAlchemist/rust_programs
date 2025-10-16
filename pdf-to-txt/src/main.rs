use anyhow::{anyhow, Result};
use lopdf::{Dictionary, Document, Object, ObjectId};
use std::collections::{BTreeMap, HashMap};
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
    let mut sorted_pages: Vec<u32> = pages.keys().cloned().collect();
    sorted_pages.sort();

    let start_page = find_start_page(&doc).unwrap_or(1);
    println!("Starting extraction from page {}", start_page);

    let page_numbers_to_extract: Vec<u32> = sorted_pages
        .into_iter()
        .filter(|&p| p >= start_page)
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

fn find_start_page(doc: &Document) -> Result<u32> {
    let page_map = doc.get_pages();
    let mut page_num_by_id: BTreeMap<ObjectId, u32> = BTreeMap::new();
    for (page_num, page_id) in &page_map {
        page_num_by_id.insert(*page_id, *page_num);
    }

    let catalog = doc.catalog()?;
    if let Ok(outlines) = catalog.get(b"Outlines") {
        let outlines_ref = outlines.as_reference()?;
        let outlines_dict = doc.get_object(outlines_ref)?.as_dict()?;

        if let Ok(first_ref) = outlines_dict.get(b"First") {
            let first_ref = first_ref.as_reference()?;
            let first_item_dict = doc.get_object(first_ref)?.as_dict()?;

            if let Some(page_num) = dest_to_page(doc, &page_num_by_id, first_item_dict)? {
                return Ok(page_num);
            }
        }
    }

    Err(anyhow!("No outline found"))
}

fn dest_to_page(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    item: &Dictionary,
) -> Result<Option<u32>> {
    if let Ok(dest) = item.get(b"Dest") {
        return resolve_dest(doc, page_num_by_id, dest);
    }
    if let Ok(Object::Dictionary(a)) = item.get(b"A") {
        if let Ok(d) = a.get(b"D") {
            return resolve_dest(doc, page_num_by_id, d);
        }
    }
    Ok(None)
}

fn resolve_dest(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    dest: &Object,
) -> Result<Option<u32>> {
    match dest {
        Object::Array(arr) => {
            if let Some(Object::Reference(page_ref)) = arr.get(0) {
                return Ok(page_num_by_id.get(page_ref).copied());
            }
        }
        Object::Reference(r) => {
            return Ok(page_num_by_id.get(r).copied());
        }
        Object::Name(name) => {
            return resolve_named_dest(doc, page_num_by_id, name);
        }
        Object::String(bytes, _) => {
            return resolve_named_dest(doc, page_num_by_id, bytes);
        }
        _ => {}
    }
    Ok(None)
}

fn resolve_named_dest(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    name: &[u8],
) -> Result<Option<u32>> {
    let catalog = doc.catalog()?;
    if let Ok(Object::Dictionary(dests)) = catalog.get(b"Dests") {
        if let Ok(dest) = dests.get(name) {
            return resolve_dest(doc, page_num_by_id, dest);
        }
    }
    Ok(None)
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
