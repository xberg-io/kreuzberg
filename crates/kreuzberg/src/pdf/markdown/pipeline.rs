//! Main PDF-to-Markdown pipeline orchestrator.

use crate::pdf::error::Result;
use crate::pdf::hierarchy::{BoundingBox, SegmentData, TextBlock, assign_heading_levels_smart, cluster_font_sizes};
use pdfium_render::prelude::*;

use super::assembly::{assemble_markdown, assemble_markdown_with_tables};
use super::bridge::{ImagePosition, extracted_blocks_to_paragraphs, objects_to_page_data};
use super::classify::classify_paragraphs;
use super::constants::{
    MIN_FONT_SIZE, MIN_HEADING_FONT_GAP, MIN_HEADING_FONT_RATIO, PAGE_BOTTOM_MARGIN_FRACTION, PAGE_TOP_MARGIN_FRACTION,
};
use super::lines::segments_to_lines;
use super::paragraphs::{lines_to_paragraphs, merge_continuation_paragraphs};
use super::render::inject_image_placeholders;
use super::types::PdfParagraph;

/// Render an entire PDF document as markdown.
pub fn render_document_as_markdown(document: &PdfDocument, k_clusters: usize) -> Result<String> {
    render_document_as_markdown_with_tables(document, k_clusters, &[], None, None)
}

