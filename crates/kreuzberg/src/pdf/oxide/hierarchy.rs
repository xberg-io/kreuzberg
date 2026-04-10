//! Font metrics extraction for heading hierarchy detection using the pdf_oxide backend.
//!
//! Uses pdf_oxide's span extraction to get font_size, font_weight, is_italic,
//! and font_name, converting them to `SegmentData` for the backend-agnostic
//! clustering pipeline that assigns heading levels (H1-H6) to text blocks.
//!
//! When the PDF is a tagged PDF with a reliable structure tree, heading roles
//! (H1-H6) are read directly from the tree and assigned via `SegmentData::assigned_role`,
//! bypassing font-size clustering entirely for more accurate heading detection.

use std::collections::HashMap;

use super::OxideDocument;
use crate::pdf::error::Result;
use crate::pdf::hierarchy::SegmentData;

/// Extract text segments with font metrics from a PDF page using pdf_oxide.
///
/// Returns `SegmentData` objects containing text, position, and font metadata
/// (size, bold, italic, monospace). These feed into the existing backend-agnostic
/// font size clustering pipeline for heading detection.
///
/// Uses default (top-to-bottom) reading order rather than column-aware ordering,
/// because the hierarchy/structure pipeline depends on physical span position for
/// font-size clustering and heading detection. Column-aware reordering changes
/// span sequence in ways that break single-column heading detection.
///
/// # Arguments
///
/// * `doc` - Mutable reference to the oxide document
/// * `page_index` - Zero-based page index
///
/// # Returns
///
/// Vector of `SegmentData` objects with font metrics for hierarchy detection.
pub(crate) fn extract_segments_from_page(doc: &mut OxideDocument, page_index: usize) -> Result<Vec<SegmentData>> {
    extract_segments_from_page_inner(doc, page_index, &HashMap::new())
}

/// Inner implementation of per-page segment extraction.
///
/// When `mcid_roles` is non-empty, spans with matching MCIDs receive pre-assigned
/// heading levels from the PDF structure tree.
fn extract_segments_from_page_inner(
    doc: &mut OxideDocument,
    page_index: usize,
    mcid_roles: &HashMap<u32, Option<u8>>,
) -> Result<Vec<SegmentData>> {
    // Get page height for coordinate conversion
    let page_height = doc
        .doc
        .get_page_media_box(page_index)
        .ok()
        .map(|(_, lly, _, ury)| (ury - lly).abs())
        .unwrap_or(792.0); // Letter size fallback

    let spans = match doc.doc.extract_spans(page_index) {
        Ok(spans) => spans,
        Err(e) => {
            tracing::debug!(page = page_index, "pdf_oxide extract_spans failed for hierarchy: {e}");
            return Ok(Vec::new());
        }
    };

    let segments: Vec<SegmentData> = spans
        .into_iter()
        .filter(|span| {
            // Skip page furniture (headers/footers/watermarks)
            if span.artifact_type.is_some() {
                return false;
            }
            !span.text.trim().is_empty()
        })
        .map(|span| {
            let is_bold = span.font_weight == pdf_oxide::layout::text_block::FontWeight::Bold;
            let bbox = &span.bbox;

            // Convert from screen coords (y=0 at top) to PDF coords (y=0 at bottom)
            let screen_bottom = bbox.y + bbox.height;
            let pdf_baseline_y = page_height - screen_bottom;
            let pdf_y = page_height - bbox.y - bbox.height;

            // Look up structure-tree heading role via MCID
            let assigned_role = span.mcid.and_then(|mcid| mcid_roles.get(&mcid).copied()).flatten();

            SegmentData {
                text: span.text,
                x: bbox.x,
                y: pdf_y,
                width: bbox.width,
                height: bbox.height,
                font_size: span.font_size,
                is_bold,
                is_italic: span.is_italic,
                is_monospace: span.is_monospace,
                baseline_y: pdf_baseline_y,
                assigned_role,
            }
        })
        .collect();

    Ok(segments)
}

