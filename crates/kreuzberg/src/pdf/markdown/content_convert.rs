//! Convert [`PageContent`] elements into [`PdfParagraph`]s for the markdown pipeline.
//!
//! This is the shared conversion layer: all extraction backends produce
//! `PageContent` via adapters, then this module converts elements into
//! the `PdfParagraph` representation used by heading classification,
//! layout overrides, and markdown rendering.

use super::constants::MAX_HEADING_WORD_COUNT;
use super::content::{ContentElement, ElementLevel, PageContent, SemanticRole};
use super::types::{PdfLine, PdfParagraph};
use crate::pdf::hierarchy::SegmentData;

/// Convert a page's content elements into paragraphs.
///
/// Each `ContentElement` becomes one `PdfParagraph` with a single line.
/// Semantic roles from the extraction source (structure tree tags, layout
/// model predictions) are mapped to heading levels, list items, code
/// blocks, etc.
pub(super) fn content_to_paragraphs(page: &PageContent) -> Vec<PdfParagraph> {
    let mut paragraphs = Vec::with_capacity(page.elements.len());

    for elem in &page.elements {
        if let Some(para) = element_to_paragraph(elem) {
            paragraphs.push(para);
        }
    }

    paragraphs
}

/// Convert a single `ContentElement` into a `PdfParagraph`.
///
/// Returns `None` for empty elements.
fn element_to_paragraph(elem: &ContentElement) -> Option<PdfParagraph> {
    // Build the full text, prepending list label if present.
    let full_text = if let Some(ref label) = elem.list_label {
        format!("{} {}", label, elem.text)
    } else {
        elem.text.clone()
    };

    let word_count = full_text.split_whitespace().count();
    if word_count == 0 {
        return None;
    }

    let font_size = elem.font_size.unwrap_or(12.0);

    // Determine structural properties from semantic role.
    let mut is_list_item = matches!(elem.semantic_role, Some(SemanticRole::ListItem));
    let is_code_block = matches!(elem.semantic_role, Some(SemanticRole::Code));
    let is_formula = matches!(elem.semantic_role, Some(SemanticRole::Other));
    let is_monospace = elem.is_monospace || is_code_block;

    // Detect list items from text content when not tagged.
    if !is_list_item {
        let first_word = full_text.split_whitespace().next().unwrap_or("");
        is_list_item = super::paragraphs::is_list_prefix(first_word);
    }

    // Map heading level from semantic role, with word-count guard.
    let heading_level = match elem.semantic_role {
        Some(SemanticRole::Heading { level }) if word_count <= MAX_HEADING_WORD_COUNT => Some(level),
        _ => None,
    };

    // Extract block_bbox as (left, bottom, right, top) tuple for PdfParagraph.
    let block_bbox = elem.bbox.map(|r| (r.left, r.y_min, r.right, r.y_max));

    // Create word-level segments (zero positions — spatial matching uses block_bbox).
    let segments: Vec<SegmentData> = if elem.level == ElementLevel::Line || elem.level == ElementLevel::Block {
        // Block/line-level elements: split into word segments.
        full_text
            .split_whitespace()
            .map(|w| SegmentData {
                text: w.to_string(),
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                font_size,
                is_bold: elem.is_bold,
                is_italic: elem.is_italic,
                is_monospace,
                baseline_y: 0.0,
            })
            .collect()
    } else {
        // Word-level elements: single segment.
        vec![SegmentData {
            text: full_text.clone(),
            x: elem.bbox.map_or(0.0, |r| r.left),
            y: elem.bbox.map_or(0.0, |r| r.y_min),
            width: elem.bbox.map_or(0.0, |r| r.width()),
            height: elem.bbox.map_or(0.0, |r| r.height()),
            font_size,
            is_bold: elem.is_bold,
            is_italic: elem.is_italic,
            is_monospace,
            baseline_y: elem.bbox.map_or(0.0, |r| r.y_min),
        }]
    };

    let line = PdfLine {
        segments,
        baseline_y: 0.0,
        dominant_font_size: font_size,
        is_bold: elem.is_bold,
        is_monospace,
    };

    Some(PdfParagraph {
        lines: vec![line],
        dominant_font_size: font_size,
        heading_level,
        is_bold: elem.is_bold,
        is_list_item,
        is_code_block,
        is_formula,
        is_page_furniture: false,
        layout_class: None,
        caption_for: None,
        block_bbox,
    })
}

