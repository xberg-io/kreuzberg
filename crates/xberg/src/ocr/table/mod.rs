/// TSV (tab-separated value) output parser for Tesseract word-level bounding boxes.
pub mod tsv_parser;

// Re-export core table utilities
#[cfg(feature = "paddle-ocr")]
pub(crate) use crate::table_core::HocrWord;
pub(crate) use crate::table_core::{reconstruct_table, table_to_markdown};

// Re-export PDF-specific table utilities when the pdf feature is enabled
#[cfg(feature = "pdf")]
pub(crate) use crate::pdf::table_reconstruct::post_process_table;

pub(crate) use tsv_parser::extract_words_from_tsv;
