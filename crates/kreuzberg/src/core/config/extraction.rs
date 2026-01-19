//! Main extraction configuration and environment variable handling.
//!
//! This module contains the main `ExtractionConfig` struct and related utilities
//! for loading configuration from files and applying environment variable overrides.

use crate::{KreuzbergError, Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::time::SystemTime;

use super::formats::OutputFormat;
use super::ocr::OcrConfig;
use super::page::PageConfig;
use super::processing::{ChunkingConfig, PostProcessorConfig};

/// Image extraction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageExtractionConfig {
    /// Extract images from documents
    #[serde(default = "default_true")]
    pub extract_images: bool,

    /// Target DPI for image normalization
    #[serde(default = "default_target_dpi")]
    pub target_dpi: i32,

    /// Maximum dimension for images (width or height)
    #[serde(default = "default_max_dimension")]
    pub max_image_dimension: i32,

    /// Automatically adjust DPI based on image content
    #[serde(default = "default_true")]
    pub auto_adjust_dpi: bool,

    /// Minimum DPI threshold
    #[serde(default = "default_min_dpi")]
    pub min_dpi: i32,

    /// Maximum DPI threshold
    #[serde(default = "default_max_dpi")]
    pub max_dpi: i32,
}

/// Token reduction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenReductionConfig {
    /// Reduction mode: "off", "light", "moderate", "aggressive", "maximum"
    #[serde(default = "default_reduction_mode")]
    pub mode: String,

    /// Preserve important words (capitalized, technical terms)
    #[serde(default = "default_true")]
    pub preserve_important_words: bool,
}

/// Language detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetectionConfig {
    /// Enable language detection
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Minimum confidence threshold (0.0-1.0)
    #[serde(default = "default_confidence")]
    pub min_confidence: f64,

    /// Detect multiple languages in the document
    #[serde(default)]
    pub detect_multiple: bool,
}

static CONFIG_CACHE: LazyLock<DashMap<PathBuf, (SystemTime, Arc<ExtractionConfig>)>> = LazyLock::new(DashMap::new);

/// Main extraction configuration.
///
/// This struct contains all configuration options for the extraction process.
/// It can be loaded from TOML, YAML, or JSON files, or created programmatically.
///
/// # Example
///
/// ```rust
/// use kreuzberg::core::config::ExtractionConfig;
///
/// // Create with defaults
/// let config = ExtractionConfig::default();
///
/// // Load from TOML file
/// // let config = ExtractionConfig::from_toml_file("kreuzberg.toml")?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    /// Enable caching of extraction results
    #[serde(default = "default_true")]
    pub use_cache: bool,

    /// Enable quality post-processing
    #[serde(default = "default_true")]
    pub enable_quality_processing: bool,

    /// OCR configuration (None = OCR disabled)
    #[serde(default)]
    pub ocr: Option<OcrConfig>,

    /// Force OCR even for searchable PDFs
    #[serde(default)]
    pub force_ocr: bool,

    /// Text chunking configuration (None = chunking disabled)
    #[serde(default)]
    pub chunking: Option<ChunkingConfig>,

    /// Image extraction configuration (None = no image extraction)
    #[serde(default)]
    pub images: Option<ImageExtractionConfig>,

    /// PDF-specific options (None = use defaults)
    #[cfg(feature = "pdf")]
    #[serde(default)]
    pub pdf_options: Option<super::pdf::PdfConfig>,

    /// Token reduction configuration (None = no token reduction)
    #[serde(default)]
    pub token_reduction: Option<TokenReductionConfig>,

    /// Language detection configuration (None = no language detection)
    #[serde(default)]
    pub language_detection: Option<LanguageDetectionConfig>,

    /// Page extraction configuration (None = no page tracking)
    #[serde(default)]
    pub pages: Option<PageConfig>,

    /// Keyword extraction configuration (None = no keyword extraction)
    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
    #[serde(default)]
    pub keywords: Option<crate::keywords::KeywordConfig>,

    /// Post-processor configuration (None = use defaults)
    #[serde(default)]
    pub postprocessor: Option<PostProcessorConfig>,

    /// HTML to Markdown conversion options (None = use defaults)
    ///
    /// Configure how HTML documents are converted to Markdown, including heading styles,
    /// list formatting, code block styles, and preprocessing options.
    #[cfg(feature = "html")]
    #[serde(default)]
    pub html_options: Option<html_to_markdown_rs::ConversionOptions>,

    /// Maximum concurrent extractions in batch operations (None = num_cpus * 2).
    ///
    /// Limits parallelism to prevent resource exhaustion when processing
    /// large batches. Defaults to twice the number of CPU cores.
    #[serde(default)]
    pub max_concurrent_extractions: Option<usize>,

    /// Result structure format
    ///
    /// Controls whether results are returned in unified format (default) with all
    /// content in the `content` field, or element-based format with semantic
    /// elements (for Unstructured-compatible output).
    #[serde(default)]
    pub result_format: crate::types::OutputFormat,

    /// Content text format (default: Plain).
    ///
    /// Controls the format of the extracted content:
    /// - `Plain`: Raw extracted text (default)
    /// - `Markdown`: Markdown formatted output
    /// - `Djot`: Djot markup format (requires djot feature)
    /// - `Html`: HTML formatted output
    ///
    /// When set to a structured format, extraction results will include
    /// formatted output. The `formatted_content` field may be populated
    /// when format conversion is applied.
    #[serde(default)]
    pub output_format: OutputFormat,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            use_cache: true,
            enable_quality_processing: true,
            ocr: None,
            force_ocr: false,
            chunking: None,
            images: None,
            #[cfg(feature = "pdf")]
            pdf_options: None,
            token_reduction: None,
            language_detection: None,
            pages: None,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            keywords: None,
            postprocessor: None,
            #[cfg(feature = "html")]
            html_options: None,
            max_concurrent_extractions: None,
            result_format: crate::types::OutputFormat::Unified,
            output_format: OutputFormat::Plain,
        }
    }
}

