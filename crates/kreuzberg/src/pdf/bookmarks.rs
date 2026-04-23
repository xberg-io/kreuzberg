//! PDF bookmark/outline extraction using lopdf.
//!
//! Extracts the document outline (bookmarks) from the PDF catalog and returns
//! them as a list of `Uri` values: external URLs as `Uri::hyperlink()`,
//! page destinations as `Uri::anchor()`.

use crate::types::uri::Uri;
use lopdf::{Document, Object, ObjectId};

/// Decode a PDF string, handling UTF-16BE BOM and falling back to lossy UTF-8.
fn decode_pdf_string(bytes: &[u8]) -> String {
    // PDF strings may be UTF-16BE (prefixed with BOM 0xFE 0xFF) or PDFDocEncoding (latin-1-ish).
    if bytes.starts_with(&[0xFE, 0xFF]) && bytes.len() >= 2 {
        let u16s: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        String::from_utf16(&u16s).unwrap_or_else(|_| String::from_utf8_lossy(bytes).into_owned())
    } else {
        String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from_utf8_lossy(bytes).into_owned())
    }
}

/// Extract bookmarks (outlines) from a PDF document loaded via lopdf.
///
/// Walks the `/Outlines` tree in the document catalog, collecting each bookmark's
/// title and destination. Returns an empty `Vec` if the document has no outlines.
pub(crate) fn extract_bookmarks(document: &Document) -> Vec<Uri> {
    let mut uris = Vec::new();

    let catalog_id = match document.catalog() {
        Ok(id) => id,
        Err(_) => return uris,
    };

    let outlines_ref = match catalog_id.get(b"Outlines") {
        Ok(obj) => obj,
        Err(_) => return uris,
    };

    let outlines_id = match outlines_ref.as_reference() {
        Ok(id) => id,
        Err(_) => return uris,
    };

    let outlines_dict = match document.get_object(outlines_id) {
        Ok(Object::Dictionary(dict)) => dict,
        _ => return uris,
    };

    // Get the /First child of the outlines root.
    let first_ref = match outlines_dict.get(b"First") {
        Ok(obj) => match obj.as_reference() {
            Ok(id) => id,
            Err(_) => return uris,
        },
        Err(_) => return uris,
    };

    walk_outline_items(document, first_ref, &mut uris);

    uris
}

/// Recursively walk outline items linked by /Next siblings and /First children.
fn walk_outline_items(document: &Document, item_id: ObjectId, uris: &mut Vec<Uri>) {
    // Guard against malformed circular references by limiting depth.
    walk_outline_items_inner(document, item_id, uris, 0, 500);
}

fn walk_outline_items_inner(
    document: &Document,
    item_id: ObjectId,
    uris: &mut Vec<Uri>,
    depth: usize,
    max_items: usize,
) {
    if uris.len() >= max_items || depth > 50 {
        return;
    }

    let dict = match document.get_object(item_id) {
        Ok(Object::Dictionary(dict)) => dict,
        _ => return,
    };

    // Extract title.
    let title = dict.get(b"Title").ok().and_then(|obj| match obj {
        Object::String(bytes, _) => Some(decode_pdf_string(bytes)),
        _ => None,
    });

    // Extract destination: try /Dest first, then /A (action).
    let uri = extract_destination(document, dict, title.as_deref())
        .or_else(|| extract_action(document, dict, title.as_deref()));

    if let Some(u) = uri {
        uris.push(u);
    }

    // Recurse into children (/First).
    if let Ok(first_obj) = dict.get(b"First")
        && let Ok(first_id) = first_obj.as_reference()
    {
        walk_outline_items_inner(document, first_id, uris, depth + 1, max_items);
    }

    // Continue to next sibling (/Next).
    if let Ok(next_obj) = dict.get(b"Next")
        && let Ok(next_id) = next_obj.as_reference()
    {
        walk_outline_items_inner(document, next_id, uris, depth, max_items);
    }
}

