//! Utilities for splitting and analyzing PDF paragraphs.

use super::types::PdfParagraph;

/// Merge consecutive body-text paragraphs that are continuations of the same logical paragraph.
///
/// Two consecutive paragraphs are merged if:
/// - Both are body text (no heading_level, not is_list_item)
/// - The first paragraph doesn't end with sentence-ending punctuation
/// - Font sizes are within 2pt of each other
pub(super) fn merge_continuation_paragraphs(paragraphs: &mut Vec<PdfParagraph>) {
    if paragraphs.len() < 2 {
        return;
    }

    // O(N) single-pass merge: drain the original vec and rebuild, avoiding
    // the O(N²) cost of repeated Vec::remove shifts.
    let old = std::mem::take(paragraphs);
    let mut iter = old.into_iter();
    // SAFETY: we returned early above when paragraphs.len() < 2, so `old`
    // contains at least two elements and the first next() always succeeds.
    let mut current = iter.next().unwrap();

    for next in iter {
        let should_merge =
            // Both must be body text (no heading, list, code, or formula)
            current.heading_level.is_none()
                && next.heading_level.is_none()
                && !current.is_list_item
                && !next.is_list_item
                && !current.is_code_block
                && !next.is_code_block
                && !current.is_formula
                && !next.is_formula
                // Font sizes close enough
                && (current.dominant_font_size - next.dominant_font_size).abs() < 2.0
                // Current paragraph doesn't end with sentence-ending punctuation
                && !ends_with_sentence_terminator(&current);

        if should_merge {
            current.lines.extend(next.lines);
        } else {
            paragraphs.push(current);
            current = next;
        }
    }

    paragraphs.push(current);
}

/// Check if a paragraph's last line ends with sentence-terminating punctuation.
///
/// Used by merge_continuation_paragraphs to determine if two consecutive
/// paragraphs should be merged. Supports ASCII and CJK sentence terminators.
fn ends_with_sentence_terminator(para: &PdfParagraph) -> bool {
    let last_text = para
        .lines
        .last()
        .and_then(|l| l.segments.last())
        .map(|s| s.text.trim_end())
        .unwrap_or("");
    matches!(
        last_text.chars().last(),
        Some('.' | '?' | '!' | ':' | ';' | '\u{3002}' | '\u{FF1F}' | '\u{FF01}')
    )
}

/// Split paragraphs that contain embedded bullet characters (e.g. `•`) into separate list items.
///
/// Structure tree pages sometimes merge all text into one block with inline bullets.
/// This splits "text before • item1 • item2" into separate paragraphs.
pub(super) fn split_embedded_list_items(paragraphs: &mut Vec<PdfParagraph>) {
    let old = std::mem::take(paragraphs);
    for para in old {
        // Only split non-heading, non-list, non-code paragraphs
        if para.heading_level.is_some() || para.is_list_item || para.is_code_block || para.is_formula {
            paragraphs.push(para);
            continue;
        }

        // Collect full text to check for embedded bullets
        let full_text: String = para
            .lines
            .iter()
            .flat_map(|l| l.segments.iter())
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        // Count bullet occurrences — only split if there are multiple
        let bullet_count = full_text.matches('\u{2022}').count();
        if bullet_count < 2 {
            paragraphs.push(para);
            continue;
        }

        // Split on bullet character boundaries
        let font_size = para.dominant_font_size;
        let is_bold = para.is_bold;

        // Split the full text on • and produce separate paragraphs
        let parts: Vec<&str> = full_text.split('\u{2022}').collect();
        let before = parts[0].trim();
        if !before.is_empty() {
            paragraphs.push(text_to_paragraph(before, font_size, is_bold, false));
        }
        for part in &parts[1..] {
            let item_text = part.trim();
            if !item_text.is_empty() {
                paragraphs.push(text_to_paragraph(item_text, font_size, is_bold, true));
            }
        }
    }
}

