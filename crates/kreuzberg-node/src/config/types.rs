use napi::bindgen_prelude::*;
use napi_derive::napi;

use html_to_markdown_rs::options::{
    CodeBlockStyle, ConversionOptions, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle,
    PreprocessingOptions as HtmlPreprocessingOptions, PreprocessingPreset, WhitespaceMode,
};
use kreuzberg::keywords::{
    KeywordAlgorithm as RustKeywordAlgorithm, KeywordConfig as RustKeywordConfig, RakeParams as RustRakeParams,
    YakeParams as RustYakeParams,
};
use kreuzberg::pdf::HierarchyConfig as RustHierarchyConfig;
use kreuzberg::{
    ChunkingConfig as RustChunkingConfig, EmbeddingConfig as RustEmbeddingConfig,
    EmbeddingModelType as RustEmbeddingModelType, ExtractionConfig, ImageExtractionConfig as RustImageExtractionConfig,
    LanguageDetectionConfig as RustLanguageDetectionConfig, OcrConfig as RustOcrConfig, PdfConfig as RustPdfConfig,
    PostProcessorConfig as RustPostProcessorConfig, TesseractConfig as RustTesseractConfig,
    TokenReductionConfig as RustTokenReductionConfig,
};
use std::ffi::c_char;

#[allow(improper_ctypes)]
unsafe extern "C" {
    /// Parse HeadingStyle from string to discriminant
    pub fn kreuzberg_parse_heading_style(value: *const c_char) -> i32;

    /// Parse CodeBlockStyle from string to discriminant
    pub fn kreuzberg_parse_code_block_style(value: *const c_char) -> i32;

    /// Parse HighlightStyle from string to discriminant
    pub fn kreuzberg_parse_highlight_style(value: *const c_char) -> i32;

    /// Parse ListIndentType from string to discriminant
    pub fn kreuzberg_parse_list_indent_type(value: *const c_char) -> i32;

    /// Parse WhitespaceMode from string to discriminant
    pub fn kreuzberg_parse_whitespace_mode(value: *const c_char) -> i32;

    /// Parse NewlineStyle from string to discriminant
    pub fn kreuzberg_parse_newline_style(value: *const c_char) -> i32;

    /// Parse PreprocessingPreset from string to discriminant
    pub fn kreuzberg_parse_preprocessing_preset(value: *const c_char) -> i32;
}

#[napi(object)]
pub struct JsOcrConfig {
    pub backend: String,
    pub language: Option<String>,
    pub tesseract_config: Option<JsTesseractConfig>,
}

impl From<JsOcrConfig> for RustOcrConfig {
    fn from(val: JsOcrConfig) -> Self {
        RustOcrConfig {
            backend: val.backend,
            language: val.language.unwrap_or_else(|| "eng".to_string()),
            tesseract_config: val.tesseract_config.map(Into::into),
        }
    }
}

#[napi(object)]
pub struct JsTesseractConfig {
    pub psm: Option<i32>,
    pub enable_table_detection: Option<bool>,
    pub tessedit_char_whitelist: Option<String>,
}

impl From<JsTesseractConfig> for RustTesseractConfig {
    fn from(val: JsTesseractConfig) -> Self {
        let mut config = RustTesseractConfig::default();
        if let Some(psm) = val.psm {
            config.psm = psm;
        }
        if let Some(enabled) = val.enable_table_detection {
            config.enable_table_detection = enabled;
        }
        if let Some(whitelist) = val.tessedit_char_whitelist {
            config.tessedit_char_whitelist = whitelist;
        }
        config
    }
}

/// Embedding model type configuration for Node.js bindings.
///
/// This struct represents different embedding model sources:
/// - `preset`: Use a named preset (e.g., "balanced", "fast", "quality", "multilingual")
/// - `fastembed`: Use a FastEmbed model with custom dimensions
/// - `custom`: Use a custom ONNX model
#[napi(object)]
pub struct JsEmbeddingModelType {
    /// Type of model: "preset", "fastembed", or "custom"
    pub model_type: String,
    /// For preset: preset name; for fastembed/custom: model ID
    pub value: String,
    /// Number of dimensions (only for fastembed/custom)
    pub dimensions: Option<u32>,
}

impl From<JsEmbeddingModelType> for RustEmbeddingModelType {
    fn from(val: JsEmbeddingModelType) -> Self {
        match val.model_type.as_str() {
            "preset" => RustEmbeddingModelType::Preset { name: val.value },
            "fastembed" => RustEmbeddingModelType::FastEmbed {
                model: val.value,
                dimensions: val.dimensions.unwrap_or(768) as usize,
            },
            "custom" => RustEmbeddingModelType::Custom {
                model_id: val.value,
                dimensions: val.dimensions.unwrap_or(512) as usize,
            },
            _ => RustEmbeddingModelType::Preset {
                name: "balanced".to_string(),
            },
        }
    }
}

