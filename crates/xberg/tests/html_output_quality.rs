//! HTML output formatting quality tests.
//!
//! These tests extract representative documents to HTML and validate the
//! output with local structural checks.
//!
//! Usage:
//!   cargo test -p xberg --test html_output_quality -- --nocapture

mod helpers;
use helpers::extract_uri_document_blocking;

use xberg::core::config::OutputFormat;
use xberg::extraction::derive::derive_extraction_result;
use xberg::types::internal_builder::InternalDocumentBuilder;

/// Check basic HTML output quality without depending on an external formatter.
fn assert_html_quality(html_content: &str) -> Result<(), String> {
    if html_content.trim().is_empty() {
        return Err("HTML output is empty".to_string());
    }
    if html_content.contains("\r\n") {
        return Err("HTML output contains CRLF line endings".to_string());
    }
    if html_content
        .lines()
        .any(|line| line.ends_with(' ') || line.ends_with('\t'))
    {
        return Err("HTML output contains trailing whitespace".to_string());
    }
    if html_content.contains("\n\n\n") {
        return Err("HTML output contains more than two consecutive blank lines".to_string());
    }
    if html_content.contains("<body") && !html_content.contains("</body>") {
        return Err("HTML output opens <body> without closing it".to_string());
    }
    Ok(())
}

/// Render an `InternalDocument` to HTML via the derive pipeline.
fn render_html(doc: xberg::types::internal::InternalDocument) -> String {
    let result = derive_extraction_result(doc, false, OutputFormat::Html);
    result.formatted_content.unwrap_or(result.content)
}

// ---------------------------------------------------------------------------
// Document builders
// ---------------------------------------------------------------------------

/// A rich document with headings, paragraph, list, code block, and table.
fn build_rich_document() -> xberg::types::internal::InternalDocument {
    let mut b = InternalDocumentBuilder::new("test-rich");

    b.push_heading(1, "Main Heading", None, None);
    b.push_paragraph("This is a paragraph with some descriptive text.", vec![], None, None);

    b.push_heading(2, "Details", None, None);
    b.push_list(false);
    b.push_list_item("First item", false, vec![], None, None);
    b.push_list_item("Second item", false, vec![], None, None);
    b.push_list_item("Third item", false, vec![], None, None);
    b.end_list();

    b.push_code("fn main() {\n    println!(\"hello\");\n}", Some("rust"), None, None);

    b.push_table_from_cells(
        &[
            vec!["Name".to_string(), "Value".to_string()],
            vec!["alpha".to_string(), "1".to_string()],
            vec!["beta".to_string(), "2".to_string()],
        ],
        None,
        None,
    );

    b.build()
}

/// A document with multiple heading levels.
fn build_heading_hierarchy() -> xberg::types::internal::InternalDocument {
    let mut b = InternalDocumentBuilder::new("test-headings");

    b.push_heading(1, "Title", None, None);
    b.push_paragraph("Introduction paragraph.", vec![], None, None);

    b.push_heading(2, "Section One", None, None);
    b.push_paragraph("Content of section one.", vec![], None, None);

    b.push_heading(3, "Subsection", None, None);
    b.push_paragraph("Subsection content.", vec![], None, None);

    b.push_heading(2, "Section Two", None, None);
    b.push_paragraph("Content of section two.", vec![], None, None);

    b.build()
}

/// A document with nested lists.
fn build_list_document() -> xberg::types::internal::InternalDocument {
    let mut b = InternalDocumentBuilder::new("test-lists");

    b.push_heading(1, "Lists", None, None);

    b.push_list(false);
    b.push_list_item("Unordered item one", false, vec![], None, None);
    b.push_list_item("Unordered item two", false, vec![], None, None);
    b.end_list();

    b.push_list(true);
    b.push_list_item("Ordered item one", false, vec![], None, None);
    b.push_list_item("Ordered item two", false, vec![], None, None);
    b.end_list();

    b.build()
}

/// A minimal document with a single paragraph.
fn build_minimal_document() -> xberg::types::internal::InternalDocument {
    let mut b = InternalDocumentBuilder::new("test-minimal");
    b.push_paragraph("A single paragraph of text.", vec![], None, None);
    b.build()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_rich_document_html_quality() {
    let html = render_html(build_rich_document());
    if let Err(msg) = assert_html_quality(&html) {
        panic!("Rich document HTML failed quality check:\n{msg}\n\nGenerated HTML:\n{html}");
    }
}

#[test]
fn test_heading_hierarchy_html_quality() {
    let html = render_html(build_heading_hierarchy());
    if let Err(msg) = assert_html_quality(&html) {
        panic!("Heading hierarchy HTML failed quality check:\n{msg}\n\nGenerated HTML:\n{html}");
    }
}

#[test]
fn test_list_document_html_quality() {
    let html = render_html(build_list_document());
    if let Err(msg) = assert_html_quality(&html) {
        panic!("List document HTML failed quality check:\n{msg}\n\nGenerated HTML:\n{html}");
    }
}

#[test]
fn test_minimal_document_html_quality() {
    let html = render_html(build_minimal_document());
    if let Err(msg) = assert_html_quality(&html) {
        panic!("Minimal document HTML failed quality check:\n{msg}\n\nGenerated HTML:\n{html}");
    }
}

/// Test HTML output from actual file extraction when test documents and
/// the `office` feature are available.
#[cfg(feature = "office")]
#[test]
fn test_file_extraction_html_quality() {
    use helpers::{get_test_file_path, test_documents_available};
    use xberg::core::config::ExtractionConfig;

    if !test_documents_available() {
        eprintln!("test_documents not available, skipping file extraction HTML test");
        return;
    }

    let test_files: &[&str] = &["latex/basic_sections.tex", "typst/simple.typ"];

    let config = ExtractionConfig {
        output_format: OutputFormat::Html,
        ..Default::default()
    };

    for &rel_path in test_files {
        let path = get_test_file_path(rel_path);
        if !path.exists() {
            eprintln!("Skipping {rel_path}: file not found");
            continue;
        }

        let result = extract_uri_document_blocking(&path, None, &config).expect("extraction should succeed");
        let html = result.formatted_content.as_deref().unwrap_or(&result.content);

        if let Err(msg) = assert_html_quality(html) {
            panic!("File {rel_path} HTML output failed quality check:\n{msg}\n\nGenerated HTML:\n{html}");
        }
    }
}
