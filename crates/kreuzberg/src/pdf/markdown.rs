//! PDF-to-Markdown renderer using segment-level font analysis.
//!
//! Converts PDF documents into structured markdown by analyzing pdfium text segments
//! (pre-merged character runs sharing baseline + font settings) to reconstruct headings,
//! paragraphs, inline formatting, and list items.

use crate::pdf::error::Result;
#[cfg(test)]
use crate::pdf::hierarchy::CharData;
use crate::pdf::hierarchy::{
    BoundingBox, SegmentData, TextBlock, assign_heading_levels_smart, cluster_font_sizes, extract_segments_from_page,
};
use pdfium_render::prelude::*;

// Threshold constants for spatial analysis
/// Baseline Y tolerance as a fraction of the smaller font size for same-line grouping.
const BASELINE_Y_TOLERANCE_FRACTION: f32 = 0.5;
/// Gap threshold as a fraction of the average font size for word-break detection.
#[cfg(test)]
const WORD_GAP_FRACTION: f32 = 0.3;
/// Multiplier for median line spacing to detect paragraph breaks.
const PARAGRAPH_GAP_MULTIPLIER: f32 = 1.5;
/// Font size change threshold (in points) to trigger a paragraph break.
const FONT_SIZE_CHANGE_THRESHOLD: f32 = 1.5;
/// Left indent change threshold (in points) to trigger a paragraph break.
const LEFT_INDENT_CHANGE_THRESHOLD: f32 = 10.0;
/// Maximum word count for a paragraph to qualify as a heading.
const MAX_HEADING_WORD_COUNT: usize = 12;
/// Minimum gutter width as multiple of average character width for column boundary detection.
const MIN_GUTTER_WIDTH_MULTIPLIER: f32 = 2.0;
/// Minimum fraction of page height that a gutter must span.
const MIN_GUTTER_HEIGHT_FRACTION: f32 = 0.6;
/// Histogram bin width in points for x-position projection.
const COLUMN_HISTOGRAM_BIN_WIDTH: f32 = 5.0;
/// Maximum number of lines for a paragraph to be classified as a list item.
const MAX_LIST_ITEM_LINES: usize = 5;
/// Maximum distance multiplier relative to average inter-cluster gap for heading assignment.
const MAX_HEADING_DISTANCE_MULTIPLIER: f32 = 2.0;

/// A detected column region on a page.
#[derive(Debug, Clone)]
struct ColumnRegion {
    x_min: f32,
    x_max: f32,
}

/// A single word extracted from PDF character data.
#[derive(Debug, Clone)]
struct PdfWord {
    text: String,
    x_start: f32,
    #[allow(dead_code)]
    x_end: f32,
    baseline_y: f32,
    font_size: f32,
    is_bold: bool,
    is_italic: bool,
}

/// A line of text composed of words sharing a common baseline.
#[derive(Debug, Clone)]
struct PdfLine {
    words: Vec<PdfWord>,
    baseline_y: f32,
    #[allow(dead_code)]
    y_top: f32,
    #[allow(dead_code)]
    y_bottom: f32,
    dominant_font_size: f32,
    is_bold: bool,
    is_italic: bool,
}

/// A paragraph composed of lines, with optional heading classification.
#[derive(Debug, Clone)]
struct PdfParagraph {
    lines: Vec<PdfLine>,
    dominant_font_size: f32,
    heading_level: Option<u8>,
    #[allow(dead_code)]
    is_bold: bool,
    #[allow(dead_code)]
    is_italic: bool,
    is_list_item: bool,
}

/// Detect column boundaries from segments by finding vertical gutters.
/// Returns column regions sorted left-to-right. Single-column pages return one region.
fn detect_columns_from_segments(segments: &[SegmentData], page_width: f32, page_height: f32) -> Vec<ColumnRegion> {
    if segments.is_empty() || page_width <= 0.0 || page_height <= 0.0 {
        return vec![ColumnRegion {
            x_min: 0.0,
            x_max: page_width,
        }];
    }

    // Estimate avg char width from segment widths / text lengths
    let total_width: f32 = segments.iter().map(|s| s.width).sum();
    let total_chars: usize = segments.iter().map(|s| s.text.len()).sum();
    let avg_char_width = if total_chars > 0 {
        total_width / total_chars as f32
    } else {
        COLUMN_HISTOGRAM_BIN_WIDTH
    };
    let min_gutter_width = avg_char_width * MIN_GUTTER_WIDTH_MULTIPLIER;

    // Build histogram of segment presence per x-bin, tracking y-span
    let num_bins = ((page_width / COLUMN_HISTOGRAM_BIN_WIDTH).ceil() as usize).max(1);
    let mut bin_y_min = vec![f32::INFINITY; num_bins];
    let mut bin_y_max = vec![f32::NEG_INFINITY; num_bins];
    let mut bin_count = vec![0u32; num_bins];

    for seg in segments {
        let bin_start = ((seg.x / COLUMN_HISTOGRAM_BIN_WIDTH).floor() as usize).min(num_bins - 1);
        let bin_end = (((seg.x + seg.width) / COLUMN_HISTOGRAM_BIN_WIDTH).ceil() as usize).min(num_bins);
        for b in bin_start..bin_end {
            bin_y_min[b] = bin_y_min[b].min(seg.baseline_y);
            bin_y_max[b] = bin_y_max[b].max(seg.baseline_y);
            bin_count[b] += 1;
        }
    }

    // Find gutter regions: consecutive empty bins
    let mut gutters: Vec<(f32, f32)> = Vec::new();
    let mut gutter_start: Option<usize> = None;

    for (i, &count) in bin_count.iter().enumerate() {
        if count == 0 {
            if gutter_start.is_none() {
                gutter_start = Some(i);
            }
        } else if let Some(start) = gutter_start {
            let x_start = start as f32 * COLUMN_HISTOGRAM_BIN_WIDTH;
            let x_end = i as f32 * COLUMN_HISTOGRAM_BIN_WIDTH;
            if x_end - x_start >= min_gutter_width {
                let left_y_min = bin_y_min[..start].iter().copied().fold(f32::INFINITY, f32::min);
                let left_y_max = bin_y_max[..start].iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let left_span = if left_y_max > left_y_min {
                    (left_y_max - left_y_min).abs()
                } else {
                    0.0
                };

                let right_y_min = bin_y_min[i..].iter().copied().fold(f32::INFINITY, f32::min);
                let right_y_max = bin_y_max[i..].iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let right_span = if right_y_max > right_y_min {
                    (right_y_max - right_y_min).abs()
                } else {
                    0.0
                };

                if left_span.max(right_span) >= page_height * MIN_GUTTER_HEIGHT_FRACTION {
                    gutters.push((x_start, x_end));
                }
            }
            gutter_start = None;
        }
    }

    if gutters.is_empty() {
        return vec![ColumnRegion {
            x_min: 0.0,
            x_max: page_width,
        }];
    }

    // Build column regions from gutters
    let mut columns: Vec<ColumnRegion> = Vec::new();
    let mut prev_x = 0.0_f32;
    for (gl, gr) in &gutters {
        if *gl > prev_x {
            columns.push(ColumnRegion {
                x_min: prev_x,
                x_max: *gl,
            });
        }
        prev_x = *gr;
    }
    if prev_x < page_width {
        columns.push(ColumnRegion {
            x_min: prev_x,
            x_max: page_width,
        });
    }

    // Filter out columns with no segments
    columns.retain(|col| segments.iter().any(|s| s.x >= col.x_min && s.x < col.x_max));

    if columns.is_empty() {
        vec![ColumnRegion {
            x_min: 0.0,
            x_max: page_width,
        }]
    } else {
        columns
    }
}

/// Split segments into column groups based on detected column regions.
fn split_segments_by_columns<'a>(segments: &'a [SegmentData], columns: &[ColumnRegion]) -> Vec<Vec<&'a SegmentData>> {
    let mut column_segments: Vec<Vec<&SegmentData>> = vec![Vec::new(); columns.len()];
    for seg in segments {
        let center_x = seg.x + seg.width / 2.0;
        let mut assigned = false;
        for (i, col) in columns.iter().enumerate() {
            if center_x >= col.x_min && center_x < col.x_max {
                column_segments[i].push(seg);
                assigned = true;
                break;
            }
        }
        if !assigned {
            let nearest = columns
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    let da = (center_x - (a.x_min + a.x_max) / 2.0).abs();
                    let db = (center_x - (b.x_min + b.x_max) / 2.0).abs();
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            column_segments[nearest].push(seg);
        }
    }
    column_segments
}