#[cfg(test)]
mod tests {
    use super::super::content::ExtractionSource;
    use super::super::geometry::Rect;
    use super::*;

    fn make_element(text: &str, role: Option<SemanticRole>) -> ContentElement {
        ContentElement {
            text: text.to_string(),
            bbox: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            is_monospace: false,
            confidence: None,
            semantic_role: role,
            level: ElementLevel::Block,
            list_label: None,
        }
    }

    fn make_page(elements: Vec<ContentElement>) -> PageContent {
        PageContent {
            page_number: 1,
            page_width: 612.0,
            page_height: 792.0,
            elements,
            source: ExtractionSource::StructureTree,
        }
    }

    #[test]
    fn test_heading_conversion() {
        let page = make_page(vec![
            make_element("Title Text", Some(SemanticRole::Heading { level: 1 })),
            make_element("Body text", Some(SemanticRole::Paragraph)),
        ]);
        let paras = content_to_paragraphs(&page);
        assert_eq!(paras.len(), 2);
        assert_eq!(paras[0].heading_level, Some(1));
        assert_eq!(paras[1].heading_level, None);
    }

    #[test]
    fn test_heading_too_many_words_demoted() {
        let long_heading = (0..25).map(|i| format!("word{i}")).collect::<Vec<_>>().join(" ");
        let page = make_page(vec![make_element(
            &long_heading,
            Some(SemanticRole::Heading { level: 2 }),
        )]);
        let paras = content_to_paragraphs(&page);
        assert_eq!(paras[0].heading_level, None);
    }

    #[test]
    fn test_list_item_from_role() {
        let mut elem = make_element("First item", Some(SemanticRole::ListItem));
        elem.list_label = Some("1.".to_string());
        let page = make_page(vec![elem]);
        let paras = content_to_paragraphs(&page);
        assert!(paras[0].is_list_item);
        assert_eq!(paras[0].lines[0].segments[0].text, "1.");
    }

    #[test]
    fn test_list_item_from_text_prefix() {
        let page = make_page(vec![make_element("• Bullet point", Some(SemanticRole::Paragraph))]);
        let paras = content_to_paragraphs(&page);
        assert!(paras[0].is_list_item);
    }

    #[test]
    fn test_code_block() {
        let page = make_page(vec![make_element("fn main() {}", Some(SemanticRole::Code))]);
        let paras = content_to_paragraphs(&page);
        assert!(paras[0].is_code_block);
    }

    #[test]
    fn test_empty_skipped() {
        let page = make_page(vec![
            make_element("", Some(SemanticRole::Paragraph)),
            make_element("   ", Some(SemanticRole::Paragraph)),
            make_element("Real text", Some(SemanticRole::Paragraph)),
        ]);
        let paras = content_to_paragraphs(&page);
        assert_eq!(paras.len(), 1);
        assert_eq!(paras[0].lines[0].segments[0].text, "Real");
    }

    #[test]
    fn test_block_bbox_propagated() {
        let mut elem = make_element("With bounds", Some(SemanticRole::Paragraph));
        elem.bbox = Some(Rect::from_lbrt(50.0, 100.0, 400.0, 120.0));
        let page = make_page(vec![elem]);
        let paras = content_to_paragraphs(&page);
        let bbox = paras[0].block_bbox.unwrap();
        assert!((bbox.0 - 50.0).abs() < f32::EPSILON);
        assert!((bbox.1 - 100.0).abs() < f32::EPSILON);
    }
}
