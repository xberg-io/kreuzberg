//! PST extraction integration tests.
//!
//! Tests for .pst (Microsoft Outlook Personal Folders) extraction.
//! Validates MIME type detection, metadata extraction, and content extraction.

#![cfg(feature = "email")]

use xberg::core::config::ExtractionConfig;

mod helpers;
use helpers::extract_bytes_document;

/// Test that invalid PST data returns an error, not a panic.
#[tokio::test]
async fn test_pst_invalid_data_returns_error() {
    let config = ExtractionConfig::default();
    let result = extract_bytes_document(b"not a pst file", "application/vnd.ms-outlook-pst", &config).await;
    assert!(result.is_err(), "Invalid PST data should return an error");
}

/// Test that empty PST file is accepted and returns zero messages.
#[tokio::test]
async fn test_pst_empty_file_extraction() {
    let pst_path = helpers::get_test_file_path("email/empty.pst");
    assert!(pst_path.exists(), "PST test fixture not found: {:?}", pst_path);

    let pst_bytes = std::fs::read(&pst_path).expect("Should read empty.pst");
    let config = ExtractionConfig::default();

    let result = extract_bytes_document(&pst_bytes, "application/vnd.ms-outlook-pst", &config)
        .await
        .expect("Should extract empty PST without error");

    assert_eq!(result.mime_type.as_ref(), "application/vnd.ms-outlook-pst");

    // Empty PST has no messages
    let message_count = result
        .metadata
        .additional
        .get("message_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(u64::MAX);
    assert_eq!(message_count, 0, "Empty PST should have 0 messages");
}

/// Test that message_count is present in additional metadata.
#[tokio::test]
async fn test_pst_message_count_in_metadata() {
    let pst_path = helpers::get_test_file_path("email/empty.pst");
    assert!(pst_path.exists(), "PST test fixture not found: {:?}", pst_path);

    let pst_bytes = std::fs::read(&pst_path).expect("Should read empty.pst");
    let config = ExtractionConfig::default();

    let result = extract_bytes_document(&pst_bytes, "application/vnd.ms-outlook-pst", &config)
        .await
        .expect("Should extract empty PST");

    assert!(
        result.metadata.additional.contains_key("message_count"),
        "Extraction result should include message_count in additional metadata"
    );
}
