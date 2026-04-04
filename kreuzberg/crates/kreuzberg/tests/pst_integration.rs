//! PST extraction integration tests.
//!
//! Tests for .pst (Microsoft Outlook Personal Folders) extraction.
//! Validates MIME type detection, metadata extraction, and content extraction.

#![cfg(feature = "email")]

use kreuzberg::core::config::ExtractionConfig;
use kreuzberg::core::extractor::extract_bytes;
use kreuzberg::extraction::pst::extract_pst_messages;

mod helpers;

/// Test that invalid PST data returns an error, not a panic.
#[tokio::test]
async fn test_pst_invalid_data_returns_error() {
    let config = ExtractionConfig::default();
    let result = extract_bytes(b"not a pst file", "application/vnd.ms-outlook-pst", &config).await;
    assert!(result.is_err(), "Invalid PST data should return an error");
}

/// Test that empty PST file is accepted and returns zero messages.
#[tokio::test]
async fn test_pst_empty_file_extraction() {
    let pst_path = helpers::get_test_file_path("email/empty.pst");
    assert!(pst_path.exists(), "PST test fixture not found: {:?}", pst_path);

    let pst_bytes = std::fs::read(&pst_path).expect("Should read empty.pst");
    let config = ExtractionConfig::default();

    let result = extract_bytes(&pst_bytes, "application/vnd.ms-outlook-pst", &config)
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

/// Test that extract_pst_messages returns empty vec for an empty PST.
#[test]
fn test_extract_pst_messages_empty_pst() {
    let pst_path = helpers::get_test_file_path("email/empty.pst");
    assert!(pst_path.exists(), "PST test fixture not found: {:?}", pst_path);

    let pst_bytes = std::fs::read(&pst_path).expect("Should read empty.pst");
    let (messages, _warnings) = extract_pst_messages(&pst_bytes).expect("Should parse empty PST");
    assert_eq!(messages.len(), 0, "Empty PST should yield no messages");
}

/// Test that extract_pst_messages fails gracefully on garbage input.
#[test]
fn test_extract_pst_messages_invalid_data() {
    let result = extract_pst_messages(b"garbage data");
    assert!(result.is_err(), "Garbage input should return an error");
}

/// Test that message_count is present in additional metadata.
#[tokio::test]
async fn test_pst_message_count_in_metadata() {
    let pst_path = helpers::get_test_file_path("email/empty.pst");
    assert!(pst_path.exists(), "PST test fixture not found: {:?}", pst_path);

    let pst_bytes = std::fs::read(&pst_path).expect("Should read empty.pst");
    let config = ExtractionConfig::default();

    let result = extract_bytes(&pst_bytes, "application/vnd.ms-outlook-pst", &config)
        .await
        .expect("Should extract empty PST");

    assert!(
        result.metadata.additional.contains_key("message_count"),
        "Extraction result should include message_count in additional metadata"
    );
}
