//! Core extraction functions
//!
//! Provides extraction functions for PHP.

use ext_php_rs::binary_slice::BinarySlice;
use ext_php_rs::prelude::*;

use crate::config::parse_config_from_json;
use crate::error::to_php_exception;
use crate::types::ExtractionResult;

/// Extract content from a file.
///
/// # Parameters
///
/// - `path` (string): Path to the file to extract
/// - `mime_type` (string|null): Optional MIME type hint (auto-detected if null)
/// - `config_json` (string|null): JSON-encoded extraction configuration (uses defaults if null)
///
/// # Returns
///
/// ExtractionResult with content, metadata, and tables
///
/// # Throws
///
/// - ValidationException: Invalid configuration or unsupported format
/// - ParsingException: Document parsing failed
/// - OcrException: OCR processing failed
/// - Exception: File access errors or other runtime errors
///
/// # Example
///
/// ```php
/// // Simple extraction with defaults
/// $result = kreuzberg_extract_file("document.pdf");
/// echo $result->content;
///
/// // With custom configuration
/// $config = new \Kreuzberg\Config\ExtractionConfig(
///     useCache: false,
///     ocr: new \Kreuzberg\Config\OcrConfig(language: "deu")
/// );
/// $result = kreuzberg_extract_file("german.pdf", null, $config->toJson());
/// ```
#[php_function]
pub fn kreuzberg_extract_file(
    path: String,
    mime_type: Option<String>,
    config_json: Option<String>,
) -> PhpResult<ExtractionResult> {
    let rust_config = match config_json {
        Some(json) => parse_config_from_json(&json).map_err(PhpException::from)?,
        None => Default::default(),
    };

    let result = kreuzberg::extract_file_sync(&path, mime_type.as_deref(), &rust_config).map_err(to_php_exception)?;

    ExtractionResult::from_rust(result)
}

/// Extract content from bytes.
///
/// # Parameters
///
/// - `data` (string): Binary data to extract (bytes)
/// - `mime_type` (string): MIME type of the data
/// - `config_json` (string|null): JSON-encoded extraction configuration (uses defaults if null)
///
/// # Returns
///
/// ExtractionResult with content, metadata, and tables
///
/// # Throws
///
/// - ValidationException: Invalid configuration or unsupported format
/// - ParsingException: Document parsing failed
/// - OcrException: OCR processing failed
/// - Exception: Runtime errors
///
/// # Example
///
/// ```php
/// $data = file_get_contents("document.pdf");
/// $result = kreuzberg_extract_bytes($data, "application/pdf");
/// echo $result->content;
///
/// // With custom configuration
/// $config = new \Kreuzberg\Config\ExtractionConfig(forceOcr: true);
/// $result = kreuzberg_extract_bytes($data, "application/pdf", $config->toJson());
/// ```
#[php_function]
pub fn kreuzberg_extract_bytes(
    data: BinarySlice<u8>,
    mime_type: String,
    config_json: Option<String>,
) -> PhpResult<ExtractionResult> {
    let rust_config = match config_json {
        Some(json) => parse_config_from_json(&json).map_err(PhpException::from)?,
        None => Default::default(),
    };

    if crate::plugins::has_custom_extractor(&mime_type) {
        match crate::plugins::call_custom_extractor(&mime_type, data.as_ref()) {
            Ok(result_zval) => {
                if let Some(result_array) = result_zval.array() {
                    let content = result_array
                        .get("content")
                        .and_then(|v| v.str())
                        .ok_or_else(|| PhpException::default("Custom extractor result missing 'content'".to_string()))?
                        .to_string();

                    let metadata = if let Some(_meta_val) = result_array.get("metadata") {
                        // TODO: Implement metadata conversion
                        Default::default()
                    } else {
                        Default::default()
                    };

                    let tables = if let Some(_tables_val) = result_array.get("tables") {
                        // TODO: Implement tables conversion
                        vec![]
                    } else {
                        vec![]
                    };

                    let rust_result = kreuzberg::types::ExtractionResult {
                        content,
                        mime_type: mime_type.clone(),
                        metadata,
                        tables,
                        detected_languages: None,
                        chunks: None,
                        images: None,
                        pages: None,
                    };

                    return ExtractionResult::from_rust(rust_result);
                }
            }
            Err(e) => {
                eprintln!("Custom extractor failed for '{}': {:?}", mime_type, e);
            }
        }
    }

    let result = kreuzberg::extract_bytes_sync(data.as_ref(), &mime_type, &rust_config).map_err(to_php_exception)?;

    ExtractionResult::from_rust(result)
}

