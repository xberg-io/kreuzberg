//! Type definitions for HTML extraction.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub use html_to_markdown_rs::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, PreprocessingOptions,
    PreprocessingPreset, WhitespaceMode,
};

/// Result of HTML extraction with optional images and warnings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlExtractionResult {
    pub markdown: String,
    pub images: Vec<ExtractedInlineImage>,
    pub warnings: Vec<String>,
}

/// Extracted inline image with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedInlineImage {
    /// Uses `bytes::Bytes` for cheap cloning of large buffers.
    pub data: Bytes,
    pub format: String,
    pub filename: Option<String>,
    pub description: Option<String>,
    pub dimensions: Option<(u32, u32)>,
    pub attributes: Vec<(String, String)>,
}
