//! Layout-detection-based paragraph classification overrides.
//!
//! When layout detection is enabled, this module applies layout hints
//! to override or augment the font-size-based paragraph classification
//! from the standard markdown pipeline.

use super::types::{LayoutHint, LayoutHintClass, PdfParagraph};

/// Apply layout detection overrides to classified paragraphs.
///
/// Uses two matching strategies:
/// 1. **Spatial matching** (heuristic pages): computes bounding boxes from segment
///    positions and matches by containment overlap.
/// 2. **Proportional matching** (structure tree pages): paragraphs without positional
///    data are matched to hints by estimated vertical position, since both are in
///    reading order.
///
/// Structure-tree headings are preserved: only paragraphs without existing
/// heading classification receive heading overrides from layout detection.
pub(super) fn apply_layout_overrides(
    paragraphs: &mut [PdfParagraph],
    hints: &[LayoutHint],
    min_confidence: f32,
    min_containment: f32,
) {
    if hints.is_empty() {
        return;
    }

    // Separate paragraphs into those with and without positional data.
    let has_any_positions = paragraphs.iter().any(|p| compute_paragraph_bbox(p).is_some());

    if has_any_positions {
        // Spatial matching for paragraphs with positional data
        apply_spatial_overrides(paragraphs, hints, min_confidence, min_containment);
    } else {
        // Proportional matching for structure tree pages (no positional data)
        apply_proportional_overrides(paragraphs, hints, min_confidence);
    }
}

/// Spatial matching: match paragraphs to hints by bounding box overlap.
///
/// Uses a two-tier strategy:
/// 1. **2D containment** (intersection_area / paragraph_area): best for paragraphs
///    that horizontally overlap with the layout hint.
/// 2. **Vertical-only overlap** (vertical_intersection / paragraph_height): fallback
///    for paragraphs where horizontal alignment differs (e.g., centered text vs
///    left-aligned detection box).
///
/// The vertical fallback requires higher confidence to reduce false positives.
fn apply_spatial_overrides(
    paragraphs: &mut [PdfParagraph],
    hints: &[LayoutHint],
    min_confidence: f32,
    min_containment: f32,
) {
    let confident_hints: Vec<&LayoutHint> = hints.iter().filter(|h| h.confidence >= min_confidence).collect();

    for para in paragraphs.iter_mut() {
        let para_bbox = match compute_paragraph_bbox(para) {
            Some(bbox) => bbox,
            None => continue,
        };

        let para_height = para_bbox.top - para_bbox.bottom;
        if para_height <= 0.0 {
            continue;
        }

        // Try 2D containment first (most precise).
        let best_2d = confident_hints
            .iter()
            .filter_map(|hint| {
                let containment = hint_containment(hint, &para_bbox);
                if containment >= min_containment {
                    Some((*hint, containment))
                } else {
                    None
                }
            })
            .max_by(|a, b| a.1.total_cmp(&b.1));

        if let Some((hint, _)) = best_2d {
            apply_hint_to_paragraph(para, hint);
        }
    }
}

