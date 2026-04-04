//! Integration tests for OCR table inlining into markdown content (issue #421).
//!
//! Verifies that when `output_format = Markdown` and OCR detects tables,
//! the tables are inlined into `result.content` at their correct positions
//! rather than only appearing in `result.tables`.

#![cfg(feature = "ocr")]

mod helpers;

use helpers::*;
use kreuzberg::core::config::{ExtractionConfig, OcrConfig, OutputFormat};
use kreuzberg::extract_file_sync;

/// Helper: create an ExtractionConfig with OCR + Markdown output.
fn ocr_markdown_config() -> ExtractionConfig {
    ExtractionConfig {
        output_format: OutputFormat::Markdown,
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng".to_string(),
            ..Default::default()
        }),
        force_ocr: false,
        ..Default::default()
    }
}

/// Helper: create an ExtractionConfig with OCR + Plain output.
fn ocr_plain_config() -> ExtractionConfig {
    ExtractionConfig {
        output_format: OutputFormat::Plain,
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng".to_string(),
            ..Default::default()
        }),
        force_ocr: false,
        ..Default::default()
    }
}

/// When tables are detected and output_format=Markdown, the content should
/// contain the markdown pipe table syntax (not just raw OCR text).
#[test]
fn test_ocr_markdown_inlines_table_into_content() {
    if skip_if_missing("images/simple_table.png") {
        return;
    }

    let file_path = get_test_file_path("images/simple_table.png");
    let result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract table image with OCR");

    assert_non_empty_content(&result);

    // If tables were detected, the content must include pipe table syntax
    if !result.tables.is_empty() {
        assert!(
            result.content.contains('|'),
            "Markdown content should contain pipe table syntax when tables are detected.\n\
             Tables found: {}\nContent preview: {}",
            result.tables.len(),
            &result.content[..result.content.len().min(500)]
        );
    }
}

/// Markdown output should differ from plain output when tables are detected.
#[test]
fn test_ocr_markdown_differs_from_plain_when_tables_found() {
    if skip_if_missing("images/simple_table.png") {
        return;
    }

    let file_path = get_test_file_path("images/simple_table.png");

    let plain_result =
        extract_file_sync(&file_path, None, &ocr_plain_config()).expect("Should extract with plain output");

    let md_result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract with markdown output");

    // Both should have content
    assert_non_empty_content(&plain_result);
    assert_non_empty_content(&md_result);

    // If tables were detected in the markdown result, content should differ from plain
    if !md_result.tables.is_empty() {
        assert_ne!(
            plain_result.content,
            md_result.content,
            "Markdown content should differ from plain when tables are detected.\n\
             Tables: {}\nPlain len: {}\nMarkdown len: {}",
            md_result.tables.len(),
            plain_result.content.len(),
            md_result.content.len()
        );
    }
}

/// Tables should have bounding boxes populated when detected via OCR.
#[test]
fn test_ocr_table_has_bounding_box() {
    if skip_if_missing("images/simple_table.png") {
        return;
    }

    let file_path = get_test_file_path("images/simple_table.png");
    let result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract table image with OCR");

    for (idx, table) in result.tables.iter().enumerate() {
        assert!(
            table.bounding_box.is_some(),
            "Table {} should have a bounding_box populated from OCR word positions",
            idx
        );
        let bbox = table.bounding_box.as_ref().unwrap();
        assert!(
            bbox.x1 > bbox.x0 && bbox.y1 > bbox.y0,
            "Bounding box should have positive area: x0={}, y0={}, x1={}, y1={}",
            bbox.x0,
            bbox.y0,
            bbox.x1,
            bbox.y1
        );
    }
}

