//! Integration tests for core extraction functionality.
//!
//! These tests verify the end-to-end behavior of the extraction pipeline,
//! config loading, MIME detection, and batch processing.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use xberg::core::mime::validate_mime_type;
use xberg::{
    ExtractInput, ExtractedDocument, ExtractionConfig, ExtractionResult, detect_mime_type, extract, extract_batch,
};

fn trim_trailing_newlines(value: &str) -> &str {
    value.trim_end_matches(['\n', '\r'])
}

fn assert_text_content(actual: &str, expected: &str) {
    assert_eq!(
        trim_trailing_newlines(actual),
        expected,
        "Content mismatch after trimming trailing newlines"
    );
}

fn file_input(path: impl AsRef<Path>, mime_type: Option<&str>) -> ExtractInput {
    let mut input = ExtractInput::from_uri(path.as_ref().to_string_lossy().into_owned());
    input.mime_type = mime_type.map(str::to_string);
    input
}

fn successful_single_result(output: &ExtractionResult) -> &ExtractedDocument {
    assert_successful_envelope(output, 1, 1);
    &output.results[0]
}

fn assert_successful_envelope(output: &ExtractionResult, input_count: usize, result_count: usize) {
    assert_eq!(output.summary.inputs, input_count);
    assert_eq!(output.summary.results, result_count);
    assert_eq!(output.summary.errors, 0);
    assert_eq!(output.results.len(), result_count);
    assert!(output.errors.is_empty(), "Expected no extraction errors");
}

/// Test basic file extraction with MIME detection.
#[tokio::test]
async fn test_extract_file_basic() {
    let dir = tempdir().expect("Operation failed");
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).expect("Operation failed");
    file.write_all(b"Hello, Xberg!").expect("Operation failed");

    let config = ExtractionConfig::default();
    let result = extract(file_input(&file_path, None), &config).await;

    assert!(result.is_ok(), "Basic file extraction should succeed");
    let output = result.expect("Operation failed");
    let result = successful_single_result(&output);

    assert_text_content(&result.content, "Hello, Xberg!");
    assert_eq!(result.mime_type, "text/plain");
    assert!(result.chunks.is_none(), "Chunks should be None without chunking config");
    assert!(result.detected_languages.is_none(), "Language detection not enabled");
    assert!(result.tables.is_empty(), "Text file should not have tables");
}

/// Test extraction with explicit MIME type override.
#[tokio::test]
async fn test_extract_file_with_mime_override() {
    let dir = tempdir().expect("Operation failed");
    let file_path = dir.path().join("data.bin");
    let mut file = File::create(&file_path).expect("Operation failed");
    file.write_all(b"Binary content").expect("Operation failed");

    let config = ExtractionConfig::default();
    let result = extract(file_input(&file_path, Some("text/plain")), &config).await;

    assert!(result.is_ok(), "MIME override should work");
    let output = result.expect("Operation failed");
    let result = successful_single_result(&output);

    assert_eq!(result.mime_type, "text/plain");
    assert!(!result.content.is_empty(), "Should extract content");
    assert!(result.chunks.is_none(), "Chunks should be None without chunking config");
}

/// Test extraction of multiple file types.
#[tokio::test]
async fn test_extract_multiple_file_types() {
    let dir = tempdir().expect("Operation failed");
    let config = ExtractionConfig::default();

    let test_files: Vec<(&str, &[u8], &str)> = vec![
        ("test.txt", b"text content", "text/plain"),
        ("test.json", b"{\"key\": \"value\"}", "application/json"),
        #[cfg(feature = "xml")]
        ("test.xml", b"<root>data</root>", "application/xml"),
        #[cfg(feature = "html")]
        ("test.html", b"<html><body>test</body></html>", "text/html"),
    ];

    for (filename, content, expected_mime) in test_files {
        let file_path = dir.path().join(filename);
        fs::write(&file_path, content).expect("Operation failed");

        let output = extract(file_input(&file_path, None), &config)
            .await
            .expect("Async operation failed");
        let result = successful_single_result(&output);

        assert_eq!(result.mime_type, expected_mime, "MIME type mismatch for {}", filename);
        assert!(
            !result.content.is_empty(),
            "Content should not be empty for {}",
            filename
        );
        assert!(result.chunks.is_none(), "Chunks should be None for {}", filename);
        assert!(
            result.detected_languages.is_none(),
            "Language detection not enabled for {}",
            filename
        );
    }
}