impl ExtractionConfig {
    /// Check if image processing is needed by examining OCR and image extraction settings.
    ///
    /// Returns `true` if either OCR is enabled or image extraction is configured,
    /// indicating that image decompression and processing should occur.
    /// Returns `false` if both are disabled, allowing optimization to skip unnecessary
    /// image decompression for text-only extraction workflows.
    ///
    /// # Optimization Impact
    /// For text-only extractions (no OCR, no image extraction), skipping image
    /// decompression can improve CPU utilization by 5-10% by avoiding wasteful
    /// image I/O and processing when results won't be used.
    pub fn needs_image_processing(&self) -> bool {
        let ocr_enabled = self.ocr.is_some();

        let image_extraction_enabled = self.images.as_ref().map(|i| i.extract_images).unwrap_or(false);

        ocr_enabled || image_extraction_enabled
    }

    /// Apply environment variable overrides to configuration.
    ///
    /// Environment variables have the highest precedence and will override any values
    /// loaded from configuration files. This method supports the following environment variables:
    ///
    /// - `KREUZBERG_OCR_LANGUAGE`: OCR language (ISO 639-1 or 639-3 code, e.g., "eng", "fra", "deu")
    /// - `KREUZBERG_OCR_BACKEND`: OCR backend ("tesseract", "easyocr", or "paddleocr")
    /// - `KREUZBERG_CHUNKING_MAX_CHARS`: Maximum characters per chunk (positive integer)
    /// - `KREUZBERG_CHUNKING_MAX_OVERLAP`: Maximum overlap between chunks (non-negative integer)
    /// - `KREUZBERG_CACHE_ENABLED`: Cache enabled flag ("true" or "false")
    /// - `KREUZBERG_TOKEN_REDUCTION_MODE`: Token reduction mode ("off", "light", "moderate", "aggressive", or "maximum")
    ///
    /// # Behavior
    ///
    /// - If an environment variable is set and valid, it overrides the current configuration value
    /// - If a required parent config is `None` (e.g., `self.ocr` is None), it's created with defaults before applying the override
    /// - Invalid values return a `KreuzbergError::Validation` with helpful error messages
    /// - Missing or unset environment variables are silently ignored
    ///
    /// # Example
    ///
    /// ```rust
    /// # use kreuzberg::core::config::ExtractionConfig;
    /// # fn example() -> kreuzberg::Result<()> {
    /// let mut config = ExtractionConfig::from_file("config.toml")?;
    /// // Set KREUZBERG_OCR_LANGUAGE=fra before calling
    /// config.apply_env_overrides()?; // OCR language is now "fra"
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if:
    /// - An environment variable contains an invalid value
    /// - A number cannot be parsed as the expected type
    /// - A boolean is not "true" or "false"
    pub fn apply_env_overrides(&mut self) -> Result<()> {
        use crate::core::config_validation::{
            validate_chunking_params, validate_language_code, validate_ocr_backend, validate_token_reduction_level,
        };

        // KREUZBERG_OCR_LANGUAGE override
        if let Ok(lang) = std::env::var("KREUZBERG_OCR_LANGUAGE") {
            validate_language_code(&lang)?;
            if self.ocr.is_none() {
                self.ocr = Some(OcrConfig::default());
            }
            if let Some(ref mut ocr) = self.ocr {
                ocr.language = lang;
            }
        }

        // KREUZBERG_OCR_BACKEND override
        if let Ok(backend) = std::env::var("KREUZBERG_OCR_BACKEND") {
            validate_ocr_backend(&backend)?;
            if self.ocr.is_none() {
                self.ocr = Some(OcrConfig::default());
            }
            if let Some(ref mut ocr) = self.ocr {
                ocr.backend = backend;
            }
        }

        // KREUZBERG_CHUNKING_MAX_CHARS override
        if let Ok(max_chars_str) = std::env::var("KREUZBERG_CHUNKING_MAX_CHARS") {
            let max_chars: usize = max_chars_str.parse().map_err(|_| KreuzbergError::Validation {
                message: format!(
                    "Invalid value for KREUZBERG_CHUNKING_MAX_CHARS: '{}'. Must be a positive integer.",
                    max_chars_str
                ),
                source: None,
            })?;

            if max_chars == 0 {
                return Err(KreuzbergError::Validation {
                    message: "KREUZBERG_CHUNKING_MAX_CHARS must be greater than 0".to_string(),
                    source: None,
                });
            }

            if self.chunking.is_none() {
                self.chunking = Some(ChunkingConfig {
                    max_chars: 1000,
                    max_overlap: 200,
                    embedding: None,
                    preset: None,
                });
            }

            if let Some(ref mut chunking) = self.chunking {
                // Validate against current overlap before updating
                validate_chunking_params(max_chars, chunking.max_overlap)?;
                chunking.max_chars = max_chars;
            }
        }

        // KREUZBERG_CHUNKING_MAX_OVERLAP override
        if let Ok(max_overlap_str) = std::env::var("KREUZBERG_CHUNKING_MAX_OVERLAP") {
            let max_overlap: usize = max_overlap_str.parse().map_err(|_| KreuzbergError::Validation {
                message: format!(
                    "Invalid value for KREUZBERG_CHUNKING_MAX_OVERLAP: '{}'. Must be a non-negative integer.",
                    max_overlap_str
                ),
                source: None,
            })?;

            if self.chunking.is_none() {
                self.chunking = Some(ChunkingConfig {
                    max_chars: 1000,
                    max_overlap: 200,
                    embedding: None,
                    preset: None,
                });
            }

            if let Some(ref mut chunking) = self.chunking {
                // Validate against current max_chars before updating
                validate_chunking_params(chunking.max_chars, max_overlap)?;
                chunking.max_overlap = max_overlap;
            }
        }

        // KREUZBERG_CACHE_ENABLED override
        if let Ok(cache_str) = std::env::var("KREUZBERG_CACHE_ENABLED") {
            let cache_enabled = match cache_str.to_lowercase().as_str() {
                "true" => true,
                "false" => false,
                _ => {
                    return Err(KreuzbergError::Validation {
                        message: format!(
                            "Invalid value for KREUZBERG_CACHE_ENABLED: '{}'. Must be 'true' or 'false'.",
                            cache_str
                        ),
                        source: None,
                    });
                }
            };
            self.use_cache = cache_enabled;
        }

        // KREUZBERG_TOKEN_REDUCTION_MODE override
        if let Ok(mode) = std::env::var("KREUZBERG_TOKEN_REDUCTION_MODE") {
            validate_token_reduction_level(&mode)?;
            if self.token_reduction.is_none() {
                self.token_reduction = Some(TokenReductionConfig {
                    mode: "off".to_string(),
                    preserve_important_words: true,
                });
            }
            if let Some(ref mut token_reduction) = self.token_reduction {
                token_reduction.mode = mode;
            }
        }

        // KREUZBERG_OUTPUT_FORMAT override
        if let Ok(val) = std::env::var("KREUZBERG_OUTPUT_FORMAT") {
            self.output_format = val.parse().map_err(|e: String| KreuzbergError::Validation {
                message: format!("Invalid value for KREUZBERG_OUTPUT_FORMAT: {}", e),
                source: None,
            })?;
        }

        Ok(())
    }

