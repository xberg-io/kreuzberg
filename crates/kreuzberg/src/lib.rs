//! Kreuzberg - High-Performance Document Intelligence Library
//!
//! Kreuzberg is a Rust-first document extraction library with language-agnostic plugin support.
//! It provides fast, accurate extraction from PDFs, images, Office documents, emails, and more.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use kreuzberg::{extract_file_sync, ExtractionConfig};
//!
//! # fn main() -> kreuzberg::Result<()> {
//! // Extract content from a file
//! let config = ExtractionConfig::default();
//! let result = extract_file_sync("document.pdf", None, &config)?;
//! println!("Extracted: {}", result.content);
//! # Ok(())
//! # }
//! ```
//!
//! # Architecture
//!
//! - **Core Module** (`core`): Main extraction orchestration, MIME detection, config loading
//! - **Plugin System**: Language-agnostic plugin architecture
//! - **Extractors**: Format-specific extraction (PDF, images, Office docs, email, etc.)
//! - **OCR**: Multiple OCR backend support (Tesseract, EasyOCR, PaddleOCR)
//!
//! # Features
//!
//! - Fast parallel processing with async/await
//! - Priority-based extractor selection
//! - Comprehensive MIME type detection (118+ file extensions)
//! - Configurable caching and quality processing
//! - Cross-language plugin support (Python, Node.js planned)

#![deny(unsafe_code)]

pub mod cache;
pub(crate) mod cache_dir;
pub mod cancellation;
pub mod core;
pub mod error;
pub mod extraction;
pub mod extractors;
#[cfg(feature = "layout-detection")]
pub mod model_cache;
pub mod plugins;
pub mod rendering;
pub mod telemetry;
pub mod text;
pub mod types;
pub mod utils;

#[cfg(any(feature = "ocr", feature = "pdf", feature = "paddle-ocr"))]
pub mod table_core;

#[cfg(feature = "tower-service")]
pub mod service;

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "mcp")]
pub mod mcp;

#[cfg(feature = "chunking")]
pub mod chunking;

#[cfg(all(feature = "liter-llm", not(target_os = "windows"), not(target_arch = "wasm32")))]
pub mod llm;

#[cfg(feature = "embeddings")]
pub mod embeddings;

#[cfg(feature = "ocr")]
pub mod image;

#[cfg(feature = "language-detection")]
pub mod language_detection;

// Note: `image` module (DPI, resize, preprocessing) requires full `ocr` feature
// due to fast_image_resize dependency. The `ocr` module requires tokio and native
// deps (JP2, JBIG2), so it stays `ocr`-only. WASM OCR uses the JS bridge instead.

#[cfg(feature = "stopwords")]
pub mod stopwords;

#[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
pub mod keywords;

#[cfg(feature = "ocr")]
pub mod ocr;

#[cfg(any(
    feature = "paddle-ocr",
    feature = "embeddings",
    feature = "layout-detection",
    feature = "auto-rotate"
))]
pub mod ort_discovery;

#[cfg(any(feature = "paddle-ocr", feature = "layout-detection", feature = "auto-rotate"))]
pub(crate) mod model_download;

#[cfg(feature = "paddle-ocr")]
pub mod paddle_ocr;

#[cfg(feature = "auto-rotate")]
pub mod doc_orientation;

#[cfg(feature = "layout-detection")]
pub mod layout;

#[cfg(feature = "pdf")]
pub mod pdf;

pub use cancellation::CancellationToken;
pub use error::{KreuzbergError, Result};
pub use types::*;

#[cfg(feature = "tokio-runtime")]
pub use core::extractor::{batch_extract_bytes, batch_extract_file};
pub use core::extractor::{extract_bytes, extract_file};

pub use core::extractor::{batch_extract_bytes_sync, extract_bytes_sync};

#[cfg(feature = "tokio-runtime")]
pub use core::extractor::{batch_extract_file_sync, extract_file_sync};

