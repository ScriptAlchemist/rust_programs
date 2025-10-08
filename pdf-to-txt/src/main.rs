// Extract sections and page metadata from a single PDF without rewriting it.
// Outputs JSON you can pipe or persist for later processing.
//
// Build deps (Cargo.toml):
// anyhow = "1"
// lopdf = "0.38.0"
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

use anyhow::{anyhow, bail, Result};
use lopdf::{Dictionary, Document, Object, ObjectId, Stream};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;

#[derive(Debug, Serialize)]
struct PdfSummary {
    file: String,
    page_count: usize,
    sections: Vec<Section>,
}

#[derive(Debug, Serialize)]
struct Section {
    title: String,
    start_page: u32,
    end_page: u32,
    pages: Vec<PageInfo>,
}

#[derive(Debug, Serialize)]
struct PageInfo {
    page_num: u32,
    object_id: String,
    media_box: Option<[f64; 4]>,
    rotate: Option<i32>,
    content_object_ids: Vec<String>,
    fonts: BTreeSet<String>,
    xobjects: BTreeSet<String>,
    images: Vec<ImageInfo>,
}

#[derive(Debug, Serialize)]
struct ImageInfo {
    name: String,
    object_id: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    color_space: Option<String>,
    bpc: Option<i64>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pdf-introspect <file.pdf>");
        std::process::exit(1);
    }
    let path = &args[1];

    let doc = Document::load(path)?;

    // page_num -> ObjectId
    let pages = doc.get_pages();
    if pages.is_empty() {
        bail!("No pages found");
    }

    // Reverse mapping: page object -> page number
    let mut page_num_by_id: BTreeMap<ObjectId, u32> = BTreeMap::new();
    for (n, id) in pages.iter() {
        page_num_by_id.insert(*id, *n as u32);
    }

    // Sections from top-level outline; fallback to whole doc
    let sections = compute_sections(&doc, &page_num_by_id, pages.len() as u32)?;

    // Enrich each section with page metadata
    let mut out_sections: Vec<Section> = Vec::with_capacity(sections.len());
    for (title, start, end) in sections {
        let mut pages_info = Vec::new();
        for p in start..=end {
            let page_id = *pages
                .get(&(p as u32))
                .ok_or_else(|| anyhow!("Missing page {}", p))?;
            pages_info.push(introspect_page(&doc, page_id, p)?);
        }
        out_sections.push(Section {
            title,
            start_page: start,
            end_page: end,
            pages: pages_info,
        });
    }

    let summary = PdfSummary {
        file: path.to_string(),
        page_count: pages.len(),
        sections: out_sections,
    };

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn compute_sections(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    page_count: u32,
) -> Result<Vec<(String, u32, u32)>> {
    // Catalog
    let catalog_id = doc.trailer.get(b"Root")?.as_reference()?;
    let catalog = doc.get_object(catalog_id)?.as_dict()?.clone();

    // If no outlines, return single section
    let outlines_ref = match catalog.get(b"Outlines") {
        Ok(Object::Reference(r)) => *r,
        _ => return Ok(vec![("Document".into(), 1, page_count)]),
    };

    // Collect top-level outline items: (title, start_page)
    let mut top: Vec<(String, u32)> = Vec::new();
    let mut cur = first_child(doc, outlines_ref)?;
    while let Some(item_ref) = cur {
        let item = doc.get_object(item_ref)?.as_dict()?;
        let title = match item.get(b"Title") {
            Ok(Object::String(bytes, _)) => String::from_utf8_lossy(bytes).into_owned(),
            Ok(Object::Name(name)) => String::from_utf8_lossy(name.as_slice()).into_owned(),
            _ => "Untitled".into(),
        };
        if let Some(p) = outline_dest_to_page(doc, page_num_by_id, item)? {
            top.push((title, p));
        }
        cur = next_sibling(doc, item_ref)?;
    }

    if top.is_empty() {
        return Ok(vec![("Document".into(), 1, page_count)]);
    }

    top.sort_by_key(|(_, p)| *p);
    let mut sections: Vec<(String, u32, u32)> = Vec::new();
    for i in 0..top.len() {
        let (ref title, start) = top[i];
        let end = if i + 1 < top.len() {
            top[i + 1].1 - 1
        } else {
            page_count
        };
        sections.push((title.clone(), start, end));
    }
    Ok(sections)
}