/// Batch extract content from multiple files.
///
/// MIME types are auto-detected for each file.
///
/// # Parameters
///
/// - `paths` (array): Array of file paths (strings)
/// - `config` (ExtractionConfig|null): Extraction configuration (uses defaults if null)
///
/// # Returns
///
/// Array of ExtractionResult objects (one per file)
///
/// # Throws
///
/// - ValidationException: Invalid configuration
/// - ParsingException: Document parsing failed
/// - Exception: File access errors or runtime errors
///
/// # Example
///
/// ```php
/// $paths = ["doc1.pdf", "doc2.docx", "doc3.txt"];
/// $results = kreuzberg_batch_extract_files($paths);
///
/// foreach ($results as $i => $result) {
///     echo "Document {$i}: {$result->mime_type}\n";
///     echo substr($result->content, 0, 100) . "...\n";
/// }
///
/// // With custom configuration
/// $config = new \Kreuzberg\Config\ExtractionConfig(useCache: false);
/// $results = kreuzberg_batch_extract_files($paths, $config->toJson());
/// ```
#[php_function]
pub fn kreuzberg_batch_extract_files(
    paths: Vec<String>,
    config_json: Option<String>,
) -> PhpResult<Vec<ExtractionResult>> {
    let rust_config = match config_json {
        Some(json) => parse_config_from_json(&json).map_err(PhpException::from)?,
        None => Default::default(),
    };

    let results = kreuzberg::batch_extract_file_sync(paths, &rust_config).map_err(to_php_exception)?;

    results.into_iter().map(ExtractionResult::from_rust).collect()
}

/// Batch extract content from multiple byte arrays.
///
/// # Parameters
///
/// - `data_list` (array): Array of binary data (bytes)
/// - `mime_types` (array): Array of MIME types (one per data element)
/// - `config_json` (string|null): JSON-encoded extraction configuration (uses defaults if null)
///
/// # Returns
///
/// Array of ExtractionResult objects (one per data element)
///
/// # Throws
///
/// - ValidationException: Invalid configuration or list length mismatch
/// - ParsingException: Document parsing failed
/// - Exception: Runtime errors
///
/// # Example
///
/// ```php
/// $data1 = file_get_contents("doc1.pdf");
/// $data2 = file_get_contents("doc2.pdf");
/// $data_list = [$data1, $data2];
/// $mime_types = ["application/pdf", "application/pdf"];
///
/// $results = kreuzberg_batch_extract_bytes($data_list, $mime_types);
///
/// foreach ($results as $result) {
///     echo $result->content . "\n\n";
/// }
/// ```
#[php_function]
pub fn kreuzberg_batch_extract_bytes(
    data_list: Vec<BinarySlice<u8>>,
    mime_types: Vec<String>,
    config_json: Option<String>,
) -> PhpResult<Vec<ExtractionResult>> {
    if data_list.len() != mime_types.len() {
        return Err(format!(
            "data_list and mime_types must have the same length (got {} and {})",
            data_list.len(),
            mime_types.len()
        )
        .into());
    }

    let rust_config = match config_json {
        Some(json) => parse_config_from_json(&json).map_err(PhpException::from)?,
        None => Default::default(),
    };

    // Convert Vec<BinarySlice<u8>> to Vec<(Vec<u8>, String)> for the core function
    // BinarySlice provides a reference to the binary data from PHP
    let owned_contents: Vec<(Vec<u8>, String)> = data_list
        .into_iter()
        .zip(mime_types)
        .map(|(binary_slice, mime)| {
            let bytes: &[u8] = binary_slice.as_ref();
            (bytes.to_vec(), mime)
        })
        .collect();

    let results = kreuzberg::batch_extract_bytes_sync(owned_contents, &rust_config).map_err(to_php_exception)?;

    results.into_iter().map(ExtractionResult::from_rust).collect()
}

