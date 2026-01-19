//! Cache command - Manage cache operations
//!
//! This module provides commands for cache management including statistics
//! and clearing operations.

use anyhow::{Context, Result};
use kreuzberg::cache;
use serde_json::json;
use std::path::PathBuf;

use crate::OutputFormat;

/// Execute cache stats command
pub fn stats_command(cache_dir: Option<PathBuf>, format: OutputFormat) -> Result<()> {
    let default_cache_dir = std::env::current_dir()
        .context("Failed to get current directory")?
        .join(".kreuzberg");

    let cache_path = cache_dir.unwrap_or(default_cache_dir);
    let cache_dir_str = cache_path.to_string_lossy();

    let stats = cache::get_cache_metadata(&cache_dir_str).with_context(|| {
        format!(
            "Failed to get cache statistics from directory '{}'. Ensure the directory exists and is readable.",
            cache_dir_str
        )
    })?;

    match format {
        OutputFormat::Text => {
            println!("Cache Statistics");
            println!("================");
            println!("Directory: {}", cache_dir_str);
            println!("Total files: {}", stats.total_files);
            println!("Total size: {:.2} MB", stats.total_size_mb);
            println!("Available space: {:.2} MB", stats.available_space_mb);
            println!("Oldest file age: {:.2} days", stats.oldest_file_age_days);
            println!("Newest file age: {:.2} days", stats.newest_file_age_days);
        }
        OutputFormat::Json => {
            let output = json!({
                "directory": cache_dir_str,
                "total_files": stats.total_files,
                "total_size_mb": stats.total_size_mb,
                "available_space_mb": stats.available_space_mb,
                "oldest_file_age_days": stats.oldest_file_age_days,
                "newest_file_age_days": stats.newest_file_age_days,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize cache statistics to JSON")?
            );
        }
    }

    Ok(())
}

/// Execute cache clear command
pub fn clear_command(cache_dir: Option<PathBuf>, format: OutputFormat) -> Result<()> {
    let default_cache_dir = std::env::current_dir()
        .context("Failed to get current directory")?
        .join(".kreuzberg");

    let cache_path = cache_dir.unwrap_or(default_cache_dir);
    let cache_dir_str = cache_path.to_string_lossy();

    let (removed_files, freed_mb) = cache::clear_cache_directory(&cache_dir_str).with_context(|| {
        format!(
            "Failed to clear cache directory '{}'. Ensure you have write permissions.",
            cache_dir_str
        )
    })?;

    match format {
        OutputFormat::Text => {
            println!("Cache cleared successfully");
            println!("Directory: {}", cache_dir_str);
            println!("Removed files: {}", removed_files);
            println!("Freed space: {:.2} MB", freed_mb);
        }
        OutputFormat::Json => {
            let output = json!({
                "directory": cache_dir_str,
                "removed_files": removed_files,
                "freed_mb": freed_mb,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize cache clear results to JSON")?
            );
        }
    }

    Ok(())
}
