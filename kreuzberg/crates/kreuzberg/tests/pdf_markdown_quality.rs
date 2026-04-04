//! PDF markdown quality smoke tests: verify extraction produces structural elements.
//!
//! These are lightweight assertions — detailed quality scoring and A/B comparisons
//! live in `tools/benchmark-harness` (subcommands: `compare`, `pipeline-benchmark`).
//!
//! Usage:
//!   cargo test -p kreuzberg --features "pdf,bundled-pdfium" \
//!     --test pdf_markdown_quality -- --nocapture

#![cfg(feature = "pdf")]

mod helpers;

use helpers::*;
use kreuzberg::core::config::{ExtractionConfig, OutputFormat};
use kreuzberg::extract_file_sync;

/// Documents with markdown ground truth.
const MARKDOWN_GT_DOCS: &[(&str, &str)] = &[("docling", "pdf/docling.pdf")];

fn extract_markdown(pdf_path: &std::path::Path) -> String {
    let config = ExtractionConfig {
        output_format: OutputFormat::Markdown,
        ..Default::default()
    };
    extract_file_sync(pdf_path, None, &config)
        .expect("extraction should succeed")
        .content
}

#[cfg(feature = "layout-detection")]
fn extract_markdown_with_layout(pdf_path: &std::path::Path) -> String {
    use kreuzberg::core::config::layout::LayoutDetectionConfig;

    let config = ExtractionConfig {
        output_format: OutputFormat::Markdown,
        layout: Some(LayoutDetectionConfig::default()),
        ..Default::default()
    };
    extract_file_sync(pdf_path, None, &config)
        .expect("layout extraction should succeed")
        .content
}

/// Count structural elements in markdown content.
fn count_headings(md: &str) -> usize {
    md.lines().filter(|l| l.starts_with('#')).count()
}

fn count_table_rows(md: &str) -> usize {
    md.lines()
        .filter(|l| l.starts_with('|') && l.ends_with('|') && !l.contains("---"))
        .count()
}

fn count_list_items(md: &str) -> usize {
    md.lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with("- ") || t.starts_with("* ") || t.starts_with("+ ")
        })
        .count()
}

fn has_code_blocks(md: &str) -> bool {
    md.contains("```")
}

#[test]
fn test_baseline_produces_structural_markdown() {
    if !test_documents_available() {
        println!("Skipping: test_documents not available");
        return;
    }

    for &(name, pdf_rel) in MARKDOWN_GT_DOCS {
        let pdf_path = get_test_file_path(pdf_rel);
        if !pdf_path.exists() {
            println!("Skipping {}: file not found", name);
            continue;
        }

        let content = extract_markdown(&pdf_path);

        // Basic structural assertions
        assert!(
            !content.trim().is_empty(),
            "{}: extraction produced empty content",
            name
        );
        assert!(
            content.len() > 500,
            "{}: content too short ({} chars)",
            name,
            content.len()
        );
        assert!(count_headings(&content) > 0, "{}: expected at least one heading", name);

        println!(
            "{}: {} chars, {} headings, {} table rows, {} list items, code={}",
            name,
            content.len(),
            count_headings(&content),
            count_table_rows(&content),
            count_list_items(&content),
            has_code_blocks(&content),
        );
    }
}

#[cfg(feature = "layout-detection")]
#[test]
fn test_layout_does_not_regress_text_content() {
    if !test_documents_available() {
        println!("Skipping: test_documents not available");
        return;
    }

    for &(name, pdf_rel) in MARKDOWN_GT_DOCS {
        let pdf_path = get_test_file_path(pdf_rel);
        if !pdf_path.exists() {
            println!("Skipping {}: file not found", name);
            continue;
        }

        let baseline = extract_markdown(&pdf_path);
        let layout = extract_markdown_with_layout(&pdf_path);

        // Layout extraction should not lose significant content
        let baseline_len = baseline.len();
        let layout_len = layout.len();

        // Allow up to 20% content loss (layout may restructure)
        assert!(
            layout_len as f64 >= baseline_len as f64 * 0.8,
            "{}: layout content ({} chars) is significantly shorter than baseline ({} chars)",
            name,
            layout_len,
            baseline_len,
        );

        // Layout should still have headings
        assert!(
            count_headings(&layout) > 0,
            "{}: layout extraction lost all headings",
            name
        );

        println!(
            "{}: baseline={} chars, layout={} chars, layout headings={}",
            name,
            baseline_len,
            layout_len,
            count_headings(&layout),
        );
    }
}
