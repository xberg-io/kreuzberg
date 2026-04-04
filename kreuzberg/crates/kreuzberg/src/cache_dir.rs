//! Centralized cache directory resolution for all kreuzberg modules.
//!
//! Provides a single function that all modules use to determine where to store
//! cached data (models, OCR results, tessdata, etc.). This avoids per-CWD
//! `.kreuzberg/` directories and uses platform-appropriate global cache locations.

use std::path::PathBuf;

/// Resolve the kreuzberg cache base directory (without a module suffix).
///
/// Uses the same resolution order as [`resolve_cache_dir`] but returns
/// the top-level kreuzberg cache directory.
#[allow(dead_code)]
pub fn resolve_cache_base() -> PathBuf {
    if let Ok(env_path) = std::env::var("KREUZBERG_CACHE_DIR") {
        return PathBuf::from(env_path);
    }
    if let Some(cache) = dirs::cache_dir() {
        return cache.join("kreuzberg");
    }
    if let Some(home) = dirs::home_dir() {
        return home.join(".cache").join("kreuzberg");
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".kreuzberg")
}

/// Resolve the kreuzberg cache directory for a given module.
///
/// Resolution order:
/// 1. `KREUZBERG_CACHE_DIR` env var + `/{module}` (explicit override)
/// 2. Platform-appropriate global cache directory:
///    - macOS: `~/Library/Caches/kreuzberg/{module}`
///    - Linux: `$XDG_CACHE_HOME/kreuzberg/{module}` or `~/.cache/kreuzberg/{module}`
///    - Windows: `%LOCALAPPDATA%/kreuzberg/{module}`
/// 3. Home directory fallback: `~/.cache/kreuzberg/{module}`
/// 4. CWD-relative fallback: `.kreuzberg/{module}` (last resort, e.g. no HOME set)
pub fn resolve_cache_dir(module: &str) -> PathBuf {
    if let Ok(env_path) = std::env::var("KREUZBERG_CACHE_DIR") {
        return PathBuf::from(env_path).join(module);
    }
    if let Some(cache) = dirs::cache_dir() {
        return cache.join("kreuzberg").join(module);
    }
    if let Some(home) = dirs::home_dir() {
        return home.join(".cache").join("kreuzberg").join(module);
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".kreuzberg")
        .join(module)
}
