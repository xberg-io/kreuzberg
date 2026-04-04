//! Djot markup format extractor and utilities.
//!
//! This module provides:
//! - Djot parsing using the jotdown crate
//! - YAML frontmatter metadata extraction (same as Markdown)
//! - Table extraction as structured data
//! - Heading structure preservation
//! - Code block and link extraction
//! - Djot content rendering and conversion APIs
//!
//! Djot is a modern markup language with simpler parsing rules than CommonMark.
//! See https://djot.net for the specification.
//!
//! Requires the `djot` feature.

pub mod attributes;
pub mod conversion;
pub mod extractor;
pub mod parsing;
pub mod rendering;

// Re-export public API
pub use conversion::{djot_content_to_djot, djot_to_html, extraction_result_to_djot};
pub use extractor::DjotExtractor;