/// Test extract_bytes_document with various MIME types.
#[tokio::test]
async fn test_extract_bytes_various_mime_types() {
    let config = ExtractionConfig::default();

    let test_cases: Vec<(&[u8], &str)> = vec![
        (b"text content", "text/plain"),
        (b"{\"key\": \"value\"}", "application/json"),
        #[cfg(feature = "xml")]
        (b"<root>data</root>", "application/xml"),
    ];

    for (content, mime_type) in test_cases {
        let result = extract(
            ExtractInput::from_bytes(content.to_vec(), mime_type.to_string(), None),
            &config,
        )
        .await;
        assert!(result.is_ok(), "Extract bytes failed for MIME type: {}", mime_type);

        let output = result.expect("Operation failed");
        let result = successful_single_result(&output);

        assert_eq!(result.mime_type, mime_type, "MIME type mismatch");
        assert!(
            !result.content.is_empty(),
            "Content should not be empty for {}",
            mime_type
        );
        assert!(result.chunks.is_none(), "Chunks should be None without chunking config");
        assert!(result.detected_languages.is_none(), "Language detection not enabled");
    }
}

/// Test batch extraction with concurrent processing.
#[tokio::test]
async fn test_batch_extract_file_concurrency() {
    let dir = tempdir().expect("Operation failed");
    let config = ExtractionConfig::default();

    let num_files = 10;
    let mut paths = Vec::new();

    for i in 0..num_files {
        let file_path = dir.path().join(format!("test_{}.txt", i));
        fs::write(&file_path, format!("Content {}", i)).expect("Operation failed");
        paths.push(file_path);
    }

    let results = extract_batch(
        paths
            .clone()
            .into_iter()
            .map(|path| file_input(path, None))
            .collect::<Vec<_>>(),
        &config,
    )
    .await;
    assert!(results.is_ok());

    let results = results.expect("Operation failed");
    assert_successful_envelope(&results, num_files, num_files);

    for (i, result) in results.results.iter().enumerate() {
        assert!(
            result.content.contains(&i.to_string()),
            "Content should contain file number"
        );
        assert_eq!(result.mime_type, "text/plain", "MIME type should be text/plain");
        assert!(result.chunks.is_none(), "Chunks should be None without chunking config");
        assert!(result.detected_languages.is_none(), "Language detection not enabled");
        assert!(result.metadata.error.is_none(), "Should not have errors");
    }
}

/// Test batch extraction with empty input.
#[tokio::test]
async fn test_batch_extract_empty() {
    let config = ExtractionConfig::default();

    let results = extract_batch(Vec::<ExtractInput>::new(), &config).await;
    assert!(results.is_ok());
    assert_successful_envelope(&results.expect("Operation failed"), 0, 0);
}

/// Test extract_bytes_documents with concurrent processing.
#[tokio::test]
async fn test_batch_extract_bytes_concurrency() {
    let config = ExtractionConfig::default();

    let contents = vec![
        (b"content 1".as_slice(), "text/plain"),
        (b"content 2".as_slice(), "text/plain"),
        (b"content 3".as_slice(), "text/plain"),
        (b"content 4".as_slice(), "text/plain"),
        (b"content 5".as_slice(), "text/plain"),
    ];

    let inputs: Vec<ExtractInput> = contents
        .into_iter()
        .map(|(bytes, mime)| ExtractInput::from_bytes(bytes.to_vec(), mime.to_string(), None))
        .collect();

    let results = extract_batch(inputs, &config).await;
    assert!(results.is_ok());

    let results = results.expect("Operation failed");
    assert_successful_envelope(&results, 5, 5);

    for (i, result) in results.results.iter().enumerate() {
        let expected_content = format!("content {}", i + 1);
        assert_eq!(
            trim_trailing_newlines(&result.content),
            expected_content,
            "Content mismatch for item {}",
            i
        );
        assert_eq!(result.mime_type, "text/plain", "MIME type should be text/plain");
        assert!(result.chunks.is_none(), "Chunks should be None without chunking config");
        assert!(result.detected_languages.is_none(), "Language detection not enabled");
        assert!(result.metadata.error.is_none(), "Should not have errors");
    }
}

