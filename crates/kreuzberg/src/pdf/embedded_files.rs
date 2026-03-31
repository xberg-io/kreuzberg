//! PDF embedded file (portfolio/attachment) extraction using lopdf.
//!
//! PDFs can contain file attachments via the `/Names` → `/EmbeddedFiles` name tree
//! in the document catalog. This module extracts those files, detects their MIME
//! type, and returns them as `ArchiveEntry` values for recursive processing.

use crate::types::{ArchiveEntry, ProcessingWarning};
use lopdf::{Document, Object};
use std::borrow::Cow;

/// Embedded file descriptor extracted from the PDF name tree.
pub struct EmbeddedFile {
    /// The filename as stored in the PDF name tree.
    pub name: String,
    /// Raw file bytes from the embedded stream.
    pub data: Vec<u8>,
    /// MIME type if specified in the filespec, otherwise `None`.
    pub mime_type: Option<String>,
}

/// Extract embedded file descriptors from a PDF document loaded via lopdf.
///
/// Walks the `/Names` → `/EmbeddedFiles` name tree in the catalog.
/// Returns an empty `Vec` if the document has no embedded files.
pub fn extract_embedded_files(document: &Document) -> Vec<EmbeddedFile> {
    let mut files = Vec::new();

    let catalog = match document.catalog() {
        Ok(cat) => cat,
        Err(_) => return files,
    };

    // Get /Names dictionary.
    let names_obj = match catalog.get(b"Names") {
        Ok(obj) => resolve_object(document, obj),
        Err(_) => return files,
    };

    let names_dict = match names_obj {
        Some(Object::Dictionary(dict)) => dict,
        _ => return files,
    };

    // Get /EmbeddedFiles from /Names.
    let ef_obj = match names_dict.get(b"EmbeddedFiles") {
        Ok(obj) => resolve_object(document, obj),
        Err(_) => return files,
    };

    let ef_dict = match ef_obj {
        Some(Object::Dictionary(dict)) => dict,
        _ => return files,
    };

    // The name tree can have /Names (leaf) or /Kids (intermediate nodes).
    collect_from_name_tree(document, &ef_dict, &mut files);

    files
}

/// Recursively collect embedded files from a PDF name tree node.
fn collect_from_name_tree(document: &Document, dict: &lopdf::Dictionary, files: &mut Vec<EmbeddedFile>) {
    // Leaf node: /Names array with alternating [name filespec name filespec ...].
    if let Ok(Object::Array(names_arr)) = dict.get(b"Names") {
        let mut i = 0;
        while i + 1 < names_arr.len() {
            let name = match &names_arr[i] {
                Object::String(bytes, _) => String::from_utf8_lossy(bytes).into_owned(),
                _ => {
                    i += 2;
                    continue;
                }
            };

            let filespec = resolve_object(document, &names_arr[i + 1]);
            if let Some(Object::Dictionary(fs_dict)) = filespec
                && let Some(ef) = extract_file_from_filespec(document, &name, &fs_dict)
            {
                files.push(ef);
            }

            i += 2;
        }
    }

    // Intermediate node: /Kids array of child name tree nodes.
    if let Ok(Object::Array(kids)) = dict.get(b"Kids") {
        for kid in kids {
            let kid_obj = resolve_object(document, kid);
            if let Some(Object::Dictionary(kid_dict)) = kid_obj {
                collect_from_name_tree(document, &kid_dict, files);
            }
        }
    }
}

