//! Validation function wrappers
//!
//! Delegates to kreuzberg core validation functions for proper checking
//! of OCR backend names, language codes, and output formats.

use crate::error::kreuzberg_error;

pub fn validate_ocr_backend_impl(backend: &str) -> extendr_api::Result<bool> {
    match kreuzberg::core::validate_ocr_backend(backend) {
        Ok(()) => Ok(true),
        Err(e) => Err(kreuzberg_error(e)),
    }
}

pub fn validate_language_code_impl(code_str: &str) -> extendr_api::Result<bool> {
    match kreuzberg::core::validate_language_code(code_str) {
        Ok(()) => Ok(true),
        Err(e) => Err(kreuzberg_error(e)),
    }
}

pub fn validate_output_format_impl(format: &str) -> extendr_api::Result<bool> {
    match kreuzberg::core::validate_output_format(format) {
        Ok(()) => Ok(true),
        Err(e) => Err(kreuzberg_error(e)),
    }
}
