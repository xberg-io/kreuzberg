//! Chunk calculation algorithms.
//!
//! Provides functions to calculate optimal chunk boundaries based on
//! document characteristics and [`HeuristicsConfig`].

use crate::heuristics::config::HeuristicsConfig;
use crate::heuristics::decision::{ChunkInfo, ChunkPlan, ChunkingReason, PageRange};

/// Estimated milliseconds per page for OCR processing.
const OCR_MS_PER_PAGE: u64 = 3000;

/// Estimated milliseconds per page for text extraction.
const TEXT_MS_PER_PAGE: u64 = 100;

/// Calculate a chunking plan for a document.
///
/// # Arguments
///
/// * `page_count` - Total number of pages in the document
/// * `size_bytes` - File size in bytes
/// * `needs_ocr` - Whether OCR will be required
/// * `config` - Heuristics configuration
///
/// # Returns
///
/// A [`ChunkPlan`] with optimal chunk boundaries.
pub fn calculate_chunk_plan(page_count: u32, size_bytes: u64, needs_ocr: bool, config: &HeuristicsConfig) -> ChunkPlan {
    let pages_per_chunk = calculate_pages_per_chunk(page_count, needs_ocr, config);
    let num_chunks = page_count.div_ceil(pages_per_chunk);

    let mut chunks = Vec::with_capacity(num_chunks as usize);
    let mut current_page = 0u32;

    for index in 0..num_chunks {
        let start = current_page;
        let remaining = page_count - start;
        let chunk_pages = remaining.min(pages_per_chunk);
        let end = start + chunk_pages - 1;

        let estimated_time_ms = estimate_chunk_time(chunk_pages, needs_ocr);

        chunks.push(ChunkInfo {
            index,
            pages: PageRange::new(start, end),
            estimated_time_ms,
        });

        current_page = end + 1;
    }

    let total_estimated_time_ms = chunks.iter().map(|c| c.estimated_time_ms).sum();
    let use_disk_processing = size_bytes >= config.disk_processing_threshold_bytes;

    let reason = determine_chunking_reason(page_count, size_bytes, needs_ocr, config);

    ChunkPlan {
        total_chunks: num_chunks,
        chunks,
        total_estimated_time_ms,
        use_disk_processing,
        reason,
    }
}

/// Calculate the optimal number of pages per chunk.
fn calculate_pages_per_chunk(page_count: u32, needs_ocr: bool, config: &HeuristicsConfig) -> u32 {
    // For OCR documents, use smaller chunks to enable parallelism.
    let base_pages = if needs_ocr {
        config.target_pages_per_chunk
    } else {
        // For non-OCR, we can process more pages per chunk.
        config.target_pages_per_chunk * 2
    };

    // Ensure we don't exceed max_pages_per_chunk.
    let pages = base_pages.min(config.max_pages_per_chunk);

    // Ensure at least 1 page per chunk, at most page_count.
    pages.max(1).min(page_count)
}

/// Estimate processing time for a chunk in milliseconds.
fn estimate_chunk_time(pages: u32, needs_ocr: bool) -> u64 {
    let ms_per_page = if needs_ocr { OCR_MS_PER_PAGE } else { TEXT_MS_PER_PAGE };
    pages as u64 * ms_per_page
}

/// Determine the reason for chunking.
fn determine_chunking_reason(
    page_count: u32,
    size_bytes: u64,
    needs_ocr: bool,
    config: &HeuristicsConfig,
) -> ChunkingReason {
    let large_file = size_bytes >= config.file_size_threshold_bytes;
    let many_pages = page_count >= config.page_count_threshold;

    if large_file && many_pages {
        ChunkingReason::LargeAndManyPages { size_bytes, page_count }
    } else if needs_ocr && page_count > config.target_pages_per_chunk {
        ChunkingReason::OcrRequired {
            page_count,
            force_ocr: needs_ocr,
        }
    } else if many_pages {
        ChunkingReason::ManyPages {
            page_count,
            threshold: config.page_count_threshold,
        }
    } else {
        ChunkingReason::LargeFile {
            size_bytes,
            threshold_bytes: config.file_size_threshold_bytes,
        }
    }
}

