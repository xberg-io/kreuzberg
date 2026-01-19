use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

// Plugin impl stub - not currently functional after module refactoring

/// Register a custom OCR backend
///
/// Registers a JavaScript OCR backend that can process images and extract text.
///
/// # Arguments
///
/// * `backend` - JavaScript object with the following interface:
///   - `name(): string` - Unique backend name
///   - `supportedLanguages(): string[]` - Array of supported ISO 639-2/3 language codes
///   - `processImage(imageBytes: string, language: string): Promise<result>` - Process image and return extraction result
///
/// # Implementation Notes
///
/// Due to NAPI ThreadsafeFunction limitations, the processImage function receives:
/// - `imageBytes` as a Base64 string (first argument)
/// - `language` as string (second argument)
///
/// And must return a Promise resolving to a JSON-serializable object with:
/// ```typescript
/// {
///   content: string,
///   mime_type: string,  // default: "text/plain"
///   metadata: object,   // default: {}
///   tables: array       // default: []
/// }
/// ```
///
/// # Example
///
/// ```typescript
/// import { registerOcrBackend } from '@kreuzberg/node';
///
/// registerOcrBackend({
///   name: () => "my-ocr",
///   supportedLanguages: () => ["eng", "deu", "fra"],
///   processImage: async (imageBytes, language) => {
///     const buffer = Buffer.from(imageBytes, "base64");
///     const text = await myOcrLibrary.process(buffer, language);
///     return {
///       content: text,
///       mime_type: "text/plain",
///       metadata: { confidence: 0.95 },
///       tables: []
///     };
///   }
/// });
/// ```
#[allow(dead_code)]
pub struct JsOcrBackend {
    pub name: String,
    pub process_image_fn: ThreadsafeFunction<(String, String)>,
    pub supported_languages: Vec<String>,
}

#[napi]
pub fn register_ocr_backend(_backend: Object) -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "register_ocr_backend not yet implemented for refactored module",
    ))
}

#[napi]
pub fn unregister_ocr_backend(_name: String) -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "unregister_ocr_backend not yet implemented",
    ))
}

#[napi]
pub fn list_ocr_backends() -> Result<Vec<String>> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "list_ocr_backends not yet implemented",
    ))
}

#[napi]
pub fn clear_ocr_backends() -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "clear_ocr_backends not yet implemented",
    ))
}
