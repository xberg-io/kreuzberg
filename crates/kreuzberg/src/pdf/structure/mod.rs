//! PDF-to-structure renderer using segment-level font analysis.
//!
//! Converts PDF documents into structured `InternalDocument` by analyzing pdf_oxide
//! text segments to reconstruct headings, paragraphs, inline formatting, and list items.

pub(crate) mod adapters;
mod assembly;
mod classify;
mod constants;
pub(crate) mod content;
mod content_convert;
pub(crate) mod geometry;
pub(crate) mod layout_classify;
mod lines;
mod paragraphs;
mod pipeline;
mod regions;
mod text_repair;
pub(crate) mod types;

#[allow(unused_imports)] // Used by extractors/pdf/ocr.rs for building InternalDocument from OCR paragraphs
pub(crate) use assembly::assemble_internal_document;
#[allow(unused_imports)] // Used by extractors/pdf/ocr.rs when ocr feature is enabled
pub(crate) use content_convert::{content_to_paragraphs, reorder_elements_reading_order};
pub(crate) use pipeline::{SegmentStructureConfig, extract_document_structure_from_segments};
