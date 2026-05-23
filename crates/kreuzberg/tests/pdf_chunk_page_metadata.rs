//! Regression tests for issue #1013: chunk `first_page`/`last_page` null when
//! extracting multi-page PDFs with chunking enabled.
//!
//! Root cause: `extract_text_with_tracking` stored raw `page_text` in
//! `PageContent.content` while `result.content` held the cleaned version
//! (after `fix_pdf_control_chars`).  When the two diverged, the substring
//! search in `recompute_boundaries_from_pages` failed, producing null page
//! metadata on the affected chunks.

#![cfg(all(feature = "pdf", feature = "chunking"))]

mod helpers;

use helpers::*;
use kreuzberg::core::config::{ChunkingConfig, ExtractionConfig};
use kreuzberg::extract_file_sync;

/// All chunks produced from a multi-page PDF must have non-null page metadata.
///
/// Regression guard for issue #1013: previously, chunks whose underlying page text
/// contained PDF control characters (U+0001–U+001F) would lose their
/// `first_page`/`last_page` because `recompute_boundaries_from_pages` could not
/// locate the raw page text inside the cleaned `result.content`.
#[test]
fn chunks_from_multi_page_pdf_all_have_page_metadata() {
    if skip_if_missing("pdf/multi_page.pdf") {
        eprintln!("skipping: fixture pdf/multi_page.pdf not found");
        return;
    }

    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 500,
            overlap: 50,
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file_sync(get_test_file_path("pdf/multi_page.pdf"), None, &config)
        .expect("multi_page.pdf extraction should succeed");

    let chunks = result.chunks.expect("chunking was configured — chunks must be present");

    assert!(!chunks.is_empty(), "multi-page PDF should produce at least one chunk");

    let null_page_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| c.metadata.first_page.is_none() || c.metadata.last_page.is_none())
        .collect();

    assert!(
        null_page_chunks.is_empty(),
        "{} of {} chunks have null page metadata (first_page or last_page is None). \
         Chunk indices with null metadata: {:?}",
        null_page_chunks.len(),
        chunks.len(),
        null_page_chunks
            .iter()
            .map(|c| c.metadata.chunk_index)
            .collect::<Vec<_>>()
    );
}

/// Chunks from a multi-page PDF must have monotonically non-decreasing page numbers.
///
/// Verifies that page boundaries are contiguous and in order — a secondary property
/// that would be violated if `recompute_boundaries_from_pages` miscalculated
/// `search_offset` for any page.
#[test]
fn chunks_from_multi_page_pdf_have_monotonic_page_numbers() {
    if skip_if_missing("pdf/multi_page.pdf") {
        eprintln!("skipping: fixture pdf/multi_page.pdf not found");
        return;
    }

    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 500,
            overlap: 50,
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file_sync(get_test_file_path("pdf/multi_page.pdf"), None, &config)
        .expect("multi_page.pdf extraction should succeed");

    let chunks = result.chunks.expect("chunking was configured — chunks must be present");

    assert!(
        chunks.iter().all(|c| c.metadata.first_page.is_some()),
        "all chunks must have first_page before checking order"
    );

    let mut prev_first_page = 0u32;
    for chunk in &chunks {
        if let Some(first) = chunk.metadata.first_page {
            assert!(
                first >= prev_first_page,
                "chunk {} first_page ({}) must be >= previous first_page ({})",
                chunk.metadata.chunk_index,
                first,
                prev_first_page
            );
            prev_first_page = first;
        }
    }
}
