//! Core extraction FFI functions.
//!
//! This module provides the main FFI entry points for document extraction operations.
//! These functions are the most critical part of the FFI layer and handle both
//! synchronous file and byte array extraction operations, including batch processing.
//!
//! # Safety
//!
//! All functions in this module are marked as `unsafe extern "C"` because they interact
//! with raw C pointers and must follow strict memory management rules. Callers are
//! responsible for ensuring:
//! - All input pointers are valid and properly aligned
//! - All returned pointers are freed with the appropriate free functions
//! - Configuration JSON is valid UTF-8 and valid JSON
//!
//! # Memory Management
//!
//! The batch extraction functions have special memory management requirements:
//! - They allocate a vector of results and convert it to a boxed slice
//! - The slice pointer is cast and stored in the CBatchResult
//! - Deallocation must reconstruct the slice before freeing
//! - This is handled by `kreuzberg_free_batch_result` in the memory module

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

use kreuzberg::core::config::ExtractionConfig;

use crate::ffi_panic_guard;
use crate::helpers::{
    clear_last_error, parse_extraction_config_from_json, parse_file_config_from_json, set_last_error,
    to_c_extraction_result,
};
use crate::memory::kreuzberg_free_result;
use crate::types::{CBatchResult, CBytesWithMime, CExtractionResult};