/// Embedding generation configuration for Node.js bindings.
#[napi(object)]
pub struct JsEmbeddingConfig {
    /// Embedding model configuration
    pub model: Option<JsEmbeddingModelType>,
    /// Whether to normalize embeddings (L2 normalization)
    pub normalize: Option<bool>,
    /// Batch size for embedding generation
    pub batch_size: Option<u32>,
    /// Whether to show download progress for models
    pub show_download_progress: Option<bool>,
    /// Custom cache directory for model storage
    pub cache_dir: Option<String>,
}

impl From<JsEmbeddingConfig> for RustEmbeddingConfig {
    fn from(val: JsEmbeddingConfig) -> Self {
        RustEmbeddingConfig {
            model: val.model.map(Into::into).unwrap_or(RustEmbeddingModelType::Preset {
                name: "balanced".to_string(),
            }),
            normalize: val.normalize.unwrap_or(true),
            batch_size: val.batch_size.unwrap_or(32) as usize,
            show_download_progress: val.show_download_progress.unwrap_or(false),
            cache_dir: val.cache_dir.map(std::path::PathBuf::from),
        }
    }
}

#[napi(object)]
pub struct JsChunkingConfig {
    pub max_chars: Option<u32>,
    pub max_overlap: Option<u32>,
    /// Optional embedding configuration for generating embeddings
    pub embedding: Option<JsEmbeddingConfig>,
    /// Optional preset name for chunking parameters
    pub preset: Option<String>,
}

impl From<JsChunkingConfig> for RustChunkingConfig {
    fn from(val: JsChunkingConfig) -> Self {
        RustChunkingConfig {
            max_chars: val.max_chars.unwrap_or(1000) as usize,
            max_overlap: val.max_overlap.unwrap_or(200) as usize,
            embedding: val.embedding.map(Into::into),
            preset: val.preset,
        }
    }
}

#[napi(object)]
pub struct JsLanguageDetectionConfig {
    pub enabled: Option<bool>,
    pub min_confidence: Option<f64>,
    pub detect_multiple: Option<bool>,
}

impl From<JsLanguageDetectionConfig> for RustLanguageDetectionConfig {
    fn from(val: JsLanguageDetectionConfig) -> Self {
        RustLanguageDetectionConfig {
            enabled: val.enabled.unwrap_or(true),
            min_confidence: val.min_confidence.unwrap_or(0.8),
            detect_multiple: val.detect_multiple.unwrap_or(false),
        }
    }
}

#[napi(object)]
pub struct JsTokenReductionConfig {
    pub mode: Option<String>,
    pub preserve_important_words: Option<bool>,
}

impl From<JsTokenReductionConfig> for RustTokenReductionConfig {
    fn from(val: JsTokenReductionConfig) -> Self {
        RustTokenReductionConfig {
            mode: val.mode.unwrap_or_else(|| "off".to_string()),
            preserve_important_words: val.preserve_important_words.unwrap_or(true),
        }
    }
}

#[napi(object)]
pub struct JsHierarchyConfig {
    pub enabled: Option<bool>,
    pub k_clusters: Option<i32>,
    pub include_bbox: Option<bool>,
    pub ocr_coverage_threshold: Option<f64>,
}

impl From<JsHierarchyConfig> for RustHierarchyConfig {
    fn from(val: JsHierarchyConfig) -> Self {
        RustHierarchyConfig {
            enabled: val.enabled.unwrap_or(true),
            k_clusters: val.k_clusters.map(|v| v as usize).unwrap_or(6),
            include_bbox: val.include_bbox.unwrap_or(true),
            ocr_coverage_threshold: val.ocr_coverage_threshold.map(|v| v as f32),
        }
    }
}

#[napi(object)]
pub struct JsPdfConfig {
    pub extract_images: Option<bool>,
    pub passwords: Option<Vec<String>>,
    pub extract_metadata: Option<bool>,
    pub hierarchy: Option<JsHierarchyConfig>,
}

impl From<JsPdfConfig> for RustPdfConfig {
    fn from(val: JsPdfConfig) -> Self {
        RustPdfConfig {
            extract_images: val.extract_images.unwrap_or(false),
            passwords: val.passwords,
            extract_metadata: val.extract_metadata.unwrap_or(true),
            hierarchy: val.hierarchy.map(|h| h.into()),
        }
    }
}

#[napi(object)]
pub struct JsImageExtractionConfig {
    pub extract_images: Option<bool>,
    pub target_dpi: Option<i32>,
    pub max_image_dimension: Option<i32>,
    pub auto_adjust_dpi: Option<bool>,
    pub min_dpi: Option<i32>,
    pub max_dpi: Option<i32>,
}

