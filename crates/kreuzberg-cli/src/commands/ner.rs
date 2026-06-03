//! NER model download commands.
//!
//! Mirrors `tree_sitter::download_command` — eagerly fetches GLiNER ONNX
//! models into the kreuzberg cache so air-gapped / container-pre-bake
//! workflows do not need a network call at inference time.

use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;

use crate::{WireFormat, style};

/// Execute `kreuzberg warm --ner` / `--ner-model` / `--all-ner-models`.
///
/// `ner` is a "download the pinned default" flag. `models` is an explicit
/// list of HuggingFace repo ids. `all` downloads every variant kreuzberg
/// knows about.
pub fn download_command(
    ner: bool,
    models: Vec<String>,
    all: bool,
    cache_dir: Option<PathBuf>,
    format: WireFormat,
) -> Result<()> {
    let mut to_download: Vec<String> = Vec::new();

    if all {
        to_download.extend(kreuzberg::text::ner::known_models().iter().map(|s| s.to_string()));
    } else if !models.is_empty() {
        to_download.extend(models);
    } else if ner {
        to_download.push(kreuzberg::text::ner::default_model_name().to_string());
    } else {
        anyhow::bail!("No NER model specified. Use --ner, --ner-model <MODEL>, or --all-ner-models.");
    }

    let mut downloaded: Vec<String> = Vec::with_capacity(to_download.len());
    for repo in &to_download {
        let path = kreuzberg::text::ner::download_model(repo, cache_dir.clone())
            .with_context(|| format!("Failed to download NER model '{repo}'"))?;
        downloaded.push(format!("{repo} -> {}", path.display()));
    }

    match format {
        WireFormat::Text => {
            println!("{}", style::header("NER Model Download"));
            println!("{}", style::dim("=================="));
            for d in &downloaded {
                println!("  {}", style::success(d));
            }
            println!("{}", style::success("Done"));
        }
        WireFormat::Json => {
            let output = json!({
                "downloaded": downloaded,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize NER download results to JSON")?
            );
        }
        WireFormat::Toon => {
            let output = json!({
                "downloaded": downloaded,
            });
            println!(
                "{}",
                serde_toon::to_string(&output).context("Failed to serialize NER download results to TOON")?
            );
        }
    }

    Ok(())
}
