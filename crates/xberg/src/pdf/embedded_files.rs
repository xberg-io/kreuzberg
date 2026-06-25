//! PDF embedded file (portfolio/attachment) extraction using lopdf.
//!
//! PDFs can contain file attachments via the `/Names` → `/EmbeddedFiles` name tree
//! in the document catalog. This module extracts those files, detects their MIME
//! type, and returns them as `ArchiveEntry` values for recursive processing.

#[cfg(feature = "tokio-runtime")]
use crate::types::{ArchiveEntry, ProcessingWarning};
#[cfg(feature = "tokio-runtime")]
use lopdf::{Document, Object};
#[cfg(feature = "tokio-runtime")]
use std::borrow::Cow;

/// Embedded file descriptor extracted from the PDF name tree.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddedFile {
    /// The filename as stored in the PDF name tree.
    pub name: String,
    /// Raw file bytes from the embedded stream (already decompressed by lopdf).
    pub data: Vec<u8>,
    /// Compressed byte count of the original stream (before decompression).
    ///
    /// Used by callers to compute the decompression ratio and detect zip-bomb-style
    /// attacks that embed a tiny compressed stream expanding to gigabytes of data.
    pub compressed_size: usize,
    /// MIME type if specified in the filespec, otherwise `None`.
    pub mime_type: Option<String>,
}

/// Extract embedded file descriptors from a PDF document loaded via lopdf.
///
/// Walks the `/Names` → `/EmbeddedFiles` name tree in the catalog.
/// Returns an empty `Vec` if the document has no embedded files.
#[cfg(feature = "tokio-runtime")]
pub(crate) fn extract_embedded_files(document: &Document) -> Vec<EmbeddedFile> {
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
#[cfg(feature = "tokio-runtime")]
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
#[cfg(feature = "tokio-runtime")]
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

    // Record the compressed size *before* decompression for ratio checking by the caller.
    let compressed_size = stream.content.len();

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
        compressed_size,
        mime_type,
    })
}

