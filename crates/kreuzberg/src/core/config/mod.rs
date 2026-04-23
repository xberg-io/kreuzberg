//! Configuration loading and management.
//!
//! This module provides utilities for loading extraction configuration from various
//! sources (TOML, YAML, JSON) and discovering configuration files in the project hierarchy.

pub mod acceleration;
pub mod concurrency;
pub mod content_filter;
pub mod email;
pub mod extraction;
pub mod formats;
#[cfg(feature = "html")]
pub mod html_output;
pub mod layout;
pub mod llm;
pub mod merge;
pub mod ocr;
pub mod page;
pub mod pdf;
pub mod processing;
#[cfg(feature = "tree-sitter")]
pub mod tree_sitter;

// Re-export main types for backward compatibility
pub use acceleration::{AccelerationConfig, ExecutionProviderType};
pub use concurrency::ConcurrencyConfig;
pub use content_filter::ContentFilterConfig;
pub use email::EmailConfig;
pub use extraction::{
    ExtractionConfig, FileExtractionConfig, ImageExtractionConfig, LanguageDetectionConfig, TokenReductionOptions,
};
pub use formats::OutputFormat;
#[cfg(feature = "html")]
pub use html_output::{HtmlOutputConfig, HtmlTheme};
#[cfg(feature = "layout-detection")]
pub use layout::{LayoutDetectionConfig, TableModel};
pub use llm::{LlmConfig, StructuredExtractionConfig};
pub use ocr::{OcrConfig, OcrPipelineConfig, OcrPipelineStage, OcrQualityThresholds};
pub use page::PageConfig;
#[cfg(feature = "pdf")]
pub use pdf::{HierarchyConfig, PdfBackend, PdfConfig};
pub use processing::{
    ChunkSizing, ChunkerType, ChunkingConfig, EmbeddingConfig, EmbeddingModelType, PostProcessorConfig,
};
#[cfg(feature = "tree-sitter")]
pub use tree_sitter::{CodeContentMode, TreeSitterConfig, TreeSitterProcessConfig};
