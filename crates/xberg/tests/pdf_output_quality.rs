//! PDF output quality integration tests.
//!
//! Regression tests verifying that extraction output is clean and free of
//! common noise patterns (figure-internal text, arXiv watermarks, reference
//! entries misclassified as headings, repeating conference headers).
//!
//! Benchmark documents:
//! - `docling.pdf` — academic paper with figures, tables, arXiv sidebar
//! - `multi_page.pdf` — clean multi-page document (no noise expected)

#![cfg(feature = "pdf")]

mod helpers;
use helpers::extract_uri_document_blocking;

use helpers::*;
use xberg::core::config::{ExtractionConfig, OutputFormat};

fn extract_markdown(relative_path: &str) -> String {
    let pdf_path = get_test_file_path(relative_path);
    if !pdf_path.exists() {
        panic!("Test document not found: {}", relative_path);
    }
    let config = ExtractionConfig {
        output_format: OutputFormat::Markdown,
        ..Default::default()
    };
    extract_uri_document_blocking(&pdf_path, None, &config)
        .expect("extraction should succeed")
        .content
}

#[cfg(feature = "layout-detection")]
fn extract_markdown_with_layout(relative_path: &str) -> String {
    use xberg::core::config::layout::LayoutDetectionConfig;

    let pdf_path = get_test_file_path(relative_path);
    if !pdf_path.exists() {
        panic!("Test document not found: {}", relative_path);
    }
    let config = ExtractionConfig {
        output_format: OutputFormat::Markdown,
        layout: Some(LayoutDetectionConfig::default()),
        ..Default::default()
    };
    extract_uri_document_blocking(&pdf_path, None, &config)
        .expect("layout extraction should succeed")
        .content
}

// ── Noise filtering: figure-internal text ────────────────────────────

#[cfg(feature = "layout-detection")]
#[ignore = "TODO: pdf_oxide upstream — https://github.com/yfedoseev/pdf_oxide/issues/484"]
#[test]
fn test_docling_no_figure_internal_text() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown_with_layout("pdf/docling.pdf");

    // "Circling Minimums" is a heading from inside an appendix figure — should be suppressed
    assert!(
        !content.contains("Circling Minimums"),
        "Figure-internal heading 'Circling Minimums' leaked into output"
    );

    // Figure diagram labels from Figure 1 should not appear as body text
    assert!(
        !content.contains("{;} Parse PDF pages"),
        "Figure 1 diagram text leaked into output"
    );
}

#[cfg(feature = "layout-detection")]
#[test]
fn test_docling_no_figure_text_as_headings() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown_with_layout("pdf/docling.pdf");

    // "{;} Parse PDF pages" is from the pipeline diagram (Figure 1)
    for line in content.lines() {
        if line.starts_with('#') {
            assert!(
                !line.contains("{;}"),
                "Figure diagram text promoted to heading: {}",
                line
            );
            assert!(
                !line.contains("Parse PDF pages Table Structure OCR"),
                "Figure diagram text promoted to heading: {}",
                line
            );
        }
    }
}

// ── Noise filtering: arXiv watermark ─────────────────────────────────

#[cfg(feature = "layout-detection")]
#[test]
fn test_docling_no_arxiv_watermark() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown_with_layout("pdf/docling.pdf");

    // The arXiv sidebar watermark "arXiv:2408.09869v5" should be stripped.
    // Legitimate references to arXiv in body text are fine (they don't include the ID).
    assert!(
        !content.contains("arXiv:2408.09869"),
        "arXiv watermark identifier not stripped from output"
    );
}

// ── Noise filtering: references as headings ──────────────────────────

#[cfg(feature = "layout-detection")]
#[test]
fn test_docling_references_not_headings() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown_with_layout("pdf/docling.pdf");

    // Individual reference entries should not be promoted to ## headings
    let heading_lines: Vec<&str> = content.lines().filter(|l| l.starts_with("## ")).collect();
    for h in &heading_lines {
        assert!(
            !h.contains("PyPDFium2"),
            "Reference entry misclassified as heading: {}",
            h
        );
        assert!(
            !h.contains("LlamaIndex"),
            "Reference entry misclassified as heading: {}",
            h
        );
        assert!(
            !h.contains("PyttiuPDF"),
            "Reference entry misclassified as heading: {}",
            h
        );
    }
}

// ── Content preservation ─────────────────────────────────────────────

#[cfg(feature = "layout-detection")]
#[test]
fn test_docling_key_content_preserved() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown_with_layout("pdf/docling.pdf");

    assert!(
        content.contains("Docling Technical Report"),
        "Title not found in output"
    );
    assert!(
        content.contains("Processing pipeline") || content.contains("processing pipeline"),
        "Section 'Processing pipeline' not found"
    );
    assert!(content.contains("TableFormer"), "'TableFormer' not found");
    assert!(
        content.contains("PDF backend") || content.contains("PDF backends"),
        "'PDF backends' section not found"
    );
}

#[test]
fn test_multipage_clean_output() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown("pdf/multi_page.pdf");

    assert!(content.contains("Evolution of the Word Processor"), "Title not found");
    assert!(
        content.contains("Pre-Digital Era"),
        "Section 'Pre-Digital Era' not found"
    );
    assert!(content.contains("IBM MT/ST"), "'IBM MT/ST' not found");
}

#[test]
fn test_multipage_no_noise() {
    if !test_documents_available() {
        return;
    }
    let content = extract_markdown("pdf/multi_page.pdf");

    // multipage.pdf is a clean document — should have no arXiv noise
    assert!(
        !content.contains("arXiv:"),
        "multipage.pdf should have no arXiv identifiers"
    );
}
