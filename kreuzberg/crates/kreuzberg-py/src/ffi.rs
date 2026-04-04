//! PyO3 wrappers for FFI error classification functions (Phase 2).
//!
//! Exposes error details retrieval and error message classification from
//! the kreuzberg-ffi crate through Python-friendly interfaces.
//!
//! Functions:
//! - get_error_details() -> dict with error information
//! - classify_error(message: str) -> int (error code)
//! - error_code_name(code: int) -> str

#![allow(unsafe_code)]

use pyo3::prelude::*;
use std::ffi::{CStr, CString};

use kreuzberg_ffi::{
    get_last_error_code as ffi_get_last_error_code, get_last_panic_context as ffi_get_last_panic_context,
    kreuzberg_classify_error, kreuzberg_error_code_name, kreuzberg_get_error_details,
};

/// Error details from kreuzberg-ffi.
///
/// Retrieves detailed error information from the thread-local FFI error storage.
/// Returns a dictionary with the following keys:
/// - "message" (str): Human-readable error message
/// - "error_code" (int): Numeric error code (0-7)
/// - "error_type" (str): Error type name (e.g., "validation", "ocr")
/// - "source_file" (str | None): Source file path if available
/// - "source_function" (str | None): Function name if available
/// - "source_line" (int): Line number (0 if unknown)
/// - "context_info" (str | None): Additional context if available
/// - "is_panic" (bool): Whether error came from a panic
///
/// Returns:
///     dict: Structured error details
#[pyfunction]
pub fn get_error_details(py: Python<'_>) -> PyResult<pyo3::Bound<'_, pyo3::types::PyDict>> {
    let details = kreuzberg_get_error_details();

    let result = pyo3::types::PyDict::new(py);

    unsafe {
        let message = if !details.message.is_null() {
            CStr::from_ptr(details.message).to_string_lossy().into_owned()
        } else {
            String::new()
        };

        let error_type = if !details.error_type.is_null() {
            CStr::from_ptr(details.error_type).to_string_lossy().into_owned()
        } else {
            "unknown".to_string()
        };

        let source_file = if !details.source_file.is_null() {
            Some(CStr::from_ptr(details.source_file).to_string_lossy().into_owned())
        } else {
            None
        };

        let source_function = if !details.source_function.is_null() {
            Some(CStr::from_ptr(details.source_function).to_string_lossy().into_owned())
        } else {
            None
        };

        let context_info = if !details.context_info.is_null() {
            Some(CStr::from_ptr(details.context_info).to_string_lossy().into_owned())
        } else {
            None
        };

        result.set_item("message", message)?;
        result.set_item("error_code", details.error_code)?;
        result.set_item("error_type", error_type)?;
        result.set_item("source_file", source_file)?;
        result.set_item("source_function", source_function)?;
        result.set_item("source_line", details.source_line)?;
        result.set_item("context_info", context_info)?;
        result.set_item("is_panic", details.is_panic != 0)?;

        Ok(result)
    }
}

/// Classify an error based on an error message string.
///
/// Analyzes the error message and returns the most likely Kreuzberg error code.
///
/// Args:
///     message (str): The error message to classify
///
/// Returns:
///     int: Error code (0-7) representing the classification
///
/// Classification:
/// - 0 (Validation): Invalid parameters, constraints, format mismatches
/// - 1 (Parsing): Parse errors, corrupt data, malformed content
/// - 2 (OCR): OCR processing failures
/// - 3 (MissingDependency): Missing libraries or system dependencies
/// - 4 (Io): File I/O, permissions, disk errors
/// - 5 (Plugin): Plugin loading or registry errors
/// - 6 (UnsupportedFormat): Unsupported MIME types or formats
/// - 7 (Internal): Unknown or internal errors
#[pyfunction]
pub fn classify_error(message: &str) -> PyResult<u32> {
    let c_message =
        CString::new(message).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    let code = unsafe { kreuzberg_classify_error(c_message.as_ptr()) };

    Ok(code)
}

/// Get the human-readable name of an error code.
///
/// Args:
///     code (int): Numeric error code (0-7)
///
/// Returns:
///     str: Human-readable error code name (e.g., "validation", "ocr")
///
/// Returns "unknown" for codes outside the valid range.
#[pyfunction]
pub fn error_code_name(code: u32) -> PyResult<String> {
    let name_ptr = kreuzberg_error_code_name(code);

    if name_ptr.is_null() {
        return Ok("unknown".to_string());
    }

    let name = unsafe { CStr::from_ptr(name_ptr).to_string_lossy().into_owned() };

    Ok(name)
}

/// Get the last error code from the FFI panic shield.
///
/// Returns the error code from the most recent error or panic caught by the
/// kreuzberg-ffi panic shield. Returns 0 (Success) if no error has occurred.
///
/// Error codes:
/// - 0: Success (no error)
/// - 1: Generic error
/// - 2: Panic was caught
/// - 3: Invalid argument
/// - 4: IO error
/// - 5: Parsing error
/// - 6: OCR error
/// - 7: Missing dependency
///
/// Returns:
///     int: Numeric error code representing the last error type
pub fn get_last_error_code() -> i32 {
    ffi_get_last_error_code() as i32
}

/// Get the last panic context from the FFI panic shield as a JSON string.
///
/// Retrieves the panic context from the most recent panic caught by the
/// kreuzberg-ffi panic shield. The context is serialized to JSON format
/// for easy consumption by Python code.
///
/// The returned JSON object (when present) contains:
/// - "file" (str): Source file where the panic occurred
/// - "line" (int): Line number where the panic occurred
/// - "function" (str): Function name where the panic occurred
/// - "message" (str): Panic message
/// - "timestamp" (str): Seconds since UNIX epoch when panic was captured
///
/// Returns:
///     str | None: JSON serialized panic context if a panic has been caught,
///                 None if no panic has occurred or error retrieving context
pub fn get_last_panic_context() -> Option<String> {
    ffi_get_last_panic_context().and_then(|ctx| {
        let json = serde_json::json!({
            "file": ctx.file,
            "line": ctx.line,
            "function": ctx.function,
            "message": ctx.message,
            "timestamp": ctx.timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d: std::time::Duration| d.as_secs_f64().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
        });
        serde_json::to_string(&json).ok()
    })
}
