//! Synchronous wrappers for extraction operations.
//!
//! This module provides blocking synchronous wrappers around async extraction functions
//! for use in non-async contexts. Uses a global Tokio runtime for optimal performance.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::core::config::extraction::FileExtractionConfig;
use crate::types::ExtractionResult;

#[cfg(feature = "tokio-runtime")]
use std::path::{Path, PathBuf};

#[cfg(feature = "tokio-runtime")]
use once_cell::sync::OnceCell;

#[cfg(feature = "tokio-runtime")]
use super::batch::{batch_extract_bytes, batch_extract_file};
#[cfg(feature = "tokio-runtime")]
use super::bytes::extract_bytes;
#[cfg(feature = "tokio-runtime")]
use super::file::extract_file;

#[cfg(not(feature = "tokio-runtime"))]
use super::helpers::error_extraction_result;

/// Global Tokio runtime cell for synchronous operations.
///
/// Lazily initialized on first use and shared across all sync wrappers.
/// Using a global runtime instead of creating one per call provides 100x+ performance improvement.
///
/// # Availability
///
/// This static is only available when the `tokio-runtime` feature is enabled.
/// For WASM targets, use the truly synchronous extraction functions instead.
#[cfg(feature = "tokio-runtime")]
static GLOBAL_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();

/// Returns a reference to the global Tokio runtime, initializing it on first call.
///
/// Returns an error if the runtime cannot be created (e.g. system resource exhaustion).
#[cfg(feature = "tokio-runtime")]
fn global_runtime() -> crate::Result<&'static tokio::runtime::Runtime> {
    GLOBAL_RUNTIME.get_or_try_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| crate::KreuzbergError::Plugin {
                message: format!("Failed to create global Tokio runtime: {e}"),
                plugin_name: "runtime".to_string(),
            })
    })
}

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
    global_runtime()?.block_on(extract_file(path, mime_type, config))
}

/// FFI-friendly variant of `extract_file_sync` with concrete, non-generic parameter types.
///
/// Alef's FFI codegen cannot translate the generic `impl AsRef<Path>` of `extract_file_sync`,
/// so it stubs the corresponding C symbol. This wrapper exposes the same behavior with
/// FFI-compatible types so Alef can emit a real binding. An empty `mime_type` string is
/// interpreted as "auto-detect" (i.e. `None`).
#[cfg(feature = "tokio-runtime")]
pub fn extract_file_sync_ffi(
    path: &str,
    mime_type: &str,
    config: &ExtractionConfig,
) -> Result<ExtractionResult> {
    let mt = if mime_type.is_empty() { None } else { Some(mime_type) };
    extract_file_sync(path, mt, config)
}

/// FFI-friendly variant of `extract_file` (async) with concrete, non-generic parameter types.
///
/// See [`extract_file_sync_ffi`] for rationale. Empty `mime_type` means auto-detect.
#[cfg(feature = "tokio-runtime")]
pub async fn extract_file_ffi(
    path: &str,
    mime_type: &str,
    config: &ExtractionConfig,
) -> Result<ExtractionResult> {
    let mt = if mime_type.is_empty() { None } else { Some(mime_type) };
    extract_file(path, mt, config).await
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
    global_runtime()?.block_on(extract_bytes(content, mime_type, config))
}

/// Synchronous wrapper for `extract_bytes` (WASM-compatible version).
///
/// This is a truly synchronous implementation without tokio runtime dependency.
/// It calls `extract_bytes_sync_impl()` to perform the extraction.
#[cfg(not(feature = "tokio-runtime"))]
pub fn extract_bytes_sync(content: &[u8], mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult> {
    super::legacy::extract_bytes_sync_impl(content, Some(mime_type), Some(config))
}

/// Synchronous wrapper for `batch_extract_file`.
///
/// Uses the global Tokio runtime for optimal performance.
/// Only available with `tokio-runtime` (WASM has no filesystem).
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::batch_extract_file_sync;
/// use kreuzberg::core::config::ExtractionConfig;
/// use kreuzberg::FileExtractionConfig;
/// use std::path::PathBuf;
///
/// let config = ExtractionConfig::default();
/// let items: Vec<(PathBuf, Option<FileExtractionConfig>)> = vec![
///     ("doc1.pdf".into(), Some(FileExtractionConfig { force_ocr: Some(true), ..Default::default() })),
///     ("doc2.pdf".into(), None),
/// ];
/// let results = batch_extract_file_sync(items, &config)?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// ```
#[cfg(feature = "tokio-runtime")]
pub fn batch_extract_file_sync(
    items: Vec<(PathBuf, Option<FileExtractionConfig>)>,
    config: &ExtractionConfig,
) -> Result<Vec<ExtractionResult>> {
    global_runtime()?.block_on(batch_extract_file(items, config))
}

/// Synchronous wrapper for `batch_extract_bytes`.
///
/// Uses the global Tokio runtime for optimal performance.
/// With the `tokio-runtime` feature, this blocks the current thread using the global
/// Tokio runtime. Without it (WASM), this calls a truly synchronous implementation
/// that iterates through items and calls `extract_bytes_sync()`.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzberg::core::extractor::batch_extract_bytes_sync;
/// use kreuzberg::core::config::ExtractionConfig;
/// use kreuzberg::FileExtractionConfig;
///
/// let config = ExtractionConfig::default();
/// let items = vec![
///     (b"content".to_vec(), "text/plain".to_string(), None),
///     (b"other".to_vec(), "text/plain".to_string(),
///      Some(FileExtractionConfig { force_ocr: Some(true), ..Default::default() })),
/// ];
/// let results = batch_extract_bytes_sync(items, &config)?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// ```
#[cfg(feature = "tokio-runtime")]
pub fn batch_extract_bytes_sync(
    items: Vec<(Vec<u8>, String, Option<FileExtractionConfig>)>,
    config: &ExtractionConfig,
) -> Result<Vec<ExtractionResult>> {
    global_runtime()?.block_on(batch_extract_bytes(items, config))
}

/// Synchronous wrapper for `batch_extract_bytes` (WASM-compatible version).
///
/// Iterates through items sequentially, applying per-file config overrides.
#[cfg(not(feature = "tokio-runtime"))]
pub fn batch_extract_bytes_sync(
    items: Vec<(Vec<u8>, String, Option<FileExtractionConfig>)>,
    config: &ExtractionConfig,
) -> Result<Vec<ExtractionResult>> {
    let mut results = Vec::with_capacity(items.len());
    for (content, mime_type, file_config) in items {
        let resolved = match &file_config {
            Some(fc) => config.with_file_overrides(fc),
            None => config.clone(),
        };
        let result = extract_bytes_sync(&content, &mime_type, &resolved);
        results.push(result.unwrap_or_else(|e| error_extraction_result(&e, None)));
    }
    Ok(results)
}
