//! Integration tests for HEIF / HEIC / AVIF support via the public API.
//!
//! Exercises:
//! - `extract_bytes_document` end-to-end for HEIC, HEIF, AVIF inputs (decoded to PNG
//!   internally, then routed through the image extractor; OCR is disabled so
//!   the test is independent of any Tesseract binary).
//! - `list_supported_formats()` reports HEIC / HEIF / AVIF / HEICS / AVCS, so
//!   every language binding picks them up via the single Rust registry.
//!
//! Fixtures come from the repo-root `test_documents/` submodule. No HEIF
//! binaries live in the xberg codebase itself.

#![cfg(feature = "heic")]

mod helpers;
use helpers::extract_bytes_document;

use xberg::FormatMetadata;
use xberg::core::config::ExtractionConfig;
use xberg::core::mime::list_supported_formats;

const TEST_HEIC: &[u8] = include_bytes!("../../../test_documents/images/test.heic");
const TEST_HEIF: &[u8] = include_bytes!("../../../test_documents/images/test.heif");
const TEST_AVIF: &[u8] = include_bytes!("../../../test_documents/images/test.avif");

fn ocr_disabled_config() -> ExtractionConfig {
    ExtractionConfig {
        disable_ocr: true,
        ..ExtractionConfig::default()
    }
}

async fn extract_image_metadata(bytes: &'static [u8], mime: &str) -> xberg::ImageMetadata {
    let config = ocr_disabled_config();
    let result = extract_bytes_document(bytes, mime, &config)
        .await
        .unwrap_or_else(|e| panic!("extract_bytes_document failed for {mime}: {e}"));
    match result.metadata.format {
        Some(FormatMetadata::Image(img)) => img,
        other => panic!("expected ImageMetadata for {mime}, got {other:?}"),
    }
}

#[tokio::test]
async fn extract_bytes_handles_heic() {
    let img = extract_image_metadata(TEST_HEIC, "image/heic").await;
    assert!(img.width > 0, "HEIC width should be > 0");
    assert!(img.height > 0, "HEIC height should be > 0");
    assert_eq!(img.format, "HEIF");
}

#[tokio::test]
async fn extract_bytes_handles_heif() {
    let img = extract_image_metadata(TEST_HEIF, "image/heif").await;
    assert!(img.width > 0);
    assert!(img.height > 0);
    assert_eq!(img.format, "HEIF");
}

#[tokio::test]
async fn extract_bytes_handles_avif() {
    let img = extract_image_metadata(TEST_AVIF, "image/avif").await;
    assert!(img.width > 0);
    assert!(img.height > 0);
    assert_eq!(img.format, "HEIF");
}

#[test]
fn heif_mime_types_and_extensions_are_registered() {
    let formats = list_supported_formats();
    let mimes: std::collections::HashSet<_> = formats.iter().map(|f| f.mime_type.as_str()).collect();
    for mime in ["image/heic", "image/heif", "image/avif", "image/avcs"] {
        assert!(mimes.contains(mime), "{mime} missing from list_supported_formats()");
    }

    let exts: std::collections::HashSet<_> = formats.iter().map(|f| f.extension.as_str()).collect();
    for ext in ["heic", "heics", "heif", "avif", "avcs"] {
        assert!(exts.contains(ext), "extension `{ext}` missing");
    }
}
