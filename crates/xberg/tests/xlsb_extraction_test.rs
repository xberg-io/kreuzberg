//! TODO: Restored from 245539484 alef-migration cleanup. Currently exercises
//! pub(crate) APIs that the migration deliberately narrowed; gated until
//! either (a) these APIs are re-exposed publicly, or (b) the test is
//! rewritten against the public extraction surface.

#![cfg(any())]

// Original content preserved below; recompiled once gating cfg drops.
// Disabled by the file-level cfg(any()) above.

/*
//! Integration test for XLSB (Excel Binary Spreadsheet) extraction
#![cfg(feature = "excel")]

use xberg::extraction::excel::read_excel_file;

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed")
        .to_path_buf()
}

#[test]
fn test_xlsb_extraction() {
    let test_file = workspace_root().join("test_documents/xlsx/test_xlsb.xlsb");

    if !test_file.exists() {
        println!("Skipping test: Test file not found at {:?}", test_file);
        return;
    }

    let file_path = test_file.to_str().expect("File path should be valid UTF-8");
    let result = read_excel_file(file_path).expect("Should extract XLSB successfully");

    assert!(!result.sheets.is_empty(), "Should have at least one sheet");
    assert!(
        result.metadata.contains_key("sheet_count"),
        "Should have sheet count metadata"
    );

    let all_content: String = result
        .sheets
        .iter()
        .map(|s| s.markdown.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    assert!(
        !all_content.trim().is_empty(),
        "XLSB extraction should produce non-empty content"
    );
}

*/