impl From<JsImageExtractionConfig> for RustImageExtractionConfig {
    fn from(val: JsImageExtractionConfig) -> Self {
        RustImageExtractionConfig {
            extract_images: val.extract_images.unwrap_or(true),
            target_dpi: val.target_dpi.unwrap_or(300),
            max_image_dimension: val.max_image_dimension.unwrap_or(4096),
            auto_adjust_dpi: val.auto_adjust_dpi.unwrap_or(true),
            min_dpi: val.min_dpi.unwrap_or(72),
            max_dpi: val.max_dpi.unwrap_or(600),
        }
    }
}

#[napi(object)]
pub struct JsPostProcessorConfig {
    pub enabled: Option<bool>,
    pub enabled_processors: Option<Vec<String>>,
    pub disabled_processors: Option<Vec<String>>,
}

impl From<JsPostProcessorConfig> for RustPostProcessorConfig {
    fn from(val: JsPostProcessorConfig) -> Self {
        let enabled_set = val
            .enabled_processors
            .as_ref()
            .map(|procs| procs.iter().cloned().collect());
        let disabled_set = val
            .disabled_processors
            .as_ref()
            .map(|procs| procs.iter().cloned().collect());

        RustPostProcessorConfig {
            enabled: val.enabled.unwrap_or(true),
            enabled_processors: val.enabled_processors,
            disabled_processors: val.disabled_processors,
            enabled_set,
            disabled_set,
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct JsHtmlPreprocessingOptions {
    pub enabled: Option<bool>,
    pub preset: Option<String>,
    pub remove_navigation: Option<bool>,
    pub remove_forms: Option<bool>,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsHtmlOptions {
    pub heading_style: Option<String>,
    pub list_indent_type: Option<String>,
    pub list_indent_width: Option<u32>,
    pub bullets: Option<String>,
    pub strong_em_symbol: Option<String>,
    pub escape_asterisks: Option<bool>,
    pub escape_underscores: Option<bool>,
    pub escape_misc: Option<bool>,
    pub escape_ascii: Option<bool>,
    pub code_language: Option<String>,
    pub autolinks: Option<bool>,
    pub default_title: Option<bool>,
    pub br_in_tables: Option<bool>,
    pub hocr_spatial_tables: Option<bool>,
    pub highlight_style: Option<String>,
    pub extract_metadata: Option<bool>,
    pub whitespace_mode: Option<String>,
    pub strip_newlines: Option<bool>,
    pub wrap: Option<bool>,
    pub wrap_width: Option<u32>,
    pub convert_as_inline: Option<bool>,
    pub sub_symbol: Option<String>,
    pub sup_symbol: Option<String>,
    pub newline_style: Option<String>,
    pub code_block_style: Option<String>,
    pub keep_inline_images_in: Option<Vec<String>>,
    pub encoding: Option<String>,
    pub debug: Option<bool>,
    pub strip_tags: Option<Vec<String>>,
    pub preserve_tags: Option<Vec<String>>,
    pub preprocessing: Option<JsHtmlPreprocessingOptions>,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsYakeParams {
    pub window_size: Option<u32>,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsRakeParams {
    pub min_word_length: Option<u32>,
    pub max_words_per_phrase: Option<u32>,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsKeywordConfig {
    pub algorithm: Option<String>,
    pub max_keywords: Option<u32>,
    pub min_score: Option<f64>,
    #[napi(ts_type = "[number, number] | undefined")]
    pub ngram_range: Option<Vec<u32>>,
    pub language: Option<String>,
    pub yake_params: Option<JsYakeParams>,
    pub rake_params: Option<JsRakeParams>,
}

impl TryFrom<JsHtmlOptions> for ConversionOptions {
    type Error = Error;

    fn try_from(options: JsHtmlOptions) -> Result<Self> {
        let mut opts = ConversionOptions::default();

        if let Some(style) = options.heading_style {
            opts.heading_style = parse_heading_style(&style)?;
        }
        if let Some(indent) = options.list_indent_type {
            opts.list_indent_type = parse_list_indent_type(&indent)?;
        }
        if let Some(width) = options.list_indent_width {
            opts.list_indent_width = width as usize;
        }
        if let Some(bullets) = options.bullets {
            opts.bullets = bullets;
        }
        if let Some(symbol) = options.strong_em_symbol {
            let mut chars = symbol.chars();
            let first = chars.next().ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    "htmlOptions.strongEmSymbol must contain at least one character",
                )
            })?;
            opts.strong_em_symbol = first;
        }
        if let Some(value) = options.escape_asterisks {
            opts.escape_asterisks = value;
        }
        if let Some(value) = options.escape_underscores {
            opts.escape_underscores = value;
        }
        if let Some(value) = options.escape_misc {
            opts.escape_misc = value;
        }
        if let Some(value) = options.escape_ascii {
            opts.escape_ascii = value;
        }
        if let Some(language) = options.code_language {
            opts.code_language = language;
        }
        if let Some(value) = options.autolinks {
            opts.autolinks = value;
        }
        if let Some(value) = options.default_title {
            opts.default_title = value;
        }
        if let Some(value) = options.br_in_tables {
            opts.br_in_tables = value;
        }
        if let Some(value) = options.hocr_spatial_tables {
            opts.hocr_spatial_tables = value;
        }
        if let Some(style) = options.highlight_style {
            opts.highlight_style = parse_highlight_style(&style)?;
        }
        if let Some(value) = options.extract_metadata {
            opts.extract_metadata = value;
        }
        if let Some(mode) = options.whitespace_mode {
            opts.whitespace_mode = parse_whitespace_mode(&mode)?;
        }
        if let Some(value) = options.strip_newlines {
            opts.strip_newlines = value;
        }
        if let Some(value) = options.wrap {
            opts.wrap = value;
        }
        if let Some(width) = options.wrap_width {
            opts.wrap_width = width as usize;
        }
        if let Some(value) = options.convert_as_inline {
            opts.convert_as_inline = value;
        }
        if let Some(symbol) = options.sub_symbol {
            opts.sub_symbol = symbol;
        }
        if let Some(symbol) = options.sup_symbol {
            opts.sup_symbol = symbol;
        }
        if let Some(style) = options.newline_style {
            opts.newline_style = parse_newline_style(&style)?;
        }
        if let Some(style) = options.code_block_style {
            opts.code_block_style = parse_code_block_style(&style)?;
        }
        if let Some(tags) = options.keep_inline_images_in {
            opts.keep_inline_images_in = tags;
        }
        if let Some(encoding) = options.encoding {
            opts.encoding = encoding;
        }
        if let Some(debug) = options.debug {
            opts.debug = debug;
        }
        if let Some(tags) = options.strip_tags {
            opts.strip_tags = tags;
        }
        if let Some(tags) = options.preserve_tags {
            opts.preserve_tags = tags;
        }
        if let Some(pre) = options.preprocessing {
            let mut preprocessing = opts.preprocessing.clone();
            if let Some(enabled) = pre.enabled {
                preprocessing.enabled = enabled;
            }
            if let Some(preset) = pre.preset {
                preprocessing.preset = parse_preprocessing_preset(&preset)?;
            }
            if let Some(remove_navigation) = pre.remove_navigation {
                preprocessing.remove_navigation = remove_navigation;
            }
            if let Some(remove_forms) = pre.remove_forms {
                preprocessing.remove_forms = remove_forms;
            }
            opts.preprocessing = preprocessing;
        }

        Ok(opts)
    }
}

impl From<&HtmlPreprocessingOptions> for JsHtmlPreprocessingOptions {
    fn from(opts: &HtmlPreprocessingOptions) -> Self {
        Self {
            enabled: Some(opts.enabled),
            preset: Some(preprocessing_preset_to_string(opts.preset).to_string()),
            remove_navigation: Some(opts.remove_navigation),
            remove_forms: Some(opts.remove_forms),
        }
    }
}

impl From<&ConversionOptions> for JsHtmlOptions {
    fn from(opts: &ConversionOptions) -> Self {
        Self {
            heading_style: Some(heading_style_to_string(opts.heading_style).to_string()),
            list_indent_type: Some(list_indent_type_to_string(opts.list_indent_type).to_string()),
            list_indent_width: Some(opts.list_indent_width as u32),
            bullets: Some(opts.bullets.clone()),
            strong_em_symbol: Some(opts.strong_em_symbol.to_string()),
            escape_asterisks: Some(opts.escape_asterisks),
            escape_underscores: Some(opts.escape_underscores),
            escape_misc: Some(opts.escape_misc),
            escape_ascii: Some(opts.escape_ascii),
            code_language: Some(opts.code_language.clone()),
            autolinks: Some(opts.autolinks),
            default_title: Some(opts.default_title),
            br_in_tables: Some(opts.br_in_tables),
            hocr_spatial_tables: Some(opts.hocr_spatial_tables),
            highlight_style: Some(highlight_style_to_string(opts.highlight_style).to_string()),
            extract_metadata: Some(opts.extract_metadata),
            whitespace_mode: Some(whitespace_mode_to_string(opts.whitespace_mode).to_string()),
            strip_newlines: Some(opts.strip_newlines),
            wrap: Some(opts.wrap),
            wrap_width: Some(opts.wrap_width as u32),
            convert_as_inline: Some(opts.convert_as_inline),
            sub_symbol: Some(opts.sub_symbol.clone()),
            sup_symbol: Some(opts.sup_symbol.clone()),
            newline_style: Some(newline_style_to_string(opts.newline_style).to_string()),
            code_block_style: Some(code_block_style_to_string(opts.code_block_style).to_string()),
            keep_inline_images_in: Some(opts.keep_inline_images_in.clone()),
            encoding: Some(opts.encoding.clone()),
            debug: Some(opts.debug),
            strip_tags: Some(opts.strip_tags.clone()),
            preserve_tags: Some(opts.preserve_tags.clone()),
            preprocessing: Some(JsHtmlPreprocessingOptions::from(&opts.preprocessing)),
        }
    }
}

impl TryFrom<JsKeywordConfig> for RustKeywordConfig {
    type Error = Error;

    fn try_from(config: JsKeywordConfig) -> Result<Self> {
        let mut keywords = RustKeywordConfig::default();

        if let Some(max) = config.max_keywords {
            keywords.max_keywords = max as usize;
        }
        if let Some(score) = config.min_score {
            keywords.min_score = score as f32;
        }
        if let Some(range) = config.ngram_range {
            if range.len() != 2 {
                return Err(Error::new(
                    Status::InvalidArg,
                    "keywords.ngramRange must contain exactly two elements",
                ));
            }
            keywords.ngram_range = (range[0] as usize, range[1] as usize);
        }
        if let Some(language) = config.language {
            keywords.language = Some(language);
        }
        if let Some(algorithm) = config.algorithm {
            keywords.algorithm = parse_keyword_algorithm(&algorithm)?;
        }
        if let Some(yake) = config.yake_params {
            let mut params = RustYakeParams::default();
            if let Some(window) = yake.window_size {
                params.window_size = window as usize;
            }
            keywords.yake_params = Some(params);
        }
        if let Some(rake) = config.rake_params {
            let mut params = RustRakeParams::default();
            if let Some(min_len) = rake.min_word_length {
                params.min_word_length = min_len as usize;
            }
            if let Some(max_words) = rake.max_words_per_phrase {
                params.max_words_per_phrase = max_words as usize;
            }
            keywords.rake_params = Some(params);
        }

        Ok(keywords)
    }
}

impl From<RustKeywordConfig> for JsKeywordConfig {
    fn from(config: RustKeywordConfig) -> Self {
        Self {
            algorithm: Some(keyword_algorithm_to_string(config.algorithm).to_string()),
            max_keywords: Some(config.max_keywords as u32),
            min_score: Some(config.min_score as f64),
            ngram_range: Some(vec![config.ngram_range.0 as u32, config.ngram_range.1 as u32]),
            language: config.language,
            yake_params: config.yake_params.map(|params| JsYakeParams {
                window_size: Some(params.window_size as u32),
            }),
            rake_params: config.rake_params.map(|params| JsRakeParams {
                min_word_length: Some(params.min_word_length as u32),
                max_words_per_phrase: Some(params.max_words_per_phrase as u32),
            }),
        }
    }
}

fn parse_heading_style(value: &str) -> Result<HeadingStyle> {
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.headingStyle '{}'", value),
        )
    })?;

    let _discriminant = unsafe { kreuzberg_parse_heading_style(c_str.as_ptr()) };

    match _discriminant {
        0 => Ok(HeadingStyle::Atx),
        1 => Ok(HeadingStyle::Underlined),
        2 => Ok(HeadingStyle::AtxClosed),
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.headingStyle '{}'", value),
        )),
    }
}

