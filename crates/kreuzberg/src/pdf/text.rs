//! PDF text utilities (backend-agnostic).
//!
//! Contains pure utility functions for text post-processing used by the
//! oxide extraction backend.

use std::borrow::Cow;

/// Replace PDF font encoding artifacts in extracted text.
///
/// Some PDFs have broken ToUnicode mappings that produce control characters
/// (U+0001–U+001F) where printable characters (typically hyphens) should appear.
/// This function replaces such control characters with hyphens when they appear
/// between word characters, or removes them otherwise. Tab, newline, and carriage
/// return are preserved.
///
/// Returns `Cow::Borrowed` when no replacements are needed (zero-cost for clean text).
pub(crate) fn fix_pdf_control_chars(text: &str) -> Cow<'_, str> {
    // Quick scan: skip allocation if no problematic chars exist.
    if !text.bytes().any(|b| b < 0x20 && b != b'\t' && b != b'\n' && b != b'\r') {
        return Cow::Borrowed(text);
    }

    let chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len());

    for (i, &ch) in chars.iter().enumerate() {
        if matches!(ch, '\u{0001}'..='\u{001F}') && ch != '\t' && ch != '\n' && ch != '\r' {
            // Check if the control char is between alphanumeric/word characters.
            // If so, it likely represents a hyphen from a broken ToUnicode mapping.
            let prev_is_word = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '-');
            let next_is_word = i + 1 < chars.len() && (chars[i + 1].is_alphanumeric() || chars[i + 1] == '-');

            if prev_is_word && next_is_word {
                result.push('-');
            }
            // Otherwise, drop the control character entirely.
        } else {
            result.push(ch);
        }
    }

    Cow::Owned(result)
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