/// Detect column boundaries by finding vertical gutters in character x-positions.
/// Returns column regions sorted left-to-right. Single-column pages return one region.
#[cfg(test)]
fn detect_columns(chars: &[CharData], page_width: f32, page_height: f32) -> Vec<ColumnRegion> {
    if chars.is_empty() || page_width <= 0.0 || page_height <= 0.0 {
        return vec![ColumnRegion {
            x_min: 0.0,
            x_max: page_width,
        }];
    }

    let avg_char_width = chars
        .iter()
        .filter(|c| !c.text.trim().is_empty())
        .map(|c| c.width)
        .sum::<f32>()
        / chars.iter().filter(|c| !c.text.trim().is_empty()).count().max(1) as f32;
    let min_gutter_width = avg_char_width * MIN_GUTTER_WIDTH_MULTIPLIER;

    // Build histogram of character presence per x-bin, tracking y-span
    let num_bins = ((page_width / COLUMN_HISTOGRAM_BIN_WIDTH).ceil() as usize).max(1);
    let mut bin_y_min = vec![f32::INFINITY; num_bins];
    let mut bin_y_max = vec![f32::NEG_INFINITY; num_bins];
    let mut bin_count = vec![0u32; num_bins];

    for ch in chars {
        if ch.text.trim().is_empty() {
            continue;
        }
        let bin_start = ((ch.x / COLUMN_HISTOGRAM_BIN_WIDTH).floor() as usize).min(num_bins - 1);
        let bin_end = (((ch.x + ch.width) / COLUMN_HISTOGRAM_BIN_WIDTH).ceil() as usize).min(num_bins);
        for b in bin_start..bin_end {
            bin_y_min[b] = bin_y_min[b].min(ch.baseline_y);
            bin_y_max[b] = bin_y_max[b].max(ch.baseline_y);
            bin_count[b] += 1;
        }
    }

    // Find gutter regions: consecutive empty bins
    let mut gutters: Vec<(f32, f32)> = Vec::new();
    let mut gutter_start: Option<usize> = None;

    for (i, &count) in bin_count.iter().enumerate() {
        if count == 0 {
            if gutter_start.is_none() {
                gutter_start = Some(i);
            }
        } else if let Some(start) = gutter_start {
            let x_start = start as f32 * COLUMN_HISTOGRAM_BIN_WIDTH;
            let x_end = i as f32 * COLUMN_HISTOGRAM_BIN_WIDTH;
            if x_end - x_start >= min_gutter_width {
                // Check that columns adjacent to the gutter span enough page height.
                // Accumulate y-span across all bins in the left/right column region,
                // not just the single adjacent bin, for robustness.
                let left_y_min = bin_y_min[..start].iter().copied().fold(f32::INFINITY, f32::min);
                let left_y_max = bin_y_max[..start].iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let left_span = if left_y_max > left_y_min {
                    (left_y_max - left_y_min).abs()
                } else {
                    0.0
                };

                let right_y_min = bin_y_min[i..].iter().copied().fold(f32::INFINITY, f32::min);
                let right_y_max = bin_y_max[i..].iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let right_span = if right_y_max > right_y_min {
                    (right_y_max - right_y_min).abs()
                } else {
                    0.0
                };

                if left_span.max(right_span) >= page_height * MIN_GUTTER_HEIGHT_FRACTION {
                    gutters.push((x_start, x_end));
                }
            }
            gutter_start = None;
        }
    }

    if gutters.is_empty() {
        return vec![ColumnRegion {
            x_min: 0.0,
            x_max: page_width,
        }];
    }

    // Build column regions from gutters
    let mut columns: Vec<ColumnRegion> = Vec::new();
    let mut prev_x = 0.0_f32;
    for (gl, gr) in &gutters {
        if *gl > prev_x {
            columns.push(ColumnRegion {
                x_min: prev_x,
                x_max: *gl,
            });
        }
        prev_x = *gr;
    }
    if prev_x < page_width {
        columns.push(ColumnRegion {
            x_min: prev_x,
            x_max: page_width,
        });
    }

    // Filter out columns with no characters
    columns.retain(|col| {
        chars
            .iter()
            .any(|c| !c.text.trim().is_empty() && c.x >= col.x_min && c.x < col.x_max)
    });

    if columns.is_empty() {
        vec![ColumnRegion {
            x_min: 0.0,
            x_max: page_width,
        }]
    } else {
        columns
    }
}

/// Render an entire PDF document as markdown using character-level font analysis.
///
/// Extracts characters from every page, clusters font sizes globally to determine
/// heading levels, then assembles structured markdown with headings, paragraphs,
/// bold/italic inline formatting, and list items.
///
/// # Arguments
///
/// * `document` - The PDF document to render
/// * `k_clusters` - Number of clusters for font-size k-means (typically 3-5)
///
/// # Returns
///
/// A `Result<String>` containing the full markdown text of the document.
pub fn render_document_as_markdown(document: &PdfDocument, k_clusters: usize) -> Result<String> {
    render_document_as_markdown_with_tables(document, k_clusters, &[])
}

/// Render a PDF document as markdown with inline table embedding.
///
/// Tables with bounding boxes are inserted at their correct vertical position,
/// and segments overlapping table regions are filtered to avoid duplication.
pub fn render_document_as_markdown_with_tables(
    document: &PdfDocument,
    k_clusters: usize,
    tables: &[crate::types::Table],
) -> Result<String> {
    let pages = document.pages();
    let page_count = pages.len();

    // Stage 0: Try structure tree extraction for each page.
    // Track which pages were successfully extracted via structure tree.
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
    let mut all_page_segments: Vec<Vec<SegmentData>> = vec![Vec::new(); page_count as usize];
    let mut page_dimensions: Vec<(f32, f32)> = vec![(0.0, 0.0); page_count as usize];

    for &i in &heuristic_pages {
        let page = pages.get(i as PdfPageIndex).map_err(|e| {
            crate::pdf::error::PdfError::TextExtractionFailed(format!("Failed to get page {}: {:?}", i, e))
        })?;
        let mut segments = extract_segments_from_page(&page)?;
        let (page_w, page_h) = (page.width().value, page.height().value);
        page_dimensions[i] = (page_w, page_h);

        // Filter out segments that fall within table bounding boxes on this page
        let page_tables: Vec<&crate::types::Table> = tables.iter().filter(|t| t.page_number == i + 1).collect();
        if !page_tables.is_empty() {
            segments.retain(|seg| {
                !page_tables.iter().any(|t| {
                    if let Some(ref bbox) = t.bounding_box {
                        let seg_center_x = seg.x + seg.width / 2.0;
                        seg_center_x >= bbox.x0 as f32
                            && seg_center_x <= bbox.x1 as f32
                            && seg.baseline_y >= bbox.y0 as f32
                            && seg.baseline_y <= bbox.y1 as f32
                    } else {
                        false
                    }
                })
            });
        }

        all_page_segments[i] = segments;
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
        assign_heading_levels_smart(&clusters)
    };

    // Stage 3: Per-page structured extraction (heuristic for remaining pages).
    let mut all_page_paragraphs: Vec<Vec<PdfParagraph>> = Vec::with_capacity(page_count as usize);
    for i in 0..page_count as usize {
        if let Some(paragraphs) = struct_tree_results[i].take() {
            all_page_paragraphs.push(paragraphs);
        } else {
            let page_segments = &all_page_segments[i];
            let (page_w, page_h) = page_dimensions[i];
            let columns = detect_columns_from_segments(page_segments, page_w, page_h);

            let mut page_paragraphs: Vec<PdfParagraph> = Vec::new();

            if columns.len() <= 1 {
                let words = segments_to_words(page_segments);
                let lines = words_to_lines(words);
                let mut paragraphs = lines_to_paragraphs(lines);
                classify_paragraphs(&mut paragraphs, &heading_map);
                page_paragraphs = paragraphs;
            } else {
                let column_segment_groups = split_segments_by_columns(page_segments, &columns);
                for col_segments in &column_segment_groups {
                    if col_segments.is_empty() {
                        continue;
                    }
                    let owned: Vec<SegmentData> = col_segments.iter().map(|s| (*s).clone()).collect();
                    let words = segments_to_words(&owned);
                    let lines = words_to_lines(words);
                    let mut paragraphs = lines_to_paragraphs(lines);
                    classify_paragraphs(&mut paragraphs, &heading_map);
                    page_paragraphs.extend(paragraphs);
                }
            }

            all_page_paragraphs.push(page_paragraphs);
        }
    }

    // Stage 4: Assemble markdown with inline tables
    Ok(assemble_markdown_with_tables(all_page_paragraphs, tables))
}

/// Returns true if the character is a CJK ideograph, Hiragana, Katakana, or Hangul.
/// Used for word boundary detection — CJK characters don't use spaces between words.
fn is_cjk_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x4E00..=0x9FFF     // CJK Unified Ideographs
        | 0x3040..=0x309F   // Hiragana
        | 0x30A0..=0x30FF   // Katakana
        | 0xAC00..=0xD7AF   // Hangul Syllables
        | 0x3400..=0x4DBF   // CJK Extension A
        | 0xF900..=0xFAFF   // CJK Compatibility Ideographs
        | 0x20000..=0x2A6DF // CJK Extension B
        | 0x2A700..=0x2B73F // CJK Extension C
        | 0x2B740..=0x2B81F // CJK Extension D
        | 0x2B820..=0x2CEAF // CJK Extension E
        | 0x2CEB0..=0x2EBEF // CJK Extension F
        | 0x30000..=0x3134F // CJK Extension G
        | 0x31350..=0x323AF // CJK Extension H
        | 0x2F800..=0x2FA1F // CJK Compatibility Ideographs Supplement
    )
}

/// Returns true if a space should be inserted between two adjacent words.
/// CJK words should not have spaces between them.
fn needs_space_between(prev: &str, next: &str) -> bool {
    let prev_ends_cjk = prev.chars().last().is_some_and(is_cjk_char);
    let next_starts_cjk = next.chars().next().is_some_and(is_cjk_char);
    // No space when both sides are CJK
    !(prev_ends_cjk && next_starts_cjk)
}

