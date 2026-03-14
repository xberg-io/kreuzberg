//! PDF-to-Markdown renderer using segment-level font analysis.
//!
//! Converts PDF documents into structured markdown by analyzing pdfium text segments
//! (pre-merged character runs sharing baseline + font settings) to reconstruct headings,
//! paragraphs, inline formatting, and list items.

#[allow(dead_code)] // Wired incrementally — consumers added in subsequent commits.
pub(crate) mod adapters;
mod assembly;
mod bridge;
mod classify;
mod columns;
mod constants;
#[allow(dead_code)] // Wired incrementally — consumers added in subsequent commits.
pub(crate) mod content;
mod content_convert;
pub(crate) mod geometry;
pub(crate) mod layout_classify;
mod lines;
mod paragraphs;
mod pipeline;
mod regions;
mod render;
mod text_repair;
pub(crate) mod types;

pub(crate) use pipeline::render_document_as_markdown_with_tables;
pub use render::inject_image_placeholders;