/// Detect MIME type from file bytes.
///
/// # Parameters
///
/// - `data` (string): File content as bytes
///
/// # Returns
///
/// Detected MIME type string (e.g., "application/pdf", "image/png")
///
/// # Example
///
/// ```php
/// $data = file_get_contents("unknown_file");
/// $mime_type = kreuzberg_detect_mime_type_from_bytes($data);
/// echo "Detected type: $mime_type\n";
/// ```
#[php_function]
pub fn kreuzberg_detect_mime_type_from_bytes(data: BinarySlice<u8>) -> PhpResult<String> {
    kreuzberg::detect_mime_type_from_bytes(data.as_ref())
        .map_err(|e| format!("Failed to detect MIME type: {}", e).into())
}

/// Detect MIME type from file bytes (alias for kreuzberg_detect_mime_type_from_bytes).
///
/// # Parameters
///
/// - `data` (string): File content as bytes
///
/// # Returns
///
/// Detected MIME type string (e.g., "application/pdf", "image/png")
///
/// # Example
///
/// ```php
/// $data = file_get_contents("unknown_file");
/// $mime_type = kreuzberg_detect_mime_type($data);
/// echo "Detected type: $mime_type\n";
/// ```
#[php_function]
pub fn kreuzberg_detect_mime_type(data: BinarySlice<u8>) -> PhpResult<String> {
    kreuzberg::detect_mime_type_from_bytes(data.as_ref())
        .map_err(|e| format!("Failed to detect MIME type: {}", e).into())
}

/// Detect MIME type from file path.
///
/// # Parameters
///
/// - `path` (string): Path to the file
///
/// # Returns
///
/// Detected MIME type string (e.g., "application/pdf", "text/plain")
///
/// # Example
///
/// ```php
/// $mime_type = kreuzberg_detect_mime_type_from_path("document.pdf");
/// echo "File type: $mime_type\n";
/// ```
#[php_function]
pub fn kreuzberg_detect_mime_type_from_path(path: String) -> PhpResult<String> {
    kreuzberg::detect_mime_type(&path, true).map_err(|e| format!("Failed to detect MIME type: {}", e).into())
}

/// Validate and normalize a MIME type.
///
/// # Parameters
///
/// - `mime_type` (string): MIME type to validate
///
/// # Returns
///
/// Normalized MIME type string
///
/// # Throws
///
/// - Exception: If the MIME type is not supported
///
/// # Example
///
/// ```php
/// try {
///     $normalized = kreuzberg_validate_mime_type("application/pdf");
///     echo "Valid MIME type: $normalized\n";
/// } catch (Exception $e) {
///     echo "Invalid MIME type: {$e->getMessage()}\n";
/// }
/// ```
#[php_function]
pub fn kreuzberg_validate_mime_type(mime_type: String) -> PhpResult<String> {
    kreuzberg::validate_mime_type(&mime_type).map_err(|e| format!("Invalid MIME type: {}", e).into())
}

/// Get file extensions for a MIME type.
///
/// # Parameters
///
/// - `mime_type` (string): MIME type (e.g., "application/pdf")
///
/// # Returns
///
/// Array of file extensions (e.g., ["pdf"])
///
/// # Example
///
/// ```php
/// $extensions = kreuzberg_get_extensions_for_mime("application/pdf");
/// print_r($extensions); // ["pdf"]
/// ```
#[php_function]
pub fn kreuzberg_get_extensions_for_mime(mime_type: String) -> PhpResult<Vec<String>> {
    kreuzberg::get_extensions_for_mime(&mime_type).map_err(|e| format!("Failed to get extensions: {}", e).into())
}

/// Returns all function builders for the extraction module.
pub fn get_function_builders() -> Vec<ext_php_rs::builders::FunctionBuilder<'static>> {
    vec![
        wrap_function!(kreuzberg_extract_file),
        wrap_function!(kreuzberg_extract_bytes),
        wrap_function!(kreuzberg_batch_extract_files),
        wrap_function!(kreuzberg_batch_extract_bytes),
        wrap_function!(kreuzberg_detect_mime_type_from_bytes),
        wrap_function!(kreuzberg_detect_mime_type),
        wrap_function!(kreuzberg_detect_mime_type_from_path),
        wrap_function!(kreuzberg_validate_mime_type),
        wrap_function!(kreuzberg_get_extensions_for_mime),
    ]
}
