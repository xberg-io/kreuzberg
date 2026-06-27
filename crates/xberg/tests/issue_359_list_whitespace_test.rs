//! Regression test for https://github.com/xberg-io/xberg/issues/359
//!
//! DOCX list items with multiple text runs should preserve whitespace between runs.
//! e.g. "Sermocination ypsiliform" must not become "Sermocinationypsiliform".

#![cfg(feature = "office")]

mod helpers;
use helpers::extract_uri_document;

use xberg::ExtractionConfig;

#[tokio::test]
async fn test_issue_359_docx_list_run_whitespace() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("parent")
        .parent()
        .expect("workspace root");
    let test_file = workspace_root.join("test_documents/docx/issue_359_list_whitespace.docx");

    if !test_file.exists() {
        println!("Skipping test: {:?} not found", test_file);
        return;
    }

    let result = extract_uri_document(&test_file, None, &ExtractionConfig::default())
        .await
        .expect("Should extract DOCX successfully");

    assert!(
        result.content.contains("Sermocination ypsiliform"),
        "Expected 'Sermocination ypsiliform' with space between runs, got: {:?}",
        result.content
    );
}