/// Extract text and metadata from a file (synchronous).
///
/// # Safety
///
/// - `file_path` must be a valid null-terminated C string
/// - The returned pointer must be freed with `kreuzberg_free_result`
/// - Returns NULL on error (check `kreuzberg_last_error` for details)
///
/// # Example (C)
///
/// ```c
/// const char* path = "/path/to/document.pdf";
/// CExtractionResult* result = kreuzberg_extract_file_sync(path);
/// if (result != NULL && result->success) {
///     printf("Content: %s\n", result->content);
///     printf("MIME: %s\n", result->mime_type);
///     kreuzberg_free_result(result);
/// } else {
///     const char* error = kreuzberg_last_error();
///     printf("Error: %s\n", error);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_extract_file_sync(file_path: *const c_char) -> *mut CExtractionResult {
    ffi_panic_guard!("kreuzberg_extract_file_sync", {
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

        let path = Path::new(path_str);
        let config = ExtractionConfig::default();

        match kreuzberg::extract_file_sync(path, None, &config) {
            Ok(result) => match to_c_extraction_result(result) {
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

/// Extract text and metadata from a file with custom configuration (synchronous).
///
/// # Safety
///
/// - `file_path` must be a valid null-terminated C string
/// - `config_json` must be a valid null-terminated C string containing JSON, or NULL for default config
/// - The returned pointer must be freed with `kreuzberg_free_result`
/// - Returns NULL on error (check `kreuzberg_last_error` for details)
///
/// # Example (C)
///
/// ```c
/// const char* path = "/path/to/document.pdf";
/// const char* config = "{\"force_ocr\": true, \"ocr\": {\"language\": \"deu\"}}";
/// CExtractionResult* result = kreuzberg_extract_file_sync_with_config(path, config);
/// if (result != NULL && result->success) {
///     printf("Content: %s\n", result->content);
///     kreuzberg_free_result(result);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_extract_file_sync_with_config(
    file_path: *const c_char,
    config_json: *const c_char,
) -> *mut CExtractionResult {
    ffi_panic_guard!("kreuzberg_extract_file_sync_with_config", {
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

        let path = Path::new(path_str);

        let config = if config_json.is_null() {
            ExtractionConfig::default()
        } else {
            let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(format!("Invalid UTF-8 in config JSON: {}", e));
                    return ptr::null_mut();
                }
            };

            match parse_extraction_config_from_json(config_str) {
                Ok(cfg) => cfg,
                Err(e) => {
                    set_last_error(e);
                    return ptr::null_mut();
                }
            }
        };

        match kreuzberg::extract_file_sync(path, None, &config) {
            Ok(result) => match to_c_extraction_result(result) {
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

/// Extract text and metadata from byte array (synchronous).
///
/// # Safety
///
/// - `data` must be a valid pointer to a byte array of length `data_len`
/// - `mime_type` must be a valid null-terminated C string
/// - The returned pointer must be freed with `kreuzberg_free_result`
/// - Returns NULL on error (check `kreuzberg_last_error` for details)
///
/// # Example (C)
///
/// ```c
/// const uint8_t* data = ...; // Document bytes
/// size_t len = ...;           // Length of data
/// const char* mime = "application/pdf";
/// CExtractionResult* result = kreuzberg_extract_bytes_sync(data, len, mime);
/// if (result != NULL && result->success) {
///     printf("Content: %s\n", result->content);
///     kreuzberg_free_result(result);
/// } else {
///     const char* error = kreuzberg_last_error();
///     printf("Error: %s\n", error);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_extract_bytes_sync(
    data: *const u8,
    data_len: usize,
    mime_type: *const c_char,
) -> *mut CExtractionResult {
    ffi_panic_guard!("kreuzberg_extract_bytes_sync", {
        clear_last_error();

        if data.is_null() {
            set_last_error("data cannot be NULL".to_string());
            return ptr::null_mut();
        }

        if mime_type.is_null() {
            set_last_error("mime_type cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let bytes = unsafe { std::slice::from_raw_parts(data, data_len) };

        let mime_str = match unsafe { CStr::from_ptr(mime_type) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in MIME type: {}", e));
                return ptr::null_mut();
            }
        };

        let config = ExtractionConfig::default();

        match kreuzberg::extract_bytes_sync(bytes, mime_str, &config) {
            Ok(result) => match to_c_extraction_result(result) {
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

/// Extract text and metadata from byte array with custom configuration (synchronous).
///
/// # Safety
///
/// - `data` must be a valid pointer to a byte array of length `data_len`
/// - `mime_type` must be a valid null-terminated C string
/// - `config_json` must be a valid null-terminated C string containing JSON, or NULL for default config
/// - The returned pointer must be freed with `kreuzberg_free_result`
/// - Returns NULL on error (check `kreuzberg_last_error` for details)
///
/// # Example (C)
///
/// ```c
/// const uint8_t* data = ...; // Document bytes
/// size_t len = ...;           // Length of data
/// const char* mime = "application/pdf";
/// const char* config = "{\"force_ocr\": true, \"ocr\": {\"language\": \"deu\"}}";
/// CExtractionResult* result = kreuzberg_extract_bytes_sync_with_config(data, len, mime, config);
/// if (result != NULL && result->success) {
///     printf("Content: %s\n", result->content);
///     kreuzberg_free_result(result);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_extract_bytes_sync_with_config(
    data: *const u8,
    data_len: usize,
    mime_type: *const c_char,
    config_json: *const c_char,
) -> *mut CExtractionResult {
    ffi_panic_guard!("kreuzberg_extract_bytes_sync_with_config", {
        clear_last_error();

        if data.is_null() {
            set_last_error("data cannot be NULL".to_string());
            return ptr::null_mut();
        }

        if mime_type.is_null() {
            set_last_error("mime_type cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let bytes = unsafe { std::slice::from_raw_parts(data, data_len) };

        let mime_str = match unsafe { CStr::from_ptr(mime_type) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in MIME type: {}", e));
                return ptr::null_mut();
            }
        };

        let config = if config_json.is_null() {
            ExtractionConfig::default()
        } else {
            let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(format!("Invalid UTF-8 in config JSON: {}", e));
                    return ptr::null_mut();
                }
            };

            match parse_extraction_config_from_json(config_str) {
                Ok(cfg) => cfg,
                Err(e) => {
                    set_last_error(e);
                    return ptr::null_mut();
                }
            }
        };

        match kreuzberg::extract_bytes_sync(bytes, mime_str, &config) {
            Ok(result) => match to_c_extraction_result(result) {
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

/// Batch extract text and metadata from multiple files with optional per-file config overrides (synchronous).
///
/// # Safety
///
/// - `file_paths` must be a valid pointer to an array of null-terminated C strings
/// - `file_config_jsons` must be NULL (no per-file configs) or a valid pointer to an array of
///   `count` nullable C strings (null entries use the base config, non-null entries are parsed as
///   JSON `FileExtractionConfig`)
/// - `count` must be the number of items in both arrays
/// - `config_json` must be a valid null-terminated C string containing JSON, or NULL for default config
/// - The returned pointer must be freed with `kreuzberg_free_batch_result`
/// - Returns NULL on error (check `kreuzberg_last_error` for details)
///
/// # Critical Memory Management
///
/// This function shares the same critical memory management pattern as
/// `kreuzberg_batch_extract_files_sync`. See that function's documentation
/// for details on the Box/Vec/slice allocation pattern.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_batch_extract_files_sync(
    file_paths: *const *const c_char,
    file_config_jsons: *const *const c_char,
    count: usize,
    config_json: *const c_char,
) -> *mut CBatchResult {
    ffi_panic_guard!("kreuzberg_batch_extract_files_sync", {
        clear_last_error();

        if file_paths.is_null() {
            set_last_error("file_paths cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let config = if config_json.is_null() {
            ExtractionConfig::default()
        } else {
            let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(format!("Invalid UTF-8 in config JSON: {}", e));
                    return ptr::null_mut();
                }
            };

            match parse_extraction_config_from_json(config_str) {
                Ok(cfg) => cfg,
                Err(e) => {
                    set_last_error(e);
                    return ptr::null_mut();
                }
            }
        };

        let mut items: Vec<(std::path::PathBuf, Option<kreuzberg::FileExtractionConfig>)> = Vec::with_capacity(count);
        for i in 0..count {
            let path_ptr = unsafe { *file_paths.add(i) };
            if path_ptr.is_null() {
                set_last_error(format!("File path at index {} is NULL", i));
                return ptr::null_mut();
            }

            let path_str = match unsafe { CStr::from_ptr(path_ptr) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(format!("Invalid UTF-8 in file path at index {}: {}", i, e));
                    return ptr::null_mut();
                }
            };

            let file_config = if file_config_jsons.is_null() {
                None
            } else {
                let file_config_ptr = unsafe { *file_config_jsons.add(i) };
                match unsafe { parse_file_config_from_json(file_config_ptr) } {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        set_last_error(format!("Failed to parse file config at index {}: {}", i, e));
                        return ptr::null_mut();
                    }
                }
            };

            items.push((std::path::PathBuf::from(path_str), file_config));
        }

        match kreuzberg::batch_extract_file_sync(items, &config) {
            Ok(results) => {
                let mut c_results = Vec::with_capacity(results.len());
                for result in results {
                    match to_c_extraction_result(result) {
                        Ok(ptr) => c_results.push(ptr),
                        Err(e) => {
                            for c_res in c_results {
                                unsafe { kreuzberg_free_result(c_res) };
                            }
                            set_last_error(e);
                            return ptr::null_mut();
                        }
                    }
                }

                let actual_count = c_results.len();
                let results_array = c_results.into_boxed_slice();
                let results_ptr = Box::into_raw(results_array) as *mut *mut CExtractionResult;

                Box::into_raw(Box::new(CBatchResult {
                    results: results_ptr,
                    count: actual_count,
                    success: true,
                    _padding2: [0u8; 7],
                }))
            }
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Batch extract text and metadata from multiple byte arrays with per-file config overrides (synchronous).
///
/// # Safety
///
/// - `items` must be a valid pointer to an array of CBytesWithMime structures
/// - `file_config_jsons` must be NULL (no per-file configs) or a valid pointer to an array of
///   `count` nullable C strings (null entries use the base config, non-null entries are parsed as
///   JSON `FileExtractionConfig`)
/// - `count` must be the number of items in both arrays
/// - `config_json` must be a valid null-terminated C string containing JSON, or NULL for default config
/// - The returned pointer must be freed with `kreuzberg_free_batch_result`
/// - Returns NULL on error (check `kreuzberg_last_error` for details)
///
/// # Critical Memory Management
///
/// This function shares the same critical memory management pattern as
/// `kreuzberg_batch_extract_files_sync`. See that function's documentation
/// for details on the Box/Vec/slice allocation pattern.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_batch_extract_bytes_sync(
    items: *const CBytesWithMime,
    file_config_jsons: *const *const c_char,
    count: usize,
    config_json: *const c_char,
) -> *mut CBatchResult {
    ffi_panic_guard!("kreuzberg_batch_extract_bytes_sync", {
        clear_last_error();

        if items.is_null() {
            set_last_error("items cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let config = if config_json.is_null() {
            ExtractionConfig::default()
        } else {
            let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(format!("Invalid UTF-8 in config JSON: {}", e));
                    return ptr::null_mut();
                }
            };

            match parse_extraction_config_from_json(config_str) {
                Ok(cfg) => cfg,
                Err(e) => {
                    set_last_error(e);
                    return ptr::null_mut();
                }
            }
        };

        let mut contents: Vec<(Vec<u8>, String, Option<kreuzberg::FileExtractionConfig>)> = Vec::with_capacity(count);
        for i in 0..count {
            let item = unsafe { &*items.add(i) };

            if item.data.is_null() {
                set_last_error(format!("Data at index {} is NULL", i));
                return ptr::null_mut();
            }

            if item.mime_type.is_null() {
                set_last_error(format!("MIME type at index {} is NULL", i));
                return ptr::null_mut();
            }

            let bytes = unsafe { std::slice::from_raw_parts(item.data, item.data_len) };

            let mime_str = match unsafe { CStr::from_ptr(item.mime_type) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(format!("Invalid UTF-8 in MIME type at index {}: {}", i, e));
                    return ptr::null_mut();
                }
            };

            let file_config = if file_config_jsons.is_null() {
                None
            } else {
                let file_config_ptr = unsafe { *file_config_jsons.add(i) };
                match unsafe { parse_file_config_from_json(file_config_ptr) } {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        set_last_error(format!("Failed to parse file config at index {}: {}", i, e));
                        return ptr::null_mut();
                    }
                }
            };

            contents.push((bytes.to_vec(), mime_str.to_string(), file_config));
        }

        match kreuzberg::batch_extract_bytes_sync(contents, &config) {
            Ok(results) => {
                let mut c_results = Vec::with_capacity(results.len());
                for result in results {
                    match to_c_extraction_result(result) {
                        Ok(ptr) => c_results.push(ptr),
                        Err(e) => {
                            for c_res in c_results {
                                unsafe { kreuzberg_free_result(c_res) };
                            }
                            set_last_error(e);
                            return ptr::null_mut();
                        }
                    }
                }

                let actual_count = c_results.len();
                let results_array = c_results.into_boxed_slice();
                let results_ptr = Box::into_raw(results_array) as *mut *mut CExtractionResult;

                Box::into_raw(Box::new(CBatchResult {
                    results: results_ptr,
                    count: actual_count,
                    success: true,
                    _padding2: [0u8; 7],
                }))
            }
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}
