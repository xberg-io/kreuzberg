use napi::bindgen_prelude::*;
use napi_derive::napi;

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

/// Detect MIME type from a file path.
///
/// Determines the MIME type based on the file extension in the provided path.
/// By default, checks if the file exists; can be disabled with check_exists parameter.
///
/// # Parameters
///
/// * `path` - The file path to detect MIME type from (e.g., 'document.pdf')
/// * `check_exists` - Whether to verify the file exists (default: true)
///
/// # Returns
///
/// The detected MIME type as a string (e.g., 'application/pdf').
///
/// # Errors
///
/// Throws an error if MIME type cannot be determined from the file extension,
/// or if check_exists is true and the file does not exist.
///
/// # Example
///
/// ```typescript
/// import { detectMimeTypeFromPath } from 'kreuzberg';
///
/// // Detect MIME type from existing file
/// const mimeType = detectMimeTypeFromPath('/path/to/document.pdf');
///
/// // Detect without checking file existence
/// const mimeType2 = detectMimeTypeFromPath('document.docx', false);
/// ```
#[napi]
pub fn detect_mime_type_from_path(path: String, check_exists: Option<bool>) -> Result<String> {
    kreuzberg::core::mime::detect_mime_type(&path, check_exists.unwrap_or(true)).map_err(convert_error)
}