    /// Load configuration from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML file
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if file doesn't exist or is invalid TOML.
    pub fn from_toml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let metadata = std::fs::metadata(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        let mtime = metadata.modified().map_err(|e| {
            KreuzbergError::validation(format!("Failed to get modification time for {}: {}", path.display(), e))
        })?;

        if let Some(entry) = CONFIG_CACHE.get(path)
            && entry.0 == mtime
        {
            return Ok((*entry.1).clone());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let config: Self = toml::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid TOML in {}: {}", path.display(), e)))?;

        let config_arc = Arc::new(config.clone());
        CONFIG_CACHE.insert(path.to_path_buf(), (mtime, config_arc));

        Ok(config)
    }

    /// Load configuration from a YAML file.
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let metadata = std::fs::metadata(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        let mtime = metadata.modified().map_err(|e| {
            KreuzbergError::validation(format!("Failed to get modification time for {}: {}", path.display(), e))
        })?;

        if let Some(entry) = CONFIG_CACHE.get(path)
            && entry.0 == mtime
        {
            return Ok((*entry.1).clone());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let config: Self = serde_yaml_ng::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid YAML in {}: {}", path.display(), e)))?;

        let config_arc = Arc::new(config.clone());
        CONFIG_CACHE.insert(path.to_path_buf(), (mtime, config_arc));

        Ok(config)
    }

    /// Load configuration from a JSON file.
    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let metadata = std::fs::metadata(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        let mtime = metadata.modified().map_err(|e| {
            KreuzbergError::validation(format!("Failed to get modification time for {}: {}", path.display(), e))
        })?;

        if let Some(entry) = CONFIG_CACHE.get(path)
            && entry.0 == mtime
        {
            return Ok((*entry.1).clone());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let config: Self = serde_json::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid JSON in {}: {}", path.display(), e)))?;

        let config_arc = Arc::new(config.clone());
        CONFIG_CACHE.insert(path.to_path_buf(), (mtime, config_arc));

        Ok(config)
    }

    /// Load configuration from a file, auto-detecting format by extension.
    ///
    /// Supported formats:
    /// - `.toml` - TOML format
    /// - `.yaml` - YAML format
    /// - `.json` - JSON format
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if:
    /// - File doesn't exist
    /// - File extension is not supported
    /// - File content is invalid for the detected format
    ///
    /// # Example
    ///
    /// ```rust
    /// use kreuzberg::core::config::ExtractionConfig;
    ///
    /// // Auto-detects TOML format
    /// // let config = ExtractionConfig::from_file("kreuzberg.toml")?;
    ///
    /// // Auto-detects YAML format
    /// // let config = ExtractionConfig::from_file("kreuzberg.yaml")?;
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let metadata = std::fs::metadata(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        let mtime = metadata.modified().map_err(|e| {
            KreuzbergError::validation(format!("Failed to get modification time for {}: {}", path.display(), e))
        })?;

        if let Some(entry) = CONFIG_CACHE.get(path)
            && entry.0 == mtime
        {
            return Ok((*entry.1).clone());
        }

        let extension = path.extension().and_then(|ext| ext.to_str()).ok_or_else(|| {
            KreuzbergError::validation(format!(
                "Cannot determine file format: no extension found in {}",
                path.display()
            ))
        })?;

        let config = match extension.to_lowercase().as_str() {
            "toml" => Self::from_toml_file(path)?,
            "yaml" | "yml" => Self::from_yaml_file(path)?,
            "json" => Self::from_json_file(path)?,
            _ => {
                return Err(KreuzbergError::validation(format!(
                    "Unsupported config file format: .{}. Supported formats: .toml, .yaml, .json",
                    extension
                )));
            }
        };

        let config_arc = Arc::new(config.clone());
        CONFIG_CACHE.insert(path.to_path_buf(), (mtime, config_arc));

        Ok(config)
    }

    /// Discover configuration file in parent directories.
    ///
    /// Searches for `kreuzberg.toml` in current directory and parent directories.
    ///
    /// # Returns
    ///
    /// - `Some(config)` if found
    /// - `None` if no config file found
    pub fn discover() -> Result<Option<Self>> {
        let mut current = std::env::current_dir().map_err(KreuzbergError::Io)?;

        loop {
            let kreuzberg_toml = current.join("kreuzberg.toml");
            if kreuzberg_toml.exists() {
                return Ok(Some(Self::from_toml_file(kreuzberg_toml)?));
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        Ok(None)
    }
}

fn default_true() -> bool {
    true
}

fn default_target_dpi() -> i32 {
    300
}

fn default_max_dimension() -> i32 {
    4096
}

fn default_min_dpi() -> i32 {
    72
}

fn default_max_dpi() -> i32 {
    600
}

fn default_reduction_mode() -> String {
    "off".to_string()
}

fn default_confidence() -> f64 {
    0.8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ExtractionConfig::default();
        assert!(config.use_cache);
        assert!(config.enable_quality_processing);
        assert!(config.ocr.is_none());
    }

    #[test]
    fn test_needs_image_processing() {
        let mut config = ExtractionConfig::default();
        assert!(!config.needs_image_processing());

        config.ocr = Some(OcrConfig::default());
        assert!(config.needs_image_processing());
    }
}
