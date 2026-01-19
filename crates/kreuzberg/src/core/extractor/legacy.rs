//! Legacy synchronous extraction for WASM compatibility.
//!
//! This module provides truly synchronous extraction implementations
//! for environments where Tokio runtime is not available (e.g., WASM).

/// Synchronous extraction implementation for WASM compatibility.
///
/// This function performs extraction without requiring a tokio runtime.
/// It calls the sync extractor methods directly.
///
/// # Arguments
///
/// * `content` - The byte content to extract
/// * `mime_type` - Optional MIME type to validate/use
/// * `config` - Optional extraction configuration
///
/// # Returns
///
/// An `ExtractionResult` or a `KreuzbergError`
///
/// # Implementation Notes
///
/// This is called when the `tokio-runtime` feature is disabled.
/// It replicates the logic of `extract_bytes` but uses synchronous extractor methods.
#[cfg(not(feature = "tokio-runtime"))]
pub(super) fn extract_bytes_sync_impl(
    content: Vec<u8>,
    mime_type: Option<String>,
    config: Option<ExtractionConfig>,
) -> Result<ExtractionResult> {
    use crate::core::mime;

    let config = config.unwrap_or_default();

    let validated_mime = if let Some(mime) = mime_type {
        mime::validate_mime_type(&mime)?
    } else {
        return Err(KreuzbergError::Validation {
            message: "MIME type is required for synchronous extraction".to_string(),
            source: None,
        });
    };

    crate::extractors::ensure_initialized()?;

    let extractor = get_extractor(&validated_mime)?;

    let sync_extractor = extractor.as_sync_extractor().ok_or_else(|| {
        KreuzbergError::UnsupportedFormat(format!(
            "Extractor for '{}' does not support synchronous extraction",
            validated_mime
        ))
    })?;

    let mut result = sync_extractor.extract_sync(&content, &validated_mime, &config)?;

    result = crate::core::pipeline::run_pipeline_sync(result, &config)?;

    Ok(result)
}