fn heading_style_to_string(style: HeadingStyle) -> &'static str {
    match style {
        HeadingStyle::Atx => "atx",
        HeadingStyle::Underlined => "underlined",
        HeadingStyle::AtxClosed => "atx_closed",
    }
}

fn parse_list_indent_type(value: &str) -> Result<ListIndentType> {
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.listIndentType '{}'", value),
        )
    })?;

    let discriminant = unsafe { kreuzberg_parse_list_indent_type(c_str.as_ptr()) };

    match discriminant {
        0 => Ok(ListIndentType::Spaces),
        1 => Ok(ListIndentType::Tabs),
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.listIndentType '{}'", value),
        )),
    }
}

fn list_indent_type_to_string(value: ListIndentType) -> &'static str {
    match value {
        ListIndentType::Spaces => "spaces",
        ListIndentType::Tabs => "tabs",
    }
}

fn parse_highlight_style(value: &str) -> Result<HighlightStyle> {
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.highlightStyle '{}'", value),
        )
    })?;

    let discriminant = unsafe { kreuzberg_parse_highlight_style(c_str.as_ptr()) };

    match discriminant {
        0 => Ok(HighlightStyle::DoubleEqual),
        1 => Ok(HighlightStyle::Html),
        2 => Ok(HighlightStyle::Bold),
        3 => Ok(HighlightStyle::None),
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.highlightStyle '{}'", value),
        )),
    }
}