/// Proportional matching: match paragraphs to hints using range-overlap.
///
/// Structure tree paragraphs have no positional data but are in reading order
/// (top-to-bottom). Layout hints have PDF coordinates with known bounding boxes.
///
/// Strategy:
/// 1. Sort hints by vertical position (top-to-bottom in reading order).
/// 2. Each paragraph occupies a fractional range `[i/n, (i+1)/n]` of the page.
/// 3. Each hint occupies a fractional range `[(page_height - top)/page_height, (page_height - bottom)/page_height]`.
/// 4. Match each paragraph to the hint with the most fractional overlap.
///
/// This is more accurate than point-estimate matching because it accounts for
/// hints that span large vertical ranges (e.g., a code block or table covering
/// half the page).
fn apply_proportional_overrides(paragraphs: &mut [PdfParagraph], hints: &[LayoutHint], min_confidence: f32) {
    let n = paragraphs.len();
    if n == 0 {
        return;
    }

    // Filter hints by confidence.
    let confident_hints: Vec<&LayoutHint> = hints.iter().filter(|h| h.confidence >= min_confidence).collect();
    if confident_hints.is_empty() {
        return;
    }

    // Infer page height from hint coordinates (max top value).
    let page_height = hints.iter().map(|h| h.top).fold(0.0_f32, f32::max);
    if page_height <= 0.0 {
        return;
    }

    tracing::debug!(
        paragraph_count = n,
        hint_count = confident_hints.len(),
        page_height,
        "Proportional matching: structure tree paragraphs without positions"
    );

    // Precompute each hint's fractional range on the page.
    // In PDF coords, y=0 is bottom, y=page_height is top.
    // Reading order: top-to-bottom → fraction 0.0 = top of page, 1.0 = bottom.
    let hint_ranges: Vec<(f32, f32, &LayoutHint)> = confident_hints
        .iter()
        .map(|h| {
            let frac_start = (page_height - h.top) / page_height; // top of hint → lower fraction
            let frac_end = (page_height - h.bottom) / page_height; // bottom of hint → higher fraction
            (frac_start.max(0.0), frac_end.min(1.0), *h)
        })
        .collect();

    for (i, para) in paragraphs.iter_mut().enumerate() {
        // This paragraph occupies fractional range [i/n, (i+1)/n]
        let para_start = i as f32 / n as f32;
        let para_end = (i as f32 + 1.0) / n as f32;

        // Find the hint with the most overlap.
        let best = hint_ranges
            .iter()
            .filter_map(|&(h_start, h_end, hint)| {
                let overlap_start = para_start.max(h_start);
                let overlap_end = para_end.min(h_end);
                let overlap = (overlap_end - overlap_start).max(0.0);
                if overlap > 0.0 { Some((hint, overlap)) } else { None }
            })
            .max_by(|a, b| a.1.total_cmp(&b.1));

        if let Some((hint, overlap)) = best {
            tracing::trace!(
                para_idx = i,
                total_paragraphs = n,
                ?hint.class,
                hint_confidence = hint.confidence,
                overlap,
                para_frac = format_args!("[{:.2}, {:.2}]", para_start, para_end),
                "Proportional match candidate"
            );
            let para_span = para_end - para_start;
            let overlap_frac = if para_span > 0.0 { overlap / para_span } else { 0.0 };

            match hint.class {
                // Furniture: reliably at page extremes, lower overlap threshold
                LayoutHintClass::PageHeader if i == 0 && overlap_frac > 0.25 => {
                    tracing::trace!(para_idx = i, ?hint.class, "Applying furniture override");
                    apply_hint_to_paragraph(para, hint);
                }
                LayoutHintClass::PageFooter if i == n - 1 && overlap_frac > 0.25 => {
                    tracing::trace!(para_idx = i, ?hint.class, "Applying furniture override");
                    apply_hint_to_paragraph(para, hint);
                }
                // Headings: apply layout model heading detection to struct tree
                // paragraphs that don't already have a heading from the tree.
                // Requires high overlap and word count guard.
                LayoutHintClass::SectionHeader | LayoutHintClass::Title
                    if para.heading_level.is_none()
                        && !para.is_list_item
                        && !para.is_code_block
                        && overlap_frac > 0.7 =>
                {
                    let word_count: usize = para
                        .lines
                        .iter()
                        .flat_map(|l| l.segments.iter())
                        .map(|s| s.text.split_whitespace().count())
                        .sum();
                    if word_count <= 12 {
                        let text: String = para
                            .lines
                            .iter()
                            .flat_map(|l| l.segments.iter())
                            .map(|s| s.text.as_str())
                            .collect::<Vec<_>>()
                            .join(" ");
                        if !is_separator_text(&text) {
                            let level = infer_heading_level_from_text(&text, hint.class);
                            tracing::trace!(
                                para_idx = i,
                                ?hint.class,
                                level,
                                word_count,
                                overlap_frac,
                                "Applying heading override from layout model"
                            );
                            para.heading_level = Some(level);
                            para.layout_class = Some(hint.class);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Check if text is a separator/filler line (dashes, underscores, tildes, etc.)
/// that should never be classified as a heading.
pub(super) fn is_separator_text(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    let total = trimmed.chars().count();
    let alnum = trimmed.chars().filter(|c| c.is_alphanumeric()).count();
    // Pure separator: no alphanumeric characters at all
    if alnum == 0 {
        return true;
    }
    // Mostly separator: very few alphanumeric chars among filler (dashes, underscores, tildes, etc.)
    // e.g. "------------- M W _ _ _ _ _ _" or "---~ ---------"
    // Require at least 6 total chars and <15% alphanumeric ratio
    total >= 6 && (alnum as f64 / total as f64) < 0.15
}

/// Infer heading level from section numbering in the text.
///
/// Academic papers use numbering to indicate heading depth:
/// - "1 Introduction" → H2 (top-level section)
/// - "3.2 AI models" → H3 (sub-section)
/// - "3.2.1 Details" → H4 (sub-sub-section)
/// - "Layout Analysis Model" (no number) → H2 (default for SectionHeader)
pub(super) fn infer_heading_level_from_text(text: &str, hint_class: LayoutHintClass) -> u8 {
    if hint_class == LayoutHintClass::Title {
        return 1;
    }

    let trimmed = text.trim();
    // Check for section numbering pattern: digits and dots at the start
    let numbering_end = trimmed.find(|c: char| !c.is_ascii_digit() && c != '.').unwrap_or(0);

    if numbering_end == 0 {
        // No numbering → default H2 for SectionHeader
        return 2;
    }

    let numbering = &trimmed[..numbering_end];
    // Count dots to determine depth: "3" → 0 dots → H2, "3.2" → 1 dot → H3
    let dot_count = numbering.chars().filter(|&c| c == '.').count();

    // Trailing dot (e.g., "3.") doesn't count as depth indicator
    let effective_dots = if numbering.ends_with('.') {
        dot_count.saturating_sub(1)
    } else {
        dot_count
    };

    match effective_dots {
        0 => 2, // "1 Introduction" → H2
        1 => 3, // "3.2 AI models" → H3
        _ => 4, // "3.2.1 Details" → H4
    }
}

/// Apply a single hint's classification to a paragraph.
pub(super) fn apply_hint_to_paragraph(para: &mut PdfParagraph, hint: &LayoutHint) {
    para.layout_class = Some(hint.class);

    let para_text: String = para
        .lines
        .iter()
        .flat_map(|l| l.segments.iter())
        .map(|s| s.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let is_sep = is_separator_text(&para_text);

    match hint.class {
        LayoutHintClass::Title => {
            if para.heading_level.is_none() && !is_sep {
                para.heading_level = Some(1);
            }
        }
        LayoutHintClass::SectionHeader => {
            if para.heading_level.is_none() && !is_sep {
                para.heading_level = Some(2);
            }
        }
        LayoutHintClass::Code => {
            para.is_code_block = true;
            para.heading_level = None;
        }
        LayoutHintClass::Formula => {
            para.is_formula = true;
            para.heading_level = None;
        }
        LayoutHintClass::ListItem => {
            para.is_list_item = true;
        }
        LayoutHintClass::PageHeader | LayoutHintClass::PageFooter => {
            para.is_page_furniture = true;
        }
        _ => {}
    }
}

/// Simple bounding box for a paragraph in PDF coordinate space.
struct ParaBBox {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

/// Compute a paragraph's bounding box from its line segments' positional data.
///
/// Returns `None` if the paragraph has no segments with valid positional data.
///
/// In PDF coordinates (y=0 at bottom, y increases upward):
/// - `seg.y` / `seg.baseline_y` is the text baseline (near the bottom of glyphs).
/// - Text extends UPWARD from the baseline by roughly the ascent (~80% of font size).
/// - Text extends DOWNWARD from the baseline by the descent (~20% of font size).
///
/// For layout detection matching, we approximate the visual text extent as:
/// - top = baseline + height (covers ascenders)
/// - bottom = baseline (descent is small and usually within the layout hint's margin)
fn compute_paragraph_bbox(para: &PdfParagraph) -> Option<ParaBBox> {
    let mut left = f32::MAX;
    let mut right = f32::MIN;
    let mut bottom = f32::MAX;
    let mut top = f32::MIN;
    let mut has_data = false;

    for line in &para.lines {
        for seg in &line.segments {
            // Skip segments with no positional data (structure tree path)
            if seg.x == 0.0 && seg.width == 0.0 && seg.y == 0.0 && seg.height == 0.0 {
                continue;
            }
            has_data = true;
            left = left.min(seg.x);
            right = right.max(seg.x + seg.width);
            // seg.y is the baseline. Text extends upward by ~font_size (seg.height).
            top = top.max(seg.y + seg.height);
            bottom = bottom.min(seg.y);
        }
    }

    if has_data {
        Some(ParaBBox {
            left,
            bottom,
            right,
            top,
        })
    } else {
        None
    }
}

/// Compute what fraction of the paragraph bbox is contained within the hint bbox.
///
/// Both are in PDF coordinate space (points, y=0 at bottom).
fn hint_containment(hint: &LayoutHint, para: &ParaBBox) -> f32 {
    let para_area = (para.right - para.left) * (para.top - para.bottom);
    if para_area <= 0.0 {
        return 0.0;
    }

    // Intersection
    let ix1 = hint.left.max(para.left);
    let iy1 = hint.bottom.max(para.bottom);
    let ix2 = hint.right.min(para.right);
    let iy2 = hint.top.min(para.top);

    let inter_area = (ix2 - ix1).max(0.0) * (iy2 - iy1).max(0.0);
    inter_area / para_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::hierarchy::SegmentData;
    use crate::pdf::markdown::types::PdfLine;

    fn make_segment(text: &str, x: f32, y: f32, width: f32, height: f32) -> SegmentData {
        SegmentData {
            text: text.to_string(),
            x,
            y,
            width,
            height,
            font_size: 12.0,
            is_bold: false,
            is_italic: false,
            is_monospace: false,
            baseline_y: y,
        }
    }

    fn make_line_at(segments: Vec<SegmentData>, baseline_y: f32) -> PdfLine {
        PdfLine {
            segments,
            baseline_y,
            dominant_font_size: 12.0,
            is_bold: false,
            is_monospace: false,
        }
    }

    fn make_line(segments: Vec<SegmentData>) -> PdfLine {
        make_line_at(segments, 700.0)
    }

    fn make_para(x: f32, y: f32, width: f32, height: f32) -> PdfParagraph {
        PdfParagraph {
            lines: vec![make_line(vec![make_segment("text", x, y, width, height)])],
            dominant_font_size: 12.0,
            heading_level: None,
            is_bold: false,
            is_list_item: false,
            is_code_block: false,
            is_formula: false,
            is_page_furniture: false,
            layout_class: None,
            caption_for: None,
        }
    }

    fn make_hint(class: LayoutHintClass, confidence: f32, left: f32, bottom: f32, right: f32, top: f32) -> LayoutHint {
        LayoutHint {
            class,
            confidence,
            left,
            bottom,
            right,
            top,
        }
    }

    // ── apply_layout_overrides tests (paragraph-level, used for struct tree path) ──

    #[test]
    fn test_title_override() {
        let mut paragraphs = vec![make_para(50.0, 750.0, 500.0, 20.0)];
        let hints = vec![make_hint(LayoutHintClass::Title, 0.9, 40.0, 745.0, 560.0, 775.0)];
        apply_layout_overrides(&mut paragraphs, &hints, 0.5, 0.5);
        assert_eq!(paragraphs[0].heading_level, Some(1));
        assert_eq!(paragraphs[0].layout_class, Some(LayoutHintClass::Title));
    }

    #[test]
    fn test_section_header_override() {
        let mut paragraphs = vec![make_para(50.0, 600.0, 300.0, 16.0)];
        let hints = vec![make_hint(
            LayoutHintClass::SectionHeader,
            0.85,
            40.0,
            598.0,
            400.0,
            620.0,
        )];
        apply_layout_overrides(&mut paragraphs, &hints, 0.5, 0.5);
        assert_eq!(paragraphs[0].heading_level, Some(2));
    }

    #[test]
    fn test_low_confidence_ignored() {
        let mut paragraphs = vec![make_para(50.0, 750.0, 500.0, 20.0)];
        let hints = vec![make_hint(LayoutHintClass::Title, 0.3, 40.0, 745.0, 560.0, 775.0)];
        apply_layout_overrides(&mut paragraphs, &hints, 0.5, 0.5);
        assert_eq!(paragraphs[0].heading_level, None);
        assert_eq!(paragraphs[0].layout_class, None);
    }

    #[test]
    fn test_existing_heading_preserved() {
        let mut paragraphs = vec![make_para(50.0, 750.0, 500.0, 20.0)];
        paragraphs[0].heading_level = Some(3);
        let hints = vec![make_hint(
            LayoutHintClass::SectionHeader,
            0.9,
            40.0,
            745.0,
            560.0,
            775.0,
        )];
        apply_layout_overrides(&mut paragraphs, &hints, 0.5, 0.5);
        assert_eq!(paragraphs[0].heading_level, Some(3));
    }

    #[test]
    fn test_empty_hints() {
        let mut paragraphs = vec![make_para(50.0, 750.0, 500.0, 20.0)];
        apply_layout_overrides(&mut paragraphs, &[], 0.5, 0.5);
        assert_eq!(paragraphs[0].heading_level, None);
    }

    #[test]
    fn test_hint_containment_full() {
        let hint = make_hint(LayoutHintClass::Text, 0.9, 0.0, 0.0, 612.0, 792.0);
        let para = ParaBBox {
            left: 50.0,
            bottom: 100.0,
            right: 550.0,
            top: 200.0,
        };
        let containment = hint_containment(&hint, &para);
        assert!(
            (containment - 1.0).abs() < 0.01,
            "Full containment expected: {}",
            containment
        );
    }

    #[test]
    fn test_hint_containment_none() {
        let hint = make_hint(LayoutHintClass::Text, 0.9, 0.0, 500.0, 100.0, 600.0);
        let para = ParaBBox {
            left: 200.0,
            bottom: 100.0,
            right: 500.0,
            top: 200.0,
        };
        let containment = hint_containment(&hint, &para);
        assert!(
            (containment - 0.0).abs() < 0.01,
            "No containment expected: {}",
            containment
        );
    }

    // ── infer_heading_level_from_text tests ──

    #[test]
    fn test_infer_heading_level_title() {
        assert_eq!(
            infer_heading_level_from_text("Docling Report", LayoutHintClass::Title),
            1
        );
    }

    #[test]
    fn test_infer_heading_level_top_section() {
        // "3 Processing pipeline" → H2
        assert_eq!(
            infer_heading_level_from_text("3 Processing pipeline", LayoutHintClass::SectionHeader),
            2
        );
    }

    #[test]
    fn test_infer_heading_level_subsection() {
        // "3.2 AI models" → H3
        assert_eq!(
            infer_heading_level_from_text("3.2 AI models", LayoutHintClass::SectionHeader),
            3
        );
    }

    #[test]
    fn test_infer_heading_level_subsubsection() {
        // "3.2.1 Details" → H4
        assert_eq!(
            infer_heading_level_from_text("3.2.1 Details", LayoutHintClass::SectionHeader),
            4
        );
    }

    #[test]
    fn test_infer_heading_level_trailing_dot() {
        // "3. Processing" → trailing dot, still H2
        assert_eq!(
            infer_heading_level_from_text("3. Processing", LayoutHintClass::SectionHeader),
            2
        );
    }

    #[test]
    fn test_infer_heading_level_no_number() {
        // "Layout Analysis Model" → no number, default H2
        assert_eq!(
            infer_heading_level_from_text("Layout Analysis Model", LayoutHintClass::SectionHeader),
            2
        );
    }

    // ── proportional matching tests (structure tree path) ──

    #[test]
    fn test_no_positional_data_proportional_applies_page_furniture() {
        // Proportional matching only applies PageHeader/PageFooter (furniture)
        // because positional imprecision makes heading/list/code overrides unreliable.
        let mut paragraphs = vec![PdfParagraph {
            lines: vec![make_line(vec![make_segment("text", 0.0, 0.0, 0.0, 0.0)])],
            dominant_font_size: 12.0,
            heading_level: None,
            is_bold: false,
            is_list_item: false,
            is_code_block: false,
            is_formula: false,
            is_page_furniture: false,
            layout_class: None,
            caption_for: None,
        }];

        // Title hint IS applied via proportional matching (heading level inferred)
        let hints = vec![make_hint(LayoutHintClass::Title, 0.9, 40.0, 0.0, 560.0, 760.0)];
        apply_layout_overrides(&mut paragraphs, &hints, 0.5, 0.5);
        assert_eq!(paragraphs[0].heading_level, Some(1));
        assert_eq!(paragraphs[0].layout_class, Some(LayoutHintClass::Title));

        // Reset for next test
        paragraphs[0].heading_level = None;
        paragraphs[0].layout_class = None;

        // PageHeader hint SHOULD be applied via proportional matching
        let hints = vec![make_hint(LayoutHintClass::PageHeader, 0.9, 40.0, 0.0, 560.0, 760.0)];
        apply_layout_overrides(&mut paragraphs, &hints, 0.5, 0.5);
        assert!(paragraphs[0].is_page_furniture);
        assert_eq!(paragraphs[0].layout_class, Some(LayoutHintClass::PageHeader));
    }
}
