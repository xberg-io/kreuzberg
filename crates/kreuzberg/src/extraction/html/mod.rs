//! HTML to Markdown conversion functions.
//!
//! This module provides HTML to Markdown conversion using the `html-to-markdown-rs` library.
//! It supports inline image extraction and YAML frontmatter parsing for HTML metadata.
//!
//! # Features
//!
//! - **HTML to Markdown conversion**: Clean, readable Markdown output
//! - **Inline image extraction**: Extract base64 and data URI images
//! - **YAML frontmatter**: Parse YAML metadata from Markdown output
//! - **Customizable conversion**: Full access to `html-to-markdown-rs` options
//!
//! # Example
//!
//! ```rust
//! use kreuzberg::extraction::html::convert_html_to_markdown;
//!
//! # fn example() -> kreuzberg::Result<()> {
//! let html = r#"<h1>Title</h1><p>This is <strong>bold</strong> text.</p>"#;
//! let markdown = convert_html_to_markdown(html, None)?;
//!
//! assert!(markdown.contains("# Title"));
//! assert!(markdown.contains("**bold**"));
//! # Ok(())
//! # }
//! ```

mod converter;
mod image_handling;
mod processor;
mod stack_management;
mod types;

// Public API re-exports
pub use converter::convert_html_to_markdown;
pub use converter::convert_html_to_markdown_with_metadata;
pub use processor::process_html;
pub use types::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, PreprocessingOptions,
    PreprocessingPreset, WhitespaceMode,
};
pub use types::{ExtractedInlineImage, HtmlExtractionResult};

// Re-export from html-to-markdown-rs for convenience
pub use html_to_markdown_rs::ConversionOptions;