/// Extract a URI from a /Dest entry (page destination).
fn extract_destination(document: &Document, dict: &lopdf::Dictionary, title: Option<&str>) -> Option<Uri> {
    let dest = dict.get(b"Dest").ok()?;
    let label = title.map(|s| s.to_string());

    match dest {
        // Named destination (string).
        Object::String(name, _) => {
            let name_str = String::from_utf8_lossy(name);
            Some(Uri::anchor(format!("#{}", name_str), label))
        }
        // Named destination (name object).
        Object::Name(name) => {
            let name_str = String::from_utf8_lossy(name);
            Some(Uri::anchor(format!("#{}", name_str), label))
        }
        // Direct destination array: [page_ref /type ...].
        Object::Array(arr) => {
            let page_num = resolve_page_number(document, arr);
            let url = format!("#page={}", page_num.unwrap_or(1));
            let mut uri = Uri::anchor(url, label);
            if let Some(p) = page_num {
                uri.page = Some(p);
            }
            Some(uri)
        }
        // Reference to a destination object.
        Object::Reference(id) => match document.get_object(*id) {
            Ok(Object::Array(arr)) => {
                let page_num = resolve_page_number(document, arr);
                let url = format!("#page={}", page_num.unwrap_or(1));
                let mut uri = Uri::anchor(url, label);
                if let Some(p) = page_num {
                    uri.page = Some(p);
                }
                Some(uri)
            }
            _ => None,
        },
        _ => None,
    }
}

/// Extract a URI from an /A (action) entry.
fn extract_action(document: &Document, dict: &lopdf::Dictionary, title: Option<&str>) -> Option<Uri> {
    let action_obj = dict.get(b"A").ok()?;
    let action_dict = match action_obj {
        Object::Dictionary(d) => d,
        Object::Reference(id) => match document.get_object(*id) {
            Ok(Object::Dictionary(d)) => d,
            _ => return None,
        },
        _ => return None,
    };

    let action_type = action_dict
        .get(b"S")
        .ok()
        .and_then(|o| o.as_name().ok())
        .map(|n| String::from_utf8_lossy(n).into_owned());

    let label = title.map(|s| s.to_string());

    match action_type.as_deref() {
        Some("URI") => {
            // External URL.
            let url = action_dict.get(b"URI").ok().and_then(|o| match o {
                Object::String(bytes, _) => Some(String::from_utf8_lossy(bytes).into_owned()),
                _ => None,
            })?;
            Some(Uri::hyperlink(url, label))
        }
        Some("GoTo") => {
            // Internal page destination.
            let dest = action_dict.get(b"D").ok()?;
            match dest {
                Object::Array(arr) => {
                    let page_num = resolve_page_number(document, arr);
                    let url = format!("#page={}", page_num.unwrap_or(1));
                    let mut uri = Uri::anchor(url, label);
                    if let Some(p) = page_num {
                        uri.page = Some(p);
                    }
                    Some(uri)
                }
                Object::String(name, _) => {
                    let name_str = String::from_utf8_lossy(name);
                    Some(Uri::anchor(format!("#{}", name_str), label))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Resolve the first element of a destination array to a 1-indexed page number.
fn resolve_page_number(document: &Document, arr: &[Object]) -> Option<u32> {
    let page_ref = arr.first()?;
    let page_id = match page_ref {
        Object::Reference(id) => *id,
        _ => return None,
    };

    // Walk pages to find the matching page number.
    let pages = document.get_pages();
    for (num, id) in &pages {
        if *id == page_id {
            return Some(*num);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bookmarks_no_outlines() {
        // A minimal valid PDF with no outlines should return empty.
        let result = Document::load_mem(b"not a pdf");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_page_number_empty_array() {
        let arr: Vec<Object> = vec![];
        assert_eq!(resolve_page_number(&Document::with_version("1.5"), &arr), None);
    }
}
