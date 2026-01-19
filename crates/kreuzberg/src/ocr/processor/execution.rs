//! OCR execution and result processing.
//!
//! This module handles the core OCR execution logic, including image processing,
//! text extraction, and result formatting.

use super::config::{apply_tesseract_variables, hash_config};
use super::validation::{resolve_tessdata_path, strip_control_characters, validate_language_and_traineddata};
use crate::ocr::cache::OcrCache;
use crate::ocr::error::OcrError;
use crate::ocr::hocr::convert_hocr_to_markdown;
use crate::ocr::table::{extract_words_from_tsv, reconstruct_table, table_to_markdown};
use crate::ocr::types::{BatchItemResult, TesseractConfig};
use crate::types::{OcrExtractionResult, OcrTable};
use kreuzberg_tesseract::{TessPageSegMode, TesseractAPI};
use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

/// CI debug logging utility.
///
/// Logs debug messages when KREUZBERG_CI_DEBUG environment variable is set.
fn log_ci_debug<F>(enabled: bool, stage: &str, details: F)
where
    F: FnOnce() -> String,
{
    if !enabled {
        return;
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    tracing::debug!("[ci-debug][ocr::processor::{stage}] {timestamp:.3}s {}", details());
}

/// Perform OCR on an image using Tesseract.
///
/// This function handles the complete OCR pipeline:
/// 1. Image loading and preprocessing
/// 2. Tesseract initialization and configuration
/// 3. Text recognition
/// 4. Output formatting (text, markdown, hOCR, or TSV)
/// 5. Optional table detection
///
/// # Arguments
///
/// * `image_bytes` - Raw image data
/// * `config` - OCR configuration
///
/// # Returns
///
/// OCR extraction result containing text and optional tables
pub(super) fn perform_ocr(image_bytes: &[u8], config: &TesseractConfig) -> Result<OcrExtractionResult, OcrError> {
    let ci_debug_enabled = env::var_os("KREUZBERG_CI_DEBUG").is_some();
    log_ci_debug(ci_debug_enabled, "perform_ocr:start", || {
        format!(
            "bytes={} language={} output={} use_cache={}",
            image_bytes.len(),
            config.language,
            config.output_format,
            config.use_cache
        )
    });

    let img = image::load_from_memory(image_bytes)
        .map_err(|e| OcrError::ImageProcessingFailed(format!("Failed to decode image: {}", e)))?;

    let rgb_image = img.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let bytes_per_pixel = 3;
    let bytes_per_line = width * bytes_per_pixel;

    log_ci_debug(ci_debug_enabled, "image", || {
        format!(
            "dimensions={}x{} bytes_per_line={} color_type=RGB8",
            width, height, bytes_per_line
        )
    });

    let api = TesseractAPI::new();
    let tessdata_path = resolve_tessdata_path();

    log_ci_debug(ci_debug_enabled, "tessdata", || {
        let path_preview = env::var_os("PATH").map(|paths| {
            env::split_paths(&paths)
                .take(6)
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        });
        let resolved_exists = !tessdata_path.is_empty() && std::path::Path::new(&tessdata_path).exists();

        format!(
            "env={:?} resolved={} exists={} path_preview={:?}",
            env::var("TESSDATA_PREFIX").ok(),
            if tessdata_path.is_empty() {
                "unset"
            } else {
                &tessdata_path
            },
            resolved_exists,
            path_preview
        )
    });

    log_ci_debug(ci_debug_enabled, "tesseract_version", || {
        format!("version={}", TesseractAPI::version())
    });

    // Validate language and traineddata files
    validate_language_and_traineddata(&config.language, &tessdata_path)?;

    let init_result = api.init(&tessdata_path, &config.language);
    log_ci_debug(ci_debug_enabled, "init", || match &init_result {
        Ok(_) => format!("language={} datapath='{}'", config.language, tessdata_path),
        Err(err) => format!(
            "language={} datapath='{}' error={:?}",
            config.language, tessdata_path, err
        ),
    });

    init_result.map_err(|e| {
        OcrError::TesseractInitializationFailed(format!("Failed to initialize language '{}': {}", config.language, e))
    })?;

    if ci_debug_enabled {
        match api.get_available_languages() {
            Ok(languages) => {
                log_ci_debug(ci_debug_enabled, "available_languages", move || {
                    let preview = languages.iter().take(10).cloned().collect::<Vec<_>>();
                    format!("count={} preview={:?}", languages.len(), preview)
                });
            }
            Err(err) => {
                log_ci_debug(ci_debug_enabled, "available_languages_error", move || {
                    format!("error={:?}", err)
                });
            }
        }
    }

    let psm_mode = TessPageSegMode::from_int(config.psm as i32);
    let psm_result = api.set_page_seg_mode(psm_mode);
    log_ci_debug(ci_debug_enabled, "set_psm", || match &psm_result {
        Ok(_) => format!("mode={}", config.psm),
        Err(err) => format!("error={:?}", err),
    });
    psm_result.map_err(|e| OcrError::InvalidConfiguration(format!("Failed to set PSM mode: {}", e)))?;

    apply_tesseract_variables(&api, config)?;

    api.set_image(
        rgb_image.as_raw(),
        width as i32,
        height as i32,
        bytes_per_pixel as i32,
        bytes_per_line as i32,
    )
    .map_err(|e| OcrError::ProcessingFailed(format!("Failed to set image: {}", e)))?;

    log_ci_debug(ci_debug_enabled, "set_image", || {
        format!(
            "width={} height={} bytes_per_pixel={} bytes_per_line={}",
            width, height, bytes_per_pixel, bytes_per_line
        )
    });

    api.recognize()
        .map_err(|e| OcrError::ProcessingFailed(format!("Failed to recognize text: {}", e)))?;

    log_ci_debug(ci_debug_enabled, "recognize", || "completed".to_string());

    let tsv_data_for_tables = if config.enable_table_detection || config.output_format == "tsv" {
        Some(
            api.get_tsv_text(0)
                .map_err(|e| OcrError::ProcessingFailed(format!("Failed to extract TSV: {}", e)))?,
        )
    } else {
        None
    };

    let (raw_content, mime_type) = match config.output_format.as_str() {
        "text" => {
            let text = api
                .get_utf8_text()
                .map_err(|e| OcrError::ProcessingFailed(format!("Failed to extract text: {}", e)))?;
            (text, "text/plain".to_string())
        }
        "markdown" => {
            let hocr = api
                .get_hocr_text(0)
                .map_err(|e| OcrError::ProcessingFailed(format!("Failed to extract hOCR: {}", e)))?;

            let markdown = convert_hocr_to_markdown(&hocr, None)?;
            (markdown, "text/markdown".to_string())
        }
        "hocr" => {
            let hocr = api
                .get_hocr_text(0)
                .map_err(|e| OcrError::ProcessingFailed(format!("Failed to extract hOCR: {}", e)))?;
            (hocr, "text/html".to_string())
        }
        "tsv" => {
            let tsv = tsv_data_for_tables
                .as_ref()
                .expect("TSV data should be extracted when output_format is 'tsv'")
                .clone();
            (tsv, "text/plain".to_string())
        }
        _ => {
            return Err(OcrError::InvalidConfiguration(format!(
                "Unsupported output format: {}",
                config.output_format
            )));
        }
    };

    let mut metadata = HashMap::new();
    metadata.insert(
        "language".to_string(),
        serde_json::Value::String(config.language.clone()),
    );
    metadata.insert("psm".to_string(), serde_json::Value::String(config.psm.to_string()));
    metadata.insert(
        "output_format".to_string(),
        serde_json::Value::String(config.output_format.clone()),
    );
    metadata.insert("table_count".to_string(), serde_json::Value::String("0".to_string()));
    metadata.insert(
        "tables_detected".to_string(),
        serde_json::Value::String("0".to_string()),
    );
    if config.output_format == "markdown" {
        metadata.insert(
            "source_format".to_string(),
            serde_json::Value::String("hocr".to_string()),
        );
    }

    let mut tables = Vec::new();

    if config.enable_table_detection {
        let tsv_data = tsv_data_for_tables.unwrap();

        let words = extract_words_from_tsv(&tsv_data, config.table_min_confidence)?;

        if !words.is_empty() {
            let table = reconstruct_table(&words, config.table_column_threshold, config.table_row_threshold_ratio);
            if !table.is_empty() {
                metadata.insert("table_count".to_string(), serde_json::Value::String("1".to_string()));
                metadata.insert(
                    "tables_detected".to_string(),
                    serde_json::Value::String("1".to_string()),
                );
                metadata.insert(
                    "table_rows".to_string(),
                    serde_json::Value::String(table.len().to_string()),
                );
                metadata.insert(
                    "table_cols".to_string(),
                    serde_json::Value::String(table[0].len().to_string()),
                );

                let markdown_table = table_to_markdown(&table);
                tables.push(OcrTable {
                    cells: table,
                    markdown: markdown_table,
                    page_number: 0,
                });
            }
        }
    }

    let content = strip_control_characters(&raw_content);

    Ok(OcrExtractionResult {
        content,
        mime_type,
        metadata,
        tables,
    })
}

/// Process an image file and return OCR results.
///
/// # Arguments
///
/// * `file_path` - Path to image file
/// * `config` - OCR configuration
/// * `cache` - Cache instance
///
/// # Returns
///
/// OCR extraction result
pub(super) fn process_file_with_cache(
    file_path: &str,
    config: &TesseractConfig,
    cache: &OcrCache,
) -> Result<OcrExtractionResult, OcrError> {
    let image_bytes = std::fs::read(file_path)
        .map_err(|e| OcrError::IOError(format!("Failed to read file '{}': {}", file_path, e)))?;
    process_image_with_cache(&image_bytes, config, cache)
}

/// Process an image and return OCR results, using cache if enabled.
///
/// # Arguments
///
/// * `image_bytes` - Raw image data
/// * `config` - OCR configuration
/// * `cache` - Cache instance
///
/// # Returns
///
/// OCR extraction result
pub(super) fn process_image_with_cache(
    image_bytes: &[u8],
    config: &TesseractConfig,
    cache: &OcrCache,
) -> Result<OcrExtractionResult, OcrError> {
    config.validate().map_err(OcrError::InvalidConfiguration)?;

    let mut hasher = ahash::AHasher::default();
    use std::hash::{Hash, Hasher};
    image_bytes.hash(&mut hasher);
    let image_hash = format!("{:016x}", hasher.finish());

    let config_str = hash_config(config);

    if config.use_cache
        && let Some(cached_result) = cache.get_cached_result(&image_hash, "tesseract", &config_str)?
    {
        #[cfg(feature = "otel")]
        tracing::Span::current().record("cache.hit", true);
        return Ok(cached_result);
    }

    #[cfg(feature = "otel")]
    tracing::Span::current().record("cache.hit", false);

    let result = perform_ocr(image_bytes, config)?;

    if config.use_cache {
        let _ = cache.set_cached_result(&image_hash, "tesseract", &config_str, &result);
    }

    Ok(result)
}

/// Process multiple image files in parallel using Rayon.
///
/// This method processes OCR operations in parallel across CPU cores for improved throughput.
/// Results are returned in the same order as the input file paths.
pub(super) fn process_files_batch(
    file_paths: Vec<String>,
    config: &TesseractConfig,
    cache: &OcrCache,
) -> Vec<BatchItemResult> {
    use rayon::prelude::*;

    file_paths
        .par_iter()
        .map(|path| match process_file_with_cache(path, config, cache) {
            Ok(result) => BatchItemResult {
                file_path: path.clone(),
                success: true,
                result: Some(result),
                error: None,
            },
            Err(e) => BatchItemResult {
                file_path: path.clone(),
                success: false,
                result: None,
                error: Some(e.to_string()),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_compute_image_hash_deterministic() {
        use ahash::AHasher;
        use std::hash::{Hash, Hasher};

        let image_bytes = vec![1, 2, 3, 4, 5];

        let mut hasher1 = AHasher::default();
        image_bytes.hash(&mut hasher1);
        let hash1 = format!("{:016x}", hasher1.finish());

        let mut hasher2 = AHasher::default();
        image_bytes.hash(&mut hasher2);
        let hash2 = format!("{:016x}", hasher2.finish());

        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 16);
    }

    #[test]
    fn test_compute_image_hash_different_data() {
        use ahash::AHasher;
        use std::hash::{Hash, Hasher};

        let image_bytes1 = vec![1, 2, 3, 4, 5];
        let image_bytes2 = vec![5, 4, 3, 2, 1];

        let mut hasher1 = AHasher::default();
        image_bytes1.hash(&mut hasher1);
        let hash1 = format!("{:016x}", hasher1.finish());

        let mut hasher2 = AHasher::default();
        image_bytes2.hash(&mut hasher2);
        let hash2 = format!("{:016x}", hasher2.finish());

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_log_ci_debug_disabled() {
        log_ci_debug(false, "test_stage", || "test message".to_string());
    }

    #[test]
    fn test_log_ci_debug_enabled() {
        log_ci_debug(true, "test_stage", || "test message".to_string());
    }

    #[test]
    fn test_process_file_nonexistent() {
        let temp_dir = tempdir().unwrap();
        let cache = OcrCache::new(Some(temp_dir.path().to_path_buf())).unwrap();
        let config = TesseractConfig {
            output_format: "text".to_string(),
            enable_table_detection: false,
            use_cache: false,
            ..TesseractConfig::default()
        };

        let result = process_file_with_cache("/nonexistent/file.png", &config, &cache);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to read file"));
    }

    #[test]
    fn test_process_image_invalid_image_data() {
        let temp_dir = tempdir().unwrap();
        let cache = OcrCache::new(Some(temp_dir.path().to_path_buf())).unwrap();
        let config = TesseractConfig {
            output_format: "text".to_string(),
            enable_table_detection: false,
            use_cache: false,
            ..TesseractConfig::default()
        };

        let invalid_data = vec![0, 1, 2, 3, 4];
        let result = process_image_with_cache(&invalid_data, &config, &cache);

        assert!(result.is_err());
    }
}