/// Calculate a chunk plan from user-specified page ranges.
///
/// Validates and processes user overrides into a proper chunk plan.
pub fn calculate_plan_from_overrides(
    user_chunks: &[PageRange],
    total_pages: u32,
    size_bytes: u64,
    config: &HeuristicsConfig,
) -> ChunkPlan {
    let chunks: Vec<ChunkInfo> = user_chunks
        .iter()
        .enumerate()
        .map(|(idx, range)| {
            let pages = range.page_count();
            ChunkInfo {
                index: idx as u32,
                pages: range.clone(),
                estimated_time_ms: estimate_chunk_time(pages, true), // Assume OCR for safety.
            }
        })
        .collect();

    let total_estimated_time_ms = chunks.iter().map(|c| c.estimated_time_ms).sum();
    let use_disk_processing = size_bytes >= config.disk_processing_threshold_bytes;

    ChunkPlan {
        total_chunks: chunks.len() as u32,
        chunks,
        total_estimated_time_ms,
        use_disk_processing,
        reason: ChunkingReason::ManyPages {
            page_count: total_pages,
            threshold: config.page_count_threshold,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> HeuristicsConfig {
        HeuristicsConfig {
            enable_pdf_text_heuristics: true,
            text_layer_threshold: 0.7,
            file_size_threshold_bytes: 10 * 1024 * 1024,
            page_count_threshold: 50,
            target_pages_per_chunk: 10,
            max_pages_per_chunk: 25,
            disk_processing_threshold_bytes: 50 * 1024 * 1024,
            min_chars_per_page: 50,
            max_xlsx_sheet_count: 200,
            max_xlsx_workbook_cells: 5_000_000,
            max_pptx_embedded_count: 50,
        }
    }

    #[test]
    fn test_calculate_chunk_plan_basic() {
        let config = test_config();
        let plan = calculate_chunk_plan(100, 20 * 1024 * 1024, true, &config);

        assert_eq!(plan.total_chunks, 10);
        assert_eq!(plan.total_pages(), 100);
        assert!(!plan.use_disk_processing);

        assert_eq!(plan.chunks[0].pages.start, 0);
        assert_eq!(plan.chunks[0].pages.end, 9);
        assert_eq!(plan.chunks[9].pages.start, 90);
        assert_eq!(plan.chunks[9].pages.end, 99);
    }

    #[test]
    fn test_calculate_chunk_plan_uneven_pages() {
        let config = test_config();
        let plan = calculate_chunk_plan(55, 20 * 1024 * 1024, true, &config);

        assert_eq!(plan.total_chunks, 6);
        assert_eq!(plan.total_pages(), 55);

        let last_chunk = plan.chunks.last().unwrap();
        assert_eq!(last_chunk.pages.page_count(), 5);
    }

    #[test]
    fn test_calculate_chunk_plan_non_ocr_uses_larger_chunks() {
        let config = test_config();

        let ocr_plan = calculate_chunk_plan(100, 20 * 1024 * 1024, true, &config);
        let text_plan = calculate_chunk_plan(100, 20 * 1024 * 1024, false, &config);

        assert!(text_plan.total_chunks < ocr_plan.total_chunks);
    }

    #[test]
    fn test_calculate_chunk_plan_disk_processing() {
        let config = test_config();

        let small_plan = calculate_chunk_plan(100, 10 * 1024 * 1024, true, &config);
        assert!(!small_plan.use_disk_processing);

        let large_plan = calculate_chunk_plan(100, 60 * 1024 * 1024, true, &config);
        assert!(large_plan.use_disk_processing);
    }

    #[test]
    fn test_calculate_pages_per_chunk() {
        let config = test_config();

        let ocr_pages = calculate_pages_per_chunk(100, true, &config);
        assert_eq!(ocr_pages, 10);

        let text_pages = calculate_pages_per_chunk(100, false, &config);
        assert_eq!(text_pages, 20);
    }

    #[test]
    fn test_estimate_chunk_time() {
        let ocr_time = estimate_chunk_time(10, true);
        assert_eq!(ocr_time, 30000); // 10 pages * 3000ms

        let text_time = estimate_chunk_time(10, false);
        assert_eq!(text_time, 1000); // 10 pages * 100ms
    }

    #[test]
    fn test_calculate_plan_from_overrides() {
        let config = test_config();
        let overrides = vec![PageRange::new(0, 9), PageRange::new(10, 19), PageRange::new(20, 24)];

        let plan = calculate_plan_from_overrides(&overrides, 25, 10 * 1024 * 1024, &config);

        assert_eq!(plan.total_chunks, 3);
        assert_eq!(plan.chunks[0].pages.start, 0);
        assert_eq!(plan.chunks[0].pages.end, 9);
        assert_eq!(plan.chunks[2].pages.page_count(), 5);
    }

    #[test]
    fn test_determine_chunking_reason_large_and_many_pages() {
        let config = test_config();
        let reason = determine_chunking_reason(100, 15 * 1024 * 1024, false, &config);

        match reason {
            ChunkingReason::LargeAndManyPages { size_bytes, page_count } => {
                assert_eq!(size_bytes, 15 * 1024 * 1024);
                assert_eq!(page_count, 100);
            }
            _ => panic!("Expected LargeAndManyPages, got {:?}", reason),
        }
    }

    #[test]
    fn test_determine_chunking_reason_ocr_required() {
        let config = test_config();
        let reason = determine_chunking_reason(25, 5 * 1024 * 1024, true, &config);

        match reason {
            ChunkingReason::OcrRequired { page_count, force_ocr } => {
                assert_eq!(page_count, 25);
                assert!(force_ocr);
            }
            _ => panic!("Expected OcrRequired, got {:?}", reason),
        }
    }

    #[test]
    fn test_determine_chunking_reason_many_pages() {
        let config = test_config();
        let reason = determine_chunking_reason(75, 5 * 1024 * 1024, false, &config);

        match reason {
            ChunkingReason::ManyPages { page_count, threshold } => {
                assert_eq!(page_count, 75);
                assert_eq!(threshold, 50);
            }
            _ => panic!("Expected ManyPages, got {:?}", reason),
        }
    }

    #[test]
    fn test_determine_chunking_reason_large_file() {
        let config = test_config();
        let reason = determine_chunking_reason(20, 15 * 1024 * 1024, false, &config);

        match reason {
            ChunkingReason::LargeFile {
                size_bytes,
                threshold_bytes,
            } => {
                assert_eq!(size_bytes, 15 * 1024 * 1024);
                assert_eq!(threshold_bytes, 10 * 1024 * 1024);
            }
            _ => panic!("Expected LargeFile, got {:?}", reason),
        }
    }

    #[test]
    fn test_very_large_document_thousands_of_pages_ocr() {
        let config = test_config();
        let page_count = 5000u32;
        let plan = calculate_chunk_plan(page_count, 500 * 1024 * 1024, true, &config);

        assert_eq!(plan.total_chunks, 500);
        assert_eq!(plan.total_pages(), page_count);
        assert!(plan.use_disk_processing);

        assert_eq!(plan.chunks[0].pages.start, 0);
        assert_eq!(plan.chunks[0].pages.end, 9);
        assert_eq!(plan.chunks[499].pages.start, 4990);
        assert_eq!(plan.chunks[499].pages.end, 4999);

        let expected_total_time = page_count as u64 * OCR_MS_PER_PAGE;
        assert_eq!(plan.total_estimated_time_ms, expected_total_time);
    }

    #[test]
    fn test_single_page_document_ocr() {
        let config = test_config();
        let plan = calculate_chunk_plan(1, 1024 * 1024, true, &config);

        assert_eq!(plan.total_chunks, 1);
        assert_eq!(plan.total_pages(), 1);
        assert_eq!(plan.chunks[0].pages.start, 0);
        assert_eq!(plan.chunks[0].pages.end, 0);
        assert_eq!(plan.chunks[0].estimated_time_ms, OCR_MS_PER_PAGE);
    }

    #[test]
    fn test_boundary_conditions_page_threshold() {
        let config = test_config();

        let at_threshold = determine_chunking_reason(50, 5 * 1024 * 1024, false, &config);
        match at_threshold {
            ChunkingReason::ManyPages { page_count, .. } => {
                assert_eq!(page_count, 50);
            }
            _ => panic!("Expected ManyPages at threshold boundary"),
        }

        let below_threshold = determine_chunking_reason(49, 5 * 1024 * 1024, false, &config);
        assert!(matches!(below_threshold, ChunkingReason::LargeFile { .. }));
    }
}