/// Convert pre-merged text segments into words by splitting on whitespace.
///
/// Each segment's `.text` already has correct word boundaries from pdfium's merging.
/// We split on whitespace to get individual words, distributing the segment's bounding
/// box proportionally across words. This replaces the fragile character-gap detection.
fn segments_to_words(segments: &[SegmentData]) -> Vec<PdfWord> {
    if segments.is_empty() {
        return Vec::new();
    }

    let mut words: Vec<PdfWord> = Vec::new();

    for seg in segments {
        let trimmed = seg.text.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Split segment text into individual words on whitespace
        let seg_words: Vec<&str> = trimmed.split_whitespace().collect();
        if seg_words.is_empty() {
            continue;
        }

        // Total character count for proportional width distribution
        let total_chars: usize = seg_words.iter().map(|w| w.len()).sum();
        if total_chars == 0 {
            continue;
        }

        // Distribute the segment's width proportionally across words
        let mut x_offset = seg.x;
        for word_text in &seg_words {
            let word_frac = word_text.len() as f32 / total_chars as f32;
            let word_width = seg.width * word_frac;

            // Handle CJK: each CJK character should be its own word
            let chars_vec: Vec<char> = word_text.chars().collect();
            let has_cjk = chars_vec.iter().any(|c| is_cjk_char(*c));

            if has_cjk {
                // Split CJK characters individually, keep non-CJK runs together
                let char_width = if chars_vec.is_empty() {
                    word_width
                } else {
                    word_width / chars_vec.len() as f32
                };

                let mut run_start = 0;
                let mut run_x = x_offset;
                while run_start < chars_vec.len() {
                    let is_cjk_run = is_cjk_char(chars_vec[run_start]);
                    if is_cjk_run {
                        // Each CJK char is its own word
                        words.push(PdfWord {
                            text: chars_vec[run_start].to_string(),
                            x_start: run_x,
                            x_end: run_x + char_width,
                            baseline_y: seg.baseline_y,
                            font_size: seg.font_size,
                            is_bold: seg.is_bold,
                            is_italic: seg.is_italic,
                        });
                        run_x += char_width;
                        run_start += 1;
                    } else {
                        // Collect non-CJK run
                        let mut run_end = run_start + 1;
                        while run_end < chars_vec.len() && !is_cjk_char(chars_vec[run_end]) {
                            run_end += 1;
                        }
                        let run_text: String = chars_vec[run_start..run_end].iter().collect();
                        let run_w = char_width * (run_end - run_start) as f32;
                        words.push(PdfWord {
                            text: run_text,
                            x_start: run_x,
                            x_end: run_x + run_w,
                            baseline_y: seg.baseline_y,
                            font_size: seg.font_size,
                            is_bold: seg.is_bold,
                            is_italic: seg.is_italic,
                        });
                        run_x += run_w;
                        run_start = run_end;
                    }
                }
            } else {
                words.push(PdfWord {
                    text: word_text.to_string(),
                    x_start: x_offset,
                    x_end: x_offset + word_width,
                    baseline_y: seg.baseline_y,
                    font_size: seg.font_size,
                    is_bold: seg.is_bold,
                    is_italic: seg.is_italic,
                });
            }

            x_offset += word_width;
        }
    }

    words
}

