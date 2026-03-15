//! Cache command - Manage cache operations
//!
//! This module provides commands for cache management including statistics,
//! clearing, manifest generation, and model warming.

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

/// Execute cache manifest command - outputs expected model files with checksums.
pub fn manifest_command(format: OutputFormat) -> Result<()> {
    let mut entries = Vec::new();

    #[cfg(feature = "paddle-ocr")]
    {
        entries.extend(kreuzberg::paddle_ocr::ModelManager::manifest());
    }

    #[cfg(feature = "layout-detection")]
    {
        entries.extend(kreuzberg::layout::LayoutModelManager::manifest());
    }

    let total_size_bytes: u64 = entries.iter().map(|e| e.size_bytes).sum();
    let version = env!("CARGO_PKG_VERSION");

    match format {
        OutputFormat::Text => {
            println!("Model Manifest (kreuzberg {})", version);
            println!("====================================");
            println!("{:<50} {:>12} SHA256", "PATH", "SIZE");
            println!("{:<50} {:>12} ------", "----", "----");
            for entry in &entries {
                let size_str = if entry.size_bytes > 0 {
                    format!("{:.1} MB", entry.size_bytes as f64 / 1_048_576.0)
                } else {
                    "unknown".to_string()
                };
                println!("{:<50} {:>12} {}", entry.relative_path, size_str, &entry.sha256[..12]);
            }
            println!();
            println!(
                "Total: {} files, {:.1} MB",
                entries.len(),
                total_size_bytes as f64 / 1_048_576.0
            );
        }
        OutputFormat::Json => {
            let output = json!({
                "kreuzberg_version": version,
                "total_size_bytes": total_size_bytes,
                "model_count": entries.len(),
                "models": entries,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize manifest to JSON")?
            );
        }
    }

    Ok(())
}

/// Execute cache warm command - eagerly downloads all models.
pub fn warm_command(
    cache_dir: Option<PathBuf>,
    format: OutputFormat,
    all_embeddings: bool,
    embedding_model: Option<String>,
) -> Result<()> {
    let cache_base = resolve_cache_base(cache_dir);

    let mut downloaded = Vec::new();
    let mut already_cached = Vec::new();

    #[cfg(feature = "paddle-ocr")]
    {
        let paddle_dir = cache_base.join("paddle-ocr");
        let manager = kreuzberg::paddle_ocr::ModelManager::new(paddle_dir);

        // ensure_all_models downloads v2 det (server+mobile), cls (PP-LCNet),
        // doc_ori, v2 unified rec models, and all per-script rec families
        manager
            .ensure_all_models()
            .context("Failed to download PaddleOCR v2 models")?;
        downloaded.push("paddle-ocr v2 (server+mobile det, cls, doc_ori, unified+per-script rec)".to_string());
    }

    #[cfg(feature = "layout-detection")]
    {
        let layout_dir = cache_base.join("layout");
        let manager = kreuzberg::layout::LayoutModelManager::new(Some(layout_dir));

        let was_cached = manager.is_rtdetr_cached() && manager.is_slanet_plus_cached();

        if was_cached {
            already_cached.push("layout (rtdetr, slanet-plus)".to_string());
        } else {
            manager
                .ensure_all_models()
                .context("Failed to download layout models")?;
            downloaded.push("layout (rtdetr, slanet-plus)".to_string());
        }
    }

    #[cfg(feature = "embeddings")]
    {
        let embeddings_dir = cache_base.join("embeddings");
        let presets_to_warm: Vec<&kreuzberg::EmbeddingPreset> = if all_embeddings {
            kreuzberg::EMBEDDING_PRESETS.iter().collect()
        } else if let Some(ref name) = embedding_model {
            match kreuzberg::get_preset(name) {
                Some(preset) => vec![preset],
                None => {
                    let available: Vec<&str> = kreuzberg::list_presets();
                    anyhow::bail!(
                        "Unknown embedding preset '{}'. Available: {}",
                        name,
                        available.join(", ")
                    );
                }
            }
        } else {
            vec![]
        };

        for preset in &presets_to_warm {
            let label = format!("embedding ({})", preset.name);
            kreuzberg::warm_model(
                &kreuzberg::core::config::EmbeddingModelType::Preset {
                    name: preset.name.to_string(),
                },
                Some(embeddings_dir.clone()),
            )
            .map_err(|e| anyhow::anyhow!("Failed to download embedding model '{}': {}", preset.name, e))?;
            downloaded.push(label);
        }
    }

    #[cfg(not(feature = "embeddings"))]
    {
        if all_embeddings || embedding_model.is_some() {
            anyhow::bail!("Embedding model warming requires the 'embeddings' feature to be enabled");
        }
    }

    match format {
        OutputFormat::Text => {
            if !downloaded.is_empty() {
                println!("Downloaded:");
                for d in &downloaded {
                    println!("  {}", d);
                }
            }
            if !already_cached.is_empty() {
                println!("Already cached:");
                for c in &already_cached {
                    println!("  {}", c);
                }
            }
            println!("All models ready in {}", cache_base.display());
        }
        OutputFormat::Json => {
            let output = json!({
                "cache_dir": cache_base.to_string_lossy(),
                "downloaded": downloaded,
                "already_cached": already_cached,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize warm results to JSON")?
            );
        }
    }

    Ok(())
}

/// Resolve the cache base directory.
fn resolve_cache_base(cache_dir: Option<PathBuf>) -> PathBuf {
    if let Some(dir) = cache_dir {
        return dir;
    }
    if let Ok(env_path) = std::env::var("KREUZBERG_CACHE_DIR") {
        return PathBuf::from(env_path);
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".kreuzberg")
}
