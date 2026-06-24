//! Test that DOCX files are correctly detected as DOCX, not ZIP.
//!
//! This tests the fix for https://github.com/xberg-io/kreuzberg/issues/350

use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_docx_detected_from_bytes_not_zip() {
    // Create a minimal DOCX-like ZIP file with the word/document.xml marker
    // This simulates the structure of a DOCX file
    let mut file = NamedTempFile::new().unwrap();

    // Write a minimal ZIP with word/document.xml entry
    let docx_content: &[u8] = &[
        // ZIP local file header
        0x50, 0x4b, 0x03, 0x04, // signature
        0x14, 0x00, // version needed
        0x00, 0x00, // flags
        0x00, 0x00, // compression method
        0x00, 0x00, // mod time
        0x00, 0x00, // mod date
        0x00, 0x00, 0x00, 0x00, // crc
        0x00, 0x00, 0x00, 0x00, // compressed size
        0x00, 0x00, 0x00, 0x00, // uncompressed size
        0x11, 0x00, // file name length (17)
        0x00, 0x00, // extra field length
        // "word/document.xml"
        b'w', b'o', b'r', b'd', b'/', b'd', b'o', b'c', b'u', b'm', b'e', b'n', b't', b'.', b'x', b'm', b'l',
    ];

    file.write_all(docx_content).unwrap();
    file.flush().unwrap();

    let content = std::fs::read(file.path()).unwrap();
    let mime = kreuzberg::core::mime::detect_mime_type_from_bytes(&content).unwrap();

    assert_eq!(
        mime, "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "DOCX file should be detected as DOCX MIME type, not ZIP"
    );
}

#[test]
fn test_xlsx_detected_from_bytes_not_zip() {
    // Create a minimal XLSX-like ZIP file with the xl/workbook.xml marker
    let xlsx_content: &[u8] = &[
        // ZIP local file header
        0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, // file name length (15)
        0x00, 0x00, // "xl/workbook.xml"
        b'x', b'l', b'/', b'w', b'o', b'r', b'k', b'b', b'o', b'o', b'k', b'.', b'x', b'm', b'l',
    ];

    let mime = kreuzberg::core::mime::detect_mime_type_from_bytes(xlsx_content).unwrap();

    assert_eq!(
        mime, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "XLSX file should be detected as XLSX MIME type, not ZIP"
    );
}

#[test]
fn test_pptx_detected_from_bytes_not_zip() {
    // Create a minimal PPTX-like ZIP file with the ppt/presentation.xml marker
    let pptx_content: &[u8] = &[
        // ZIP local file header
        0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x00, // file name length (20)
        0x00, 0x00, // "ppt/presentation.xml"
        b'p', b'p', b't', b'/', b'p', b'r', b'e', b's', b'e', b'n', b't', b'a', b't', b'i', b'o', b'n', b'.', b'x',
        b'm', b'l',
    ];

    let mime = kreuzberg::core::mime::detect_mime_type_from_bytes(pptx_content).unwrap();

    assert_eq!(
        mime, "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "PPTX file should be detected as PPTX MIME type, not ZIP"
    );
}

#[test]
fn test_plain_zip_still_detected_as_zip() {
    // Plain ZIP without Office markers should remain as ZIP
    let plain_zip_content: &[u8] = &[
        // ZIP local file header
        0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, // file name length (8)
        0x00, 0x00, // "test.txt"
        b't', b'e', b's', b't', b'.', b't', b'x', b't',
    ];

    let mime = kreuzberg::core::mime::detect_mime_type_from_bytes(plain_zip_content).unwrap();

    assert_eq!(mime, "application/zip", "Plain ZIP should remain as application/zip");
}

#[test]
fn test_legacy_docx_mime_validates_ok() {
    // kreuzberg-cloud receives real traffic with this non-standard value; it must not be rejected.
    let result = kreuzberg::core::mime::validate_mime_type("application/docx");
    assert!(
        result.is_ok(),
        "validate_mime_type should accept legacy application/docx alias, got: {:?}",
        result
    );
}

#[test]
fn test_ext_to_mime_docx_still_canonical() {
    // Extension dispatch must continue to produce the RFC MIME — the alias is for validation only.
    let path = std::path::Path::new("document.docx");
    let mime = kreuzberg::core::mime::detect_mime_type(path, false).unwrap();
    assert_eq!(
        mime, "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ".docx extension must resolve to the RFC canonical MIME, not the legacy alias"
    );
}

#[cfg(feature = "office")]
#[test]
fn test_both_docx_mimes_resolve_to_same_extractor() {
    // Both the RFC MIME and the legacy alias must be declared by DocxExtractor,
    // ensuring the registry maps both to the same plugin.
    use kreuzberg::extractors::DocxExtractor;
    use kreuzberg::plugins::DocumentExtractor;

    let extractor = DocxExtractor;
    let supported = extractor.supported_mime_types();

    assert!(
        supported.contains(&"application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        "DocxExtractor must declare the RFC canonical MIME"
    );
    assert!(
        supported.contains(&"application/docx"),
        "DocxExtractor must declare the legacy alias so the registry routes both to the same plugin"
    );
}