/// Try to extract segments using the PDF structure tree for heading detection.
///
/// Checks `MarkInfo` to see if the structure tree is reliable (marked && !suspects),
/// then traverses the tree to build MCID → heading-level mappings per page.
/// Spans are then extracted normally but annotated with `assigned_role` from the tree.
///
/// Returns `(segments, used_structure_tree)`. When `used_structure_tree` is true,
/// the caller should skip font-size clustering and use the pre-assigned roles.
fn extract_segments_with_structure_tree(doc: &mut OxideDocument) -> Result<(Vec<Vec<SegmentData>>, bool)> {
    // Check MarkInfo — cheap, no tree parsing required
    let mark_info = match doc.doc.mark_info() {
        Ok(mi) => mi,
        Err(e) => {
            tracing::debug!("pdf_oxide: mark_info() failed, skipping structure tree: {e}");
            return Ok((Vec::new(), false));
        }
    };

    if !mark_info.is_structure_reliable() {
        tracing::debug!(
            marked = mark_info.marked,
            suspects = mark_info.suspects,
            "pdf_oxide: structure tree not reliable, falling back to font-size clustering"
        );
        return Ok((Vec::new(), false));
    }

    // Parse the structure tree
    let struct_tree = match doc.doc.structure_tree() {
        Ok(Some(tree)) => tree,
        Ok(None) => {
            tracing::debug!("pdf_oxide: no structure tree found despite marked=true");
            return Ok((Vec::new(), false));
        }
        Err(e) => {
            tracing::debug!("pdf_oxide: structure_tree() failed: {e}");
            return Ok((Vec::new(), false));
        }
    };

    // Traverse the tree once for all pages
    let all_page_content = pdf_oxide::structure::traverse_structure_tree_all_pages(&struct_tree);

    // Count heading elements across all pages — require meaningful coverage
    // to avoid trusting a tree that only has 1-2 tagged headings but misses
    // most section headers (common in partially-tagged PDFs).
    let heading_count: usize = all_page_content
        .values()
        .flat_map(|contents| contents.iter())
        .filter(|c| c.parsed_type.heading_level().is_some())
        .count();

    if heading_count < 3 {
        tracing::debug!(
            heading_count,
            "pdf_oxide: structure tree has too few heading elements (< 3), falling back to font-size clustering"
        );
        return Ok((Vec::new(), false));
    }

    // Build per-page MCID → heading-level maps
    let page_count = doc.doc.page_count().map_err(|e| {
        crate::pdf::error::PdfError::TextExtractionFailed(format!("pdf_oxide: failed to get page count: {e}"))
    })?;

    let mut all_pages: Vec<Vec<SegmentData>> = Vec::with_capacity(page_count);
    let mut total_role_assigned = 0usize;

    for page_idx in 0..page_count {
        // Build MCID → role map for this page
        let mcid_roles: HashMap<u32, Option<u8>> = all_page_content
            .get(&(page_idx as u32))
            .map(|contents| {
                contents
                    .iter()
                    .filter_map(|c| c.mcid.map(|mcid| (mcid, c.parsed_type.heading_level())))
                    .collect()
            })
            .unwrap_or_default();

        let segments = extract_segments_from_page_inner(doc, page_idx, &mcid_roles)?;
        total_role_assigned += segments.iter().filter(|s| s.assigned_role.is_some()).count();
        all_pages.push(segments);
    }

    tracing::debug!(
        page_count,
        total_role_assigned,
        "pdf_oxide: structure tree heading detection complete"
    );

    Ok((all_pages, true))
}

/// Extract text segments from all pages of a PDF document using pdf_oxide.
///
/// Attempts structure tree extraction first for tagged PDFs. Falls back to
/// plain font-metric extraction when the structure tree is unavailable or
/// unreliable.
///
/// Returns `(segments, used_structure_tree)` where the flag indicates whether
/// heading roles were pre-assigned from the structure tree.
///
/// # Arguments
///
/// * `doc` - Mutable reference to the oxide document
///
/// # Returns
///
/// Tuple of (per-page segment vectors, structure-tree-used flag).
pub(crate) fn extract_all_segments(doc: &mut OxideDocument) -> Result<(Vec<Vec<SegmentData>>, bool)> {
    // Try structure tree first
    let (tree_segments, used_tree) = extract_segments_with_structure_tree(doc)?;
    if used_tree && !tree_segments.is_empty() {
        return Ok((tree_segments, true));
    }

    // Fallback: plain font-metric extraction
    let page_count = doc.doc.page_count().map_err(|e| {
        crate::pdf::error::PdfError::TextExtractionFailed(format!("pdf_oxide: failed to get page count: {e}"))
    })?;

    let mut all_pages: Vec<Vec<SegmentData>> = Vec::with_capacity(page_count);

    for page_idx in 0..page_count {
        let segments = extract_segments_from_page(doc, page_idx)?;
        all_pages.push(segments);
    }

    Ok((all_pages, false))
}
