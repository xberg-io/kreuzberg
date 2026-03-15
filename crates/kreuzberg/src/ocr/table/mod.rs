pub mod tsv_parser;

// Re-export core table utilities from html-to-markdown-rs (always available when html feature is enabled)
#[cfg(feature = "html")]
pub use html_to_markdown_rs::hocr::{HocrWord, reconstruct_table, table_to_markdown};

// Re-export PDF-specific table utilities when the pdf feature is enabled
#[cfg(feature = "pdf")]
pub use crate::pdf::table_reconstruct::post_process_table;

pub use tsv_parser::extract_words_from_tsv;