/// Convert raw character data into words by detecting spatial gaps.
///
/// Characters are sorted by baseline_y then x. Characters sharing a baseline
/// (within tolerance) are grouped into lines, then split into words when the
/// horizontal gap exceeds a fraction of the average font size.
#[cfg(test)]
fn chars_to_words(chars: &[CharData]) -> Vec<PdfWord> {
    if chars.is_empty() {
        return Vec::new();
    }

    // Filter out control characters (CR, LF, tab, etc.) but keep spaces as word-break signals.
    let filtered: Vec<&CharData> = chars
        .iter()
        .filter(|c| c.text.chars().all(|ch| !ch.is_control()))
        .collect();

    if filtered.is_empty() {
        return Vec::new();
    }

    // Sort by baseline_y DESCENDING (top-to-bottom reading order), then x ascending.
    // PDF coordinates have y=0 at page bottom, increasing upward, so larger y = higher on page.
    let mut sorted = filtered;
    sorted.sort_by(|a, b| {
        b.baseline_y
            .partial_cmp(&a.baseline_y)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut words: Vec<PdfWord> = Vec::new();
    let mut word_chars: Vec<&CharData> = Vec::new();

    for ch in &sorted {
        // Space characters act as explicit word breaks
        if ch.text.trim().is_empty() {
            if !word_chars.is_empty() {
                words.push(finalize_word(&word_chars));
                word_chars.clear();
            }
            continue;
        }

        if word_chars.is_empty() {
            word_chars.push(ch);
            continue;
        }

        let prev = word_chars.last().unwrap();

        // Check if same baseline
        let min_fs = prev.font_size.min(ch.font_size).max(1.0);
        let same_line = (prev.baseline_y - ch.baseline_y).abs() < BASELINE_Y_TOLERANCE_FRACTION * min_fs;

        if same_line {
            // CJK characters always form word boundaries — each CJK char is its own word.
            // Check if either the previous or current character is CJK.
            let prev_is_cjk = prev.text.chars().any(is_cjk_char);
            let curr_is_cjk = ch.text.chars().any(is_cjk_char);

            if prev_is_cjk || curr_is_cjk {
                // Always break word at CJK character boundaries
                words.push(finalize_word(&word_chars));
                word_chars.clear();
            } else {
                // Check horizontal gap for word break (non-CJK logic)
                let prev_end = prev.x + prev.width;
                let gap = ch.x - prev_end;
                let avg_fs = ((prev.font_size + ch.font_size) / 2.0).max(1.0);

                if gap > WORD_GAP_FRACTION * avg_fs {
                    words.push(finalize_word(&word_chars));
                    word_chars.clear();
                }
            }
        } else {
            // Different line => finalize word
            words.push(finalize_word(&word_chars));
            word_chars.clear();
        }

        word_chars.push(ch);
    }

    if !word_chars.is_empty() {
        words.push(finalize_word(&word_chars));
    }

    words
}

/// Build a PdfWord from a sequence of characters.
#[cfg(test)]
fn finalize_word(chars: &[&CharData]) -> PdfWord {
    let text: String = chars.iter().map(|c| c.text.as_str()).collect();
    let x_start = chars.iter().map(|c| c.x).fold(f32::INFINITY, f32::min);
    let x_end = chars.iter().map(|c| c.x + c.width).fold(f32::NEG_INFINITY, f32::max);
    let baseline_y = chars.iter().map(|c| c.baseline_y).sum::<f32>() / chars.len() as f32;
    let font_size = chars.iter().map(|c| c.font_size).sum::<f32>() / chars.len() as f32;

    let bold_count = chars.iter().filter(|c| c.is_bold).count();
    let italic_count = chars.iter().filter(|c| c.is_italic).count();
    let majority = chars.len() / 2;

    PdfWord {
        text,
        x_start,
        x_end,
        baseline_y,
        font_size,
        is_bold: bold_count > majority,
        is_italic: italic_count > majority,
    }
}

/// Group words into lines by baseline proximity.
fn words_to_lines(words: Vec<PdfWord>) -> Vec<PdfLine> {
    if words.is_empty() {
        return Vec::new();
    }

    // Sort words by baseline_y DESCENDING (top-to-bottom), then x_start ascending.
    let mut sorted = words;
    sorted.sort_by(|a, b| {
        b.baseline_y
            .partial_cmp(&a.baseline_y)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.x_start.partial_cmp(&b.x_start).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut lines: Vec<PdfLine> = Vec::new();
    let mut current_words: Vec<PdfWord> = vec![sorted.remove(0)];

    for word in sorted {
        let current_baseline = current_words.iter().map(|w| w.baseline_y).sum::<f32>() / current_words.len() as f32;
        let min_fs = current_words
            .iter()
            .map(|w| w.font_size)
            .fold(f32::INFINITY, f32::min)
            .min(word.font_size)
            .max(1.0);

        if (word.baseline_y - current_baseline).abs() < BASELINE_Y_TOLERANCE_FRACTION * min_fs {
            current_words.push(word);
        } else {
            lines.push(finalize_line(current_words));
            current_words = vec![word];
        }
    }

    if !current_words.is_empty() {
        lines.push(finalize_line(current_words));
    }

    lines
}

/// Build a PdfLine from a set of words, sorting them left-to-right.
fn finalize_line(mut words: Vec<PdfWord>) -> PdfLine {
    // Sort words left-to-right within the line
    words.sort_by(|a, b| a.x_start.partial_cmp(&b.x_start).unwrap_or(std::cmp::Ordering::Equal));

    let baseline_y = words.iter().map(|w| w.baseline_y).sum::<f32>() / words.len() as f32;
    let y_top = words
        .iter()
        .map(|w| w.baseline_y - w.font_size)
        .fold(f32::INFINITY, f32::min);
    let y_bottom = words.iter().map(|w| w.baseline_y).fold(f32::NEG_INFINITY, f32::max);

    // Dominant font size: most frequent (rounded to nearest 0.5)
    let dominant_font_size = dominant_font_size_of_words(&words);

    let bold_count = words.iter().filter(|w| w.is_bold).count();
    let italic_count = words.iter().filter(|w| w.is_italic).count();
    let majority = words.len().div_ceil(2);

    PdfLine {
        baseline_y,
        y_top,
        y_bottom,
        dominant_font_size,
        is_bold: bold_count >= majority,
        is_italic: italic_count >= majority,
        words,
    }
}

/// Compute the dominant (most frequent) font size from a set of words.
fn dominant_font_size_of_words(words: &[PdfWord]) -> f32 {
    if words.is_empty() {
        return 0.0;
    }
    // Round font sizes to nearest 0.5pt for grouping
    let mut counts: Vec<(i32, usize)> = Vec::new();
    for w in words {
        let key = (w.font_size * 2.0).round() as i32;
        if let Some(entry) = counts.iter_mut().find(|(k, _)| *k == key) {
            entry.1 += 1;
        } else {
            counts.push((key, 1));
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts[0].0 as f32 / 2.0
}

/// Group lines into paragraphs based on vertical gaps, font size changes, and indentation.
fn lines_to_paragraphs(lines: Vec<PdfLine>) -> Vec<PdfParagraph> {
    if lines.is_empty() {
        return Vec::new();
    }

    if lines.len() == 1 {
        return vec![finalize_paragraph(lines)];
    }

    // Compute baseline line spacing for paragraph break detection.
    // We use the MINIMUM of filtered spacings:
    // - Filter out tiny gaps (< 40% of avg font size) to exclude superscripts/subscripts
    // - Use the minimum of remaining gaps as baseline line spacing
    // - Threshold = minimum * 1.5 catches paragraph-level gaps
    // We avoid the median (fails for memos where most lines are standalone paragraphs)
    // and raw minimum (fails when superscripts create tiny gaps).
    let avg_font_size = lines.iter().map(|l| l.dominant_font_size).sum::<f32>() / lines.len() as f32;

    let mut spacings: Vec<f32> = Vec::new();
    for pair in lines.windows(2) {
        let gap = (pair[1].baseline_y - pair[0].baseline_y).abs();
        // Filter out tiny gaps below 40% of avg font size (likely superscripts/artifacts)
        if gap > avg_font_size * 0.4 {
            spacings.push(gap);
        }
    }

    let base_spacing = if spacings.is_empty() {
        // Fallback: use average font size as spacing estimate
        avg_font_size
    } else {
        spacings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        // Use minimum of filtered spacings as line spacing baseline.
        // The 40% superscript filter above already removes tiny artifact gaps,
        // so the minimum here is the tightest real line spacing.
        spacings[0]
    };

    let paragraph_gap_threshold = base_spacing * PARAGRAPH_GAP_MULTIPLIER;

    let mut paragraphs: Vec<PdfParagraph> = Vec::new();
    let mut current_lines: Vec<PdfLine> = vec![lines[0].clone()];

    for line in lines.into_iter().skip(1) {
        let prev = current_lines.last().unwrap();

        let vertical_gap = (line.baseline_y - prev.baseline_y).abs();
        let font_size_change = (line.dominant_font_size - prev.dominant_font_size).abs();

        // Compute left indent change
        let prev_left = prev.words.first().map(|w| w.x_start).unwrap_or(0.0);
        let curr_left = line.words.first().map(|w| w.x_start).unwrap_or(0.0);
        let indent_change = (curr_left - prev_left).abs();

        // Paragraph break detection:
        // 1. Vertical gap alone exceeds threshold (primary signal)
        // 2. Font size change OR indent change, but ONLY when combined with at least
        //    some vertical gap (> base_spacing * 0.8). This prevents over-splitting
        //    when font size varies within tightly-spaced lines (e.g., slide decks,
        //    multi-column academic papers with footnotes/captions).
        let has_significant_gap = vertical_gap > paragraph_gap_threshold;
        let has_some_gap = vertical_gap > base_spacing * 0.8;
        let has_font_change = font_size_change > FONT_SIZE_CHANGE_THRESHOLD;
        let has_indent_change = indent_change > LEFT_INDENT_CHANGE_THRESHOLD;

        let is_paragraph_break = has_significant_gap || (has_some_gap && (has_font_change || has_indent_change));

        if is_paragraph_break {
            paragraphs.push(finalize_paragraph(current_lines));
            current_lines = vec![line];
        } else {
            current_lines.push(line);
        }
    }

    if !current_lines.is_empty() {
        paragraphs.push(finalize_paragraph(current_lines));
    }

    paragraphs
}

/// Build a PdfParagraph from a set of lines.
fn finalize_paragraph(lines: Vec<PdfLine>) -> PdfParagraph {
    let dominant_font_size = if lines.is_empty() {
        0.0
    } else {
        // Use the font size that appears in the most lines
        let mut fs_counts: Vec<(i32, usize)> = Vec::new();
        for l in &lines {
            let key = (l.dominant_font_size * 2.0).round() as i32;
            if let Some(entry) = fs_counts.iter_mut().find(|(k, _)| *k == key) {
                entry.1 += 1;
            } else {
                fs_counts.push((key, 1));
            }
        }
        fs_counts.sort_by(|a, b| b.1.cmp(&a.1));
        fs_counts[0].0 as f32 / 2.0
    };

    let bold_count = lines.iter().filter(|l| l.is_bold).count();
    let italic_count = lines.iter().filter(|l| l.is_italic).count();
    let majority = lines.len().div_ceil(2);

    // Detect list items: first word of first line starts with bullet or number prefix
    let is_list_item = lines.len() <= MAX_LIST_ITEM_LINES
        && lines
            .first()
            .and_then(|l| l.words.first())
            .map(|w| is_list_prefix(&w.text))
            .unwrap_or(false);

    PdfParagraph {
        dominant_font_size,
        heading_level: None, // Set during classification
        is_bold: bold_count >= majority,
        is_italic: italic_count >= majority,
        is_list_item,
        lines,
    }
}

/// Check if a word text looks like a list item prefix.
fn is_list_prefix(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed == "-" || trimmed == "*" || trimmed == "\u{2022}" {
        return true;
    }
    // Check for numbered list: "1." "2)" "10." etc.
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    // Find where digits end
    let digit_end = bytes.iter().position(|&b| !b.is_ascii_digit()).unwrap_or(bytes.len());
    if digit_end > 0 && digit_end < bytes.len() {
        let suffix = bytes[digit_end];
        return suffix == b'.' || suffix == b')';
    }
    false
}

/// Classify paragraphs as headings or body using the global heading map.
fn classify_paragraphs(paragraphs: &mut [PdfParagraph], heading_map: &[(f32, Option<u8>)]) {
    for para in paragraphs.iter_mut() {
        // Count total words in the paragraph
        let word_count: usize = para.lines.iter().map(|l| l.words.len()).sum();

        // Look up this paragraph's dominant font size in the heading map
        let heading_level = find_heading_level(para.dominant_font_size, heading_map);

        if let Some(level) = heading_level {
            // Only assign heading if the paragraph is short enough
            if word_count <= MAX_HEADING_WORD_COUNT {
                para.heading_level = Some(level);
            }
        }
    }
}

/// Find the heading level for a given font size by matching against the cluster centroids.
fn find_heading_level(font_size: f32, heading_map: &[(f32, Option<u8>)]) -> Option<u8> {
    if heading_map.is_empty() {
        return None;
    }
    if heading_map.len() == 1 {
        return heading_map[0].1;
    }

    // Find closest centroid
    let mut best_distance = f32::INFINITY;
    let mut best_level: Option<u8> = None;
    for &(centroid, level) in heading_map {
        let dist = (font_size - centroid).abs();
        if dist < best_distance {
            best_distance = dist;
            best_level = level;
        }
    }

    // Compute average inter-cluster gap
    let mut centroids: Vec<f32> = heading_map.iter().map(|(c, _)| *c).collect();
    centroids.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let gaps: Vec<f32> = centroids.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
    let avg_gap = if gaps.is_empty() {
        f32::INFINITY
    } else {
        gaps.iter().sum::<f32>() / gaps.len() as f32
    };

    // Reject matches that are too far from any centroid
    if best_distance > MAX_HEADING_DISTANCE_MULTIPLIER * avg_gap {
        return None;
    }

    best_level
}

/// Assemble final markdown string from classified paragraphs across all pages.
fn assemble_markdown(pages: Vec<Vec<PdfParagraph>>) -> String {
    let mut output = String::new();

    for (page_idx, paragraphs) in pages.iter().enumerate() {
        if page_idx > 0 && !output.is_empty() {
            output.push_str("\n\n");
        }

        for (para_idx, para) in paragraphs.iter().enumerate() {
            if para_idx > 0 {
                output.push_str("\n\n");
            }

            if let Some(level) = para.heading_level {
                // Heading: prefix with # symbols
                let prefix = "#".repeat(level as usize);
                let text = join_line_texts(&para.lines);
                output.push_str(&prefix);
                output.push(' ');
                output.push_str(&text);
            } else if para.is_list_item {
                // List items: preserve each line individually
                for (line_idx, line) in para.lines.iter().enumerate() {
                    if line_idx > 0 {
                        output.push('\n');
                    }
                    let text = render_line_with_inline_markup(line);
                    output.push_str(&text);
                }
            } else {
                // Body paragraph: join lines with space, apply inline markup
                let text = render_paragraph_with_inline_markup(para);
                output.push_str(&text);
            }
        }
    }

    output
}

/// Assemble markdown from paragraphs with inline table insertion.
///
/// Tables are inserted at their vertical position relative to surrounding paragraphs.
/// For tables without bounding boxes, they are appended at the end of their page.
fn assemble_markdown_with_tables(pages: Vec<Vec<PdfParagraph>>, tables: &[crate::types::Table]) -> String {
    if tables.is_empty() || tables.iter().all(|t| t.bounding_box.is_none()) {
        // No positioned tables, use simple assembly
        return assemble_markdown(pages);
    }

    let mut output = String::new();

    for (page_idx, paragraphs) in pages.iter().enumerate() {
        let page_number = page_idx + 1;

        if page_idx > 0 && !output.is_empty() {
            output.push_str("\n\n");
        }

        // Collect tables for this page, split by positioned and unpositioned
        let page_tables: Vec<&crate::types::Table> = tables.iter().filter(|t| t.page_number == page_number).collect();

        let positioned_tables: Vec<&crate::types::Table> = page_tables
            .iter()
            .filter(|t| t.bounding_box.is_some())
            .copied()
            .collect();

        let unpositioned_tables: Vec<&crate::types::Table> = page_tables
            .iter()
            .filter(|t| t.bounding_box.is_none())
            .copied()
            .collect();

        if positioned_tables.is_empty() {
            // No positioned tables on this page, render normally then append unpositioned
            for (para_idx, para) in paragraphs.iter().enumerate() {
                if para_idx > 0 {
                    output.push_str("\n\n");
                }
                render_paragraph_to_output(para, &mut output);
            }
            for table in &unpositioned_tables {
                output.push_str("\n\n");
                output.push_str(table.markdown.trim());
            }
        } else {
            // Build a unified list: paragraphs (with y-position) + tables (with y-position)
            // Sort by y-position descending (top of page first, since PDF y increases upward)
            enum PageItem<'a> {
                Paragraph(&'a PdfParagraph),
                Table(&'a crate::types::Table),
            }

            let mut items: Vec<(f32, PageItem)> = Vec::new();

            for para in paragraphs {
                // Use the first line's baseline_y as the paragraph's y position
                let y = para.lines.first().map(|l| l.baseline_y).unwrap_or(0.0);
                items.push((y, PageItem::Paragraph(para)));
            }

            for table in &positioned_tables {
                // Use the table's top y-coordinate (y1 in PDF coords = top of table)
                let y = table.bounding_box.as_ref().map(|b| b.y1 as f32).unwrap_or(0.0);
                items.push((y, PageItem::Table(table)));
            }

            // Sort by y descending (top-of-page first in PDF coords)
            items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

            let mut first = true;
            for (_, item) in &items {
                if !first {
                    output.push_str("\n\n");
                }
                first = false;
                match item {
                    PageItem::Paragraph(para) => render_paragraph_to_output(para, &mut output),
                    PageItem::Table(table) => output.push_str(table.markdown.trim()),
                }
            }

            // Append unpositioned tables at the end
            for table in &unpositioned_tables {
                output.push_str("\n\n");
                output.push_str(table.markdown.trim());
            }
        }
    }

    output
}

/// Render a single paragraph to the output string.
fn render_paragraph_to_output(para: &PdfParagraph, output: &mut String) {
    if let Some(level) = para.heading_level {
        let prefix = "#".repeat(level as usize);
        let text = join_line_texts(&para.lines);
        output.push_str(&prefix);
        output.push(' ');
        output.push_str(&text);
    } else if para.is_list_item {
        for (line_idx, line) in para.lines.iter().enumerate() {
            if line_idx > 0 {
                output.push('\n');
            }
            let text = render_line_with_inline_markup(line);
            output.push_str(&text);
        }
    } else {
        let text = render_paragraph_with_inline_markup(para);
        output.push_str(&text);
    }
}

/// Inject image placeholders into markdown based on page numbers.
///
/// Appends image placeholders at the end of the markdown, grouped by page number.
/// Each placeholder references the image by its `image_index` for stable identification.
/// If the image has OCR results, includes a blockquote with the OCR text.
pub fn inject_image_placeholders(markdown: &str, images: &[crate::types::ExtractedImage]) -> String {
    if images.is_empty() {
        return markdown.to_string();
    }

    // Group images by page number
    let mut images_by_page: std::collections::BTreeMap<usize, Vec<(usize, &crate::types::ExtractedImage)>> =
        std::collections::BTreeMap::new();
    for (idx, img) in images.iter().enumerate() {
        let page = img.page_number.unwrap_or(0);
        images_by_page.entry(page).or_default().push((idx, img));
    }

    // If no images have page numbers, append all at the end
    if images_by_page.keys().all(|&k| k == 0) {
        let mut result = markdown.to_string();
        for img in images {
            let ii = img.image_index;
            result.push_str(&format!("\n\n![Image {}](embedded:i{})", ii, ii));
            if let Some(ref ocr) = img.ocr_result {
                let text = ocr.content.trim();
                if !text.is_empty() {
                    result.push_str(&format!("\n> *Image text: {}*", text));
                }
            }
        }
        return result;
    }

    // Append image placeholders grouped by page, in page order
    let mut result = markdown.to_string();

    for (&page, page_images) in &images_by_page {
        for (_idx, img) in page_images {
            let ii = img.image_index;
            let label = if page > 0 {
                format!("![Image {} (page {})](embedded:p{}_i{})", ii, page, page, ii)
            } else {
                format!("![Image {}](embedded:i{})", ii, ii)
            };
            result.push_str("\n\n");
            result.push_str(&label);
            if let Some(ref ocr) = img.ocr_result {
                let text = ocr.content.trim();
                if !text.is_empty() {
                    result.push_str(&format!("\n> *Image text: {}*", text));
                }
            }
        }
    }

    result
}

/// Join lines into a single string (no inline markup).
/// Respects CJK spacing — no space inserted between adjacent CJK words.
fn join_line_texts(lines: &[PdfLine]) -> String {
    let all_words: Vec<&str> = lines
        .iter()
        .flat_map(|l| l.words.iter().map(|w| w.text.as_str()))
        .collect();
    join_words_cjk_aware(&all_words)
}

/// Join word texts with spaces, but omit the space when both adjacent words are CJK.
fn join_words_cjk_aware(words: &[&str]) -> String {
    if words.is_empty() {
        return String::new();
    }
    let mut result = String::from(words[0]);
    for pair in words.windows(2) {
        if needs_space_between(pair[0], pair[1]) {
            result.push(' ');
        }
        result.push_str(pair[1]);
    }
    result
}

/// Render a single line with bold/italic inline markup.
fn render_line_with_inline_markup(line: &PdfLine) -> String {
    render_words_with_markup(&line.words)
}

/// Render an entire body paragraph with inline bold/italic markup.
///
/// Lines are joined with a single space; consecutive bold or italic words
/// are grouped into a single `**...**` or `*...*` run.
fn render_paragraph_with_inline_markup(para: &PdfParagraph) -> String {
    // Collect all words across lines
    let all_words: Vec<&PdfWord> = para.lines.iter().flat_map(|l| l.words.iter()).collect();
    render_words_with_markup_refs(&all_words)
}

/// Render a slice of words with run-length-encoded bold/italic markup.
fn render_words_with_markup(words: &[PdfWord]) -> String {
    let refs: Vec<&PdfWord> = words.iter().collect();
    render_words_with_markup_refs(&refs)
}

/// Core inline markup renderer working on word references.
///
/// Groups consecutive words sharing the same bold/italic state, wraps groups
/// in `**...**` or `*...*` as appropriate. If an entire run is both bold and
/// italic, emits `***...***`.
fn render_words_with_markup_refs(words: &[&PdfWord]) -> String {
    if words.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut i = 0;

    while i < words.len() {
        let bold = words[i].is_bold;
        let italic = words[i].is_italic;

        // Find the run of words with the same formatting
        let run_start = i;
        while i < words.len() && words[i].is_bold == bold && words[i].is_italic == italic {
            i += 1;
        }

        let run_words: Vec<&str> = words[run_start..i].iter().map(|w| w.text.as_str()).collect();
        let run_text = join_words_cjk_aware(&run_words);

        if !result.is_empty() {
            // Determine if we need a space between the end of the previous run
            // and the start of this run
            let prev_end = words[run_start - 1].text.as_str();
            let next_start = words[run_start].text.as_str();
            if needs_space_between(prev_end, next_start) {
                result.push(' ');
            }
        }

        match (bold, italic) {
            (true, true) => {
                result.push_str("***");
                result.push_str(&run_text);
                result.push_str("***");
            }
            (true, false) => {
                result.push_str("**");
                result.push_str(&run_text);
                result.push_str("**");
            }
            (false, true) => {
                result.push('*');
                result.push_str(&run_text);
                result.push('*');
            }
            (false, false) => {
                result.push_str(&run_text);
            }
        }
    }

    result
}

/// Converts extracted blocks from the structure tree API into the local [PdfParagraph] type
/// used by the markdown assembly pipeline.
fn extracted_blocks_to_paragraphs(blocks: &[ExtractedBlock]) -> Vec<PdfParagraph> {
    let mut paragraphs = Vec::new();

    for block in blocks {
        // Recursively process children first (e.g., table cells, list items).
        if !block.children.is_empty() {
            paragraphs.extend(extracted_blocks_to_paragraphs(&block.children));
            continue;
        }

        if block.text.is_empty() {
            continue;
        }

        let heading_level = match &block.role {
            ContentRole::Heading { level } => Some(*level),
            _ => None,
        };

        let is_list_item = matches!(&block.role, ContentRole::ListItem { .. });

        // Build the full text, prepending list label if present.
        let full_text = if let ContentRole::ListItem { label: Some(ref l) } = block.role {
            format!("{} {}", l, block.text)
        } else {
            block.text.clone()
        };

        // Create a single-line paragraph from the block text.
        let font_size = block.font_size.unwrap_or(12.0);
        let words: Vec<PdfWord> = full_text
            .split_whitespace()
            .map(|w| PdfWord {
                text: w.to_string(),
                x_start: 0.0,
                x_end: 0.0,
                baseline_y: 0.0,
                font_size,
                is_bold: block.is_bold,
                is_italic: block.is_italic,
            })
            .collect();

        if words.is_empty() {
            continue;
        }

        let line = PdfLine {
            words,
            baseline_y: 0.0,
            y_top: 0.0,
            y_bottom: 0.0,
            dominant_font_size: font_size,
            is_bold: block.is_bold,
            is_italic: block.is_italic,
        };

        paragraphs.push(PdfParagraph {
            lines: vec![line],
            dominant_font_size: font_size,
            heading_level,
            is_bold: block.is_bold,
            is_italic: block.is_italic,
            is_list_item,
        });
    }

    paragraphs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_extracted_block(role: ContentRole, text: &str) -> ExtractedBlock {
        ExtractedBlock {
            role,
            text: text.to_string(),
            bounds: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }
    }

    #[test]
    fn test_extracted_blocks_to_paragraphs_heading() {
        let blocks = vec![make_extracted_block(ContentRole::Heading { level: 2 }, "Section Title")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert_eq!(paragraphs[0].heading_level, Some(2));
        assert!(!paragraphs[0].is_list_item);
    }

    #[test]
    fn test_extracted_blocks_to_paragraphs_body() {
        let blocks = vec![make_extracted_block(ContentRole::Paragraph, "Body text")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert_eq!(paragraphs[0].heading_level, None);
        assert!(!paragraphs[0].is_list_item);
        assert_eq!(paragraphs[0].lines[0].words.len(), 2);
    }

    #[test]
    fn test_extracted_blocks_to_paragraphs_list_item_with_label() {
        let blocks = vec![ExtractedBlock {
            role: ContentRole::ListItem {
                label: Some("1.".to_string()),
            },
            text: "First item".to_string(),
            bounds: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert!(paragraphs[0].is_list_item);
        // Label should be prepended: "1. First item" = 3 words
        let text: String = paragraphs[0].lines[0]
            .words
            .iter()
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(text, "1. First item");
    }

    #[test]
    fn test_extracted_blocks_to_paragraphs_empty_text() {
        let blocks = vec![make_extracted_block(ContentRole::Paragraph, "")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_extracted_blocks_to_paragraphs_children_processed() {
        let blocks = vec![ExtractedBlock {
            role: ContentRole::Other("Table".to_string()),
            text: String::new(),
            bounds: None,
            font_size: None,
            is_bold: false,
            is_italic: false,
            children: vec![
                make_extracted_block(ContentRole::Paragraph, "Cell 1"),
                make_extracted_block(ContentRole::Paragraph, "Cell 2"),
            ],
        }];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 2);
        let text1: String = paragraphs[0].lines[0]
            .words
            .iter()
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(text1, "Cell 1");
    }

    #[test]
    fn test_extracted_blocks_to_paragraphs_whitespace_only() {
        let blocks = vec![make_extracted_block(ContentRole::Paragraph, "   ")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        // split_whitespace on "   " returns empty iterator, so no words, paragraph skipped
        assert!(paragraphs.is_empty());
    }

    /// Helper to create a CharData with specified properties.
    fn make_char(text: &str, x: f32, baseline_y: f32, font_size: f32, is_bold: bool, is_italic: bool) -> CharData {
        CharData {
            text: text.to_string(),
            x,
            y: baseline_y + font_size * 0.2, // simulate y slightly above baseline
            font_size,
            width: font_size * 0.6,
            height: font_size,
            is_bold,
            is_italic,
            baseline_y,
        }
    }

    /// Helper to create a simple non-bold, non-italic char.
    fn plain_char(text: &str, x: f32, baseline_y: f32, font_size: f32) -> CharData {
        make_char(text, x, baseline_y, font_size, false, false)
    }

    #[test]
    fn test_chars_to_words() {
        // "Hi there" with a gap between "Hi" and "there"
        let fs = 12.0;
        let cw = fs * 0.6; // char width = 7.2

        let chars = vec![
            plain_char("H", 0.0, 100.0, fs),
            plain_char("i", cw, 100.0, fs),
            // gap > 0.3 * 12 = 3.6 between "i" end and "t" start
            plain_char("t", cw * 2.0 + 5.0, 100.0, fs),
            plain_char("h", cw * 3.0 + 5.0, 100.0, fs),
            plain_char("e", cw * 4.0 + 5.0, 100.0, fs),
            plain_char("r", cw * 5.0 + 5.0, 100.0, fs),
            plain_char("e", cw * 6.0 + 5.0, 100.0, fs),
        ];

        let words = chars_to_words(&chars);
        assert_eq!(words.len(), 2, "Expected 2 words, got {}", words.len());
        assert_eq!(words[0].text, "Hi");
        assert_eq!(words[1].text, "there");
    }

    #[test]
    fn test_words_to_lines() {
        // PDF coords: larger baseline_y = higher on page = comes first.
        // "Next" at baseline 115 is higher on page than "Hello world" at 100.
        let words = vec![
            PdfWord {
                text: "Hello".to_string(),
                x_start: 0.0,
                x_end: 30.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: false,
            },
            PdfWord {
                text: "world".to_string(),
                x_start: 35.0,
                x_end: 65.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: false,
            },
            PdfWord {
                text: "Next".to_string(),
                x_start: 0.0,
                x_end: 25.0,
                baseline_y: 115.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: false,
            },
        ];

        let lines = words_to_lines(words);
        assert_eq!(lines.len(), 2, "Expected 2 lines, got {}", lines.len());
        // "Next" (baseline 115) comes first in descending sort
        assert_eq!(lines[0].words.len(), 1);
        assert_eq!(lines[0].words[0].text, "Next");
        assert_eq!(lines[1].words.len(), 2);
        assert_eq!(lines[1].words[0].text, "Hello");
        assert_eq!(lines[1].words[1].text, "world");
    }

    #[test]
    fn test_lines_to_paragraphs() {
        // Helper to create a simple line at a given baseline
        fn make_line(text: &str, baseline: f32) -> PdfLine {
            PdfLine {
                words: vec![PdfWord {
                    text: text.to_string(),
                    x_start: 0.0,
                    x_end: 30.0,
                    baseline_y: baseline,
                    font_size: 12.0,
                    is_bold: false,
                    is_italic: false,
                }],
                baseline_y: baseline,
                y_top: baseline - 12.0,
                y_bottom: baseline,
                dominant_font_size: 12.0,
                is_bold: false,
                is_italic: false,
            }
        }

        // Three lines in paragraph 1 (14pt spacing each), then a large 50pt gap,
        // then one line in paragraph 2. Median spacing is 14pt, threshold = 21pt.
        let lines = vec![
            make_line("First", 100.0),
            make_line("second", 114.0),   // 14pt gap
            make_line("third", 128.0),    // 14pt gap
            make_line("New para", 178.0), // 50pt gap -> paragraph break
        ];

        let paragraphs = lines_to_paragraphs(lines);
        assert_eq!(paragraphs.len(), 2, "Expected 2 paragraphs, got {}", paragraphs.len());
        assert_eq!(paragraphs[0].lines.len(), 3);
        assert_eq!(paragraphs[1].lines.len(), 1);
    }

    #[test]
    fn test_heading_classification() {
        // Simulate clusters: 24pt (2 members), 12pt (20 members)
        // Body should be 12pt (most frequent). 24pt should be H1.
        let clusters = vec![
            crate::pdf::hierarchy::FontSizeCluster {
                centroid: 24.0,
                members: vec![
                    TextBlock {
                        text: "Title".to_string(),
                        bbox: BoundingBox {
                            left: 0.0,
                            top: 0.0,
                            right: 100.0,
                            bottom: 24.0,
                        },
                        font_size: 24.0,
                    },
                    TextBlock {
                        text: "Subtitle".to_string(),
                        bbox: BoundingBox {
                            left: 0.0,
                            top: 30.0,
                            right: 100.0,
                            bottom: 54.0,
                        },
                        font_size: 24.0,
                    },
                ],
            },
            crate::pdf::hierarchy::FontSizeCluster {
                centroid: 12.0,
                members: (0..20)
                    .map(|i| TextBlock {
                        text: format!("body {}", i),
                        bbox: BoundingBox {
                            left: 0.0,
                            top: 60.0 + i as f32 * 14.0,
                            right: 400.0,
                            bottom: 72.0 + i as f32 * 14.0,
                        },
                        font_size: 12.0,
                    })
                    .collect(),
            },
        ];

        let heading_map = assign_heading_levels_smart(&clusters);
        assert_eq!(heading_map.len(), 2);

        // 24pt cluster -> H1
        let h24 = heading_map.iter().find(|(c, _)| (*c - 24.0).abs() < 0.1);
        assert!(h24.is_some(), "Should find 24pt cluster");
        assert_eq!(h24.unwrap().1, Some(1), "24pt should be H1");

        // 12pt cluster -> Body (None)
        let h12 = heading_map.iter().find(|(c, _)| (*c - 12.0).abs() < 0.1);
        assert!(h12.is_some(), "Should find 12pt cluster");
        assert_eq!(h12.unwrap().1, None, "12pt should be Body");
    }

    #[test]
    fn test_single_font_size_no_headings() {
        // All same font size -> single cluster -> no headings
        let clusters = vec![crate::pdf::hierarchy::FontSizeCluster {
            centroid: 12.0,
            members: (0..10)
                .map(|i| TextBlock {
                    text: format!("text {}", i),
                    bbox: BoundingBox {
                        left: 0.0,
                        top: i as f32 * 14.0,
                        right: 100.0,
                        bottom: 12.0 + i as f32 * 14.0,
                    },
                    font_size: 12.0,
                })
                .collect(),
        }];

        let heading_map = assign_heading_levels_smart(&clusters);
        assert_eq!(heading_map.len(), 1);
        assert_eq!(heading_map[0].1, None, "Single cluster should be body");
    }

    #[test]
    fn test_inline_bold_markup() {
        let words = vec![
            PdfWord {
                text: "Hello".to_string(),
                x_start: 0.0,
                x_end: 30.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: false,
            },
            PdfWord {
                text: "bold".to_string(),
                x_start: 35.0,
                x_end: 55.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: true,
                is_italic: false,
            },
            PdfWord {
                text: "text".to_string(),
                x_start: 60.0,
                x_end: 80.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: true,
                is_italic: false,
            },
            PdfWord {
                text: "end".to_string(),
                x_start: 85.0,
                x_end: 105.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: false,
            },
        ];

        let result = render_words_with_markup(&words);
        assert_eq!(result, "Hello **bold text** end");
    }

    #[test]
    fn test_inline_italic_and_bold_italic_markup() {
        let words = vec![
            PdfWord {
                text: "normal".to_string(),
                x_start: 0.0,
                x_end: 30.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: false,
            },
            PdfWord {
                text: "italic".to_string(),
                x_start: 35.0,
                x_end: 65.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: false,
                is_italic: true,
            },
            PdfWord {
                text: "both".to_string(),
                x_start: 70.0,
                x_end: 90.0,
                baseline_y: 100.0,
                font_size: 12.0,
                is_bold: true,
                is_italic: true,
            },
        ];

        let result = render_words_with_markup(&words);
        assert_eq!(result, "normal *italic* ***both***");
    }

    #[test]
    fn test_markdown_assembly() {
        // Build synthetic paragraphs: one heading, one body
        let heading_para = PdfParagraph {
            lines: vec![PdfLine {
                words: vec![PdfWord {
                    text: "Introduction".to_string(),
                    x_start: 0.0,
                    x_end: 80.0,
                    baseline_y: 50.0,
                    font_size: 24.0,
                    is_bold: true,
                    is_italic: false,
                }],
                baseline_y: 50.0,
                y_top: 26.0,
                y_bottom: 50.0,
                dominant_font_size: 24.0,
                is_bold: true,
                is_italic: false,
            }],
            dominant_font_size: 24.0,
            heading_level: Some(1),
            is_bold: true,
            is_italic: false,
            is_list_item: false,
        };

        let body_para = PdfParagraph {
            lines: vec![
                PdfLine {
                    words: vec![
                        PdfWord {
                            text: "This".to_string(),
                            x_start: 0.0,
                            x_end: 25.0,
                            baseline_y: 80.0,
                            font_size: 12.0,
                            is_bold: false,
                            is_italic: false,
                        },
                        PdfWord {
                            text: "is".to_string(),
                            x_start: 30.0,
                            x_end: 40.0,
                            baseline_y: 80.0,
                            font_size: 12.0,
                            is_bold: false,
                            is_italic: false,
                        },
                    ],
                    baseline_y: 80.0,
                    y_top: 68.0,
                    y_bottom: 80.0,
                    dominant_font_size: 12.0,
                    is_bold: false,
                    is_italic: false,
                },
                PdfLine {
                    words: vec![PdfWord {
                        text: "body.".to_string(),
                        x_start: 0.0,
                        x_end: 30.0,
                        baseline_y: 94.0,
                        font_size: 12.0,
                        is_bold: false,
                        is_italic: false,
                    }],
                    baseline_y: 94.0,
                    y_top: 82.0,
                    y_bottom: 94.0,
                    dominant_font_size: 12.0,
                    is_bold: false,
                    is_italic: false,
                },
            ],
            dominant_font_size: 12.0,
            heading_level: None,
            is_bold: false,
            is_italic: false,
            is_list_item: false,
        };

        let markdown = assemble_markdown(vec![vec![heading_para, body_para]]);
        assert_eq!(markdown, "# Introduction\n\nThis is body.");
    }

    #[test]
    fn test_list_item_detection() {
        assert!(is_list_prefix("-"));
        assert!(is_list_prefix("*"));
        assert!(is_list_prefix("\u{2022}")); // bullet
        assert!(is_list_prefix("1."));
        assert!(is_list_prefix("10)"));
        assert!(!is_list_prefix("Hello"));
        assert!(!is_list_prefix(""));
    }

    #[test]
    fn test_empty_document() {
        let paragraphs: Vec<Vec<PdfParagraph>> = vec![vec![]];
        let markdown = assemble_markdown(paragraphs);
        assert_eq!(markdown, "");
    }

    #[test]
    fn test_chars_to_words_multiline() {
        // Characters on two different baselines should produce separate words.
        // PDF coords: larger baseline_y = higher on page = comes first in reading order.
        let fs = 12.0;
        let cw = fs * 0.6;
        let chars = vec![
            plain_char("A", 0.0, 100.0, fs),
            plain_char("B", cw, 100.0, fs),
            plain_char("C", 0.0, 120.0, fs), // higher baseline = higher on page
            plain_char("D", cw, 120.0, fs),
        ];

        let words = chars_to_words(&chars);
        assert_eq!(words.len(), 2, "Expected 2 words on different lines");
        // baseline 120 (higher on page) comes first in descending sort
        assert_eq!(words[0].text, "CD");
        assert_eq!(words[1].text, "AB");
    }

    #[test]
    fn test_body_is_most_frequent_cluster() {
        // Bug regression: 12pt body text (frequent) with 10pt captions (infrequent)
        // Body should be 12pt, not 10pt. 10pt should NOT be a heading.
        let clusters = vec![
            crate::pdf::hierarchy::FontSizeCluster {
                centroid: 12.0,
                members: (0..50)
                    .map(|i| TextBlock {
                        text: format!("body {}", i),
                        bbox: BoundingBox {
                            left: 0.0,
                            top: i as f32 * 14.0,
                            right: 400.0,
                            bottom: 12.0 + i as f32 * 14.0,
                        },
                        font_size: 12.0,
                    })
                    .collect(),
            },
            crate::pdf::hierarchy::FontSizeCluster {
                centroid: 10.0,
                members: (0..5)
                    .map(|i| TextBlock {
                        text: format!("caption {}", i),
                        bbox: BoundingBox {
                            left: 0.0,
                            top: 700.0 + i as f32 * 12.0,
                            right: 200.0,
                            bottom: 710.0 + i as f32 * 12.0,
                        },
                        font_size: 10.0,
                    })
                    .collect(),
            },
        ];

        let heading_map = assign_heading_levels_smart(&clusters);

        // 12pt (most frequent) should be body
        let h12 = heading_map.iter().find(|(c, _)| (*c - 12.0).abs() < 0.1);
        assert_eq!(h12.unwrap().1, None, "12pt (most frequent) should be body");

        // 10pt should also be body (smaller than body, not a heading)
        let h10 = heading_map.iter().find(|(c, _)| (*c - 10.0).abs() < 0.1);
        assert_eq!(h10.unwrap().1, None, "10pt (smaller than body) should NOT be a heading");
    }

    #[test]
    fn test_detect_columns_single_column() {
        let chars: Vec<CharData> = (0..20)
            .map(|i| CharData {
                text: "x".to_string(),
                x: i as f32 * 20.0,
                y: 500.0,
                font_size: 12.0,
                width: 7.0,
                height: 12.0,
                is_bold: false,
                is_italic: false,
                baseline_y: 500.0,
            })
            .collect();
        let columns = detect_columns(&chars, 400.0, 800.0);
        assert_eq!(columns.len(), 1);
    }

    #[test]
    fn test_detect_columns_two_columns() {
        let mut chars: Vec<CharData> = Vec::new();
        for row in 0..30 {
            let y = 700.0 - row as f32 * 20.0;
            for col in 0..10 {
                chars.push(CharData {
                    text: "a".to_string(),
                    x: 10.0 + col as f32 * 18.0,
                    y,
                    font_size: 12.0,
                    width: 7.0,
                    height: 12.0,
                    is_bold: false,
                    is_italic: false,
                    baseline_y: y,
                });
            }
            for col in 0..10 {
                chars.push(CharData {
                    text: "b".to_string(),
                    x: 300.0 + col as f32 * 18.0,
                    y,
                    font_size: 12.0,
                    width: 7.0,
                    height: 12.0,
                    is_bold: false,
                    is_italic: false,
                    baseline_y: y,
                });
            }
        }
        let columns = detect_columns(&chars, 500.0, 800.0);
        assert!(
            columns.len() >= 2,
            "Should detect at least 2 columns, got {}",
            columns.len()
        );
    }

    #[test]
    fn test_detect_columns_empty() {
        let columns = detect_columns(&[], 400.0, 800.0);
        assert_eq!(columns.len(), 1);
    }

    #[test]
    fn test_find_heading_level_outlier_rejected() {
        let heading_map = vec![(24.0, Some(1)), (12.0, None)];
        assert_eq!(find_heading_level(100.0, &heading_map), None);
    }

    #[test]
    fn test_find_heading_level_close_match() {
        let heading_map = vec![(24.0, Some(1)), (12.0, None)];
        assert_eq!(find_heading_level(23.5, &heading_map), Some(1));
    }

    #[test]
    fn test_is_cjk_char() {
        assert!(is_cjk_char('中')); // CJK Unified Ideograph
        assert!(is_cjk_char('あ')); // Hiragana
        assert!(is_cjk_char('ア')); // Katakana
        assert!(is_cjk_char('한')); // Hangul
        assert!(!is_cjk_char('A')); // Latin
        assert!(!is_cjk_char('1')); // Digit
        assert!(!is_cjk_char(' ')); // Space
    }

    #[test]
    fn test_chars_to_words_cjk_boundary() {
        // CJK characters should each become their own word
        let fs = 12.0;
        let cw = fs * 0.6;
        let chars = vec![
            CharData {
                text: "中".to_string(), // CJK
                x: 0.0,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
            CharData {
                text: "文".to_string(), // CJK
                x: cw,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
            CharData {
                text: "字".to_string(), // CJK
                x: cw * 2.0,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
        ];

        let words = chars_to_words(&chars);
        assert_eq!(words.len(), 3, "Expected 3 CJK words, each character separate");
        assert_eq!(words[0].text, "中");
        assert_eq!(words[1].text, "文");
        assert_eq!(words[2].text, "字");
    }

    #[test]
    fn test_chars_to_words_cjk_latin_mixing() {
        // CJK and Latin should break at boundaries
        let fs = 12.0;
        let cw = fs * 0.6;
        let chars = vec![
            CharData {
                text: "A".to_string(),
                x: 0.0,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
            CharData {
                text: "B".to_string(),
                x: cw,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
            CharData {
                text: "中".to_string(), // CJK boundary break
                x: cw * 2.0,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
            CharData {
                text: "C".to_string(), // Another boundary break
                x: cw * 3.0,
                y: 100.0,
                font_size: fs,
                width: cw,
                height: fs,
                is_bold: false,
                is_italic: false,
                baseline_y: 100.0,
            },
        ];

        let words = chars_to_words(&chars);
        assert_eq!(words.len(), 3, "Expected 3 words (AB, 中, C)");
        assert_eq!(words[0].text, "AB", "Latin characters should stay together");
        assert_eq!(words[1].text, "中", "CJK character should be separate");
        assert_eq!(words[2].text, "C", "Latin after CJK should be separate");
    }

    #[test]
    fn test_needs_space_between() {
        // CJK-CJK: no space
        assert!(!needs_space_between("中", "文"));
        assert!(!needs_space_between("あ", "い"));
        // Latin-Latin: space
        assert!(needs_space_between("hello", "world"));
        // CJK-Latin: space (CJK ends, Latin starts)
        assert!(needs_space_between("中", "hello"));
        // Latin-CJK: space (Latin ends, CJK starts)
        assert!(needs_space_between("hello", "中"));
    }

    #[test]
    fn test_join_words_cjk_aware() {
        // CJK words should be joined without spaces
        assert_eq!(join_words_cjk_aware(&["中", "文", "字"]), "中文字");
        // Latin words should be joined with spaces
        assert_eq!(join_words_cjk_aware(&["hello", "world"]), "hello world");
        // Mixed: CJK block then Latin
        assert_eq!(join_words_cjk_aware(&["中", "文", "test"]), "中文 test");
        // Mixed: Latin then CJK
        assert_eq!(join_words_cjk_aware(&["test", "中", "文"]), "test 中文");
        // Single word
        assert_eq!(join_words_cjk_aware(&["hello"]), "hello");
        // Empty
        assert_eq!(join_words_cjk_aware(&[]), "");
    }

    #[test]
    fn test_inject_image_placeholders_empty() {
        let md = "# Hello\n\nSome text.";
        let result = inject_image_placeholders(md, &[]);
        assert_eq!(result, md);
    }

    #[test]
    fn test_inject_image_placeholders_uses_image_index() {
        use bytes::Bytes;
        use std::borrow::Cow;

        let md = "# Page 1 content";
        let images = vec![crate::types::ExtractedImage {
            data: Bytes::from_static(&[0xFF]),
            format: Cow::Borrowed("jpeg"),
            image_index: 5,
            page_number: Some(1),
            width: None,
            height: None,
            colorspace: None,
            bits_per_component: None,
            is_mask: false,
            description: None,
            ocr_result: None,
            bounding_box: None,
        }];
        let result = inject_image_placeholders(md, &images);
        // Should use image_index (5), not enumeration index (0)
        assert!(
            result.contains("embedded:p1_i5"),
            "Should use image_index 5, got: {}",
            result
        );
        assert!(
            result.contains("![Image 5 (page 1)]"),
            "Should use image_index 5, got: {}",
            result
        );
    }

    #[test]
    fn test_inject_image_placeholders_with_ocr() {
        use bytes::Bytes;
        use std::borrow::Cow;

        let md = "Content here";
        let images = vec![crate::types::ExtractedImage {
            data: Bytes::from_static(&[0x89]),
            format: Cow::Borrowed("png"),
            image_index: 0,
            page_number: None,
            width: None,
            height: None,
            colorspace: None,
            bits_per_component: None,
            is_mask: false,
            description: None,
            ocr_result: Some(Box::new(crate::types::ExtractionResult {
                content: "OCR detected text".to_string(),
                mime_type: Cow::Borrowed("text/plain"),
                metadata: crate::types::Metadata::default(),
                tables: vec![],
                detected_languages: None,
                chunks: None,
                images: None,
                djot_content: None,
                pages: None,
                elements: None,
                ocr_elements: None,
                document: None,
                #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
                extracted_keywords: None,
                quality_score: None,
                processing_warnings: Vec::new(),
            })),
            bounding_box: None,
        }];
        let result = inject_image_placeholders(md, &images);
        assert!(
            result.contains("Image text: OCR detected text"),
            "Should include OCR text, got: {}",
            result
        );
    }

    #[test]
    fn test_assemble_markdown_with_tables_no_tables() {
        let paragraphs = vec![vec![PdfParagraph {
            lines: vec![PdfLine {
                words: vec![PdfWord {
                    text: "Hello".to_string(),
                    x_start: 0.0,
                    x_end: 30.0,
                    baseline_y: 700.0,
                    font_size: 12.0,
                    is_bold: false,
                    is_italic: false,
                }],
                baseline_y: 700.0,
                y_top: 688.0,
                y_bottom: 700.0,
                dominant_font_size: 12.0,
                is_bold: false,
                is_italic: false,
            }],
            dominant_font_size: 12.0,
            heading_level: None,
            is_bold: false,
            is_italic: false,
            is_list_item: false,
        }]];
        let result = assemble_markdown_with_tables(paragraphs, &[]);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_assemble_markdown_with_tables_interleaves() {
        // Two paragraphs and a table between them (by y-position)
        let paragraphs = vec![vec![
            PdfParagraph {
                lines: vec![PdfLine {
                    words: vec![PdfWord {
                        text: "Top".to_string(),
                        x_start: 0.0,
                        x_end: 30.0,
                        baseline_y: 700.0,
                        font_size: 12.0,
                        is_bold: false,
                        is_italic: false,
                    }],
                    baseline_y: 700.0,
                    y_top: 688.0,
                    y_bottom: 700.0,
                    dominant_font_size: 12.0,
                    is_bold: false,
                    is_italic: false,
                }],
                dominant_font_size: 12.0,
                heading_level: None,
                is_bold: false,
                is_italic: false,
                is_list_item: false,
            },
            PdfParagraph {
                lines: vec![PdfLine {
                    words: vec![PdfWord {
                        text: "Bottom".to_string(),
                        x_start: 0.0,
                        x_end: 50.0,
                        baseline_y: 200.0,
                        font_size: 12.0,
                        is_bold: false,
                        is_italic: false,
                    }],
                    baseline_y: 200.0,
                    y_top: 188.0,
                    y_bottom: 200.0,
                    dominant_font_size: 12.0,
                    is_bold: false,
                    is_italic: false,
                }],
                dominant_font_size: 12.0,
                heading_level: None,
                is_bold: false,
                is_italic: false,
                is_list_item: false,
            },
        ]];

        let tables = vec![crate::types::Table {
            cells: vec![vec!["A".to_string(), "B".to_string()]],
            markdown: "| A | B |".to_string(),
            page_number: 1,
            bounding_box: Some(crate::types::BoundingBox {
                x0: 50.0,
                y0: 400.0,
                x1: 500.0,
                y1: 500.0, // y1=500 is between Top(700) and Bottom(200)
            }),
        }];

        let result = assemble_markdown_with_tables(paragraphs, &tables);
        // Order should be: Top, Table, Bottom
        let top_pos = result.find("Top").unwrap();
        let table_pos = result.find("| A | B |").unwrap();
        let bottom_pos = result.find("Bottom").unwrap();
        assert!(top_pos < table_pos, "Top should come before table");
        assert!(table_pos < bottom_pos, "Table should come before Bottom");
    }

    // ---- Segment-based extraction tests ----

    /// Helper to create a SegmentData with specified properties.
    fn make_segment(
        text: &str,
        x: f32,
        baseline_y: f32,
        width: f32,
        font_size: f32,
        is_bold: bool,
        is_italic: bool,
    ) -> SegmentData {
        SegmentData {
            text: text.to_string(),
            x,
            y: baseline_y,
            width,
            height: font_size,
            font_size,
            is_bold,
            is_italic,
            baseline_y,
        }
    }

    fn plain_segment(text: &str, x: f32, baseline_y: f32, width: f32, font_size: f32) -> SegmentData {
        make_segment(text, x, baseline_y, width, font_size, false, false)
    }

    #[test]
    fn test_segments_to_words_single_word() {
        let segments = vec![plain_segment("Hello", 0.0, 100.0, 50.0, 12.0)];
        let words = segments_to_words(&segments);
        assert_eq!(words.len(), 1);
        assert_eq!(words[0].text, "Hello");
    }

    #[test]
    fn test_segments_to_words_multiple_words_in_segment() {
        // A single segment containing multiple space-separated words
        let segments = vec![plain_segment("Hello World Foo", 0.0, 100.0, 150.0, 12.0)];
        let words = segments_to_words(&segments);
        assert_eq!(words.len(), 3);
        assert_eq!(words[0].text, "Hello");
        assert_eq!(words[1].text, "World");
        assert_eq!(words[2].text, "Foo");
    }

    #[test]
    fn test_segments_to_words_preserves_bold_italic() {
        let segments = vec![make_segment("Bold text", 0.0, 100.0, 100.0, 12.0, true, false)];
        let words = segments_to_words(&segments);
        assert_eq!(words.len(), 2);
        assert!(words[0].is_bold);
        assert!(!words[0].is_italic);
        assert!(words[1].is_bold);
    }

    #[test]
    fn test_segments_to_words_multiple_segments() {
        let segments = vec![
            plain_segment("First", 0.0, 100.0, 50.0, 12.0),
            plain_segment("Second", 60.0, 100.0, 60.0, 12.0),
        ];
        let words = segments_to_words(&segments);
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text, "First");
        assert_eq!(words[1].text, "Second");
    }

    #[test]
    fn test_segments_to_words_cjk_splitting() {
        // CJK characters should each be their own word
        let segments = vec![plain_segment("\u{4e16}\u{754c}", 0.0, 100.0, 24.0, 12.0)];
        let words = segments_to_words(&segments);
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text, "\u{4e16}");
        assert_eq!(words[1].text, "\u{754c}");
    }

    #[test]
    fn test_segments_to_words_empty_segments() {
        let segments: Vec<SegmentData> = vec![];
        let words = segments_to_words(&segments);
        assert!(words.is_empty());
    }

    #[test]
    fn test_segments_to_words_whitespace_only() {
        let segments = vec![plain_segment("   ", 0.0, 100.0, 30.0, 12.0)];
        let words = segments_to_words(&segments);
        assert!(words.is_empty());
    }

    #[test]
    fn test_segments_to_words_proportional_x_positions() {
        // Two words of equal length should split the x-range equally
        let segments = vec![plain_segment("AB CD", 10.0, 100.0, 100.0, 12.0)];
        let words = segments_to_words(&segments);
        assert_eq!(words.len(), 2);
        // "AB" is 2 chars, "CD" is 2 chars => equal split
        assert!((words[0].x_start - 10.0).abs() < 0.01);
        assert!((words[1].x_start - 60.0).abs() < 0.01); // 10 + 100*0.5
    }

    #[test]
    fn test_segments_to_words_different_baselines_become_separate_lines() {
        // Segments on different baselines should produce words that words_to_lines groups separately
        let segments = vec![
            plain_segment("Line one", 0.0, 100.0, 80.0, 12.0),
            plain_segment("Line two", 0.0, 85.0, 80.0, 12.0),
        ];
        let words = segments_to_words(&segments);
        let lines = words_to_lines(words);
        assert_eq!(lines.len(), 2);
    }
}
