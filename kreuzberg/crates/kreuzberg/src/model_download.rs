//! Shared utilities for downloading and verifying ONNX models from HuggingFace Hub.
//!
//! Used by both layout detection and PaddleOCR model managers.

use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// Download a file from a HuggingFace Hub repository.
///
/// Uses `hf-hub`'s built-in caching so repeated calls for the same file are fast.
pub fn hf_download(repo_id: &str, remote_filename: &str) -> Result<PathBuf, String> {
    tracing::info!(repo = repo_id, filename = remote_filename, "Downloading via hf-hub");

    let api = hf_hub::api::sync::ApiBuilder::from_env()
        .with_progress(true)
        .build()
        .map_err(|e| format!("Failed to initialize HuggingFace Hub API: {e}"))?;

    let repo = api.model(repo_id.to_string());
    let cached_path = repo
        .get(remote_filename)
        .map_err(|e| format!("Failed to download '{remote_filename}' from {repo_id}: {e}"))?;

    Ok(cached_path)
}

/// Verify the SHA256 checksum of a file using streaming reads.
///
/// Streams the file in 64 KiB chunks to avoid loading large model files (100MB+) entirely
/// into memory. Returns `Ok(())` if the checksum matches or is empty (skip verification).
pub fn verify_sha256(path: &Path, expected: &str, label: &str) -> Result<(), String> {
    if expected.is_empty() {
        return Ok(());
    }

    let file = std::fs::File::open(path).map_err(|e| format!("Failed to open file for checksum: {e}"))?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut hasher = Sha256::new();

    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("Failed to read file for checksum: {e}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let hash_hex = hex::encode(hasher.finalize());

    if hash_hex != expected {
        return Err(format!(
            "Checksum mismatch for {label}: expected {expected}, got {hash_hex}"
        ));
    }

    tracing::debug!(label, "Checksum verified");
    Ok(())
}

/// Resolve the kreuzberg cache directory for a given module.
///
/// Delegates to [`crate::cache_dir::resolve_cache_dir`] for centralized,
/// platform-aware cache directory resolution.
#[cfg(feature = "layout-detection")]
pub fn resolve_cache_dir(module: &str) -> PathBuf {
    crate::cache_dir::resolve_cache_dir(module)
}
