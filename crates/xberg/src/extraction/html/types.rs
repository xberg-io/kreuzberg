//! Type definitions for HTML extraction.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub use html_to_markdown_rs::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, PreprocessingOptions,
    PreprocessingPreset, WhitespaceMode,
};

/// Result of HTML extraction with optional images and warnings.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlExtractionResult {
    /// Extracted content as Markdown.
    pub markdown: String,
    /// Inline images extracted from the HTML document.
    pub images: Vec<ExtractedInlineImage>,
    /// Non-fatal warnings generated during extraction (e.g. unsupported elements).
    pub warnings: Vec<String>,
}

/// Extracted inline image with metadata.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedInlineImage {
    /// Raw image bytes (shared via `Bytes` for cheap cloning).
    pub data: Bytes,
    /// Image format string (e.g. `"png"`, `"jpeg"`, `"gif"`).
    pub format: String,
    /// Optional original filename from `src` or `data-filename` attribute.
    pub filename: Option<String>,
    /// Alt-text or title used as the image description.
    pub description: Option<String>,
    /// Image dimensions as `(width, height)` in pixels, if known.
    pub dimensions: Option<(u32, u32)>,
    /// All HTML attributes from the `<img>` element.
    pub attributes: Vec<(String, String)>,
}
