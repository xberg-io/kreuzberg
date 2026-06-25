//! PDF text utilities (backend-agnostic).
//!
//! Contains pure utility functions for text post-processing used by the
//! oxide extraction backend.

use std::borrow::Cow;

/// Replace PDF font encoding artifacts in extracted text, including ligature glyphs.
///
/// Some PDFs have broken ToUnicode mappings that produce control characters
/// (U+0001–U+001F) in place of ligature glyphs or other printable characters.
///
/// This function is deliberately conservative — it only decodes the one mapping the issue
/// (#1135) establishes with evidence, and drops the rest rather than guessing (which would
/// corrupt unrelated words):
/// 1. **Decode the well-evidenced ligature** – U+0003 (ETX) following a letter → `ft`
///    (e.g. `blij␃`→`blijft`, `So␃ware`→`Software`, `veiligheidsvoorschri␃en`→`…schriften`).
/// 2. **Drop other C0 controls** – U+0002 (ambiguous per the issue) and any other
///    U+0001–U+001F are decoding artifacts that are removed. Tab, newline, and carriage
///    return are preserved.
///
/// Returns `Cow::Borrowed` when no replacements are needed (zero-cost for clean text).
pub(crate) fn fix_pdf_control_chars(text: &str) -> Cow<'_, str> {
    // Quick scan: skip allocation if no problematic chars exist.
    if !text.bytes().any(|b| b < 0x20 && b != b'\t' && b != b'\n' && b != b'\r') {
        return Cow::Borrowed(text);
    }

    // First pass: decode known ligature control characters
    let ligature_decoded = decode_ligature_control_chars(text);

    // Second pass: repair remaining control characters
    let chars: Vec<char> = ligature_decoded.as_ref().chars().collect();
    let mut result = String::with_capacity(ligature_decoded.len());

    for (i, &ch) in chars.iter().enumerate() {
        if matches!(ch, '\u{0001}'..='\u{001F}') && ch != '\t' && ch != '\n' && ch != '\r' {
            // Residual control char (likely a ligature we couldn't decode).
            // Use contextual heuristic to guess the ligature.
            let replacement = heuristic_ligature_repair(ch, &chars, i);
            result.push_str(&replacement);
        } else {
            result.push(ch);
        }
    }

    if result == text {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(result)
    }
}

/// Decode control characters that are known to represent ligature glyphs.
///
/// U+0002 (STX) and U+0003 (ETX) commonly map to ligature glyphs in broken ToUnicode
/// CMaps. This function maps them to the most likely ligature decomposition based on
/// context. Uses `Cow` to avoid allocation when no ligatures are present.
fn decode_ligature_control_chars(text: &str) -> Cow<'_, str> {
    // Quick scan for the known ligature control characters.
    if !text.contains('\u{0002}') && !text.contains('\u{0003}') {
        return Cow::Borrowed(text);
    }

    let chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len());

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '\u{0002}' => {
                // STX (U+0002): per issue #1135 its ligature mapping is ambiguous —
                // it appears in non-"ft" words (e.g. "ingebruik␂name" is "ingebruikname",
                // "CV␂comfort" is not "CVftcomfort"). We cannot map it to a specific
                // ligature reliably, so drop it as a decoding artifact rather than inject
                // a guess that would corrupt the word. Tab/newline/CR are handled elsewhere.
            }
            '\u{0003}' => {
                // ETX (U+0003) represents a ligature glyph (most commonly "ft" per issue #1135).
                // When between alphabetic characters, decode to "ft" (the most common ligature).
                // At word end, also decode to "ft".
                let prev_is_alpha = i > 0 && chars[i - 1].is_alphabetic();

                if prev_is_alpha {
                    // After an alphabetic character: decode to "ft"
                    // (the most reliable default based on issue #1135 evidence).
                    result.push_str("ft");
                } else {
                    // Not in mid-word context; preserve the control char for the fallback heuristic.
                    result.push(ch);
                }
            }
            _ => result.push(ch),
        }
    }

    Cow::Owned(result)
}

