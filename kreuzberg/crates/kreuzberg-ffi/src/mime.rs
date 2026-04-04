//! MIME type detection and validation functions.
//!
//! This module provides FFI functions for:
//! - Detecting MIME types from file paths, bytes, or content
//! - Validating MIME types against supported formats
//! - Getting file extensions for MIME types

use crate::ffi_panic_guard;
use crate::helpers::{clear_last_error, set_last_error, string_to_c_string};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

/// Detect MIME type from a file path.
///
/// # Safety
///
/// - `file_path` must be a valid null-terminated C string
/// - The returned string must be freed with `kreuzberg_free_string`
/// - Returns NULL on error (check `kreuzberg_last_error`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_detect_mime_type(file_path: *const c_char, check_exists: bool) -> *mut c_char {
    ffi_panic_guard!("kreuzberg_detect_mime_type", {
        clear_last_error();

        if file_path.is_null() {
            set_last_error("file_path cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let path_str = match unsafe { CStr::from_ptr(file_path) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in file path: {}", e));
                return ptr::null_mut();
            }
        };

        match kreuzberg::core::mime::detect_mime_type(path_str, check_exists) {
            Ok(mime) => match string_to_c_string(mime) {
                Ok(ptr) => ptr,
                Err(e) => {
                    set_last_error(e);
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Validate that a MIME type is supported by Kreuzberg.
///
/// # Safety
///
/// - `mime_type` must be a valid null-terminated C string
/// - The returned string must be freed with `kreuzberg_free_string`
/// - Returns NULL on error (check `kreuzberg_last_error`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_validate_mime_type(mime_type: *const c_char) -> *mut c_char {
    ffi_panic_guard!("kreuzberg_validate_mime_type", {
        clear_last_error();

        if mime_type.is_null() {
            set_last_error("mime_type cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let mime_type_str = match unsafe { CStr::from_ptr(mime_type) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in mime_type: {}", e));
                return ptr::null_mut();
            }
        };

        match kreuzberg::validate_mime_type(mime_type_str) {
            Ok(validated) => match string_to_c_string(validated) {
                Ok(ptr) => ptr,
                Err(e) => {
                    set_last_error(e);
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Detect MIME type from raw bytes.
///
/// # Safety
///
/// - `bytes` must point to a valid buffer of at least `len` bytes
/// - The returned string must be freed with `kreuzberg_free_string`
/// - Returns NULL on error (check `kreuzberg_last_error`)
///
/// # Example (C)
///
/// ```c
/// uint8_t data[512];
/// // ... read data ...
/// char* mime = kreuzberg_detect_mime_type_from_bytes(data, 512);
/// if (mime != NULL) {
///     printf("Detected MIME type: %s\n", mime);
///     kreuzberg_free_string(mime);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_detect_mime_type_from_bytes(bytes: *const u8, len: usize) -> *mut c_char {
    ffi_panic_guard!("kreuzberg_detect_mime_type_from_bytes", {
        clear_last_error();

        if bytes.is_null() {
            set_last_error("bytes cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let slice = unsafe { std::slice::from_raw_parts(bytes, len) };

        match kreuzberg::core::mime::detect_mime_type_from_bytes(slice) {
            Ok(mime) => match string_to_c_string(mime) {
                Ok(ptr) => ptr,
                Err(e) => {
                    set_last_error(e);
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Detect MIME type from file path (checks extension and reads file content).
///
/// # Safety
///
/// - `file_path` must be a valid null-terminated C string
/// - The returned string must be freed with `kreuzberg_free_string`
/// - Returns NULL on error (check `kreuzberg_last_error`)
///
/// # Example (C)
///
/// ```c
/// char* mime = kreuzberg_detect_mime_type_from_path("document.pdf");
/// if (mime == NULL) {
///     const char* error = kreuzberg_last_error();
///     printf("Failed to detect MIME type: %s\n", error);
/// } else {
///     printf("MIME type: %s\n", mime);
///     kreuzberg_free_string(mime);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_detect_mime_type_from_path(file_path: *const c_char) -> *mut c_char {
    ffi_panic_guard!("kreuzberg_detect_mime_type_from_path", {
        clear_last_error();

        if file_path.is_null() {
            set_last_error("file_path cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let path_str = match unsafe { CStr::from_ptr(file_path) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in file path: {}", e));
                return ptr::null_mut();
            }
        };

        match kreuzberg::core::mime::detect_mime_type(path_str, true) {
            Ok(mime) => match string_to_c_string(mime) {
                Ok(ptr) => ptr,
                Err(e) => {
                    set_last_error(e);
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Get file extensions for a MIME type.
///
/// Returns a JSON array of file extensions (e.g., ["pdf"] for "application/pdf").
///
/// # Safety
///
/// - `mime_type` must be a valid null-terminated C string
/// - The returned string must be freed with `kreuzberg_free_string`
/// - Returns NULL on error (check `kreuzberg_last_error`)
///
/// # Example (C)
///
/// ```c
/// char* extensions = kreuzberg_get_extensions_for_mime("application/pdf");
/// if (extensions != NULL) {
///     printf("Extensions: %s\n", extensions);
///     kreuzberg_free_string(extensions);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_get_extensions_for_mime(mime_type: *const c_char) -> *mut c_char {
    ffi_panic_guard!("kreuzberg_get_extensions_for_mime", {
        clear_last_error();

        if mime_type.is_null() {
            set_last_error("mime_type cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let mime_str = match unsafe { CStr::from_ptr(mime_type) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in MIME type: {}", e));
                return ptr::null_mut();
            }
        };

        match kreuzberg::core::mime::get_extensions_for_mime(mime_str) {
            Ok(extensions) => match serde_json::to_string(&extensions) {
                Ok(json) => match string_to_c_string(json) {
                    Ok(ptr) => ptr,
                    Err(e) => {
                        set_last_error(e);
                        ptr::null_mut()
                    }
                },
                Err(e) => {
                    set_last_error(format!("Failed to serialize extensions: {}", e));
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_detect_mime_type_null_path() {
        let result = unsafe { kreuzberg_detect_mime_type(ptr::null(), false) };
        assert!(result.is_null());
    }

    #[test]
    fn test_validate_mime_type_null() {
        let result = unsafe { kreuzberg_validate_mime_type(ptr::null()) };
        assert!(result.is_null());
    }

    #[test]
    fn test_detect_mime_type_from_bytes_null() {
        let result = unsafe { kreuzberg_detect_mime_type_from_bytes(ptr::null(), 0) };
        assert!(result.is_null());
    }

    #[test]
    fn test_detect_mime_type_from_path_null() {
        let result = unsafe { kreuzberg_detect_mime_type_from_path(ptr::null()) };
        assert!(result.is_null());
    }

    #[test]
    fn test_get_extensions_for_mime_null() {
        let result = unsafe { kreuzberg_get_extensions_for_mime(ptr::null()) };
        assert!(result.is_null());
    }

    #[test]
    fn test_validate_mime_type_valid() {
        let mime = CString::new("application/pdf").unwrap();
        let result = unsafe { kreuzberg_validate_mime_type(mime.as_ptr()) };
        assert!(!result.is_null());
        unsafe {
            crate::kreuzberg_free_string(result);
        }
    }

    #[test]
    fn test_detect_mime_type_from_bytes_pdf() {
        // PDF magic bytes: %PDF-
        let pdf_bytes = b"%PDF-1.4\n";
        let result = unsafe { kreuzberg_detect_mime_type_from_bytes(pdf_bytes.as_ptr(), pdf_bytes.len()) };
        assert!(!result.is_null());

        let mime_str = unsafe { CStr::from_ptr(result).to_str().unwrap() };
        assert_eq!(mime_str, "application/pdf");

        unsafe {
            crate::kreuzberg_free_string(result);
        }
    }

    #[test]
    fn test_get_extensions_for_mime_pdf() {
        let mime = CString::new("application/pdf").unwrap();
        let result = unsafe { kreuzberg_get_extensions_for_mime(mime.as_ptr()) };
        assert!(!result.is_null());

        let extensions_str = unsafe { CStr::from_ptr(result).to_str().unwrap() };
        assert!(extensions_str.contains("pdf"));

        unsafe {
            crate::kreuzberg_free_string(result);
        }
    }
}