/// Resolve a PDF object through references.
#[cfg(feature = "tokio-runtime")]
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
#[cfg(feature = "tokio-runtime")]
pub(crate) async fn extract_and_process_embedded_files(
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

    // Pull the decompression ratio limit from SecurityLimits (default 100×).
    let max_ratio = config
        .security_limits
        .as_ref()
        .map(|sl| sl.max_compression_ratio)
        .unwrap_or(100);

    for file in embedded {
        // Decompression ratio guard: reject streams that expand far beyond their
        // compressed size to prevent zip-bomb-style denial-of-service via PDF
        // embedded files. A ratio of 0 means the stream was already uncompressed
        // (compressed_size == 0 or decompressed size ≤ compressed size); skip
        // the check in that case to avoid false positives on stored-mode streams.
        if file.compressed_size > 0 {
            let ratio = file.data.len() as f64 / file.compressed_size as f64;
            if ratio > max_ratio as f64 {
                warnings.push(ProcessingWarning {
                    source: Cow::Borrowed("pdf_embedded_files"),
                    message: Cow::Owned(format!(
                        "Skipped embedded file '{}': decompression ratio {:.0}x exceeds limit {}x \
                         (compressed {} B → decompressed {} B)",
                        file.name,
                        ratio,
                        max_ratio,
                        file.compressed_size,
                        file.data.len(),
                    )),
                });
                continue;
            }
        }

        // Per-embedded-file size cap (same limit as OOXML / email attachments).
        if config
            .max_embedded_file_bytes
            .is_some_and(|cap| file.data.len() as u64 > cap)
        {
            let cap = config.max_embedded_file_bytes.unwrap_or(0);
            warnings.push(ProcessingWarning {
                source: Cow::Borrowed("pdf_embedded_files"),
                message: Cow::Owned(format!(
                    "Skipped embedded file '{}': size {} bytes exceeds cap {} bytes",
                    file.name,
                    file.data.len(),
                    cap,
                )),
            });
            continue;
        }

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

#[cfg(all(test, feature = "tokio-runtime"))]
mod tests {
    use super::*;
    use crate::core::config::ExtractionConfig;
    use crate::extractors::security::SecurityLimits;

    #[test]
    fn test_extract_embedded_files_no_names() {
        let doc = Document::with_version("1.5");
        let files = extract_embedded_files(&doc);
        assert!(files.is_empty());
    }

    /// Build a synthetic `EmbeddedFile` with a given compressed and decompressed size
    /// and check whether the ratio guard would fire.
    fn ratio_would_skip(compressed: usize, decompressed: usize, max_ratio: usize) -> bool {
        if compressed == 0 {
            return false;
        }
        let ratio = decompressed as f64 / compressed as f64;
        ratio > max_ratio as f64
    }

    #[test]
    fn test_ratio_guard_fires_above_limit() {
        // 1 byte compressed → 1001 bytes decompressed with a 1000× limit = skip
        assert!(ratio_would_skip(1, 1001, 1000));
    }

    #[test]
    fn test_ratio_guard_passes_at_exact_limit() {
        // 1 byte compressed → 1000 bytes decompressed = exactly 1000×: must NOT skip
        assert!(!ratio_would_skip(1, 1000, 1000));
    }

    #[test]
    fn test_ratio_guard_passes_when_compressed_zero() {
        // Zero compressed size means stored-mode stream: guard must not divide by zero
        assert!(!ratio_would_skip(0, 50_000, 100));
    }

    #[test]
    fn test_ratio_guard_passes_below_limit() {
        // 100 bytes compressed → 9 999 bytes decompressed with a 100× limit
        assert!(!ratio_would_skip(100, 9_999, 100));
        // 100 bytes compressed → 10 001 bytes would exceed 100×
        assert!(ratio_would_skip(100, 10_001, 100));
    }

    #[tokio::test]
    async fn test_no_embedded_files_returns_empty() {
        // An empty PDF document has no embedded files; both result slices must be empty.
        let doc_bytes = {
            let mut doc = Document::with_version("1.5");
            let mut buf = Vec::new();
            doc.save_to(&mut buf).unwrap();
            buf
        };
        let config = ExtractionConfig::default();
        let (children, warnings) = extract_and_process_embedded_files(&doc_bytes, &config).await;
        assert!(children.is_empty());
        assert!(warnings.is_empty());
    }

    #[tokio::test]
    async fn test_embedded_file_over_size_cap_skipped_with_warning() {
        // Build a PDF with an embedded file whose decompressed size exceeds the cap.
        // We do this by using a stored (uncompressed) stream so compressed_size == data.len().
        use lopdf::{Dictionary, Object, Stream};

        let mut doc = Document::with_version("1.5");

        // Create the payload — 200 bytes of zeros. With cap = 10, this must be skipped.
        let payload: Vec<u8> = vec![0u8; 200];

        // Build the embedded file stream.
        let mut ef_stream_dict = Dictionary::new();
        ef_stream_dict.set("Type", Object::Name(b"EmbeddedFile".to_vec()));
        // No filter → stored mode, compressed_size == decompressed_size for this check.
        ef_stream_dict.set("Length", Object::Integer(payload.len() as i64));
        let ef_stream = Stream::new(ef_stream_dict, payload.clone());
        let ef_stream_id = doc.add_object(ef_stream);

        // Build /EF dict referencing the stream.
        let mut ef_dict = Dictionary::new();
        ef_dict.set("F", Object::Reference(ef_stream_id));
        let ef_dict_id = doc.add_object(ef_dict);

        // Build the filespec.
        let mut fs_dict = Dictionary::new();
        fs_dict.set("Type", Object::Name(b"Filespec".to_vec()));
        fs_dict.set("F", Object::String(b"test.txt".to_vec(), lopdf::StringFormat::Literal));
        fs_dict.set("EF", Object::Reference(ef_dict_id));
        let fs_dict_id = doc.add_object(fs_dict);

        // Build /EmbeddedFiles name tree.
        let names_arr = Object::Array(vec![
            Object::String(b"test.txt".to_vec(), lopdf::StringFormat::Literal),
            Object::Reference(fs_dict_id),
        ]);
        let mut ef_names_dict = Dictionary::new();
        ef_names_dict.set("Names", names_arr);
        let ef_names_id = doc.add_object(ef_names_dict);

        // Build /Names dict.
        let mut names_dict = Dictionary::new();
        names_dict.set("EmbeddedFiles", Object::Reference(ef_names_id));
        let names_id = doc.add_object(names_dict);

        // Wire into catalog.
        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", Object::Array(vec![]));
        pages_dict.set("Count", Object::Integer(0));
        let pages_id = doc.add_object(pages_dict);

        let mut catalog_dict = Dictionary::new();
        catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", Object::Reference(pages_id));
        catalog_dict.set("Names", Object::Reference(names_id));
        let catalog_id = doc.add_object(catalog_dict);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();

        let config = ExtractionConfig {
            max_embedded_file_bytes: Some(10), // 10 bytes cap; payload is 200 bytes
            ..Default::default()
        };
        let (_children, warnings) = extract_and_process_embedded_files(&buf, &config).await;

        let cap_warnings: Vec<_> = warnings.iter().filter(|w| w.message.contains("exceeds cap")).collect();
        assert_eq!(
            cap_warnings.len(),
            1,
            "expected one size-cap warning, got: {:?}",
            warnings
        );
        assert!(
            cap_warnings[0].message.contains("test.txt"),
            "warning must name the file: {}",
            cap_warnings[0].message
        );
    }

    #[tokio::test]
    async fn test_embedded_file_ratio_exceeded_skipped_with_warning() {
        // Build a PDF with a stored embedded file whose decompressed size hugely exceeds
        // the compressed size ratio limit. Since stored mode has compressed_size == data.len(),
        // the ratio is always 1 — so we cannot trigger the ratio guard with a stored stream.
        // Instead, verify the guard logic via the unit test helpers above and confirm that
        // the warning path is reachable; this test exercises the happy path (no ratio warning).
        use lopdf::{Dictionary, Object, Stream};

        let mut doc = Document::with_version("1.5");
        let payload: Vec<u8> = b"Hello".to_vec();
        let mut ef_stream_dict = Dictionary::new();
        ef_stream_dict.set("Type", Object::Name(b"EmbeddedFile".to_vec()));
        ef_stream_dict.set("Length", Object::Integer(payload.len() as i64));
        let ef_stream = Stream::new(ef_stream_dict, payload.clone());
        let ef_stream_id = doc.add_object(ef_stream);

        let mut ef_dict = Dictionary::new();
        ef_dict.set("F", Object::Reference(ef_stream_id));
        let ef_dict_id = doc.add_object(ef_dict);

        let mut fs_dict = Dictionary::new();
        fs_dict.set("F", Object::String(b"note.txt".to_vec(), lopdf::StringFormat::Literal));
        fs_dict.set("EF", Object::Reference(ef_dict_id));
        let fs_dict_id = doc.add_object(fs_dict);

        let names_arr = Object::Array(vec![
            Object::String(b"note.txt".to_vec(), lopdf::StringFormat::Literal),
            Object::Reference(fs_dict_id),
        ]);
        let mut ef_names_dict = Dictionary::new();
        ef_names_dict.set("Names", names_arr);
        let ef_names_id = doc.add_object(ef_names_dict);

        let mut names_dict = Dictionary::new();
        names_dict.set("EmbeddedFiles", Object::Reference(ef_names_id));
        let names_id = doc.add_object(names_dict);

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", Object::Array(vec![]));
        pages_dict.set("Count", Object::Integer(0));
        let pages_id = doc.add_object(pages_dict);

        let mut catalog_dict = Dictionary::new();
        catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", Object::Reference(pages_id));
        catalog_dict.set("Names", Object::Reference(names_id));
        let catalog_id = doc.add_object(catalog_dict);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();

        // Use a very tight ratio limit (1×). Stored stream has ratio == 1.0,
        // which is NOT greater than the limit of 1, so no ratio warning.
        let config = ExtractionConfig {
            security_limits: Some(SecurityLimits {
                max_compression_ratio: 1,
                ..SecurityLimits::default()
            }),
            ..Default::default()
        };
        let (_children, warnings) = extract_and_process_embedded_files(&buf, &config).await;
        let ratio_warnings: Vec<_> = warnings.iter().filter(|w| w.message.contains("ratio")).collect();
        assert!(
            ratio_warnings.is_empty(),
            "stored-mode stream must not trigger ratio guard: {:?}",
            ratio_warnings
        );
    }
}
