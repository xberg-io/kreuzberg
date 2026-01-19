//! Synchronous wrappers for extraction operations.
//!
//! This module provides blocking synchronous wrappers around async extraction functions
//! for use in non-async contexts. Uses a global Tokio runtime for optimal performance.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::types::ExtractionResult;
use std::path::Path;

#[cfg(feature = "tokio-runtime")]
use once_cell::sync::Lazy;

#[cfg(feature = "tokio-runtime")]
use super::batch::{batch_extract_bytes, batch_extract_file};
use super::bytes::extract_bytes;
use super::file::extract_file;

/// Global Tokio runtime for synchronous operations.
///
/// This runtime is lazily initialized on first use and shared across all sync wrappers.
/// Using a global runtime instead of creating one per call provides 100x+ performance improvement.
///
/// # Safety
///
/// The `.expect()` here is justified because:
/// 1. Runtime creation can only fail due to system resource exhaustion (OOM, thread limit)
/// 2. If runtime creation fails, the process is already in a critical state
/// 3. This is a one-time initialization - if it fails, nothing will work
/// 4. Better to fail fast than return errors from every sync operation
///
/// # Availability
///
/// This static is only available when the `tokio-runtime` feature is enabled.
/// For WASM targets, use the truly synchronous extraction functions instead.
#[cfg(feature = "tokio-runtime")]
static GLOBAL_RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create global Tokio runtime - system may be out of resources")
});

/// Synchronous wrapper for `extract_file`.
///
/// This is a convenience function that blocks the current thread until extraction completes.
/// For async code, use `extract_file` directly.
///
/// Uses the global Tokio runtime for 100x+ performance improvement over creating
/// a new runtime per call. Always uses the global runtime to avoid nested runtime issues.
///
/// This function is only available with the `tokio-runtime` feature. For WASM targets,
/// use a truly synchronous extraction approach instead.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::extract_file_sync;
/// use kreuzberg::core::config::ExtractionConfig;
///
/// let config = ExtractionConfig::default();
/// let result = extract_file_sync("document.pdf", None, &config)?;
/// println!("Content: {}", result.content);
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// ```
#[cfg(feature = "tokio-runtime")]
pub fn extract_file_sync(
    path: impl AsRef<Path>,
    mime_type: Option<&str>,
    config: &ExtractionConfig,
) -> Result<ExtractionResult> {
    GLOBAL_RUNTIME.block_on(extract_file(path, mime_type, config))
}

/// Synchronous wrapper for `extract_bytes`.
///
/// Uses the global Tokio runtime for 100x+ performance improvement over creating
/// a new runtime per call.
///
/// With the `tokio-runtime` feature, this blocks the current thread using the global
/// Tokio runtime. Without it (WASM), this calls a truly synchronous implementation.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::extract_bytes_sync;
/// use kreuzberg::core::config::ExtractionConfig;
///
/// let config = ExtractionConfig::default();
/// let bytes = b"Hello, world!";
/// let result = extract_bytes_sync(bytes, "text/plain", &config)?;
/// println!("Content: {}", result.content);
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// ```
#[cfg(feature = "tokio-runtime")]
pub fn extract_bytes_sync(content: &[u8], mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult> {
    GLOBAL_RUNTIME.block_on(extract_bytes(content, mime_type, config))
}

/// Synchronous wrapper for `extract_bytes` (WASM-compatible version).
///
/// This is a truly synchronous implementation without tokio runtime dependency.
/// It calls `extract_bytes_sync_impl()` to perform the extraction.
#[cfg(not(feature = "tokio-runtime"))]
pub fn extract_bytes_sync(content: &[u8], mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult> {
    super::legacy::extract_bytes_sync_impl(content.to_vec(), Some(mime_type.to_string()), Some(config.clone()))
}

/// Synchronous wrapper for `batch_extract_file`.
///
/// Uses the global Tokio runtime for 100x+ performance improvement over creating
/// a new runtime per call.
///
/// This function is only available with the `tokio-runtime` feature. For WASM targets,
/// use a truly synchronous extraction approach instead.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::batch_extract_file_sync;
/// use kreuzberg::core::config::ExtractionConfig;
///
/// let config = ExtractionConfig::default();
/// let paths = vec!["doc1.pdf", "doc2.pdf"];
/// let results = batch_extract_file_sync(paths, &config)?;
/// println!("Processed {} files", results.len());
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// ```
#[cfg(feature = "tokio-runtime")]
pub fn batch_extract_file_sync(
    paths: Vec<impl AsRef<Path>>,
    config: &ExtractionConfig,
) -> Result<Vec<ExtractionResult>> {
    GLOBAL_RUNTIME.block_on(batch_extract_file(paths, config))
}

/// Synchronous wrapper for `batch_extract_bytes`.
///
/// Uses the global Tokio runtime for 100x+ performance improvement over creating
/// a new runtime per call.
///
/// With the `tokio-runtime` feature, this blocks the current thread using the global
/// Tokio runtime. Without it (WASM), this calls a truly synchronous implementation
/// that iterates through items and calls `extract_bytes_sync()`.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::batch_extract_bytes_sync;
/// use kreuzberg::core::config::ExtractionConfig;
///
/// let config = ExtractionConfig::default();
/// let contents = vec![
///     (b"content 1".to_vec(), "text/plain".to_string()),
///     (b"content 2".to_vec(), "text/plain".to_string()),
/// ];
/// let results = batch_extract_bytes_sync(contents, &config)?;
/// println!("Processed {} items", results.len());
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// ```
#[cfg(feature = "tokio-runtime")]
pub fn batch_extract_bytes_sync(
    contents: Vec<(Vec<u8>, String)>,
    config: &ExtractionConfig,
) -> Result<Vec<ExtractionResult>> {
    GLOBAL_RUNTIME.block_on(batch_extract_bytes(contents, config))
}

/// Synchronous wrapper for `batch_extract_bytes` (WASM-compatible version).
///
/// This is a truly synchronous implementation that iterates through items
/// and calls `extract_bytes_sync()` for each.
#[cfg(not(feature = "tokio-runtime"))]
pub fn batch_extract_bytes_sync(
    contents: Vec<(Vec<u8>, String)>,
    config: &ExtractionConfig,
) -> Result<Vec<ExtractionResult>> {
    use crate::types::{ErrorMetadata, Metadata};
    use crate::utils::intern_mime_type;

    let mut results = Vec::with_capacity(contents.len());
    for (content, mime_type) in contents {
        let result = extract_bytes_sync(&content, &mime_type, config);
        results.push(result.unwrap_or_else(|e| ExtractionResult {
            content: format!("Error: {}", e),
            mime_type: intern_mime_type("text/plain").to_string(),
            metadata: Metadata {
                error: Some(ErrorMetadata {
                    error_type: format!("{:?}", e),
                    message: e.to_string(),
                }),
                ..Default::default()
            },
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        }));
    }
    Ok(results)
}
