//! Byte array extraction operations.
//!
//! This module handles extraction from in-memory byte arrays, including:
//! - MIME type validation
//! - Legacy format conversion (DOC, PPT)
//! - Extraction pipeline orchestration

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::core::mime::{LEGACY_POWERPOINT_MIME_TYPE, LEGACY_WORD_MIME_TYPE};
#[cfg(feature = "office")]
use crate::extraction::libreoffice::{convert_doc_to_docx, convert_ppt_to_pptx};
use crate::types::ExtractionResult;

#[cfg(feature = "office")]
use super::file::apply_libreoffice_metadata;
use super::file::extract_bytes_with_extractor;
#[cfg(feature = "otel")]
use super::file::record_error;

/// Extract content from a byte array.
///
/// This is the main entry point for in-memory extraction. It performs the following steps:
/// 1. Validate MIME type
/// 2. Handle legacy format conversion if needed
/// 3. Select appropriate extractor from registry
/// 4. Extract content
/// 5. Run post-processing pipeline
///
/// # Arguments
///
/// * `content` - The byte array to extract
/// * `mime_type` - MIME type of the content
/// * `config` - Extraction configuration
///
/// # Returns
///
/// An `ExtractionResult` containing the extracted content and metadata.
///
/// # Errors
///
/// Returns `KreuzbergError::Validation` if MIME type is invalid.
/// Returns `KreuzbergError::UnsupportedFormat` if MIME type is not supported.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::extract_bytes;
/// use kreuzberg::core::config::ExtractionConfig;
///
/// # async fn example() -> kreuzberg::Result<()> {
/// let config = ExtractionConfig::default();
/// let bytes = b"Hello, world!";
/// let result = extract_bytes(bytes, "text/plain", &config).await?;
/// println!("Content: {}", result.content);
/// # Ok(())
/// # }
/// ```
#[cfg_attr(feature = "otel", tracing::instrument(
    skip(config, content),
    fields(
        extraction.mime_type = mime_type,
        extraction.size_bytes = content.len(),
    )
))]
pub async fn extract_bytes(content: &[u8], mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult> {
    use crate::core::mime;

    let result = async {
        let validated_mime = mime::validate_mime_type(mime_type)?;

        match validated_mime.as_str() {
            #[cfg(feature = "office")]
            LEGACY_WORD_MIME_TYPE => {
                let conversion = convert_doc_to_docx(content).await?;
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
                let conversion = convert_ppt_to_pptx(content).await?;
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

        extract_bytes_with_extractor(content, &validated_mime, config).await
    }
    .await;

    #[cfg(feature = "otel")]
    if let Err(ref e) = result {
        record_error(e);
    }

    result
}