/// Extract an embedded file from a filespec dictionary.
///
/// The filespec has:
/// - `/UF` or `/F`: display filename
/// - `/EF` → `/F`: reference to the embedded file stream
/// - `/AFRelationship`: optional relationship type
fn extract_file_from_filespec(
    document: &Document,
    tree_name: &str,
    fs_dict: &lopdf::Dictionary,
) -> Option<EmbeddedFile> {
    // Determine the display filename: prefer /UF (Unicode), then /F, then the tree name.
    let display_name = fs_dict
        .get(b"UF")
        .or_else(|_| fs_dict.get(b"F"))
        .ok()
        .and_then(|obj| match obj {
            Object::String(bytes, _) => Some(String::from_utf8_lossy(bytes).into_owned()),
            _ => None,
        })
        .unwrap_or_else(|| tree_name.to_string());

    // Get /EF (embedded file dictionary).
    let ef_obj = resolve_object(document, fs_dict.get(b"EF").ok()?)?;
    let ef_dict = match ef_obj {
        Object::Dictionary(d) => d,
        _ => return None,
    };

    // Get /F stream reference from /EF.
    let stream_obj = ef_dict.get(b"F").or_else(|_| ef_dict.get(b"UF")).ok()?;
    let stream_id = stream_obj.as_reference().ok()?;

    let stream = match document.get_object(stream_id) {
        Ok(Object::Stream(s)) => s,
        _ => return None,
    };

    // Try to decompress. lopdf's `get_decompressed_content` returns decoded bytes.
    let data = stream.decompressed_content().unwrap_or_else(|_| stream.content.clone());

    // Try to get MIME type from the stream dictionary's /Subtype.
    let mime_type = stream
        .dict
        .get(b"Subtype")
        .ok()
        .and_then(|obj| obj.as_name().ok())
        .map(|name| String::from_utf8_lossy(name).into_owned())
        .or_else(|| {
            // Detect from filename extension.
            std::path::Path::new(&display_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| mime_guess::from_ext(ext).first())
                .map(|m| m.to_string())
        });

    Some(EmbeddedFile {
        name: display_name,
        data,
        mime_type,
    })
}

/// Resolve a PDF object through references.
fn resolve_object<'a>(document: &'a Document, obj: &'a Object) -> Option<Object> {
    match obj {
        Object::Reference(id) => document.get_object(*id).ok().cloned(),
        other => Some(other.clone()),
    }
}

/// Extract embedded files from PDF bytes and recursively process them.
///
/// Returns `(children, warnings)`. The children are `ArchiveEntry` values
/// suitable for attaching to `InternalDocument.children`.
pub async fn extract_and_process_embedded_files(
    pdf_bytes: &[u8],
    config: &crate::core::config::ExtractionConfig,
) -> (Vec<ArchiveEntry>, Vec<ProcessingWarning>) {
    let mut children = Vec::new();
    let mut warnings = Vec::new();

    let document = match Document::load_mem(pdf_bytes) {
        Ok(doc) => doc,
        Err(_) => return (children, warnings),
    };

    let embedded = extract_embedded_files(&document);
    if embedded.is_empty() {
        return (children, warnings);
    }

    // Don't recurse if we've exhausted archive depth.
    if config.max_archive_depth == 0 {
        return (children, warnings);
    }

    let mut child_config = config.clone();
    child_config.max_archive_depth = config.max_archive_depth.saturating_sub(1);

    for file in embedded {
        let mime = file.mime_type.unwrap_or_else(|| {
            // Detect from filename extension.
            std::path::Path::new(&file.name)
                .extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| mime_guess::from_ext(ext).first())
                .map(|m| m.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string())
        });

        if mime == "application/octet-stream" {
            continue;
        }

        match crate::core::extractor::extract_bytes(&file.data, &mime, &child_config).await {
            Ok(result) => {
                children.push(ArchiveEntry {
                    path: file.name,
                    mime_type: mime,
                    result: Box::new(result),
                });
            }
            Err(e) => {
                warnings.push(ProcessingWarning {
                    source: Cow::Borrowed("pdf_embedded_files"),
                    message: Cow::Owned(format!("Failed to extract embedded '{}': {}", file.name, e)),
                });
            }
        }
    }

    (children, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_embedded_files_no_names() {
        let doc = Document::with_version("1.5");
        let files = extract_embedded_files(&doc);
        assert!(files.is_empty());
    }
}
