//! End-to-end integration test for XLSX metadata extraction
#![cfg(feature = "excel")]

use kreuzberg::extraction::excel::{excel_to_markdown, excel_to_text, read_excel_file};

#[test]
fn test_xlsx_full_metadata_extraction() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed");
    let test_file = workspace_root.join("test_documents/xlsx/excel_tiny_excel.xlsx");

    if !test_file.exists() {
        println!("Skipping test: Test file not found at {:?}", test_file);
        return;
    }

    let file_path = test_file.to_str().expect("File path should be valid UTF-8");
    let result = read_excel_file(file_path).expect("Should extract XLSX successfully");

    assert!(!result.sheets.is_empty(), "Should have at least one sheet");

    assert!(result.metadata.contains_key("sheet_count"), "Should have sheet count");

    println!("✅ XLSX metadata extraction test passed!");
    println!("   Found {} metadata fields", result.metadata.len());
    for (key, value) in &result.metadata {
        println!("   {}: {}", key, value);
    }
}

#[test]
fn test_xlsx_multi_sheet_metadata() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed");
    let test_file = workspace_root.join("test_documents/xlsx/excel_multi_sheet.xlsx");

    if !test_file.exists() {
        println!("Skipping test: Test file not found at {:?}", test_file);
        return;
    }

    let file_path = test_file.to_str().expect("File path should be valid UTF-8");
    let result = read_excel_file(file_path).expect("Should extract multi-sheet XLSX successfully");

    assert!(
        result.sheets.len() > 1,
        "Should have multiple sheets, got {}",
        result.sheets.len()
    );

    assert!(
        result.metadata.contains_key("sheet_names"),
        "Should have sheet_names metadata"
    );

    println!("✅ XLSX multi-sheet metadata extraction test passed!");
    println!("   Found {} sheets", result.sheets.len());
}

#[test]
fn test_xlsx_minimal_metadata_extraction() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed");
    let test_file = workspace_root.join("test_documents/xlsx/tables_small_test01.xlsx");

    if !test_file.exists() {
        println!("Skipping test: Test file not found at {:?}", test_file);
        return;
    }

    let file_path = test_file.to_str().expect("File path should be valid UTF-8");
    let result = read_excel_file(file_path).expect("Should extract XLSX successfully");

    assert!(!result.sheets.is_empty(), "Content should not be empty");
    assert!(
        result.metadata.contains_key("sheet_count"),
        "Should have basic sheet metadata"
    );

    println!("✅ XLSX minimal metadata extraction test passed!");
}

/// Test for issue #331: OOM with XLSX files containing Excel Solver add-in data
///
/// This test reproduces the issue where Excel Solver stores configuration data
/// in cells at extreme positions (XFD1048550-1048575 = column 16384, rows near 1M).
/// The sheet dimension is set to "A1:XFD1048575", which could cause Kreuzberg
/// to attempt allocating memory for ~17 trillion cells (16384 × 1048575).
///
/// Expected behavior: Should handle extreme dimensions gracefully without OOM.
/// The file is only 6.8KB and contains minimal actual data.
#[test]
fn test_xlsx_excel_solver_extreme_dimensions_no_oom() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed");
    let test_file = workspace_root.join("test_documents/test_fixtures/xlsx-oom-repro/kreuzberg-oom-repro.xlsx");

    if !test_file.exists() {
        println!("Skipping test: Test file not found at {:?}", test_file);
        println!("Run: node test_documents/test_fixtures/xlsx-oom-repro/generate-oom-xlsx.mjs");
        return;
    }

    let file_path = test_file.to_str().expect("File path should be valid UTF-8");

    // This should NOT cause OOM even though dimension claims A1:XFD1048575
    // The actual data is minimal (only ~26 cells with Solver metadata)
    let result = read_excel_file(file_path).expect("Should extract XLSX with extreme dimensions without OOM");

    // Verify we got the actual data, not a massive allocation
    assert!(!result.sheets.is_empty(), "Should have at least one sheet");

    // The file has normal cells A1, B1 plus Solver cells at extreme positions
    // Verify we extracted something reasonable, not 17 trillion cells
    let sheet = &result.sheets[0];
    assert!(
        sheet.markdown.len() < 10000,
        "Sheet markdown content should be small (< 10000 chars), not massive. Got {} chars",
        sheet.markdown.len()
    );

    // Verify metadata was extracted
    assert!(
        result.metadata.contains_key("sheet_count"),
        "Should have sheet_count metadata"
    );

    println!("✅ XLSX Excel Solver extreme dimensions test passed!");
    println!(
        "   Sheet markdown length: {} chars (reasonable size)",
        sheet.markdown.len()
    );
    println!("   Successfully handled dimension A1:XFD1048575 without OOM");
}

/// Regression test for #405: XLSX extraction with output_format=Markdown
/// should produce markdown tables with pipe delimiters and separator rows,
/// not plain space-separated text.
#[test]
fn test_xlsx_markdown_vs_plain_output() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed")
        .parent()
        .expect("Operation failed");
    let test_file = workspace_root.join("test_documents/xlsx/excel_multi_sheet.xlsx");

    if !test_file.exists() {
        println!("Skipping test: Test file not found at {:?}", test_file);
        return;
    }

    let file_path = test_file.to_str().expect("File path should be valid UTF-8");
    let workbook = read_excel_file(file_path).expect("Should extract XLSX successfully");

    // excel_to_markdown should produce tables with | delimiters
    let md_content = excel_to_markdown(&workbook);
    assert!(
        md_content.contains("| "),
        "Markdown output should contain table pipe delimiters"
    );
    assert!(
        md_content.contains("---"),
        "Markdown output should contain separator rows"
    );

    // excel_to_text should produce space-separated text (no pipes)
    let text_content = excel_to_text(&workbook);
    assert!(
        !text_content.contains("| "),
        "Plain text output should not contain table pipe delimiters"
    );
}