pub use core::config::{
    AccelerationConfig, ChunkSizing, ChunkerType, ChunkingConfig, ContentFilterConfig, EmailConfig, EmbeddingConfig,
    EmbeddingModelType, ExecutionProviderType, ExtractionConfig, FileExtractionConfig, ImageExtractionConfig,
    LanguageDetectionConfig, LlmConfig, OcrConfig, OutputFormat, PageConfig, PostProcessorConfig,
    StructuredExtractionConfig, TokenReductionOptions,
};

#[cfg(feature = "quality")]
pub use text::{ReductionLevel, TokenReductionConfig};

#[cfg(feature = "api")]
pub use core::server_config::ServerConfig;

#[cfg(feature = "pdf")]
pub use core::config::{HierarchyConfig, PdfBackend, PdfConfig};

#[cfg(feature = "html")]
pub use core::config::{HtmlOutputConfig, HtmlTheme};
#[cfg(feature = "html")]
pub use rendering::StyledHtmlRenderer;

#[cfg(feature = "paddle-ocr")]
pub use paddle_ocr::{CacheStats, ModelManager, ModelPaths, PaddleLanguage, PaddleOcrBackend, PaddleOcrConfig};

#[cfg(feature = "layout-detection")]
pub use core::config::{LayoutDetectionConfig, TableModel};

#[cfg(feature = "layout-detection")]
pub use layout::types::{BBox, DetectionResult, LayoutClass, LayoutDetection};

#[cfg(all(feature = "ocr", feature = "layout-detection"))]
pub use ocr::layout_assembly::RecognizedTable;
#[cfg(feature = "ocr")]
pub use ocr::types::PSMMode;

pub use core::config::{OcrPipelineConfig, OcrPipelineStage, OcrQualityThresholds};

#[cfg(feature = "auto-rotate")]
pub use doc_orientation::OrientationResult;

#[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
pub use keywords::{Keyword, KeywordAlgorithm, KeywordConfig, extract_keywords};

#[cfg(feature = "keywords-rake")]
pub use keywords::RakeParams;

#[cfg(feature = "keywords-yake")]
pub use keywords::YakeParams;

#[cfg(feature = "tree-sitter")]
pub use core::config::{CodeContentMode, TreeSitterConfig, TreeSitterProcessConfig};
#[cfg(feature = "tree-sitter")]
pub use tree_sitter_language_pack::{
    ChunkContext, CodeChunk, CommentInfo, CommentKind, Diagnostic, DiagnosticSeverity, DocstringFormat, DocstringInfo,
    ExportInfo, ExportKind, FileMetrics, ImportInfo, ProcessConfig, ProcessResult, Span, StructureItem, StructureKind,
    SymbolInfo, SymbolKind, process as process_code,
};

pub use core::mime::{
    DOCX_MIME_TYPE, EXCEL_MIME_TYPE, HTML_MIME_TYPE, JSON_MIME_TYPE, MARKDOWN_MIME_TYPE, PDF_MIME_TYPE,
    PLAIN_TEXT_MIME_TYPE, POWER_POINT_MIME_TYPE, SupportedFormat, XML_MIME_TYPE, detect_mime_type_from_bytes,
    detect_or_validate, get_extensions_for_mime, list_supported_formats, validate_mime_type,
};

/// Detect the MIME type of a file at the given path.
///
/// Uses the file extension and optionally the file content to determine the MIME type.
/// Set `check_exists` to `true` to verify the file exists before detection.
pub fn detect_mime_type(path: String, check_exists: bool) -> crate::Result<String> {
    core::mime::detect_mime_type(path, check_exists)
}

#[cfg(feature = "language-detection")]
pub use language_detection::detect_languages;

/// Detect the image format from raw bytes.
///
/// Returns a string identifying the format (e.g., `"jpeg"`, `"png"`, `"gif"`, `"bmp"`, `"tiff"`, `"webp"`).
/// Returns `"unknown"` if the format cannot be determined.
#[cfg(feature = "ocr")]
pub fn detect_image_format(data: Vec<u8>) -> String {
    extraction::image_format::detect_image_format(&data).to_string()
}