/// Create a simple paragraph from text.
fn text_to_paragraph(text: &str, font_size: f32, is_bold: bool, is_list_item: bool) -> PdfParagraph {
    use crate::pdf::hierarchy::SegmentData;

    let segments: Vec<SegmentData> = text
        .split_whitespace()
        .map(|w| SegmentData {
            text: w.to_string(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            font_size,
            is_bold,
            is_italic: false,
            is_monospace: false,
            baseline_y: 0.0,
        })
        .collect();

    let line = super::types::PdfLine {
        segments,
        baseline_y: 0.0,
        dominant_font_size: font_size,
        is_bold,
        is_monospace: false,
    };

    PdfParagraph {
        text: String::new(),
        lines: vec![line],
        dominant_font_size: font_size,
        heading_level: None,
        is_bold,
        is_list_item,
        is_code_block: false,
        is_formula: false,
        is_page_furniture: false,
        layout_class: None,
        caption_for: None,
        block_bbox: None,
    }
}

/// Check if text looks like a list item prefix.
pub(super) fn is_list_prefix(text: &str) -> bool {
    let trimmed = text.trim();
    // Bullet characters: hyphen, asterisk, bullet, en dash, em dash, triangular bullet, white bullet
    if matches!(
        trimmed,
        "-" | "*"
            | "\u{2022}"
            | "\u{2013}"
            | "\u{2014}"
            | "\u{2023}"
            | "\u{25E6}"
            | "\u{25AA}"
            | "\u{25CF}"
            | "\u{2043}"
            | "\u{27A2}"
    ) {
        return true;
    }
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    // Numbered: 1. 2) 3: etc.
    let digit_end = bytes.iter().position(|&b| !b.is_ascii_digit()).unwrap_or(bytes.len());
    if digit_end > 0 && digit_end < bytes.len() {
        let suffix = bytes[digit_end];
        if suffix == b'.' || suffix == b')' || suffix == b':' {
            return true;
        }
    }
    // Parenthesized numbers/letters: (1) (a) (i) (A)
    // Use char-based slicing to avoid panicking on multi-byte UTF-8 boundaries.
    if bytes.len() >= 3 && bytes[0] == b'(' && bytes[bytes.len() - 1] == b')' {
        let char_count = trimmed.chars().count();
        if char_count >= 3 {
            let inner: String = trimmed.chars().skip(1).take(char_count - 2).collect();
            if inner.chars().all(|c| c.is_ascii_digit())
                || (inner.len() == 1 && inner.chars().next().is_some_and(|c| c.is_ascii_alphabetic()))
                || is_roman_numeral(&inner)
            {
                return true;
            }
        }
    }
    // Bracketed numbers/letters: [1] [a]
    // Use char-based slicing to avoid panicking on multi-byte UTF-8 boundaries.
    if bytes.len() >= 3 && bytes[0] == b'[' && bytes[bytes.len() - 1] == b']' {
        let char_count = trimmed.chars().count();
        if char_count >= 3 {
            let inner: String = trimmed.chars().skip(1).take(char_count - 2).collect();
            if inner.chars().all(|c| c.is_ascii_digit())
                || (inner.len() == 1 && inner.chars().next().is_some_and(|c| c.is_ascii_alphabetic()))
            {
                return true;
            }
        }
    }
    // Alphabetic: a. b) A. B) (single letter + period/paren)
    if bytes.len() == 2 && bytes[0].is_ascii_alphabetic() && (bytes[1] == b'.' || bytes[1] == b')') {
        return true;
    }
    // Roman numerals: i. ii. iii. iv. v. vi. I. II. III. IV. V. VI. etc.
    if trimmed.ends_with('.') || trimmed.ends_with(')') {
        let prefix = &trimmed[..trimmed.len() - 1];
        if is_roman_numeral(prefix) {
            return true;
        }
    }
    false
}

/// Check if text is a roman numeral (i-xii or I-XII range).
fn is_roman_numeral(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let lower = s.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "i" | "ii" | "iii" | "iv" | "v" | "vi" | "vii" | "viii" | "ix" | "x" | "xi" | "xii"
    )
}
