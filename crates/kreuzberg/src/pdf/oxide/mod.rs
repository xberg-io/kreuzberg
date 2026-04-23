//! pdf_oxide backend for PDF extraction.
//!
//! Pure-Rust alternative to the pdfium-render backend. Provides text extraction,
//! metadata parsing, annotation extraction, image extraction, table detection,
//! and font metrics for heading hierarchy detection.

pub(crate) mod annotations;
pub(crate) mod hierarchy;
pub(crate) mod images;
pub(crate) mod metadata;
pub(crate) mod table;
pub(crate) mod text;

use crate::Result;
use crate::error::KreuzbergError;
use std::path::Path;

/// Wraps a [`pdf_oxide::PdfDocument`] with convenient constructors that map
/// pdf_oxide errors into [`KreuzbergError::Parsing`].
pub(crate) struct OxideDocument {
    pub doc: pdf_oxide::PdfDocument,
}

impl OxideDocument {
    /// Open a PDF from a file path.
    #[allow(dead_code)]
    pub(crate) fn open_file(path: &Path) -> Result<Self> {
        let doc = pdf_oxide::PdfDocument::open(path).map_err(|e| KreuzbergError::Parsing {
            message: format!("pdf_oxide: failed to open file: {e}"),
            source: None,
        })?;
        Ok(Self { doc })
    }

    /// Open a PDF from in-memory bytes.
    pub(crate) fn open_bytes(bytes: &[u8]) -> Result<Self> {
        let doc = pdf_oxide::PdfDocument::from_bytes(bytes.to_vec()).map_err(|e| KreuzbergError::Parsing {
            message: format!("pdf_oxide: failed to load bytes: {e}"),
            source: None,
        })?;
        Ok(Self { doc })
    }
}