fn highlight_style_to_string(style: HighlightStyle) -> &'static str {
    match style {
        HighlightStyle::DoubleEqual => "double_equal",
        HighlightStyle::Html => "html",
        HighlightStyle::Bold => "bold",
        HighlightStyle::None => "none",
    }
}

fn parse_whitespace_mode(value: &str) -> Result<WhitespaceMode> {
    // Use FFI parsing to validate the input
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.whitespaceMode '{}'", value),
        )
    })?;

    let _discriminant = unsafe { kreuzberg_parse_whitespace_mode(c_str.as_ptr()) };

    // Map _discriminant to the actual enum variants
    // The FFI recognizes: "normalized" and "strict" map to discriminants
    // Map based on the original behavior
    match value.to_lowercase().as_str() {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        other => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.whitespaceMode '{}'", other),
        )),
    }
}

fn whitespace_mode_to_string(mode: WhitespaceMode) -> &'static str {
    match mode {
        WhitespaceMode::Normalized => "normalized",
        WhitespaceMode::Strict => "strict",
    }
}

fn parse_newline_style(value: &str) -> Result<NewlineStyle> {
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.newlineStyle '{}'", value),
        )
    })?;

    let _discriminant = unsafe { kreuzberg_parse_newline_style(c_str.as_ptr()) };

    // Map to actual enum variants
    match value.to_lowercase().as_str() {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        other => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.newlineStyle '{}'", other),
        )),
    }
}

