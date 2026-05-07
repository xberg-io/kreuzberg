//! JSONL (newline-delimited JSON) integration tests.
//!
//! Tests end-to-end extraction of `.jsonl` / `.ndjson` files through the full
//! extraction pipeline, verifying content preservation, metadata extraction,
//! and blank-line handling.

use kreuzberg::core::config::ExtractionConfig;
use kreuzberg::core::extractor::{extract_bytes, extract_file};

#[tokio::test]
async fn test_extract_jsonl_file() {
    let config = ExtractionConfig::default();
    let path = std::path::Path::new("test_documents/jsonl/simple.jsonl");

    let result = extract_file(path, None, &config)
        .await
        .expect("JSONL file extraction should succeed");

    assert!(result.content.contains("Alice"), "Should contain Alice");
    assert!(result.content.contains("Bob"), "Should contain Bob");
    assert!(result.content.contains("Carol"), "Should contain Carol");
}

#[tokio::test]
async fn test_extract_jsonl_bytes() {
    let config = ExtractionConfig::default();
    let jsonl = b"{\"name\": \"Alice\"}\n{\"name\": \"Bob\"}";

    let result = extract_bytes(jsonl, "application/x-ndjson", &config)
        .await
        .expect("JSONL bytes extraction should succeed");

    assert!(result.content.contains("Alice"));
    assert!(result.content.contains("Bob"));
}

#[tokio::test]
async fn test_extract_jsonl_metadata() {
    let config = ExtractionConfig::default();
    let jsonl = b"{\"title\": \"Doc One\"}\n{\"title\": \"Doc Two\"}";

    let result = extract_bytes(jsonl, "application/x-ndjson", &config)
        .await
        .expect("JSONL metadata extraction should succeed");

    let data_format = result.metadata.additional.get("data_format");
    assert!(data_format.is_some(), "Metadata should contain data_format");
    assert_eq!(
        data_format.unwrap().as_str().unwrap(),
        "jsonl",
        "data_format should be 'jsonl'"
    );
}

#[tokio::test]
async fn test_extract_jsonl_empty_lines() {
    let config = ExtractionConfig::default();
    let path = std::path::Path::new("test_documents/jsonl/with_blanks.jsonl");

    let result = extract_file(path, None, &config)
        .await
        .expect("JSONL with blanks should succeed");

    assert!(result.content.contains("First"), "Should contain First");
    assert!(result.content.contains("Second"), "Should contain Second");
    assert!(result.content.contains("Third"), "Should contain Third");
}

#[tokio::test]
async fn test_extract_jsonl_content_contains_all_objects() {
    let config = ExtractionConfig::default();
    let jsonl = b"{\"a\": 1}\n{\"b\": 2}\n{\"c\": 3}";

    let result = extract_bytes(jsonl, "application/x-ndjson", &config)
        .await
        .expect("JSONL extraction should succeed");

    // Content is pretty-printed JSON array
    assert!(result.content.contains("\"a\": 1"));
    assert!(result.content.contains("\"b\": 2"));
    assert!(result.content.contains("\"c\": 3"));
}
