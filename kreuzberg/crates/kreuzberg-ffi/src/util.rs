//! Utility functions for version and error reporting.
//!
//! This module provides FFI functions for:
//! - Getting the library version
//! - Retrieving error information (message, code, panic context)

use crate::ffi_panic_guard;
use crate::helpers::LAST_ERROR_C_STRING;
use crate::panic_shield::{get_last_error_code, get_last_panic_context};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// Get the last error message from a failed operation.
///
/// # Safety
///
/// - Returns a static string that does not need to be freed
/// - Returns NULL if no error has occurred
/// - The returned string is valid until the next Kreuzberg function call on the same thread
///
/// # Example (C)
///
/// ```c
/// CExtractionResult* result = kreuzberg_extract_file_sync(path);
/// if (result == NULL) {
///     const char* error = kreuzberg_last_error();
///     printf("Error: %s\n", error);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_last_error() -> *const c_char {
    LAST_ERROR_C_STRING.with(|last| match &*last.borrow() {
        Some(c_str) => c_str.as_ptr(),
        None => ptr::null(),
    })
}

/// Get the error code for the last error.
///
/// Returns the error code as an i32. Error codes are defined in ErrorCode enum:
/// - 0: Success (no error)
/// - 1: GenericError
/// - 2: Panic
/// - 3: InvalidArgument
/// - 4: IoError
/// - 5: ParsingError
/// - 6: OcrError
/// - 7: MissingDependency
///
/// # Safety
///
/// This function is thread-safe and always safe to call.
///
/// # Example (C)
///
/// ```c
/// CExtractionResult* result = kreuzberg_extract_file_sync(path);
/// if (result == NULL) {
///     int32_t code = kreuzberg_last_error_code();
///     if (code == 2) {
///         // A panic occurred
///     }
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_last_error_code() -> i32 {
    get_last_error_code() as i32
}

/// Get the panic context for the last error (if it was a panic).
///
/// Returns a JSON object with panic details including:
/// - file: Source file where panic occurred
/// - line: Line number in source file
/// - function: Name of the function that panicked
/// - message: Panic message
/// - timestamp_secs: Unix timestamp when panic occurred
///
/// # Safety
///
/// - The returned string must be freed with `kreuzberg_free_string`
/// - Returns NULL if the last error was not a panic or no error has occurred
///
/// # Example (C)
///
/// ```c
/// CExtractionResult* result = kreuzberg_extract_file_sync(path);
/// if (result == NULL && kreuzberg_last_error_code() == 2) {
///     char* context = kreuzberg_last_panic_context();
///     if (context != NULL) {
///         printf("Panic context: %s\n", context);
///         kreuzberg_free_string(context);
///     }
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_last_panic_context() -> *mut c_char {
    ffi_panic_guard!("kreuzberg_last_panic_context", {
        match get_last_panic_context() {
            Some(ctx) => {
                use std::time::UNIX_EPOCH;

                let timestamp_secs = ctx
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                let json_value = serde_json::json!({
                    "file": ctx.file,
                    "line": ctx.line,
                    "function": ctx.function,
                    "message": ctx.message,
                    "timestamp_secs": timestamp_secs
                });

                match serde_json::to_string(&json_value) {
                    Ok(json) => match CString::new(json) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => ptr::null_mut(),
                    },
                    Err(_) => ptr::null_mut(),
                }
            }
            None => ptr::null_mut(),
        }
    })
}

/// Get the library version string.
///
/// # Safety
///
/// - Returns a static string that does not need to be freed
/// - The returned string is always valid
///
/// # Example (C)
///
/// ```c
/// const char* version = kreuzberg_version();
/// printf("Kreuzberg version: %s\n", version);
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{clear_last_error, set_last_error};
    use std::ffi::CStr;

    #[test]
    fn test_version_not_null() {
        let version = unsafe { kreuzberg_version() };
        assert!(!version.is_null());

        let version_str = unsafe { CStr::from_ptr(version).to_str().unwrap() };
        assert!(!version_str.is_empty());
        // Version should contain dots (e.g., "0.1.0")
        assert!(version_str.contains('.'));
    }

    #[test]
    fn test_last_error_null_when_no_error() {
        clear_last_error();
        let error = unsafe { kreuzberg_last_error() };
        assert!(error.is_null());
    }

    #[test]
    fn test_last_error_returns_message() {
        set_last_error("Test error message".to_string());
        let error = unsafe { kreuzberg_last_error() };
        assert!(!error.is_null());

        let error_str = unsafe { CStr::from_ptr(error).to_str().unwrap() };
        assert_eq!(error_str, "Test error message");

        clear_last_error();
    }

    #[test]
    fn test_last_error_code_success_by_default() {
        clear_last_error();
        let code = unsafe { kreuzberg_last_error_code() };
        assert_eq!(code, 0); // Success
    }

    #[test]
    fn test_last_panic_context_null_when_no_panic() {
        let context = unsafe { kreuzberg_last_panic_context() };
        // Should be null if no panic has occurred
        if !context.is_null() {
            unsafe {
                crate::kreuzberg_free_string(context);
            }
        }
    }

    #[test]
    fn test_error_code_values() {
        // Ensure error codes are in expected range
        let code = unsafe { kreuzberg_last_error_code() };
        assert!(code >= 0);
        assert!(code < 10); // Should be within reasonable bounds
    }
}
