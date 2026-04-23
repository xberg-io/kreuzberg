//! PDF document processing utilities.
//!
//! This module provides low-level PDF processing functions for text extraction,
//! metadata parsing, image extraction, and page rendering. Used internally by
//! the PDF extractor plugin.
//!
//! # Features
//!
//! - **Text extraction**: Extract text content from PDFs using `pdfium-render`
//! - **Metadata extraction**: Parse PDF metadata (title, author, creation date, etc.)
//! - **Image extraction**: Extract embedded images from PDF pages
//! - **Page rendering**: Render PDF pages to images for OCR processing
//! - **Error handling**: Comprehensive PDF-specific error types
//!
//! # Example
//!
//! ```rust
//! use kreuzberg::pdf::{extract_text_from_pdf, extract_metadata};
//!
//! # fn example() -> kreuzberg::Result<()> {
//! let pdf_bytes = std::fs::read("document.pdf")?;
//!
//! // Extract text
//! let text = extract_text_from_pdf(&pdf_bytes)?;
//! println!("Text: {}", text);
//!
//! // Extract metadata
//! let metadata = extract_metadata(&pdf_bytes)?;
//! println!("PDF version: {:?}", metadata.pdf_version);
//! # Ok(())
//! # }
//! ```
//!
//! # Note
//!
//! This module requires the `pdf` feature. The `ocr` feature enables additional
//! functionality in the PDF extractor for rendering pages to images.
#[cfg(feature = "pdf")]
pub mod annotations;
#[cfg(feature = "pdf")]
pub(crate) mod bindings;
#[cfg(feature = "pdf")]
pub mod bookmarks;
#[cfg(all(feature = "pdf", feature = "bundled-pdfium"))]
pub mod bundled;
#[cfg(all(feature = "pdf", feature = "tokio-runtime"))]
pub mod embedded_files;
#[cfg(feature = "pdf")]
pub mod error;
#[cfg(feature = "pdf")]
pub mod fonts;
#[cfg(feature = "pdf")]
pub mod hierarchy;
#[cfg(feature = "pdf")]
pub mod images;
#[cfg(all(feature = "pdf", feature = "layout-detection"))]
pub mod layout_runner;
#[cfg(feature = "pdf")]
pub mod metadata;
#[cfg(feature = "pdf-oxide")]
pub(crate) mod oxide;
#[cfg(feature = "pdf-oxide")]
pub(crate) mod oxide_text;
#[cfg(feature = "pdf")]
pub mod rendering;
#[cfg(feature = "pdf")]
pub mod structure;
#[cfg(feature = "pdf")]
pub mod table;
#[cfg(feature = "pdf")]
pub mod table_reconstruct;
#[cfg(feature = "pdf")]
pub mod text;
#[cfg(feature = "pdf")]
pub(crate) mod text_data;
// Stub for when pdf-oxide is disabled — provides set/get for thread-local path
#[cfg(all(feature = "pdf", not(feature = "pdf-oxide")))]
#[allow(dead_code)]
pub(crate) mod oxide_text {
    #[allow(dead_code)]
    pub(crate) fn set_current_pdf_path(_path: Option<std::path::PathBuf>) {}
    #[allow(dead_code)]
    pub(crate) fn current_pdf_path() -> Option<std::path::PathBuf> {
        None
    }
}

#[cfg(feature = "pdf")]
pub use crate::core::config::HierarchyConfig;
#[cfg(all(feature = "pdf", feature = "bundled-pdfium"))]
pub(crate) use bundled::extract_bundled_pdfium;
#[cfg(feature = "pdf")]
pub use error::PdfError;
#[cfg(feature = "pdf")]
pub use text::extract_text_from_pdf;