pub use core::formats::{KNOWN_FORMATS, is_valid_format_field};

pub use plugins::registry::{
    get_document_extractor_registry, get_ocr_backend_registry, get_post_processor_registry, get_renderer_registry,
    get_validator_registry,
};

#[cfg(feature = "embeddings")]
pub use embeddings::{EMBEDDING_PRESETS, EmbeddingPreset, download_model, get_preset, list_presets, warm_model};

/// Embed a list of texts using the configured embedding model.
///
/// Returns a 2D vector where each inner vector is the embedding for the corresponding text.
#[cfg(feature = "embeddings")]
pub fn embed_texts(texts: Vec<String>, config: Option<core::config::EmbeddingConfig>) -> crate::Result<Vec<Vec<f32>>> {
    embeddings::embed_texts(&texts, &config.unwrap_or_default())
}

#[cfg(all(feature = "embeddings", feature = "tokio-runtime"))]
pub use embeddings::embed_texts_async;

// Cache utilities
pub use cache::{blake3_hash_bytes, blake3_hash_file, fast_hash, validate_cache_key};

/// Generate a deterministic cache key from a list of key-value pairs.
///
/// Each element of `parts` should be a two-element list `[key, value]`.
/// The pairs are sorted by key before hashing, so order does not affect the result.
pub fn generate_cache_key(parts: Vec<Vec<String>>) -> String {
    let owned: Vec<(String, String)> = parts
        .into_iter()
        .filter_map(|mut pair| {
            if pair.len() >= 2 {
                let v = pair.remove(1);
                let k = pair.remove(0);
                Some((k, v))
            } else {
                None
            }
        })
        .collect();
    cache::generate_cache_key(&owned)
}

// JSON/string utilities
pub use utils::{camel_to_snake, normalize_whitespace, snake_to_camel};

/// Escape HTML special characters in a string.
///
/// Converts `&`, `<`, `>`, `"`, and `'` to their HTML entity equivalents.
pub fn escape_html_entities(text: &str) -> String {
    utils::escape_html_entities(text).into_owned()
}

/// Fix mojibake (garbled text from encoding errors) in a string.
///
/// Attempts to detect and correct common encoding errors where text was
/// decoded with the wrong character set (e.g., UTF-8 bytes interpreted as Latin-1).
#[cfg(feature = "quality")]
pub fn fix_mojibake(text: &str) -> String {
    utils::fix_mojibake(text).into_owned()
}

// Text utilities
pub use text::utf8_validation::is_valid_utf8;

#[cfg(feature = "quality")]
pub use text::quality::clean_extracted_text;

// Telemetry utilities
pub use telemetry::conventions::sanitize_filename;

#[cfg(feature = "otel")]
pub use telemetry::spans::sanitize_path;

// Plugin list functions
pub use plugins::extractor::list_extractors as list_document_extractors;
pub use plugins::list_ocr_backends;
pub use plugins::list_post_processors;
pub use plugins::list_validators;

// Config validation functions
pub use core::config_validation::{
    validate_binarization_method, validate_chunking_params, validate_confidence, validate_host, validate_language_code,
    validate_ocr_backend, validate_output_format, validate_port, validate_tesseract_oem, validate_tesseract_psm,
    validate_token_reduction_level,
};

// Text annotation builder helpers (always available — used by djot/markdown extractors)
pub use types::builder::{bold, code, italic, link, strikethrough};
// Extended annotation helpers (only used by office/html/xml extractors)
#[cfg(any(feature = "office", feature = "html", feature = "xml"))]
pub use types::builder::underline;

// Extraction markdown utilities
#[cfg(any(feature = "office", feature = "html", feature = "xml"))]
pub use extraction::markdown::{cells_to_markdown, cells_to_text};