/// Apply heuristic repair for control characters that may represent ligatures.
///
/// When a control character (C0) cannot be decoded directly, we check its position
/// and context to guess which ligature it might represent. This handles cases where
/// the control char sits between alphabetic characters (mid-word), suggesting it
/// represents a ligature like "ft", "fi", "ff", etc.
fn heuristic_ligature_repair(_ctrl: char, _chars: &[char], _idx: usize) -> String {
    // Residual C0 control characters are decoding artifacts we cannot map to a specific
    // ligature reliably. Drop them rather than inject a guessed ligature — guessing
    // corrupts unrelated words (e.g. "soft␁ware" must become "software", not "softftware";
    // dropping the control yields the correct word here). Only U+0003→"ft" (issue #1135's
    // well-evidenced case) is decoded, in `decode_ligature_control_chars`.
    String::new()
}

/// Check if text likely contains embedded HTML markup.
///
/// Some PDFs embed raw HTML in their text layer (e.g. from web-to-PDF converters).
/// This function detects common HTML tags to determine if the text should be
/// converted from HTML to markdown rather than used as-is.
pub(crate) fn contains_html_markup(text: &str) -> bool {
    if !text.contains('<') {
        return false;
    }
    text.contains("</p>")
        || text.contains("<br")
        || text.contains("<p>")
        || text.contains("<div")
        || text.contains("<span")
        || text.contains("<table")
        || text.contains("<a ")
        || text.contains("/>")
}

