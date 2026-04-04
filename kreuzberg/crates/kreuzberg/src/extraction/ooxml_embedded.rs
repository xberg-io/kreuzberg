//! Embedded object extraction from OOXML (DOCX/PPTX) archives.
//!
//! OOXML files are ZIP archives that may contain embedded objects in:
//! - DOCX: `word/embeddings/` directory
//! - PPTX: `ppt/embeddings/` directory
//!
//! This module extracts those embedded files, detects their MIME type,
//! and recursively processes them through the extraction pipeline.

use crate::core::config::ExtractionConfig;
use crate::types::{ArchiveEntry, ProcessingWarning};
use std::borrow::Cow;
use std::io::{Cursor, Read};

/// Extract embedded objects from an OOXML ZIP archive and recursively process them.
///
/// Scans the given `embeddings_prefix` directory (e.g. `word/embeddings/` or
/// `ppt/embeddings/`) inside the ZIP archive for embedded files. Known formats
/// (.xlsx, .pdf, .docx, .pptx, etc.) are recursively extracted. OLE compound
/// files (oleObject*.bin) are skipped with a warning unless their format can be
/// identified.
///
/// Returns `(children, warnings)` suitable for attaching to `InternalDocument`.
pub async fn extract_ooxml_embedded_objects(
    zip_bytes: &[u8],
    embeddings_prefix: &str,
    source_label: &str,
    config: &ExtractionConfig,
) -> (Vec<ArchiveEntry>, Vec<ProcessingWarning>) {
    let mut children = Vec::new();
    let mut warnings = Vec::new();

    if config.max_archive_depth == 0 {
        return (children, warnings);
    }

    let cursor = Cursor::new(zip_bytes);
    let mut archive = match zip::ZipArchive::new(cursor) {
        Ok(a) => a,
        Err(_) => return (children, warnings),
    };

    // Collect embedding file names first to avoid borrow issues with the archive.
    let embedding_names: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            let file = archive.by_index(i).ok()?;
            let name = file.name().to_string();
            if name.starts_with(embeddings_prefix) && name.len() > embeddings_prefix.len() {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    if embedding_names.is_empty() {
        return (children, warnings);
    }

    let mut child_config = config.clone();
    child_config.max_archive_depth = config.max_archive_depth.saturating_sub(1);

    for entry_name in &embedding_names {
        // Extract the filename portion (after the prefix).
        let filename = entry_name
            .strip_prefix(embeddings_prefix)
            .unwrap_or(entry_name)
            .to_string();

        // Read the embedded file bytes.
        let data = match archive.by_name(entry_name) {
            Ok(mut file) => {
                let mut buf = Vec::with_capacity(file.size() as usize);
                if file.read_to_end(&mut buf).is_err() {
                    warnings.push(ProcessingWarning {
                        source: Cow::Owned(format!("{}_embedded_objects", source_label)),
                        message: Cow::Owned(format!("Failed to read embedded file '{}'", filename)),
                    });
                    continue;
                }
                buf
            }
            Err(_) => continue,
        };

        if data.is_empty() {
            continue;
        }

        // Skip OLE compound binary files unless we can identify their actual format.
        // OLE files start with the magic bytes D0 CF 11 E0 (Microsoft Compound File).
        let is_ole_binary = data.len() >= 4 && data[0..4] == [0xD0, 0xCF, 0x11, 0xE0];
        if is_ole_binary {
            // oleObject*.bin files are OLE containers — skip with warning.
            warnings.push(ProcessingWarning {
                source: Cow::Owned(format!("{}_embedded_objects", source_label)),
                message: Cow::Owned(format!(
                    "Skipped OLE compound file '{}': format identification not supported",
                    filename
                )),
            });
            continue;
        }

        // Detect MIME type from magic bytes first, then fall back to extension.
        let detected_mime = crate::core::mime::detect_mime_type_from_bytes(&data).ok().or_else(|| {
            std::path::Path::new(&filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| mime_guess::from_ext(ext).first())
                .map(|m| m.to_string())
        });

        let file_mime = match detected_mime {
            Some(m) if m != "application/octet-stream" => m,
            _ => {
                // Unknown format — skip silently.
                continue;
            }
        };

        match crate::core::extractor::extract_bytes(&data, &file_mime, &child_config).await {
            Ok(result) => {
                children.push(ArchiveEntry {
                    path: filename,
                    mime_type: file_mime,
                    result: Box::new(result),
                });
            }
            Err(e) => {
                warnings.push(ProcessingWarning {
                    source: Cow::Owned(format!("{}_embedded_objects", source_label)),
                    message: Cow::Owned(format!("Failed to extract embedded '{}': {}", filename, e)),
                });
            }
        }
    }

    (children, warnings)
}