fn newline_style_to_string(value: NewlineStyle) -> &'static str {
    match value {
        NewlineStyle::Spaces => "spaces",
        NewlineStyle::Backslash => "backslash",
    }
}

fn parse_code_block_style(value: &str) -> Result<CodeBlockStyle> {
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.codeBlockStyle '{}'", value),
        )
    })?;

    let discriminant = unsafe { kreuzberg_parse_code_block_style(c_str.as_ptr()) };

    match discriminant {
        0 => Ok(CodeBlockStyle::Indented),
        1 => Ok(CodeBlockStyle::Backticks),
        2 => Ok(CodeBlockStyle::Tildes),
        _ => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.codeBlockStyle '{}'", value),
        )),
    }
}

fn code_block_style_to_string(value: CodeBlockStyle) -> &'static str {
    match value {
        CodeBlockStyle::Indented => "indented",
        CodeBlockStyle::Backticks => "backticks",
        CodeBlockStyle::Tildes => "tildes",
    }
}

fn parse_preprocessing_preset(value: &str) -> Result<PreprocessingPreset> {
    let c_str = std::ffi::CString::new(value).map_err(|_| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.preprocessing.preset '{}'", value),
        )
    })?;

    let _discriminant = unsafe { kreuzberg_parse_preprocessing_preset(c_str.as_ptr()) };

    // Map to actual enum variants
    match value.to_lowercase().as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "standard" => Ok(PreprocessingPreset::Standard),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        other => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid htmlOptions.preprocessing.preset '{}'", other),
        )),
    }
}

fn preprocessing_preset_to_string(preset: PreprocessingPreset) -> &'static str {
    match preset {
        PreprocessingPreset::Minimal => "minimal",
        PreprocessingPreset::Standard => "standard",
        PreprocessingPreset::Aggressive => "aggressive",
    }
}

fn parse_keyword_algorithm(value: &str) -> Result<RustKeywordAlgorithm> {
    match value.to_lowercase().as_str() {
        "yake" => Ok(RustKeywordAlgorithm::Yake),
        "rake" => Ok(RustKeywordAlgorithm::Rake),
        other => Err(Error::new(
            Status::InvalidArg,
            format!("Invalid keywords.algorithm '{}'. Expected 'yake' or 'rake'", other),
        )),
    }
}

fn keyword_algorithm_to_string(algo: RustKeywordAlgorithm) -> &'static str {
    match algo {
        RustKeywordAlgorithm::Yake => "yake",
        RustKeywordAlgorithm::Rake => "rake",
    }
}

#[napi(object)]
pub struct JsPageConfig {
    pub extract_pages: Option<bool>,
    pub insert_page_markers: Option<bool>,
    pub marker_format: Option<String>,
}