/// Convert HTML markup in page text to markdown using the HTML converter.
///
/// Falls back to the original text if the `html` feature is not enabled
/// or if conversion fails.
#[cfg(feature = "html")]
pub(crate) fn convert_html_page_text(text: &str) -> String {
    match crate::extraction::html::convert_html_to_markdown(text, None, None) {
        Ok(converted) => converted,
        Err(_) => text.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_control_chars_no_control_chars() {
        let text = "hello world";
        assert!(matches!(fix_pdf_control_chars(text), Cow::Borrowed(_)));
        assert_eq!(fix_pdf_control_chars(text), "hello world");
    }

    #[test]
    fn test_fix_control_chars_etx_mid_word_becomes_ft() {
        // U+0003 (ETX) between alphabetic characters should decode to "ft"
        let text = "blij\u{0003}";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "blijft");
    }

    #[test]
    fn test_fix_control_chars_etx_software() {
        // "So␃ware" → "Software"
        let text = "So\u{0003}ware";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "Software");
    }

    #[test]
    fn test_fix_control_chars_etx_veiligheidsvoorschriften() {
        // "veiligheidsvoorschri␃en" → "veiligheidsvoorschriften"
        let text = "veiligheidsvoorschri\u{0003}en";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "veiligheidsvoorschriften");
    }

    #[test]
    fn test_fix_control_chars_stx_dropped() {
        // U+0002 (STX) has an ambiguous ligature mapping (issue #1135 reports it in
        // non-"ft" words), so it is dropped rather than guessed: "ingebruik␂name"
        // -> "ingebruikname".
        let text = "ingebruik\u{0002}name";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "ingebruikname");
    }

    #[test]
    fn test_fix_control_chars_etx_followed_by_e_becomes_ft() {
        // ETX in mid-word context becomes "ft"
        // "li" + "ft" + "er" = "lifter"
        let text = "li\u{0003}er";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "lifter");
    }

    #[test]
    fn test_fix_control_chars_mixed_controls() {
        // Multiple control characters in one string
        let text = "so\u{0002}ware and blij\u{0003}";
        let result = fix_pdf_control_chars(text);
        // U+0002 dropped, U+0003 -> "ft".
        assert_eq!(result, "soware and blijft");
    }

    #[test]
    fn test_fix_control_chars_control_at_word_start_dropped() {
        // Control character at start (not between word chars) should be dropped
        let text = "\u{0003}hello";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_fix_control_chars_preserves_tabs_newlines() {
        let text = "hello\tworld\ntest\rmore";
        assert!(matches!(fix_pdf_control_chars(text), Cow::Borrowed(_)));
        assert_eq!(fix_pdf_control_chars(text), "hello\tworld\ntest\rmore");
    }

    #[test]
    fn test_fix_control_chars_other_control_dropped() {
        // Non-STX/ETX C0 controls are decoding artifacts we cannot map reliably, so they
        // are dropped (not guessed). Dropping U+0001 here yields the correct word.
        let text = "soft\u{0001}ware";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "software");
    }

    #[test]
    fn test_fix_control_chars_other_control_not_between_words_dropped() {
        // Other control characters not between word characters should be dropped
        let text = "hello \u{0001}world";
        let result = fix_pdf_control_chars(text);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_fix_control_chars_issue_1135_example_cv_installatie() {
        // From issue #1135: "11.4.3 CV-installatie blij␃ ongewenst warm"
        let text = "11.4.3 CV-installatie blij\u{0003} ongewenst warm";
        let result = fix_pdf_control_chars(text);
        // After decoding: "11.4.3 CV-installatie blijft ongewenst warm"
        assert_eq!(result, "11.4.3 CV-installatie blijft ongewenst warm");
    }

    #[test]
    fn test_fix_control_chars_multiple_etx_in_long_text() {
        let text = "The quick brown fo\u{0003} jumps over the lazy do\u{0003}.";
        let result = fix_pdf_control_chars(text);
        // ETX after 'o' becomes "ft": "fo" + "ft" + " " = "foft "
        // ETX after 'o' becomes "ft": "do" + "ft" + "." = "doft."
        assert_eq!(result, "The quick brown foft jumps over the lazy doft.");
    }

    #[test]
    fn test_fix_control_chars_etx_at_end_of_string() {
        let text = "roo\u{0003}";
        let result = fix_pdf_control_chars(text);
        // ETX at end with no next char defaults to "ft"
        assert_eq!(result, "rooft");
    }

    #[test]
    fn test_decode_ligature_control_chars_stx() {
        // U+0002 is dropped (ambiguous mapping).
        let text = "so\u{0002}ware";
        let result = decode_ligature_control_chars(text);
        assert_eq!(result, "soware");
    }

    #[test]
    fn test_decode_ligature_control_chars_etx() {
        let text = "blij\u{0003}t";
        let result = decode_ligature_control_chars(text);
        // "blij" + "ft" (from ETX) + "t" = "blijftt"
        // This is a synthetic test; real PDFs don't have the 't' after the ligature glyph
        assert_eq!(result, "blijftt");
    }

    #[test]
    fn test_heuristic_ligature_repair_dropped_at_start() {
        let chars: Vec<char> = "hello".chars().collect();
        let result = heuristic_ligature_repair('\u{0001}', &chars, 0); // at start
        assert_eq!(result, "");
    }

    #[test]
    fn test_issue_1135_ligature_control_chars_integration() {
        // Integration test for GitHub issue #1135: PDF ligature glyphs decode to C0 control characters
        // Real-world examples from the issue:

        // Example 1: "veiligheidsvoorschriften" corrupted as "veiligheidsvoorschri␃en"
        // The ␃ (ETX, U+0003) represents the "ft" ligature glyph
        let veiligheid = "veiligheidsvoorschri\u{0003}en";
        let result = fix_pdf_control_chars(veiligheid);
        assert_eq!(result, "veiligheidsvoorschriften");

        // Example 2: "blijft" corrupted as "blij␃" (at word end)
        let blijft = "blij\u{0003}";
        let result = fix_pdf_control_chars(blijft);
        assert_eq!(result, "blijft");

        // Example 3: "Software" corrupted as "So␃ware"
        let software = "So\u{0003}ware";
        let result = fix_pdf_control_chars(software);
        assert_eq!(result, "Software");

        // Example 4: From issue description "11.4.3 CV-installatie blij␃ ongewenst warm"
        let full_sentence = "11.4.3 CV-installatie blij\u{0003} ongewenst warm";
        let result = fix_pdf_control_chars(full_sentence);
        assert_eq!(result, "11.4.3 CV-installatie blijft ongewenst warm");
    }
}
