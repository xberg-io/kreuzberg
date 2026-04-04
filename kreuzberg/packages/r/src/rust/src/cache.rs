//! Cache management functions

use crate::error::to_r_error;
use extendr_api::prelude::*;

pub fn clear_cache_impl() -> extendr_api::Result<()> {
    let cache_root = cache_root_dir();
    if !cache_root.exists() {
        return Ok(());
    }
    for dir in cache_directories(&cache_root)? {
        let dir_str = dir.to_str()
            .ok_or_else(|| extendr_api::Error::Other("Cache path not valid UTF-8".to_string()))?;
        kreuzberg::cache::clear_cache_directory(dir_str).map_err(to_r_error)?;
    }
    Ok(())
}

pub fn cache_stats_impl() -> extendr_api::Result<List> {
    let cache_root = cache_root_dir();
    let mut total_entries: usize = 0;
    let mut total_bytes: f64 = 0.0;

    if cache_root.exists() {
        for dir in cache_directories(&cache_root)? {
            let dir_str = dir.to_str()
                .ok_or_else(|| extendr_api::Error::Other("Cache path not valid UTF-8".to_string()))?;
            let stats = kreuzberg::cache::get_cache_metadata(dir_str).map_err(to_r_error)?;
            total_entries += stats.total_files;
            total_bytes += stats.total_size_mb * 1024.0 * 1024.0;
        }
    }

    let names = vec!["total_entries", "total_size_bytes"];
    let values: Vec<Robj> = vec![
        (total_entries as i32).into_robj(),
        (total_bytes.round() as i64).into_robj(),
    ];
    List::from_names_and_values(names, values).map_err(to_r_error)
}

fn cache_root_dir() -> std::path::PathBuf {
    // Use platform-appropriate cache directory
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return std::path::PathBuf::from(home).join("Library/Caches/kreuzberg");
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
            return std::path::PathBuf::from(xdg).join("kreuzberg");
        }
        if let Ok(home) = std::env::var("HOME") {
            return std::path::PathBuf::from(home).join(".cache/kreuzberg");
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            return std::path::PathBuf::from(local).join("kreuzberg/cache");
        }
    }
    std::path::PathBuf::from("/tmp/kreuzberg-cache")
}

fn cache_directories(root: &std::path::Path) -> extendr_api::Result<Vec<std::path::PathBuf>> {
    let mut dirs = Vec::new();
    if root.is_dir() {
        for entry in std::fs::read_dir(root).map_err(to_r_error)? {
            let entry = entry.map_err(to_r_error)?;
            if entry.file_type().map_err(to_r_error)?.is_dir() {
                dirs.push(entry.path());
            }
        }
    }
    Ok(dirs)
}