/// Test with a financial balance sheet image from issue #421.
#[test]
fn test_issue_421_balance_sheet_markdown() {
    if skip_if_missing("images/balance_sheet_1.png") {
        return;
    }

    let file_path = get_test_file_path("images/balance_sheet_1.png");
    let result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract balance sheet image");

    assert_non_empty_content(&result);

    // If tables are detected, markdown content should include them
    if !result.tables.is_empty() {
        assert!(
            result.content.contains('|'),
            "Balance sheet markdown should contain pipe table syntax.\n\
             Tables found: {}\nFirst table rows: {}\nContent preview: {}",
            result.tables.len(),
            result.tables[0].cells.len(),
            &result.content[..result.content.len().min(500)]
        );

        // Bounding box should be populated
        for table in &result.tables {
            assert!(table.bounding_box.is_some(), "OCR table should have bounding_box");
        }
    }
}

/// Test with a financial table image from issue #421.
#[test]
fn test_issue_421_financial_table_markdown() {
    if skip_if_missing("images/financial_table_1.png") {
        return;
    }

    let file_path = get_test_file_path("images/financial_table_1.png");
    let result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract financial table image");

    assert_non_empty_content(&result);

    if !result.tables.is_empty() {
        assert!(
            result.content.contains('|'),
            "Financial table markdown should contain pipe table syntax.\n\
             Tables found: {}\nContent preview: {}",
            result.tables.len(),
            &result.content[..result.content.len().min(500)]
        );
    }
}

/// Test the metadata.output_format signal for pre-formatted content.
/// When OCR inlines tables, the output_format metadata should be set to "markdown"
/// so the pipeline doesn't re-process it.
#[test]
fn test_ocr_markdown_sets_output_format_metadata() {
    if skip_if_missing("images/simple_table.png") {
        return;
    }

    let file_path = get_test_file_path("images/simple_table.png");
    let result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract table image with OCR");

    // output_format should be set to "markdown" by the pipeline
    assert_eq!(
        result.metadata.output_format,
        Some("markdown".to_string()),
        "output_format metadata should be 'markdown'"
    );
}

/// Diagnostic test (ignored by default) to visually inspect OCR table inlining.
/// Run with: cargo test --features ocr --test ocr_table_inline diagnostic -- --ignored --nocapture
#[test]
#[ignore]
fn diagnostic_print_ocr_table_content() {
    let files = [
        "images/simple_table.png",
        "images/balance_sheet_1.png",
        "images/financial_table_1.png",
    ];

    for file in &files {
        if skip_if_missing(file) {
            continue;
        }

        let path = get_test_file_path(file);

        let plain = extract_file_sync(&path, None, &ocr_plain_config()).unwrap();
        let md = extract_file_sync(&path, None, &ocr_markdown_config()).unwrap();

        eprintln!("\n============================================================");
        eprintln!("FILE: {file}");
        eprintln!("Tables: plain={} md={}", plain.tables.len(), md.tables.len());
        eprintln!("Content identical: {}", plain.content == md.content);
        eprintln!(
            "Content len: {} (plain) / {} (md)",
            plain.content.len(),
            md.content.len()
        );

        for (i, t) in md.tables.iter().enumerate() {
            eprintln!(
                "  Table {i}: {}r x {}c, bbox={:?}",
                t.cells.len(),
                t.cells.first().map_or(0, |r| r.len()),
                t.bounding_box
            );
        }

        eprintln!("\n--- MARKDOWN CONTENT ---");
        eprintln!("{}", &md.content[..md.content.len().min(2000)]);
        eprintln!("--- END ---\n");
    }
}

/// Verify that markdown table content is the same as result.tables[].markdown.
/// The inlined table in content should match the structured table markdown.
#[test]
fn test_inlined_table_matches_structured_table() {
    if skip_if_missing("images/simple_table.png") {
        return;
    }

    let file_path = get_test_file_path("images/simple_table.png");
    let result =
        extract_file_sync(&file_path, None, &ocr_markdown_config()).expect("Should extract table image with OCR");

    for table in &result.tables {
        let table_md = table.markdown.trim();
        if !table_md.is_empty() {
            assert!(
                result.content.contains(table_md),
                "Content should contain the structured table markdown.\n\
                 Table markdown:\n{}\n\nContent:\n{}",
                table_md,
                &result.content[..result.content.len().min(2000)]
            );
        }
    }
}