#[napi(object)]
pub struct JsExtractionConfig {
    pub use_cache: Option<bool>,
    pub enable_quality_processing: Option<bool>,
    pub ocr: Option<JsOcrConfig>,
    pub force_ocr: Option<bool>,
    pub chunking: Option<JsChunkingConfig>,
    pub images: Option<JsImageExtractionConfig>,
    pub pdf_options: Option<JsPdfConfig>,
    pub token_reduction: Option<JsTokenReductionConfig>,
    pub language_detection: Option<JsLanguageDetectionConfig>,
    pub postprocessor: Option<JsPostProcessorConfig>,
    pub keywords: Option<JsKeywordConfig>,
    pub html_options: Option<JsHtmlOptions>,
    pub max_concurrent_extractions: Option<u32>,
    pub pages: Option<JsPageConfig>,
}

impl TryFrom<JsPageConfig> for kreuzberg::core::config::PageConfig {
    type Error = Error;

    fn try_from(val: JsPageConfig) -> Result<Self> {
        Ok(kreuzberg::core::config::PageConfig {
            extract_pages: val.extract_pages.unwrap_or(false),
            insert_page_markers: val.insert_page_markers.unwrap_or(false),
            marker_format: val
                .marker_format
                .unwrap_or_else(|| "\n\n<!-- PAGE {page_num} -->\n\n".to_string()),
        })
    }
}

impl From<kreuzberg::core::config::PageConfig> for JsPageConfig {
    fn from(config: kreuzberg::core::config::PageConfig) -> Self {
        Self {
            extract_pages: Some(config.extract_pages),
            insert_page_markers: Some(config.insert_page_markers),
            marker_format: Some(config.marker_format),
        }
    }
}

impl TryFrom<JsExtractionConfig> for ExtractionConfig {
    type Error = Error;

    fn try_from(val: JsExtractionConfig) -> Result<Self> {
        let html_options = match val.html_options {
            Some(options) => Some(ConversionOptions::try_from(options)?),
            None => None,
        };

        let keywords = match val.keywords {
            Some(config) => Some(RustKeywordConfig::try_from(config)?),
            None => None,
        };

        Ok(ExtractionConfig {
            use_cache: val.use_cache.unwrap_or(true),
            enable_quality_processing: val.enable_quality_processing.unwrap_or(true),
            ocr: val.ocr.map(Into::into),
            force_ocr: val.force_ocr.unwrap_or(false),
            chunking: val.chunking.map(Into::into),
            images: val.images.map(Into::into),
            pdf_options: val.pdf_options.map(Into::into),
            token_reduction: val.token_reduction.map(Into::into),
            language_detection: val.language_detection.map(Into::into),
            keywords,
            postprocessor: val.postprocessor.map(Into::into),
            html_options,
            max_concurrent_extractions: val.max_concurrent_extractions.map(|v| v as usize),
            pages: val.pages.map(|p| p.try_into()).transpose()?,
            output_format: Default::default(),
            result_format: Default::default(),
        })
    }
}

impl TryFrom<ExtractionConfig> for JsExtractionConfig {
    type Error = napi::Error;

