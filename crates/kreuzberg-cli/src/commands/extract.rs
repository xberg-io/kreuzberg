//! Extract command - Extract text and data from documents
//!
//! This module provides the extract and batch extract commands for processing single
//! or multiple documents with customizable extraction configurations.

use anyhow::{Context, Result};
use kreuzberg::{
    ChunkingConfig, ExtractionConfig, LanguageDetectionConfig, OcrConfig, batch_extract_file_sync, extract_file_sync,
};
use serde_json::json;
use std::path::PathBuf;

use crate::{ContentOutputFormatArg, OutputFormat};

/// Execute single document extraction command
pub fn extract_command(
    path: PathBuf,
    config: ExtractionConfig,
    mime_type: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();

    let result = extract_file_sync(&path_str, mime_type.as_deref(), &config).with_context(|| {
        format!(
            "Failed to extract file '{}'. Ensure the file is readable and the format is supported.",
            path.display()
        )
    })?;

    match format {
        OutputFormat::Text => {
            println!("{}", result.content);
        }
        OutputFormat::Json => {
            let output = json!({
                "content": result.content,
                "mime_type": result.mime_type,
                "metadata": result.metadata,
                "tables": result.tables.iter().map(|t| json!({
                    "cells": t.cells,
                    "markdown": t.markdown,
                    "page_number": t.page_number,
                })).collect::<Vec<_>>(),
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).context("Failed to serialize extraction result to JSON")?
            );
        }
    }

    Ok(())
}

/// Execute batch extraction command
pub fn batch_command(paths: Vec<PathBuf>, config: ExtractionConfig, format: OutputFormat) -> Result<()> {
    let path_strs: Vec<String> = paths.iter().map(|p| p.to_string_lossy().to_string()).collect();

    let results = batch_extract_file_sync(path_strs, &config).with_context(|| {
        format!(
            "Failed to batch extract {} documents. Check that all files are readable and formats are supported.",
            paths.len()
        )
    })?;

    match format {
        OutputFormat::Text => {
            for (i, result) in results.iter().enumerate() {
                println!("=== Document {} ===", i + 1);
                println!("MIME Type: {}", result.mime_type);
                println!("Content:\n{}", result.content);
                println!();
            }
        }
        OutputFormat::Json => {
            let output: Vec<_> = results
                .iter()
                .map(|result| {
                    json!({
                        "content": result.content,
                        "mime_type": result.mime_type,
                        "metadata": result.metadata,
                        "tables": result.tables.iter().map(|t| json!({
                            "cells": t.cells,
                            "markdown": t.markdown,
                            "page_number": t.page_number,
                        })).collect::<Vec<_>>(),
                    })
                })
                .collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&output)
                    .context("Failed to serialize batch extraction results to JSON")?
            );
        }
    }

    Ok(())
}

/// Apply extraction CLI overrides to config
pub fn apply_extraction_overrides(
    config: &mut ExtractionConfig,
    ocr: Option<bool>,
    force_ocr: Option<bool>,
    no_cache: Option<bool>,
    chunk: Option<bool>,
    chunk_size: Option<usize>,
    chunk_overlap: Option<usize>,
    quality: Option<bool>,
    detect_language: Option<bool>,
    content_format: Option<ContentOutputFormatArg>,
) {
    if let Some(ocr_flag) = ocr {
        if ocr_flag {
            config.ocr = Some(OcrConfig {
                backend: "tesseract".to_string(),
                language: "eng".to_string(),
                tesseract_config: None,
            });
        } else {
            config.ocr = None;
        }
    }
    if let Some(force_ocr_flag) = force_ocr {
        config.force_ocr = force_ocr_flag;
    }
    if let Some(no_cache_flag) = no_cache {
        config.use_cache = !no_cache_flag;
    }
    if let Some(chunk_flag) = chunk {
        if chunk_flag {
            let max_chars = chunk_size.unwrap_or(1000);
            let max_overlap = chunk_overlap.unwrap_or(200);
            config.chunking = Some(ChunkingConfig {
                max_chars,
                max_overlap,
                embedding: None,
                preset: None,
            });
        } else {
            config.chunking = None;
        }
    } else if let Some(ref mut chunking) = config.chunking {
        if let Some(max_chars) = chunk_size {
            chunking.max_chars = max_chars;
        }
        if let Some(max_overlap) = chunk_overlap {
            chunking.max_overlap = max_overlap;
        }
    }
    if let Some(quality_flag) = quality {
        config.enable_quality_processing = quality_flag;
    }
    if let Some(detect_language_flag) = detect_language {
        if detect_language_flag {
            config.language_detection = Some(LanguageDetectionConfig {
                enabled: true,
                min_confidence: 0.8,
                detect_multiple: false,
            });
        } else {
            config.language_detection = None;
        }
    }
    if let Some(content_fmt) = content_format {
        config.output_format = content_fmt.into();
    }
}
