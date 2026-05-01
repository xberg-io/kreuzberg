//! HTML to Markdown/Djot conversion functions.
//!
//! This module provides HTML to Markdown and Djot conversion using the `html-to-markdown-rs` library.
//! It supports inline image extraction and YAML frontmatter parsing for HTML metadata.
//!
//! # Features
//!
//! - **HTML to Markdown/Djot conversion**: Clean, readable Markdown or Djot output
//! - **Inline image extraction**: Extract base64 and data URI images
//! - **YAML frontmatter**: Parse YAML metadata from Markdown output
//! - **Customizable conversion**: Full access to `html-to-markdown-rs` options
//! - **Output format selection**: Choose between Markdown and Djot formats
//!
//! # Example
//!
//! ```rust
//! use kreuzberg::extraction::html::convert_html_to_markdown;
//! use kreuzberg::core::config::OutputFormat;
//!
//! # fn example() -> kreuzberg::Result<()> {
//! let html = r#"<h1>Title</h1><p>This is <strong>bold</strong> text.</p>"#;
//!
//! // Convert to Markdown (default)
//! let markdown = convert_html_to_markdown(html, None, None)?;
//! assert!(markdown.contains("# Title"));
//! assert!(markdown.contains("**bold**"));
//!
//! // Convert to Djot
//! let djot = convert_html_to_markdown(html, None, Some(OutputFormat::Djot))?;
//! assert!(djot.contains("# Title"));
//! assert!(djot.contains("*bold*")); // Djot uses * for strong
//! # Ok(())
//! # }
//! ```

mod converter;
mod stack_management;
#[cfg(any(feature = "office", feature = "email"))]
pub(crate) mod structure;
mod types;

// Public API re-exports
pub use converter::convert_html_to_markdown;
#[cfg(feature = "office")]
pub(crate) use converter::convert_html_to_markdown_with_metadata;
pub(crate) use converter::convert_html_to_markdown_with_tables;
pub(crate) use converter::extract_html_inline_images;
pub use types::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, PreprocessingOptions,
    PreprocessingPreset, WhitespaceMode,
};
pub use types::{ExtractedInlineImage, HtmlExtractionResult};

// Re-export from html-to-markdown-rs for convenience
pub use html_to_markdown_rs::ConversionOptions;

/// Convert HTML to Markdown using default options.
///
/// This is a simplified wrapper around [`convert_html_to_markdown`] that uses
/// default conversion options and Markdown output format.
pub(crate) fn convert_html_to_markdown_simple(html: &str) -> crate::Result<String> {
    convert_html_to_markdown(html, None, None)
}
