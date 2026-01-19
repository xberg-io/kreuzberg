//! File-based extraction operations.
//!
//! This module handles extraction from filesystem paths, including:
//! - MIME type detection and validation
//! - Legacy format conversion (DOC, PPT)
//! - File validation and reading
//! - Extraction pipeline orchestration

use crate::core::config::ExtractionConfig;
use crate::core::mime::{LEGACY_POWERPOINT_MIME_TYPE, LEGACY_WORD_MIME_TYPE};
#[cfg(feature = "office")]
use crate::extraction::libreoffice::{convert_doc_to_docx, convert_ppt_to_pptx};
use crate::types::ExtractionResult;
#[cfg(feature = "office")]
use crate::types::LibreOfficeConversionResult;
use crate::{KreuzbergError, Result};
#[cfg(feature = "office")]
use serde_json::json;
use std::path::Path;

use super::helpers::pool_mime_type;

use super::helpers::get_extractor;

/// Sanitize a file path to return only the filename.
///
/// This function extracts the filename from a path to avoid recording
/// potentially sensitive full file paths in telemetry data.
///
/// # Arguments
///
/// * `path` - The path to sanitize
///
/// # Returns
///
/// The filename as a string, or "unknown" if extraction fails
///
/// # Security
///
/// This prevents PII (personally identifiable information) from appearing in
/// traces by only recording filenames instead of full paths.
///
/// # Example
///
/// ```rust,ignore
/// let path = Path::new("/home/user/documents/secret.pdf");
/// assert_eq!(sanitize_path(path), "secret.pdf");
/// ```
#[cfg(feature = "otel")]
pub(super) fn sanitize_path(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Record error information in the current OpenTelemetry span.
///
/// This function records error details in the current span when the `otel` feature is enabled.
/// It marks the span with `otel.status_code=ERROR` and adds error type and message fields.
///
/// # Arguments
///
/// * `error` - The error to record in the span
///
/// # Example
///
/// ```rust,ignore
/// let result = extract_file("doc.pdf", None, &config).await;
/// #[cfg(feature = "otel")]
/// if let Err(ref e) = result {
///     record_error(e);
/// }
/// result
/// ```
#[cfg(feature = "otel")]
pub(in crate::core::extractor) fn record_error(error: &KreuzbergError) {
    let span = tracing::Span::current();
    span.record("otel.status_code", "ERROR");
    span.record("error.type", format!("{:?}", error));
    span.record("error.message", error.to_string());
}

/// Extract content from a file.
///
/// This is the main entry point for file-based extraction. It performs the following steps:
/// 1. Check cache for existing result (if caching enabled)
/// 2. Detect or validate MIME type
/// 3. Select appropriate extractor from registry
/// 4. Extract content
/// 5. Run post-processing pipeline
/// 6. Store result in cache (if caching enabled)
///
/// # Arguments
///
/// * `path` - Path to the file to extract
/// * `mime_type` - Optional MIME type override. If None, will be auto-detected
/// * `config` - Extraction configuration
///
/// # Returns
///
/// An `ExtractionResult` containing the extracted content and metadata.
///
/// # Errors
///
/// Returns `KreuzbergError::Validation` if the file doesn't exist or path is invalid.
/// Returns `KreuzbergError::UnsupportedFormat` if MIME type is not supported.
/// Returns `KreuzbergError::Io` for file I/O errors (these always bubble up).
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::extract_file;
/// use kreuzberg::core::config::ExtractionConfig;
///
/// # async fn example() -> kreuzberg::Result<()> {
/// let config = ExtractionConfig::default();
/// let result = extract_file("document.pdf", None, &config).await?;
/// println!("Content: {}", result.content);
/// # Ok(())
/// # }
/// ```
#[cfg_attr(feature = "otel", tracing::instrument(
    skip(config, path),
    fields(
        extraction.filename = tracing::field::Empty,
    )
))]
pub async fn extract_file(
    path: impl AsRef<Path>,
    mime_type: Option<&str>,
    config: &ExtractionConfig,
) -> Result<ExtractionResult> {
    use crate::core::{io, mime};

    let path = path.as_ref();

    #[cfg(feature = "otel")]
    {
        let span = tracing::Span::current();
        span.record("extraction.filename", sanitize_path(path));
    }

    let result = async {
        io::validate_file_exists(path)?;

        let detected_mime = mime::detect_or_validate(Some(path), mime_type)?;

        match detected_mime.as_str() {
            #[cfg(feature = "office")]
            LEGACY_WORD_MIME_TYPE => {
                let original_bytes = tokio::fs::read(path).await?;
                let conversion = convert_doc_to_docx(&original_bytes).await?;
                let mut result =
                    extract_bytes_with_extractor(&conversion.converted_bytes, &conversion.target_mime, config).await?;
                apply_libreoffice_metadata(&mut result, LEGACY_WORD_MIME_TYPE, &conversion);
                return Ok(result);
            }
            #[cfg(not(feature = "office"))]
            LEGACY_WORD_MIME_TYPE => {
                return Err(KreuzbergError::UnsupportedFormat(
                    "Legacy Word conversion requires the `office` feature or LibreOffice support".to_string(),
                ));
            }
            #[cfg(feature = "office")]
            LEGACY_POWERPOINT_MIME_TYPE => {
                let original_bytes = tokio::fs::read(path).await?;
                let conversion = convert_ppt_to_pptx(&original_bytes).await?;
                let mut result =
                    extract_bytes_with_extractor(&conversion.converted_bytes, &conversion.target_mime, config).await?;
                apply_libreoffice_metadata(&mut result, LEGACY_POWERPOINT_MIME_TYPE, &conversion);
                return Ok(result);
            }
            #[cfg(not(feature = "office"))]
            LEGACY_POWERPOINT_MIME_TYPE => {
                return Err(KreuzbergError::UnsupportedFormat(
                    "Legacy PowerPoint conversion requires the `office` feature or LibreOffice support".to_string(),
                ));
            }
            _ => {}
        }

        extract_file_with_extractor(path, &detected_mime, config).await
    }
    .await;

    #[cfg(feature = "otel")]
    if let Err(ref e) = result {
        record_error(e);
    }

    result
}

pub(in crate::core::extractor) async fn extract_file_with_extractor(
    path: &Path,
    mime_type: &str,
    config: &ExtractionConfig,
) -> Result<ExtractionResult> {
    crate::extractors::ensure_initialized()?;

    let extractor = get_extractor(mime_type)?;
    let mut result = extractor.extract_file(path, mime_type, config).await?;
    result = crate::core::pipeline::run_pipeline(result, config).await?;
    Ok(result)
}

pub(in crate::core::extractor) async fn extract_bytes_with_extractor(
    content: &[u8],
    mime_type: &str,
    config: &ExtractionConfig,
) -> Result<ExtractionResult> {
    crate::extractors::ensure_initialized()?;

    let extractor = get_extractor(mime_type)?;
    let mut result = extractor.extract_bytes(content, mime_type, config).await?;
    result = crate::core::pipeline::run_pipeline(result, config).await?;
    Ok(result)
}

#[cfg(feature = "office")]
pub(in crate::core::extractor) fn apply_libreoffice_metadata(
    result: &mut ExtractionResult,
    legacy_mime: &str,
    conversion: &LibreOfficeConversionResult,
) {
    result.mime_type = pool_mime_type(legacy_mime);
    result.metadata.additional.insert(
        "libreoffice_conversion".to_string(),
        json!({
            "converter": "libreoffice",
            "original_format": conversion.original_format,
            "target_format": conversion.target_format,
            "target_mime": conversion.target_mime,
        }),
    );
}