/// Render a PDF document as markdown, with tables interleaved at their positions.
pub fn render_document_as_markdown_with_tables(
    document: &PdfDocument,
    k_clusters: usize,
    tables: &[crate::types::Table],
    top_margin: Option<f32>,
    bottom_margin: Option<f32>,
) -> Result<String> {
    let pages = document.pages();
    let page_count = pages.len();

    // Stage 0: Try structure tree extraction for each page.
    let mut struct_tree_results: Vec<Option<Vec<PdfParagraph>>> = Vec::with_capacity(page_count as usize);
    let mut heuristic_pages: Vec<usize> = Vec::new();

    for i in 0..page_count {
        let page = pages.get(i).map_err(|e| {
            crate::pdf::error::PdfError::TextExtractionFailed(format!("Failed to get page {}: {:?}", i, e))
        })?;

        match extract_page_content(&page) {
            Ok(extraction) if extraction.method == ExtractionMethod::StructureTree && !extraction.blocks.is_empty() => {
                let paragraphs = extracted_blocks_to_paragraphs(&extraction.blocks);
                if paragraphs.is_empty() {
                    struct_tree_results.push(None);
                    heuristic_pages.push(i as usize);
                } else {
                    struct_tree_results.push(Some(paragraphs));
                }
            }
            _ => {
                struct_tree_results.push(None);
                heuristic_pages.push(i as usize);
            }
        }
    }

    // Stage 1: Extract segments from pages that need heuristic extraction.
    // Uses pdfium's page objects API (via PdfParagraph::from_objects) for spatial analysis
    // and text grouping, plus image detection for position-aware placeholders.
    let mut all_page_segments: Vec<Vec<SegmentData>> = vec![Vec::new(); page_count as usize];
    let mut all_image_positions: Vec<ImagePosition> = Vec::new();
    let mut image_offset = 0usize;

    for &i in &heuristic_pages {
        let page = pages.get(i as PdfPageIndex).map_err(|e| {
            crate::pdf::error::PdfError::TextExtractionFailed(format!("Failed to get page {}: {:?}", i, e))
        })?;

        let (segments, image_positions) = objects_to_page_data(&page, i + 1, &mut image_offset);

        // Filter out segments in page margins (headers/footers/page numbers)
        let page_height = page.height().value;
        let top_frac = top_margin.unwrap_or(PAGE_TOP_MARGIN_FRACTION).clamp(0.0, 0.5);
        let bottom_frac = bottom_margin.unwrap_or(PAGE_BOTTOM_MARGIN_FRACTION).clamp(0.0, 0.5);
        let top_cutoff = page_height * (1.0 - top_frac);
        let bottom_cutoff = page_height * bottom_frac;

        let mut filtered: Vec<SegmentData> = segments
            .into_iter()
            .filter(|s| s.baseline_y <= top_cutoff && s.baseline_y >= bottom_cutoff)
            .filter(|s| s.font_size >= MIN_FONT_SIZE)
            .collect();

        // Remove standalone page numbers: short numeric-only segments that are isolated
        // (no other segment on the same baseline)
        filter_standalone_page_numbers(&mut filtered);

        all_page_segments[i] = filtered;
        all_image_positions.extend(image_positions);
    }

    // Stage 2: Global font-size clustering (only for heuristic pages).
    let mut all_blocks: Vec<TextBlock> = Vec::new();
    let empty_bbox = BoundingBox {
        left: 0.0,
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
    };
    for &i in &heuristic_pages {
        for seg in &all_page_segments[i] {
            if seg.text.trim().is_empty() {
                continue;
            }
            all_blocks.push(TextBlock {
                text: String::new(),
                bbox: empty_bbox,
                font_size: seg.font_size,
            });
        }
    }

    let heading_map = if all_blocks.is_empty() {
        Vec::new()
    } else {
        let clusters = cluster_font_sizes(&all_blocks, k_clusters)?;
        assign_heading_levels_smart(&clusters, MIN_HEADING_FONT_RATIO, MIN_HEADING_FONT_GAP)
    };

    // Stage 3: Per-page structured extraction.
    let mut all_page_paragraphs: Vec<Vec<PdfParagraph>> = Vec::with_capacity(page_count as usize);
    for i in 0..page_count as usize {
        if let Some(paragraphs) = struct_tree_results[i].take() {
            all_page_paragraphs.push(paragraphs);
        } else {
            let page_segments = &all_page_segments[i];
            let lines = segments_to_lines(page_segments.clone());
            let mut paragraphs = lines_to_paragraphs(lines);
            classify_paragraphs(&mut paragraphs, &heading_map);
            merge_continuation_paragraphs(&mut paragraphs);
            all_page_paragraphs.push(paragraphs);
        }
    }

    // Stage 4: Assemble markdown with tables interleaved
    let markdown = if tables.is_empty() {
        assemble_markdown(all_page_paragraphs)
    } else {
        assemble_markdown_with_tables(all_page_paragraphs, tables)
    };

    // Stage 5: Inject image placeholders from positions collected during object extraction
    if all_image_positions.is_empty() {
        Ok(markdown)
    } else {
        let image_metadata: Vec<crate::types::ExtractedImage> = all_image_positions
            .iter()
            .map(|img| crate::types::ExtractedImage {
                data: bytes::Bytes::new(),
                format: std::borrow::Cow::Borrowed("unknown"),
                image_index: img.image_index,
                page_number: Some(img.page_number),
                width: None,
                height: None,
                colorspace: None,
                bits_per_component: None,
                is_mask: false,
                description: None,
                ocr_result: None,
                bounding_box: None,
            })
            .collect();
        Ok(inject_image_placeholders(&markdown, &image_metadata))
    }
}

/// Remove standalone page numbers from segments.
///
/// A standalone page number is a short numeric-only segment that has no other
/// segment sharing its approximate baseline (i.e., it sits alone on its line).
fn filter_standalone_page_numbers(segments: &mut Vec<SegmentData>) {
    if segments.is_empty() {
        return;
    }

    // Identify candidate page number indices
    let tolerance = 3.0_f32; // baseline proximity tolerance in points
    let candidates: Vec<usize> = segments
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            let trimmed = s.text.trim();
            !trimmed.is_empty() && trimmed.len() <= 4 && trimmed.chars().all(|c| c.is_ascii_digit())
        })
        .filter(|(idx, s)| {
            // Check that no other segment shares this baseline
            !segments
                .iter()
                .enumerate()
                .any(|(j, other)| j != *idx && (other.baseline_y - s.baseline_y).abs() < tolerance)
        })
        .map(|(idx, _)| idx)
        .collect();

    // Remove in reverse order to preserve indices
    for &idx in candidates.iter().rev() {
        segments.remove(idx);
    }
}