    fn try_from(val: ExtractionConfig) -> Result<Self> {
        Ok(JsExtractionConfig {
            use_cache: Some(val.use_cache),
            enable_quality_processing: Some(val.enable_quality_processing),
            ocr: val.ocr.map(|ocr| JsOcrConfig {
                backend: ocr.backend,
                language: Some(ocr.language),
                tesseract_config: ocr.tesseract_config.map(|tc| JsTesseractConfig {
                    psm: Some(tc.psm),
                    enable_table_detection: Some(tc.enable_table_detection),
                    tessedit_char_whitelist: if tc.tessedit_char_whitelist.is_empty() {
                        None
                    } else {
                        Some(tc.tessedit_char_whitelist)
                    },
                }),
            }),
            force_ocr: Some(val.force_ocr),
            chunking: val.chunking.map(|chunk| JsChunkingConfig {
                max_chars: Some(chunk.max_chars as u32),
                max_overlap: Some(chunk.max_overlap as u32),
                embedding: chunk.embedding.map(|emb| JsEmbeddingConfig {
                    model: Some(JsEmbeddingModelType {
                        model_type: match emb.model {
                            RustEmbeddingModelType::Preset { .. } => "preset".to_string(),
                            RustEmbeddingModelType::FastEmbed { .. } => "fastembed".to_string(),
                            RustEmbeddingModelType::Custom { .. } => "custom".to_string(),
                        },
                        value: match &emb.model {
                            RustEmbeddingModelType::Preset { name } => name.clone(),
                            RustEmbeddingModelType::FastEmbed { model, .. } => model.clone(),
                            RustEmbeddingModelType::Custom { model_id, .. } => model_id.clone(),
                        },
                        dimensions: match emb.model {
                            RustEmbeddingModelType::FastEmbed { dimensions, .. } => Some(dimensions as u32),
                            RustEmbeddingModelType::Custom { dimensions, .. } => Some(dimensions as u32),
                            _ => None,
                        },
                    }),
                    normalize: Some(emb.normalize),
                    batch_size: Some(emb.batch_size as u32),
                    show_download_progress: Some(emb.show_download_progress),
                    cache_dir: emb.cache_dir.and_then(|p| p.to_str().map(String::from)),
                }),
                preset: chunk.preset,
            }),
            images: val.images.map(|img| JsImageExtractionConfig {
                extract_images: Some(img.extract_images),
                target_dpi: Some(img.target_dpi),
                max_image_dimension: Some(img.max_image_dimension),
                auto_adjust_dpi: Some(img.auto_adjust_dpi),
                min_dpi: Some(img.min_dpi),
                max_dpi: Some(img.max_dpi),
            }),
            pdf_options: val.pdf_options.map(|pdf| JsPdfConfig {
                extract_images: Some(pdf.extract_images),
                passwords: pdf.passwords,
                extract_metadata: Some(pdf.extract_metadata),
                hierarchy: pdf.hierarchy.map(|h| JsHierarchyConfig {
                    enabled: Some(h.enabled),
                    k_clusters: Some(h.k_clusters as i32),
                    include_bbox: Some(h.include_bbox),
                    ocr_coverage_threshold: h.ocr_coverage_threshold.map(|v| v as f64),
                }),
            }),
            token_reduction: val.token_reduction.map(|tr| JsTokenReductionConfig {
                mode: Some(tr.mode),
                preserve_important_words: Some(tr.preserve_important_words),
            }),
            language_detection: val.language_detection.map(|ld| JsLanguageDetectionConfig {
                enabled: Some(ld.enabled),
                min_confidence: Some(ld.min_confidence),
                detect_multiple: Some(ld.detect_multiple),
            }),
            postprocessor: val.postprocessor.map(|pp| JsPostProcessorConfig {
                enabled: Some(pp.enabled),
                enabled_processors: pp.enabled_processors,
                disabled_processors: pp.disabled_processors,
            }),
            keywords: val.keywords.map(JsKeywordConfig::from),
            html_options: val.html_options.as_ref().map(JsHtmlOptions::from),
            max_concurrent_extractions: val.max_concurrent_extractions.map(|v| v as u32),
            pages: val.pages.map(JsPageConfig::from),
        })
    }
}

/// Load extraction configuration from a file.
///
/// Automatically detects the file format based on extension:
/// - `.toml` - TOML format
/// - `.yaml` - YAML format
/// - `.json` - JSON format
///
/// # Parameters
///
/// * `file_path` - Path to the configuration file (absolute or relative)
///
/// # Returns
///
/// `JsExtractionConfig` object with loaded configuration.
///
/// # Errors
///
/// Throws an error if:
/// - File does not exist or is not accessible
/// - File content is not valid TOML/YAML/JSON
/// - Configuration structure is invalid
///
/// # Example
///
/// ```typescript
/// import { loadExtractionConfigFromFile } from 'kreuzberg';
///
/// // Load from TOML file
/// const config = loadExtractionConfigFromFile('kreuzberg.toml');
///
/// // Load from YAML file
/// const config2 = loadExtractionConfigFromFile('./config.yaml');
///
/// // Use with extraction
/// const result = await extractFile('document.pdf', null, config);
/// ```
#[napi(js_name = "loadExtractionConfigFromFile")]
pub fn load_extraction_config_from_file(_file_path: String) -> Result<JsExtractionConfig> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "load_extraction_config_from_file not yet fully implemented",
    ))
}

/// Discover extraction configuration file in current directory or parent directories.
///
/// Searches for configuration files in the following order:
/// 1. `kreuzberg.toml`
/// 2. `kreuzberg.yaml` / `kreuzberg.yml`
/// 3. `kreuzberg.json`
/// 4. Searches parent directories up to the filesystem root
///
/// Returns the first configuration file found or throws an error if none found.
///
/// # Returns
///
/// `JsExtractionConfig` object with discovered configuration.
///
/// # Errors
///
/// Throws an error if no configuration file is found.
///
/// # Example
///
/// ```typescript
/// import { discoverExtractionConfig } from 'kreuzberg';
///
/// // Automatically finds kreuzberg.toml or kreuzberg.yaml in current or parent directories
/// const config = discoverExtractionConfig();
/// const result = await extractFile('document.pdf', null, config);
/// ```
#[napi(js_name = "discoverExtractionConfig")]
pub fn discover_extraction_config() -> Result<JsExtractionConfig> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "discover_extraction_config not yet fully implemented",
    ))
}