/// Test MIME type detection for various extensions.
#[test]
fn test_mime_detection_comprehensive() {
    let dir = tempdir().expect("Operation failed");

    let test_cases = vec![
        ("test.txt", "text/plain"),
        ("test.md", "text/markdown"),
        ("test.html", "text/html"),
        ("test.json", "application/json"),
        ("test.yaml", "application/x-yaml"),
        ("test.toml", "application/toml"),
        ("test.xml", "application/xml"),
        ("test.pdf", "application/pdf"),
        (
            "test.xlsx",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
        (
            "test.docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
        (
            "test.pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ),
        ("test.png", "image/png"),
        ("test.jpg", "image/jpeg"),
        ("test.gif", "image/gif"),
        ("test.eml", "message/rfc822"),
    ];

    for (filename, expected_mime) in test_cases {
        let file_path = dir.path().join(filename);
        File::create(&file_path).expect("Operation failed");

        let detected = detect_mime_type(file_path.to_string_lossy().into_owned(), true).expect("Operation failed");
        assert_eq!(detected, expected_mime, "Failed for {}", filename);

        let validated = validate_mime_type(&detected);
        assert!(validated.is_ok(), "Validation failed for {}", expected_mime);
    }
}

/// Test MIME type validation.
#[test]
fn test_mime_validation() {
    assert!(validate_mime_type("application/pdf").is_ok());
    assert!(validate_mime_type("text/plain").is_ok());
    assert!(validate_mime_type("image/png").is_ok());
    assert!(validate_mime_type("image/custom-format").is_ok());

    // video/mp4 (and other audio/video) are now declared formats; extraction requires transcription feature
    assert!(validate_mime_type("application/unknown").is_err());
}

/// Test case-insensitive extension handling.
#[test]
fn test_case_insensitive_extensions() {
    let dir = tempdir().expect("Operation failed");

    let test_cases = vec![
        ("test.PDF", "application/pdf"),
        ("test.TXT", "text/plain"),
        ("test.Json", "application/json"),
        (
            "test.XLSX",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
    ];

    for (filename, expected_mime) in test_cases {
        let file_path = dir.path().join(filename);
        File::create(&file_path).expect("Operation failed");

        let detected = detect_mime_type(file_path.to_string_lossy().into_owned(), true).expect("Operation failed");
        assert_eq!(detected, expected_mime, "Failed for {}", filename);
    }
}

/// Test config loading from TOML file.
#[test]
fn test_config_loading() {
    let dir = tempdir().expect("Operation failed");
    let config_path = dir.path().join("xberg.toml");

    fs::write(
        &config_path,
        r#"
use_cache = false
enable_quality_processing = true
force_ocr = false

[ocr]
backend = "tesseract"
language = "deu"

[chunking]
max_chars = 2000
max_overlap = 300
    "#,
    )
    .expect("Operation failed");

    let config = ExtractionConfig::from_toml_file(&config_path).expect("Operation failed");

    assert!(!config.use_cache);
    assert!(config.enable_quality_processing);
    assert!(!config.force_ocr);

    let ocr_config = config.ocr.expect("Operation failed");
    assert_eq!(ocr_config.backend, "tesseract");
    assert_eq!(ocr_config.language, vec!["deu".to_string()]);

    let chunking_config = config.chunking.expect("Operation failed");
    assert_eq!(chunking_config.max_characters, 2000);
    assert_eq!(chunking_config.overlap, 300);
}

/// Test config discovery in parent directories.
#[test]
fn test_config_discovery() {
    let dir = tempdir().expect("Operation failed");
    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir).expect("Operation failed");

    let config_path = dir.path().join("xberg.toml");
    fs::write(
        &config_path,
        r#"
use_cache = false
enable_quality_processing = true
    "#,
    )
    .expect("Operation failed");

    let original_dir = std::env::current_dir().expect("Operation failed");
    std::env::set_current_dir(&subdir).expect("Operation failed");

    let config = ExtractionConfig::discover().expect("Operation failed");
    assert!(config.is_some());
    assert!(!config.expect("Operation failed").use_cache);

    std::env::set_current_dir(original_dir).expect("Operation failed");
}

/// Test error handling for nonexistent files.
#[tokio::test]
async fn test_nonexistent_file_error() {
    let config = ExtractionConfig::default();
    let result = extract(ExtractInput::from_uri("/nonexistent/file.txt"), &config).await;

    assert!(result.is_err());
    // File validation returns Io error for missing files (NotFound)
    assert!(matches!(result.unwrap_err(), xberg::XbergError::Io(_)));
}

/// Test error handling for unsupported MIME types.
#[tokio::test]
async fn test_unsupported_mime_type_error() {
    let config = ExtractionConfig::default();
    // video/mp4 is now a declared format (routes to transcription extractor when
    // that feature is enabled). Use a genuinely unsupported type instead.
    let result = extract(
        ExtractInput::from_bytes(
            b"test".to_vec(),
            "application/x-xberg-test-unsupported".to_string(),
            None,
        ),
        &config,
    )
    .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), xberg::XbergError::UnsupportedFormat(_)));
}