fn introspect_page(doc: &Document, page_id: ObjectId, page_num: u32) -> Result<PageInfo> {
    let page_obj = doc.get_object(page_id)?;
    let dict = page_obj.as_dict()?;

    let media_box = dict.get(b"MediaBox").ok().and_then(|o| match o {
        Object::Array(a) if a.len() == 4 => {
            let mut out = [0.0f64; 4];
            for i in 0..4 {
                out[i] = num_from_obj(&a[i]);
            }
            Some(out)
        }
        _ => None,
    });

    let rotate = dict.get(b"Rotate").ok().and_then(|o| match o {
        Object::Integer(i) => Some(*i as i32),
        _ => None,
    });

    // Content streams
    let mut content_ids: Vec<String> = Vec::new();
    match dict.get(b"Contents") {
        Ok(Object::Reference(id)) => content_ids.push(fmt_id(*id)),
        Ok(Object::Array(arr)) => {
            for o in arr {
                if let Object::Reference(id) = o {
                    content_ids.push(fmt_id(*id));
                }
            }
        }
        _ => {}
    }

    // Resources: Fonts, XObjects, Images
    let mut fonts = BTreeSet::new();
    let mut xobjects = BTreeSet::new();
    let mut images: Vec<ImageInfo> = Vec::new();

    if let Ok(Object::Dictionary(res)) = dict.get(b"Resources") {
        if let Ok(Object::Dictionary(fonts_dict)) = res.get(b"Font") {
            for (name, _) in fonts_dict.iter() {
                fonts.insert(bytes_to_name(name));
            }
        }
        if let Ok(Object::Dictionary(xobj_dict)) = res.get(b"XObject") {
            for (name, obj) in xobj_dict.iter() {
                xobjects.insert(bytes_to_name(name));
                if let Object::Reference(xid) = obj {
                    if let Ok(Object::Stream(stream)) = doc.get_object(*xid) {
                        if let Some(info) =
                            image_info_from_xobject(&bytes_to_name(name), xid, stream)
                        {
                            images.push(info);
                        }
                    }
                }
            }
        }
    }

    Ok(PageInfo {
        page_num,
        object_id: fmt_id(page_id),
        media_box,
        rotate,
        content_object_ids: content_ids,
        fonts,
        xobjects,
        images,
    })
}

fn image_info_from_xobject(name: &str, id: &ObjectId, stream: &Stream) -> Option<ImageInfo> {
    if let Ok(Object::Name(ref subtype)) = stream.dict.get(b"Subtype") {
        if subtype.as_slice() != b"Image" {
            return None;
        }
        let w = stream.dict.get(b"Width").ok().and_then(|o| o.as_i64().ok());
        let h = stream
            .dict
            .get(b"Height")
            .ok()
            .and_then(|o| o.as_i64().ok());
        let cs = stream.dict.get(b"ColorSpace").ok().and_then(|o| match o {
            Object::Name(n) => Some(bytes_to_name(n)),
            Object::Array(a) if !a.is_empty() => match &a[0] {
                Object::Name(n) => Some(bytes_to_name(n)),
                _ => None,
            },
            _ => None,
        });
        let bpc = stream
            .dict
            .get(b"BitsPerComponent")
            .ok()
            .and_then(|o| o.as_i64().ok());
        return Some(ImageInfo {
            name: name.into(),
            object_id: Some(fmt_id(*id)),
            width: w,
            height: h,
            color_space: cs,
            bpc,
        });
    }
    None
}

// ---- Outline helpers ----
fn first_child(doc: &Document, node_ref: ObjectId) -> Result<Option<ObjectId>> {
    let node = doc.get_object(node_ref)?.as_dict()?;
    Ok(match node.get(b"First") {
        Ok(Object::Reference(r)) => Some(*r),
        _ => None,
    })
}
fn next_sibling(doc: &Document, node_ref: ObjectId) -> Result<Option<ObjectId>> {
    let node = doc.get_object(node_ref)?.as_dict()?;
    Ok(match node.get(b"Next") {
        Ok(Object::Reference(r)) => Some(*r),
        _ => None,
    })
}

fn outline_dest_to_page(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    item: &Dictionary,
) -> Result<Option<u32>> {
    if let Ok(dest) = item.get(b"Dest") {
        if let Some(p) = dest_to_page(doc, page_num_by_id, dest)? {
            return Ok(Some(p));
        }
    }
    if let Ok(Object::Dictionary(a)) = item.get(b"A") {
        if let Ok(d) = a.get(b"D") {
            if let Some(p) = dest_to_page(doc, page_num_by_id, d)? {
                return Ok(Some(p));
            }
        }
        if let Ok(d) = a.get(b"Dest") {
            if let Some(p) = dest_to_page(doc, page_num_by_id, d)? {
                return Ok(Some(p));
            }
        }
    }
    Ok(None)
}

fn dest_to_page(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    dest: &Object,
) -> Result<Option<u32>> {
    match dest {
        Object::Array(arr) if !arr.is_empty() => {
            if let Object::Reference(page_ref) = arr[0] {
                return Ok(page_num_by_id.get(&page_ref).copied());
            }
            Ok(None)
        }
        Object::Reference(r) => Ok(page_num_by_id.get(r).copied()),
        Object::Name(_) | Object::String(_, _) => resolve_named_dest(doc, page_num_by_id, dest),
        _ => Ok(None),
    }
}

fn resolve_named_dest(
    doc: &Document,
    page_num_by_id: &BTreeMap<ObjectId, u32>,
    name_obj: &Object,
) -> Result<Option<u32>> {
    let catalog_id = match doc.trailer.get(b"Root").and_then(|o| o.as_reference()) {
        Ok(id) => id,
        Err(_) => return Ok(None),
    };
    let cat = doc.get_object(catalog_id)?.as_dict()?;
    let dests = if let Ok(Object::Dictionary(d)) = cat.get(b"Dests") {
        d
    } else {
        return Ok(None);
    };
    let key = match name_obj {
        Object::Name(n) => n.clone(),
        Object::String(bytes, _) => bytes.clone(),
        _ => return Ok(None),
    };
    if let Ok(v) = dests.get(&key) {
        return dest_to_page(doc, page_num_by_id, v);
    }
    Ok(None)
}

// ---- util ----
fn fmt_id(id: ObjectId) -> String {
    format!("{} {} R", id.0, id.1)
}
fn bytes_to_name(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}
fn num_from_obj(o: &Object) -> f64 {
    match o {
        Object::Integer(i) => *i as f64,
        Object::Real(r) => *r as f64,
        _ => 0.0,
    }
}