// Rendering utilities
pub use rendering::{render_djot, render_html, render_json, render_markdown, render_plain};
#[cfg(feature = "html")]
pub use rendering::{render_djot_str, render_html_str, render_json_str, render_markdown_str, render_plain_str};

/// Convert HTML to Markdown.
///
/// Converts an HTML string to Markdown using default conversion options.
#[cfg(feature = "html")]
pub fn convert_html_to_markdown(html: &str) -> crate::Result<String> {
    extraction::html::convert_html_to_markdown_simple(html)
}

pub use extractors::djot_format::djot_to_html;

// Format-specific extract functions
#[cfg(feature = "office")]
pub use extraction::doc::DocExtractionResult;
#[cfg(feature = "office")]
pub use extraction::doc::DocMetadata;

/// Extract text from a DOC (Word 97-2003) file.
///
/// Takes the raw bytes of a `.doc` file and returns the extracted text and metadata.
#[cfg(feature = "office")]
pub fn extract_doc_text(content: &[u8]) -> crate::Result<extraction::doc::DocExtractionResult> {
    extraction::doc::extract_doc_text(content)
}

#[cfg(feature = "email")]
pub use extraction::email::extract_email_content;

/// Extract text and metadata from a PPTX (PowerPoint) file.
///
/// Takes the raw bytes of a `.pptx` file and returns the extracted content
/// using default extraction options.
#[cfg(feature = "office")]
pub fn extract_pptx_from_bytes(data: &[u8]) -> crate::Result<types::formats::PptxExtractionResult> {
    extraction::pptx::extract_pptx_from_bytes(data, &Default::default())
}

#[cfg(feature = "pdf")]
pub use pdf::rendering::render_pdf_page_to_png;
#[cfg(feature = "pdf")]
pub use pdf::text::extract_text_from_pdf;

// OCR hash utility
#[cfg(feature = "ocr")]
pub use ocr::utils::compute_hash;

// Embeddings utility
#[cfg(feature = "embeddings")]
pub use embeddings::engine::normalize;

// Token reduction
#[cfg(feature = "quality")]
pub use text::reduce_tokens;

// Chunking functions
#[cfg(feature = "chunking")]
pub use chunking::core::{chunk_text, chunk_texts_batch};
#[cfg(feature = "chunking")]
pub use chunking::semantic::chunk_semantic;

// iWork dedup utility
#[cfg(feature = "iwork")]
pub use extractors::iwork::dedup_text;

/// Serialize an [`ExtractionResult`] to TOON (Token-Oriented Object Notation).
///
/// TOON is a token-efficient alternative to JSON for LLM prompts.
/// Losslessly convertible to/from JSON but uses fewer tokens.
pub fn serialize_to_toon(result: &ExtractionResult) -> Result<String> {
    serde_toon::to_string(result).map_err(|e| KreuzbergError::serialization(format!("TOON serialization failed: {e}")))
}

/// Serialize an [`ExtractionResult`] to pretty-printed JSON.
pub fn serialize_to_json(result: &ExtractionResult) -> Result<String> {
    serde_json::to_string_pretty(result)
        .map_err(|e| KreuzbergError::serialization(format!("JSON serialization failed: {e}")))
}

/// Convenience: extract a file and serialize the result to TOON.
///
/// Equivalent to `extract_file_sync(path, None, config).and_then(|r| serialize_to_toon(&r))`,
/// exposed as a single function so all bindings can offer the same one-call shape.
#[cfg(feature = "tokio-runtime")]
pub fn extract_file_to_toon(path: impl AsRef<std::path::Path>, config: &ExtractionConfig) -> Result<String> {
    let result = extract_file_sync(path, None, config)?;
    serialize_to_toon(&result)
}

/// Convenience: extract a file and serialize the result to pretty-printed JSON.
#[cfg(feature = "tokio-runtime")]
pub fn extract_file_to_json(path: impl AsRef<std::path::Path>, config: &ExtractionConfig) -> Result<String> {
    let result = extract_file_sync(path, None, config)?;
    serialize_to_json(&result)
}
