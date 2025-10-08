use lopdf::{Bookmark, Document, Object, ObjectId};
use std::collections::BTreeMap;

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pdf-to-txt <file.pdf>");
        std::process::exit(1);
    }
    let path = &args[1];

    let mut doc = Document::load(path)?;
    // Define a starting `max_id` (will be used as start index for object_ids).
    let max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();

    let mut first = false;
    doc.renumber_objects_with(max_id);

    documents_pages.extend(
        doc.get_pages()
            .into_iter()
            .map(|(_, object_id)| {
                if !first {
                    let _bookmark = Bookmark::new(
                        String::from(format!("Page_{}", pagenum)),
                        [0.0, 0.0, 1.0],
                        0,
                        object_id,
                    );
                    first = true;
                    pagenum += 1;
                }

                (object_id, doc.get_object(object_id).unwrap().to_owned())
            })
            .collect::<BTreeMap<ObjectId, Object>>(),
    );
    documents_objects.extend(doc.objects);

    // "Catalog" and "Pages" are mandatory.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
        // All other objects should be collected and inserted into the main Document.
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages".
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {}
        }
    }

    // If no "Pages" object found, abort.
    if pages_object.is_none() {
        println!("Pages root not found.");

        return Ok(());
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (_object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);
        }
    }

    // If no "Catalog" found, abort.
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return Ok(());
    }

    let _catalog_object = catalog_object.unwrap();
    let _pages_object = pages_object.unwrap();

    Ok(())
}
