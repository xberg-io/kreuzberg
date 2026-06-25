//! Thread-local PDF file path for oxide text extraction.
//!
//! The current PDF file path is communicated via a thread-local to avoid
//! threading it through every function signature in the extraction pipeline.

use std::cell::RefCell;
use std::path::PathBuf;

thread_local! {
    static CURRENT_PDF_PATH: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// Set the current PDF file path for oxide text extraction.
/// Call this before entering the markdown pipeline.
pub(crate) fn set_current_pdf_path(path: Option<PathBuf>) {
    CURRENT_PDF_PATH.with(|cell| {
        *cell.borrow_mut() = path;
    });
}
