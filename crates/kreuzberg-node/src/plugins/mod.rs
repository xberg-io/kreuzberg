mod ocr_backend;
/// Plugin system implementations for Kreuzberg
///
/// This module provides support for extending Kreuzberg's functionality through plugins:
/// - **PostProcessor**: Custom document post-processing
/// - **Validator**: Custom validation logic
/// - **OcrBackend**: Custom OCR implementations
mod post_processor;
mod validator;

pub use ocr_backend::*;
pub use post_processor::*;
pub use validator::*;
