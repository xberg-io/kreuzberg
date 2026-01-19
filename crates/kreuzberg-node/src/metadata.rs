use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ffi::CStr;

use crate::kreuzberg_get_valid_ocr_backends;
use crate::kreuzberg_free_string;
use crate::error_handling::convert_error;

#[napi]
pub fn list_document_extractors() -> Result<Vec<String>> {
    kreuzberg::plugins::list_extractors().map_err(convert_error)
}

/// Unregister a document extractor by name.
///
/// Removes the specified document extractor from the registry. If the extractor
/// doesn't exist, this operation is a no-op (does not throw an error).
///
/// # Parameters
///
/// * `name` - Name of the document extractor to unregister
///
/// # Example
///
/// ```typescript
/// import { unregisterDocumentExtractor } from 'kreuzberg';
///
/// // Unregister a custom extractor
/// unregisterDocumentExtractor('MyCustomExtractor');
/// ```
#[napi]
pub fn unregister_document_extractor(name: String) -> Result<()> {
    kreuzberg::plugins::unregister_extractor(&name).map_err(convert_error)
}

/// Clear all registered document extractors.
///
/// Removes all document extractors from the registry, including built-in extractors.
/// Use with caution as this will make document extraction unavailable until
/// extractors are re-registered.
///
/// # Example
///
/// ```typescript
/// import { clearDocumentExtractors } from 'kreuzberg';
///
/// clearDocumentExtractors();
/// ```
#[napi]
pub fn clear_document_extractors() -> Result<()> {
    kreuzberg::plugins::clear_extractors().map_err(convert_error)
}

/// Detect MIME type from raw bytes.
///
/// Uses content inspection (magic bytes) to determine MIME type.
/// This is more accurate than extension-based detection but requires
/// reading the file content.
///
/// # Parameters
///
/// * `bytes` - Raw file content as Buffer
///
/// # Returns
///
/// The detected MIME type string.
///
/// # Errors
///
/// Throws an error if MIME type cannot be determined from content.
///
/// # Example
///
/// ```typescript
/// import { detectMimeTypeFromBytes } from 'kreuzberg';
/// import * as fs from 'fs';
///
/// // Read file content
/// const content = fs.readFileSync('document.pdf');
///
/// // Detect MIME type from bytes
/// const mimeType = detectMimeTypeFromBytes(content);
/// ```
#[napi(js_name = "detectMimeTypeFromBytes")]
pub fn detect_mime_type_from_bytes(bytes: Buffer) -> Result<String> {
    kreuzberg::core::mime::detect_mime_type_from_bytes(bytes.as_ref()).map_err(convert_error)
}