/// Test validation for malformed unified extraction inputs.
#[tokio::test]
async fn test_malformed_extract_input_validation() {
    let config = ExtractionConfig::default();

    let mut missing_bytes = ExtractInput::from_bytes(Vec::<u8>::new(), "text/plain", None);
    missing_bytes.bytes = None;
    let error = extract(missing_bytes, &config)
        .await
        .expect_err("missing bytes field should fail validation");
    assert!(matches!(&error, xberg::XbergError::Validation { .. }));
    assert!(
        error.to_string().contains("requires the 'bytes' field"),
        "Unexpected validation error: {error}"
    );

    let error = extract(ExtractInput::default(), &config)
        .await
        .expect_err("missing uri field should fail validation");
    assert!(matches!(&error, xberg::XbergError::Validation { .. }));
    assert!(
        error.to_string().contains("requires the 'uri' field"),
        "Unexpected validation error: {error}"
    );
}

/// Test batch validation reports malformed inputs in the V1 envelope.
#[tokio::test]
async fn test_extract_batch_reports_malformed_inputs_in_envelope() {
    let config = ExtractionConfig::default();

    let mut missing_bytes = ExtractInput::from_bytes(Vec::<u8>::new(), "text/plain", None);
    missing_bytes.bytes = None;

    let output = extract_batch(vec![missing_bytes, ExtractInput::default()], &config)
        .await
        .expect("Batch extraction should collect per-input validation errors");

    assert_eq!(output.results.len(), 0);
    assert_eq!(output.errors.len(), 2);
    assert_eq!(output.summary.inputs, 2);
    assert_eq!(output.summary.results, 0);
    assert_eq!(output.summary.errors, 2);

    assert_eq!(output.errors[0].index, 0);
    assert_eq!(output.errors[0].code, 1002);
    assert_eq!(output.errors[0].error_type, "validation");
    assert_eq!(output.errors[0].source, "<bytes>");
    assert!(output.errors[0].message.contains("requires the 'bytes' field"));

    assert_eq!(output.errors[1].index, 1);
    assert_eq!(output.errors[1].code, 1002);
    assert_eq!(output.errors[1].error_type, "validation");
    assert_eq!(output.errors[1].source, "<uri>");
    assert!(output.errors[1].message.contains("requires the 'uri' field"));
}

/// Test pipeline execution (currently stub, will be expanded in Phase 2).
#[tokio::test]
async fn test_pipeline_execution() {
    let dir = tempdir().expect("Operation failed");
    let file_path = dir.path().join("pipeline_test.txt");
    fs::write(&file_path, "pipeline content").expect("Operation failed");

    let config = ExtractionConfig {
        enable_quality_processing: true,
        ..Default::default()
    };

    let result = extract(file_input(&file_path, None), &config).await;
    assert!(result.is_ok(), "Pipeline execution should succeed");

    let output = result.expect("Operation failed");
    let result = successful_single_result(&output);
    assert_text_content(&result.content, "pipeline content");
    assert_eq!(result.mime_type, "text/plain");
    assert!(result.chunks.is_none(), "Chunks should be None without chunking config");
    assert!(result.detected_languages.is_none(), "Language detection not enabled");
}

/// Test extraction with OCR config (placeholder test for Phase 2).
#[tokio::test]
async fn test_extraction_with_ocr_config() {
    let dir = tempdir().expect("Operation failed");
    let file_path = dir.path().join("ocr_test.txt");
    fs::write(&file_path, "ocr content").expect("Operation failed");

    let config = ExtractionConfig {
        ocr: Some(xberg::OcrConfig {
            backend: "tesseract".to_string(),
            language: vec!["eng".to_string()],
            ..Default::default()
        }),
        force_ocr: true,
        ..Default::default()
    };

    let result = extract(file_input(&file_path, None), &config).await;
    assert!(result.is_ok());
}

/// Test extraction with chunking config.
#[cfg(feature = "chunking")]
#[tokio::test]
async fn test_extraction_with_chunking_config() {
    let dir = tempdir().expect("Operation failed");
    let file_path = dir.path().join("chunking_test.txt");

    let long_content = "content for chunking. ".repeat(100);
    fs::write(&file_path, &long_content).expect("Operation failed");

    let config = ExtractionConfig {
        chunking: Some(xberg::ChunkingConfig {
            max_characters: 100,
            overlap: 20,
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract(file_input(&file_path, None), &config).await;
    assert!(result.is_ok(), "Extraction with chunking should succeed");

    let output = result.expect("Operation failed");
    let result = successful_single_result(&output);

    assert!(
        result.chunks.is_some(),
        "Chunks should be populated when chunking enabled"
    );

    let chunks = result.chunks.as_ref().expect("Operation failed");
    assert!(chunks.len() > 1, "Should have multiple chunks for long content");

    for chunk in chunks {
        assert!(
            chunk.content.len() <= 100 + 20,
            "Chunk length {} exceeds max_chars + overlap",
            chunk.content.len()
        );
    }
}
