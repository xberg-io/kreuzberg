//! Tree-sitter grammar management commands.
//!
//! This module provides commands for downloading, listing, and managing
//! tree-sitter grammar parsers via the tree-sitter-language-pack crate.

use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;

use crate::{WireFormat, style};

/// Execute the tree-sitter download command.
///
/// Downloads tree-sitter grammar parsers based on the provided arguments:
/// - Specific languages by name
/// - All available languages (--all)
/// - Language groups (--groups)
pub fn download_command(
    languages: Vec<String>,
    all: bool,
    groups: Option<Vec<String>>,
    cache_dir: Option<PathBuf>,
    format: WireFormat,
) -> Result<()> {
    // Apply custom cache directory if provided
    if let Some(ref dir) = cache_dir {
        let config = tree_sitter_language_pack::PackConfig {
            cache_dir: Some(dir.clone()),
            languages: None,
            groups: None,
        };
        tree_sitter_language_pack::configure(&config).context("Failed to configure custom cache directory")?;
    }

    let count: usize;
    let description: String;

    if all {
        count = tree_sitter_language_pack::download_all().context("Failed to download all tree-sitter grammars")?;
        description = "all available languages".to_string();
    } else if let Some(ref group_list) = groups {
        let config = tree_sitter_language_pack::PackConfig {
            cache_dir: cache_dir.clone(),
            languages: None,
            groups: Some(group_list.clone()),
        };
        tree_sitter_language_pack::init(&config).context("Failed to download tree-sitter grammar groups")?;
        count = 0; // init does not return a count
        description = format!("groups: {}", group_list.join(", "));
    } else if !languages.is_empty() {
        let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
        count = tree_sitter_language_pack::download(&refs).context("Failed to download tree-sitter grammars")?;
        description = format!("languages: {}", languages.join(", "));
    } else {
        anyhow::bail!("No languages specified. Use language names, --all, --groups, or --from-config.");
    }

    match format {
        WireFormat::Text => {
            println!("{}", style::header("Tree-sitter Download"));
            println!("{}", style::dim("===================="));
            println!("{} {}", style::label("Requested:"), description);
            if groups.is_none() || all || !languages.is_empty() {
                println!(
                    "{} {}",
                    style::label("Newly downloaded:"),
                    style::success(&count.to_string())
                );
            }
            if let Some(ref dir) = cache_dir {
                println!(
                    "{} {}",
                    style::label("Cache directory:"),
                    style::success(&dir.display().to_string())
                );
            }
            println!("{}", style::success("Done"));
        }
        WireFormat::Json => {
            let mut output = json!({
                "requested": description,
                "newly_downloaded": count,
            });
            if let Some(ref dir) = cache_dir {
                output["cache_dir"] = json!(dir.to_string_lossy());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize download results to JSON")?
            );
        }
        WireFormat::Toon => {
            let mut output = json!({
                "requested": description,
                "newly_downloaded": count,
            });
            if let Some(ref dir) = cache_dir {
                output["cache_dir"] = json!(dir.to_string_lossy());
            }
            println!(
                "{}",
                serde_toon::to_string(&output).context("Failed to serialize download results to TOON")?
            );
        }
    }

    Ok(())
}

/// Execute the tree-sitter list command.
///
/// Lists available or downloaded tree-sitter languages, optionally filtering
/// by a name substring.
pub fn list_command(downloaded_only: bool, filter: Option<String>, format: WireFormat) -> Result<()> {
    let languages = if downloaded_only {
        tree_sitter_language_pack::downloaded_languages()
    } else {
        tree_sitter_language_pack::manifest_languages().context("Failed to fetch tree-sitter language manifest")?
    };

    let filtered: Vec<&String> = if let Some(ref f) = filter {
        let lower = f.to_lowercase();
        languages.iter().filter(|l| l.to_lowercase().contains(&lower)).collect()
    } else {
        languages.iter().collect()
    };

    let source = if downloaded_only { "downloaded" } else { "available" };

    match format {
        WireFormat::Text => {
            println!(
                "{} ({} {}{})",
                style::header("Tree-sitter Languages"),
                filtered.len(),
                source,
                filter.as_ref().map(|f| format!(", filter: '{f}'")).unwrap_or_default()
            );
            println!("{}", style::dim("====================="));
            for lang in &filtered {
                println!("  {}", style::success(lang));
            }
        }
        WireFormat::Json => {
            let output = json!({
                "source": source,
                "count": filtered.len(),
                "filter": filter,
                "languages": filtered,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize language list to JSON")?
            );
        }
        WireFormat::Toon => {
            let output = json!({
                "source": source,
                "count": filtered.len(),
                "filter": filter,
                "languages": filtered,
            });
            println!(
                "{}",
                serde_toon::to_string(&output).context("Failed to serialize language list to TOON")?
            );
        }
    }

    Ok(())
}

/// Execute the tree-sitter cache-dir command.
///
/// Displays the effective cache directory for tree-sitter grammar parsers.
pub fn cache_dir_command(format: WireFormat) -> Result<()> {
    let dir = tree_sitter_language_pack::cache_dir().context("Failed to determine tree-sitter cache directory")?;
    let dir_str = dir.to_string_lossy();

    match format {
        WireFormat::Text => {
            println!("{} {}", style::label("Cache directory:"), style::success(&dir_str));
        }
        WireFormat::Json => {
            let output = json!({ "cache_dir": dir_str });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize cache directory to JSON")?
            );
        }
        WireFormat::Toon => {
            let output = json!({ "cache_dir": dir_str });
            println!(
                "{}",
                serde_toon::to_string(&output).context("Failed to serialize cache directory to TOON")?
            );
        }
    }

    Ok(())
}

/// Execute the tree-sitter clean command.
///
/// Clears all cached tree-sitter grammar parser shared libraries.
pub fn clean_command(format: WireFormat) -> Result<()> {
    tree_sitter_language_pack::clean_cache().context("Failed to clean tree-sitter cache")?;

    match format {
        WireFormat::Text => {
            println!("{}", style::success("Tree-sitter cache cleared successfully"));
        }
        WireFormat::Json => {
            let output = json!({ "status": "cleared" });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize clean result to JSON")?
            );
        }
        WireFormat::Toon => {
            let output = json!({ "status": "cleared" });
            println!(
                "{}",
                serde_toon::to_string(&output).context("Failed to serialize clean result to TOON")?
            );
        }
    }

    Ok(())
}
