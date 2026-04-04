//! Regression test for https://github.com/kreuzberg-dev/kreuzberg/issues/350
//!
//! Tests with real DOCX files that were reported to be detected as ZIP.
//! If the test files don't exist, the test is skipped.

use std::path::Path;

#[test]
fn test_issue_350_libreoffice_docx() {
    let path = Path::new("/tmp/docx-test/minimal_libreoffice.docx");
    if !path.exists() {
        eprintln!("Skipping test: {} not found", path.display());
        return;
    }

    let content = std::fs::read(path).unwrap();
    let mime = kreuzberg::core::mime::detect_mime_type_from_bytes(&content).unwrap();

    assert_eq!(
        mime, "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "LibreOffice DOCX should be detected as DOCX, not ZIP (issue #350)"
    );
    println!("✅ minimal_libreoffice.docx: {}", mime);
}

#[test]
fn test_issue_350_chatgpt_docx() {
    let path = Path::new("/tmp/docx-test/Acme_Inc_Organizational_Data.docx");
    if !path.exists() {
        eprintln!("Skipping test: {} not found", path.display());
        return;
    }

    let content = std::fs::read(path).unwrap();
    let mime = kreuzberg::core::mime::detect_mime_type_from_bytes(&content).unwrap();

    assert_eq!(
        mime, "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "ChatGPT-generated DOCX should be detected as DOCX, not ZIP (issue #350)"
    );
    println!("✅ Acme_Inc_Organizational_Data.docx: {}", mime);
}
