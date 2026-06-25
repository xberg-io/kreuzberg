//! Centralized cache directory resolution for all xberg modules.
//!
//! Provides a single function that all modules use to determine where to store
//! cached data (models, OCR results, tessdata, etc.). This avoids per-CWD
//! `.xberg/` directories and uses platform-appropriate global cache locations.

use std::path::PathBuf;

/// Resolve the xberg cache base directory (without a module suffix).
///
/// Uses the same resolution order as [`resolve_cache_dir`] but returns
/// the top-level xberg cache directory.
#[allow(dead_code)]
pub(crate) fn resolve_cache_base() -> PathBuf {
    if let Ok(env_path) = std::env::var("XBERG_CACHE_DIR") {
        return PathBuf::from(env_path);
    }
    if let Some(cache) = dirs::cache_dir() {
        return cache.join("xberg");
    }
    if let Some(home) = dirs::home_dir() {
        return home.join(".cache").join("xberg");
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".xberg")
}

/// Resolve the xberg cache directory for a given module.
///
/// Resolution order:
/// 1. `XBERG_CACHE_DIR` env var + `/{module}` (explicit override)
/// 2. Platform-appropriate global cache directory:
///    - macOS: `~/Library/Caches/xberg/{module}`
///    - Linux: `$XDG_CACHE_HOME/xberg/{module}` or `~/.cache/xberg/{module}`
///    - Windows: `%LOCALAPPDATA%/xberg/{module}`
/// 3. Home directory fallback: `~/.cache/xberg/{module}`
/// 4. CWD-relative fallback: `.xberg/{module}` (last resort, e.g. no HOME set)
#[cfg_attr(alef, alef(skip))]
pub(crate) fn resolve_cache_dir(module: &str) -> PathBuf {
    if let Ok(env_path) = std::env::var("XBERG_CACHE_DIR") {
        return PathBuf::from(env_path).join(module);
    }
    if let Some(cache) = dirs::cache_dir() {
        return cache.join("xberg").join(module);
    }
    if let Some(home) = dirs::home_dir() {
        return home.join(".cache").join("xberg").join(module);
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".xberg")
        .join(module)
}
