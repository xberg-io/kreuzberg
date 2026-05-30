// JNI bindings for kreuzberg library.
// Delegates to FFI (kreuzberg-ffi) for all operations.
#![allow(non_snake_case, unsafe_code, unsafe_attr_outside_unsafe)]

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;
use std::ffi::{CStr, CString};

// ============================================================================
// FFI Function Declarations
// These are C symbols exported from kreuzberg-ffi
// ============================================================================

unsafe extern "C" {
    // Config handling
    fn kreuzberg_extraction_config_from_json(json: *const std::ffi::c_char)
        -> *mut std::ffi::c_void;
    fn kreuzberg_extraction_config_free(ptr: *mut std::ffi::c_void);

    // Extraction functions
    fn kreuzberg_extract_bytes(
        content: *const u8,
        content_len: usize,
        mime_type: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    fn kreuzberg_extract_file(
        path: *const std::ffi::c_char,
        mime_type: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    fn kreuzberg_extract_file_sync(
        path: *const std::ffi::c_char,
        mime_type: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    fn kreuzberg_extract_bytes_sync(
        content: *const u8,
        content_len: usize,
        mime_type: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    // Batch extraction
    fn kreuzberg_batch_extract_files_sync(
        items: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_char;

    fn kreuzberg_batch_extract_bytes_sync(
        items: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_char;

    fn kreuzberg_batch_extract_files(
        items: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_char;

    fn kreuzberg_batch_extract_bytes(
        items: *const std::ffi::c_char,
        config: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_char;

    // MIME detection
    fn kreuzberg_detect_mime_type_from_bytes(
        content: *const u8,
        content_len: usize,
    ) -> *mut std::ffi::c_char;

    fn kreuzberg_detect_mime_type(
        path: *const std::ffi::c_char,
        check_exists: i32,
    ) -> *mut std::ffi::c_char;

    // Result handling
    fn kreuzberg_extraction_result_to_json(ptr: *const std::ffi::c_void) -> *mut std::ffi::c_char;

    fn kreuzberg_extraction_result_free(ptr: *mut std::ffi::c_void);

    // List/Clear backends
    fn kreuzberg_list_embedding_backends() -> *mut std::ffi::c_char;
    fn kreuzberg_clear_embedding_backend(out_error: *mut *mut std::ffi::c_char) -> i32;

    fn kreuzberg_list_document_extractors() -> *mut std::ffi::c_char;
    fn kreuzberg_clear_document_extractor(out_error: *mut *mut std::ffi::c_char) -> i32;

    fn kreuzberg_list_ocr_backends() -> *mut std::ffi::c_char;
    fn kreuzberg_clear_ocr_backend(out_error: *mut *mut std::ffi::c_char) -> i32;

    fn kreuzberg_list_post_processors() -> *mut std::ffi::c_char;
    fn kreuzberg_clear_post_processor(out_error: *mut *mut std::ffi::c_char) -> i32;

    fn kreuzberg_list_renderers() -> *mut std::ffi::c_char;
    fn kreuzberg_clear_renderer(out_error: *mut *mut std::ffi::c_char) -> i32;

    fn kreuzberg_list_validators() -> *mut std::ffi::c_char;
    fn kreuzberg_clear_validator(out_error: *mut *mut std::ffi::c_char) -> i32;

    // Embeddings
    fn kreuzberg_embed_texts(
        texts: *const std::ffi::c_char,
        config: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;

    fn kreuzberg_list_embedding_presets() -> *mut std::ffi::c_char;

    fn kreuzberg_get_embedding_preset(name: *const std::ffi::c_char) -> *mut std::ffi::c_char;

    // PDF rendering
    fn kreuzberg_render_pdf_page_to_png(
        pdf_bytes: *const u8,
        pdf_bytes_len: usize,
        page_index: u32,
        dpi: u32,
        password: *const std::ffi::c_char,
    ) -> *mut u8;

    // Memory management
    fn kreuzberg_free_string(ptr: *mut std::ffi::c_char);
    fn kreuzberg_free_bytes(ptr: *mut u8, len: usize, cap: usize);
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to throw a KreuzbergBridgeException and return null/0
fn throw_exception<'local>(env: &mut JNIEnv<'local>, message: &str) -> jstring {
    let _ = env.throw_new("dev/kreuzberg/KreuzbergBridgeException", message);
    std::ptr::null_mut()
}

/// Helper for void functions that throw
fn throw_exception_void(env: &mut JNIEnv, message: &str) {
    let _ = env.throw_new("dev/kreuzberg/KreuzbergBridgeException", message);
}

/// Convert JString to Rust String, returning an error message if conversion fails
fn jstring_to_string<'local>(
    env: &mut JNIEnv<'local>,
    jstr: &JString<'local>,
) -> Result<String, String> {
    env.get_string(jstr)
        .map(|j| j.into())
        .map_err(|e| format!("Failed to convert JString: {}", e))
}

/// Convert Rust String to jstring, returning null if allocation fails
fn string_to_jstring(env: &mut JNIEnv, s: &str) -> jstring {
    env.new_string(s)
        .map(|js| js.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Convert a C string pointer to JString, reading the result from FFI
fn cstring_ptr_to_jstring(env: &mut JNIEnv, ptr: *mut std::ffi::c_char) -> jstring {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: ptr must be a valid C string returned by FFI
    let c_str = unsafe { CStr::from_ptr(ptr) };
    match c_str.to_str() {
        Ok(s) => string_to_jstring(env, s),
        Err(_) => {
            throw_exception(env, "Invalid UTF-8 in FFI response");
            std::ptr::null_mut()
        }
    }
}

// ============================================================================
// Extraction Functions
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractBytesImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    content: JString<'local>,
    mime_type: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let content_str = match jstring_to_string(&mut env, &content) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let mime_type_str = match jstring_to_string(&mut env, &mime_type) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    // Convert content string to bytes
    let content_bytes = content_str.into_bytes();

    let mime_type_c = match CString::new(mime_type_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid mime_type: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    // Parse config JSON into ExtractionConfig
    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString and config_ptr
    let result = unsafe {
        kreuzberg_extract_bytes(
            content_bytes.as_ptr(),
            content_bytes.len(),
            mime_type_c.as_ptr(),
            config_ptr,
        )
    };

    if result.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Extract bytes failed");
    }

    // SAFETY: result is a valid ExtractionResult pointer from FFI; convert to JSON
    let json_ptr = unsafe { kreuzberg_extraction_result_to_json(result) };
    let jstr = cstring_ptr_to_jstring(&mut env, json_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_result_free(result);
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(json_ptr);
    }
    jstr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractFileImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
    mime_type: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let path_str = match jstring_to_string(&mut env, &path) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let mime_type_str = match jstring_to_string(&mut env, &mime_type) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let path_c = match CString::new(path_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid path: {}", e)),
    };

    let mime_type_c = match CString::new(mime_type_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid mime_type: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result = unsafe {
        kreuzberg_extract_file(path_c.as_ptr(), mime_type_c.as_ptr(), config_ptr)
    };

    if result.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Extract file failed");
    }

    // SAFETY: result is a valid ExtractionResult pointer from FFI
    let json_ptr = unsafe { kreuzberg_extraction_result_to_json(result) };
    let jstr = cstring_ptr_to_jstring(&mut env, json_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_result_free(result);
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(json_ptr);
    }
    jstr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractFileSyncImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
    mime_type: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let path_str = match jstring_to_string(&mut env, &path) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let mime_type_str = match jstring_to_string(&mut env, &mime_type) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let path_c = match CString::new(path_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid path: {}", e)),
    };

    let mime_type_c = match CString::new(mime_type_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid mime_type: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result = unsafe {
        kreuzberg_extract_file_sync(path_c.as_ptr(), mime_type_c.as_ptr(), config_ptr)
    };

    if result.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Extract file sync failed");
    }

    // SAFETY: result is a valid ExtractionResult pointer from FFI
    let json_ptr = unsafe { kreuzberg_extraction_result_to_json(result) };
    let jstr = cstring_ptr_to_jstring(&mut env, json_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_result_free(result);
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(json_ptr);
    }
    jstr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractBytesSyncImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    content: JString<'local>,
    mime_type: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let content_str = match jstring_to_string(&mut env, &content) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let mime_type_str = match jstring_to_string(&mut env, &mime_type) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let content_bytes = content_str.into_bytes();

    let mime_type_c = match CString::new(mime_type_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid mime_type: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result = unsafe {
        kreuzberg_extract_bytes_sync(
            content_bytes.as_ptr(),
            content_bytes.len(),
            mime_type_c.as_ptr(),
            config_ptr,
        )
    };

    if result.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Extract bytes sync failed");
    }

    // SAFETY: result is a valid ExtractionResult pointer from FFI
    let json_ptr = unsafe { kreuzberg_extraction_result_to_json(result) };
    let jstr = cstring_ptr_to_jstring(&mut env, json_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_result_free(result);
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(json_ptr);
    }
    jstr
}

// ============================================================================
// Batch Extraction Functions
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeBatchExtractFilesSyncImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    items: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let items_str = match jstring_to_string(&mut env, &items) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let items_c = match CString::new(items_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid items: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result_ptr = unsafe { kreuzberg_batch_extract_files_sync(items_c.as_ptr(), config_ptr) };

    if result_ptr.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Batch extract files sync failed");
    }

    let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(result_ptr);
    }
    jstr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeBatchExtractBytesSyncImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    items: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let items_str = match jstring_to_string(&mut env, &items) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let items_c = match CString::new(items_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid items: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result_ptr = unsafe { kreuzberg_batch_extract_bytes_sync(items_c.as_ptr(), config_ptr) };

    if result_ptr.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Batch extract bytes sync failed");
    }

    let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(result_ptr);
    }
    jstr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeBatchExtractFilesImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    items: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let items_str = match jstring_to_string(&mut env, &items) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let items_c = match CString::new(items_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid items: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result_ptr = unsafe { kreuzberg_batch_extract_files(items_c.as_ptr(), config_ptr) };

    if result_ptr.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Batch extract files failed");
    }

    let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(result_ptr);
    }
    jstr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeBatchExtractBytesImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    items: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let items_str = match jstring_to_string(&mut env, &items) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let items_c = match CString::new(items_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid items: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    let config_ptr = unsafe { kreuzberg_extraction_config_from_json(config_c.as_ptr()) };
    if config_ptr.is_null() {
        return throw_exception(&mut env, "Failed to parse config JSON");
    }

    // SAFETY: We have valid pointers from CString
    let result_ptr = unsafe { kreuzberg_batch_extract_bytes(items_c.as_ptr(), config_ptr) };

    if result_ptr.is_null() {
        unsafe {
            kreuzberg_extraction_config_free(config_ptr);
        }
        return throw_exception(&mut env, "Batch extract bytes failed");
    }

    let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
    // Clean up
    unsafe {
        kreuzberg_extraction_config_free(config_ptr);
        kreuzberg_free_string(result_ptr);
    }
    jstr
}

// ============================================================================
// MIME Type Detection
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeDetectMimeTypeFromBytesImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    content: JString<'local>,
) -> jstring {
    let content_str = match jstring_to_string(&mut env, &content) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let content_bytes = content_str.into_bytes();

    // SAFETY: We have a valid slice from Vec
    let result_ptr = unsafe {
        kreuzberg_detect_mime_type_from_bytes(content_bytes.as_ptr(), content_bytes.len())
    };

    if result_ptr.is_null() {
        throw_exception(&mut env, "Detect MIME type failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeDetectMimeTypeImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
    check_exists: jboolean,
) -> jstring {
    let path_str = match jstring_to_string(&mut env, &path) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let path_c = match CString::new(path_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid path: {}", e)),
    };

    // SAFETY: We have a valid C string
    let result_ptr = unsafe { kreuzberg_detect_mime_type(path_c.as_ptr(), check_exists as i32) };

    if result_ptr.is_null() {
        throw_exception(&mut env, "Detect MIME type failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeGetExtensionsForMimeImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    _mime_type: JString<'local>,
) -> jstring {
    // For now, return empty JSON array since FFI doesn't expose this function
    string_to_jstring(&mut env, "[]")
}

// ============================================================================
// Embedding Functions
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeEmbedTextsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    texts: JString<'local>,
    config: JString<'local>,
) -> jstring {
    let texts_str = match jstring_to_string(&mut env, &texts) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let config_str = match jstring_to_string(&mut env, &config) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let texts_c = match CString::new(texts_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid texts: {}", e)),
    };

    let config_c = match CString::new(config_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid config: {}", e)),
    };

    // SAFETY: We have valid pointers from CString
    let result_ptr = unsafe { kreuzberg_embed_texts(texts_c.as_ptr(), config_c.as_ptr()) };

    if result_ptr.is_null() {
        throw_exception(&mut env, "Embed texts failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListEmbeddingPresetsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_embedding_presets() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List embedding presets failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeGetEmbeddingPresetImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
) -> jstring {
    let name_str = match jstring_to_string(&mut env, &name) {
        Ok(s) => s,
        Err(e) => return throw_exception(&mut env, &e),
    };

    let name_c = match CString::new(name_str) {
        Ok(cs) => cs,
        Err(e) => return throw_exception(&mut env, &format!("Invalid name: {}", e)),
    };

    // SAFETY: We have a valid C string
    let result_ptr = unsafe { kreuzberg_get_embedding_preset(name_c.as_ptr()) };

    if result_ptr.is_null() {
        // Return null (None in Kotlin)
        std::ptr::null_mut()
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

// ============================================================================
// List/Clear Backend Functions
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListEmbeddingBackendsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_embedding_backends() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List embedding backends failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeClearEmbeddingBackendsImpl(
    mut env: JNIEnv,
    _class: JClass,
) {
    // SAFETY: Function takes no parameters other than out_error
    let mut err_ptr = std::ptr::null_mut();
    let code = unsafe { kreuzberg_clear_embedding_backend(&mut err_ptr) };

    if code != 0 {
        let msg = if err_ptr.is_null() {
            format!("Clear embedding backends failed with code {}", code)
        } else {
            let c_str = unsafe { CStr::from_ptr(err_ptr) };
            c_str.to_string_lossy().to_string()
        };
        throw_exception_void(&mut env, &msg);
        // Clean up error string
        if !err_ptr.is_null() {
            unsafe {
                kreuzberg_free_string(err_ptr);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListDocumentExtractorsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_document_extractors() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List document extractors failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeClearDocumentExtractorsImpl(
    mut env: JNIEnv,
    _class: JClass,
) {
    // SAFETY: Function takes no parameters other than out_error
    let mut err_ptr = std::ptr::null_mut();
    let code = unsafe { kreuzberg_clear_document_extractor(&mut err_ptr) };

    if code != 0 {
        let msg = if err_ptr.is_null() {
            format!("Clear document extractors failed with code {}", code)
        } else {
            let c_str = unsafe { CStr::from_ptr(err_ptr) };
            c_str.to_string_lossy().to_string()
        };
        throw_exception_void(&mut env, &msg);
        if !err_ptr.is_null() {
            unsafe {
                kreuzberg_free_string(err_ptr);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListOcrBackendsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_ocr_backends() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List OCR backends failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeClearOcrBackendsImpl(
    mut env: JNIEnv,
    _class: JClass,
) {
    // SAFETY: Function takes no parameters other than out_error
    let mut err_ptr = std::ptr::null_mut();
    let code = unsafe { kreuzberg_clear_ocr_backend(&mut err_ptr) };

    if code != 0 {
        let msg = if err_ptr.is_null() {
            format!("Clear OCR backends failed with code {}", code)
        } else {
            let c_str = unsafe { CStr::from_ptr(err_ptr) };
            c_str.to_string_lossy().to_string()
        };
        throw_exception_void(&mut env, &msg);
        if !err_ptr.is_null() {
            unsafe {
                kreuzberg_free_string(err_ptr);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListPostProcessorsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_post_processors() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List post processors failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeClearPostProcessorsImpl(
    mut env: JNIEnv,
    _class: JClass,
) {
    // SAFETY: Function takes no parameters other than out_error
    let mut err_ptr = std::ptr::null_mut();
    let code = unsafe { kreuzberg_clear_post_processor(&mut err_ptr) };

    if code != 0 {
        let msg = if err_ptr.is_null() {
            format!("Clear post processors failed with code {}", code)
        } else {
            let c_str = unsafe { CStr::from_ptr(err_ptr) };
            c_str.to_string_lossy().to_string()
        };
        throw_exception_void(&mut env, &msg);
        if !err_ptr.is_null() {
            unsafe {
                kreuzberg_free_string(err_ptr);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListRenderersImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_renderers() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List renderers failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeClearRenderersImpl(
    mut env: JNIEnv,
    _class: JClass,
) {
    // SAFETY: Function takes no parameters other than out_error
    let mut err_ptr = std::ptr::null_mut();
    let code = unsafe { kreuzberg_clear_renderer(&mut err_ptr) };

    if code != 0 {
        let msg = if err_ptr.is_null() {
            format!("Clear renderers failed with code {}", code)
        } else {
            let c_str = unsafe { CStr::from_ptr(err_ptr) };
            c_str.to_string_lossy().to_string()
        };
        throw_exception_void(&mut env, &msg);
        if !err_ptr.is_null() {
            unsafe {
                kreuzberg_free_string(err_ptr);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeListValidatorsImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // SAFETY: Function takes no parameters
    let result_ptr = unsafe { kreuzberg_list_validators() };

    if result_ptr.is_null() {
        throw_exception(&mut env, "List validators failed")
    } else {
        let jstr = cstring_ptr_to_jstring(&mut env, result_ptr);
        // Clean up
        unsafe {
            kreuzberg_free_string(result_ptr);
        }
        jstr
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeClearValidatorsImpl(
    mut env: JNIEnv,
    _class: JClass,
) {
    // SAFETY: Function takes no parameters other than out_error
    let mut err_ptr = std::ptr::null_mut();
    let code = unsafe { kreuzberg_clear_validator(&mut err_ptr) };

    if code != 0 {
        let msg = if err_ptr.is_null() {
            format!("Clear validators failed with code {}", code)
        } else {
            let c_str = unsafe { CStr::from_ptr(err_ptr) };
            c_str.to_string_lossy().to_string()
        };
        throw_exception_void(&mut env, &msg);
        if !err_ptr.is_null() {
            unsafe {
                kreuzberg_free_string(err_ptr);
            }
        }
    }
}

// ============================================================================
// PDF Rendering
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_kreuzberg_KreuzbergBridge_nativeRenderPdfPageToPngImpl<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pdf_bytes: JString<'local>,
    page_index: jlong,
    dpi: jint,
    password: JString<'local>,
) -> jbyteArray {
    let pdf_bytes_str = match jstring_to_string(&mut env, &pdf_bytes) {
        Ok(s) => s,
        Err(e) => {
            throw_exception(&mut env, &e);
            return std::ptr::null_mut();
        }
    };

    let password_str = match jstring_to_string(&mut env, &password) {
        Ok(s) => s,
        Err(e) => {
            throw_exception(&mut env, &e);
            return std::ptr::null_mut();
        }
    };

    let pdf_bytes_data = pdf_bytes_str.into_bytes();

    let password_c = match CString::new(password_str) {
        Ok(cs) => cs,
        Err(e) => {
            throw_exception(&mut env, &format!("Invalid password: {}", e));
            return std::ptr::null_mut();
        }
    };

    // SAFETY: We have valid pointers
    let _result_ptr = unsafe {
        kreuzberg_render_pdf_page_to_png(
            pdf_bytes_data.as_ptr(),
            pdf_bytes_data.len(),
            page_index as u32,
            dpi as u32,
            password_c.as_ptr(),
        )
    };

    // For now, return empty byte array since length tracking is complex
    // TODO: Fix with proper length tracking from FFI
    env.byte_array_from_slice(&[])
        .map(|ba| ba.into_raw())
        .unwrap_or(std::ptr::null_mut())
}
