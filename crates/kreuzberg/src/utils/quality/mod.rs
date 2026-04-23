//! Quality scoring and text cleaning utilities
//!
//! This module provides comprehensive quality assessment and cleaning
//! for extracted text, including OCR artifact detection, script removal,
//! and whitespace normalization.

mod heuristics;
mod patterns;
mod scoring;

// Re-export public API

use crate::text::utf8_validation;
use memchr::{memchr, memchr3};
use patterns::*;
use regex::Regex;
use std::borrow::Cow;

// ============================================================================
// Text Cleaning and Normalization
// ============================================================================

/// Apply the quality heuristics and return a cleaned representation of the text.
///
/// This function normalises whitespace, removes navigation boilerplate, and strips
/// repeated punctuation that commonly appears in OCR output.
pub fn clean_extracted_text(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut working_text = Cow::Borrowed(text);

    working_text = clean_scripts(working_text);

    working_text = clean_ocr_artifacts_cow(working_text);

    working_text = clean_navigation_elements_cow(working_text);

    working_text = clean_repeated_punctuation_cow(working_text);

    working_text = normalize_whitespace_cow(working_text);

    working_text.trim().to_string()
}

/// Collapse redundant whitespace while preserving paragraph boundaries.
pub(crate) fn normalize_spaces(text: &str) -> String {
    if text.is_empty() || text.trim().is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(text.len());

    let mut first = true;
    for paragraph in text.split("\n\n") {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !first {
            result.push_str("\n\n");
        }
        first = false;

        let collapsed = if let Some(fast) = normalize_whitespace_ascii(paragraph) {
            Cow::Owned(fast)
        } else {
            Cow::Owned(WHITESPACE_NORMALIZE.replace_all(paragraph, " ").into_owned())
        };

        let cleaned = NEWLINE_CLEANUP.replace_all(&collapsed, "\n");

        let mut first_line = true;
        for line in cleaned.split('\n') {
            let line = line.trim();
            if !line.is_empty() {
                if !first_line {
                    result.push('\n');
                }
                result.push_str(line);
                first_line = false;
            }
        }
    }

    result
}

// ============================================================================
// Internal Cleaning Functions
// ============================================================================

#[inline]
fn clean_scripts<'a>(text: Cow<'a, str>) -> Cow<'a, str> {
    let script_replacements = [
        (&*SCRIPT_TAG_PATTERN, " "),
        (&*STYLE_TAG_PATTERN, " "),
        (&*JS_FUNCTION_PATTERN, " "),
        (&*CSS_RULES_PATTERN, " "),
    ];
    chain_replacements(text, &script_replacements)
}

#[inline]
fn clean_ocr_artifacts_cow<'a>(text: Cow<'a, str>) -> Cow<'a, str> {
    let result = if let Some(fixed) = collapse_scattered_ascii(&text) {
        Cow::Owned(fixed)
    } else if SCATTERED_CHARS_PATTERN.is_match(&text) {
        Cow::Owned(
            replace_with_if_matches(&text, &SCATTERED_CHARS_PATTERN, |caps: &regex::Captures| {
                caps[0].chars().filter(|c| !c.is_whitespace()).collect::<String>()
            })
            .into_owned(),
        )
    } else {
        text
    };

    let result = clean_dashes_preserve_tables(result);

    let ocr_replacements = [
        (&*REPEATED_PUNCT_PATTERN, "..."),
        (&*ISOLATED_PUNCT_PATTERN, " "),
        (&*MALFORMED_WORDS_PATTERN, " "),
        (&*EXCESSIVE_WHITESPACE_PATTERN, " "),
    ];

    chain_replacements(result, &ocr_replacements)
}

#[inline]
fn clean_dashes_preserve_tables<'a>(text: Cow<'a, str>) -> Cow<'a, str> {
    if !DASH_PATTERN.is_match(&text) {
        return text;
    }

    let mut result = String::with_capacity(text.len());
    let mut first_line = true;

    for line in text.lines() {
        if !first_line {
            result.push('\n');
        }
        first_line = false;

        let trimmed = line.trim();
        let is_table_separator = trimmed.starts_with('|')
            && trimmed.ends_with('|')
            && trimmed
                .chars()
                .all(|c| c == '|' || c == '-' || c.is_whitespace() || c == ':');

        if is_table_separator {
            result.push_str(line);
        } else {
            let cleaned_line = DASH_PATTERN.replace_all(line, "...");
            result.push_str(&cleaned_line);
        }
    }

    Cow::Owned(result)
}

#[inline]
fn clean_navigation_elements_cow<'a>(text: Cow<'a, str>) -> Cow<'a, str> {
    let nav_replacements = [
        (&*NAV_WORDS_PATTERN, " "),
        (&*BREADCRUMB_PATTERN, " "),
        (&*PAGINATION_PATTERN, " "),
    ];

    chain_replacements(text, &nav_replacements)
}

#[inline]
fn clean_repeated_punctuation_cow<'a>(text: Cow<'a, str>) -> Cow<'a, str> {
    if let Some(cleaned) = clean_repeated_punctuation_ascii(text.as_ref()) {
        return Cow::Owned(cleaned);
    }

    if REPEATED_PUNCT_PATTERN.is_match(&text) {
        Cow::Owned(
            REPEATED_PUNCT_PATTERN
                .replace_all(&text, |caps: &regex::Captures<'_>| {
                    let ch = caps.get(0).and_then(|m| m.as_str().chars().next()).unwrap_or('.');
                    ch.to_string()
                })
                .into_owned(),
        )
    } else {
        text
    }
}

#[inline]
fn normalize_whitespace_cow<'a>(text: Cow<'a, str>) -> Cow<'a, str> {
    if let Some(fast) = normalize_whitespace_ascii(text.as_ref()) {
        return Cow::Owned(fast);
    }

    let mut result = text;

    if WHITESPACE_NORMALIZE.is_match(&result) {
        result = Cow::Owned(WHITESPACE_NORMALIZE.replace_all(&result, " ").into_owned());
    }

    if NEWLINE_NORMALIZE.is_match(&result) {
        result = Cow::Owned(NEWLINE_NORMALIZE.replace_all(&result, "\n\n").into_owned());
    }

    result
}

// ============================================================================
// ASCII Fast-Path Optimizations
// ============================================================================

fn clean_repeated_punctuation_ascii(text: &str) -> Option<String> {
    if !text.is_ascii() {
        return None;
    }

    let bytes = text.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut changed = false;
    let mut offset = 0;

    while offset < bytes.len() {
        let remaining = &bytes[offset..];
        if let Some(next) = find_next_ascii_punctuation(remaining) {
            if next > 0 {
                result.extend_from_slice(&remaining[..next]);
                offset += next;
            }

            if offset >= bytes.len() {
                break;
            }

            let current = bytes[offset];
            result.push(current);
            let mut end = offset + 1;
            while end < bytes.len() && matches!(bytes[end], b'!' | b'?' | b'.' | b',') {
                changed = true;
                end += 1;
            }
            offset = end;
        } else {
            result.extend_from_slice(remaining);
            break;
        }
    }

    if changed {
        utf8_validation::string_from_utf8(result).ok()
    } else {
        None
    }
}

#[inline]
fn find_next_ascii_punctuation(bytes: &[u8]) -> Option<usize> {
    let primary = memchr3(b'!', b'?', b'.', bytes);
    let comma = memchr(b',', bytes);
    match (primary, comma) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Normalize whitespace for ASCII text (fast path)
#[inline]
pub(crate) fn normalize_whitespace_ascii(text: &str) -> Option<String> {
    if !text.is_ascii() {
        return None;
    }

    let bytes = text.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut changed = false;
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        match bytes[i] {
            b' ' | b'\t' | b'\r' | 0x0B | 0x0C => {
                let mut j = i + 1;
                while j < len && matches!(bytes[j], b' ' | b'\t' | b'\r' | 0x0B | 0x0C) {
                    j += 1;
                }
                if j - i > 1 || bytes[i] != b' ' {
                    changed = true;
                }
                result.push(b' ');
                i = j;
            }
            b'\n' => {
                let mut j = i + 1;
                while j < len && matches!(bytes[j], b' ' | b'\t' | b'\r' | 0x0B | 0x0C) {
                    j += 1;
                    changed = true;
                }

                let mut newline_count = 1;
                while j < len && bytes[j] == b'\n' {
                    newline_count += 1;
                    j += 1;

                    while j < len && matches!(bytes[j], b' ' | b'\t' | b'\r' | 0x0B | 0x0C) {
                        j += 1;
                        changed = true;
                    }
                }

                if newline_count >= 3 {
                    result.extend_from_slice(b"\n\n");
                    changed = true;
                } else {
                    result.extend(std::iter::repeat_n(b'\n', newline_count));
                }

                i = j;
            }
            _ => {
                result.push(bytes[i]);
                i += 1;
            }
        }
    }

    let normalized = utf8_validation::string_from_utf8(result).unwrap_or_else(|_| text.to_string());

    if changed { Some(normalized) } else { None }
}

/// Collapse scattered ASCII characters (fast path)
#[inline]
pub(crate) fn collapse_scattered_ascii(text: &str) -> Option<String> {
    if !text.is_ascii() {
        return None;
    }

    let bytes = text.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut changed = false;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i].is_ascii_alphabetic() {
            let mut j = i + 1;
            let mut count = 1;
            while j < bytes.len() {
                if bytes[j].is_ascii_alphabetic() {
                    count += 1;
                    j += 1;
                } else if bytes[j].is_ascii_whitespace() {
                    j += 1;
                } else {
                    break;
                }
            }

            if count >= 3 && j - i >= (count * 2 - 1) {
                changed = true;
                for &byte in &bytes[i..j] {
                    if byte.is_ascii_alphabetic() {
                        result.push(byte.to_ascii_lowercase());
                    }
                }
                result.push(b' ');
                i = j;
                continue;
            }
        }

        result.push(bytes[i]);
        i += 1;
    }

    if changed {
        utf8_validation::string_from_utf8(result).ok()
    } else {
        None
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

fn chain_replacements<'a>(mut text: Cow<'a, str>, replacements: &[(&Regex, &str)]) -> Cow<'a, str> {
    for (pattern, replacement) in replacements {
        if pattern.is_match(&text) {
            text = Cow::Owned(pattern.replace_all(&text, *replacement).into_owned());
        }
    }
    text
}

#[inline]
fn replace_with_if_matches<'a, F>(text: &'a str, pattern: &Regex, replacer: F) -> Cow<'a, str>
where
    F: FnMut(&regex::Captures) -> String,
{
    if pattern.is_match(text) {
        Cow::Owned(pattern.replace_all(text, replacer).into_owned())
    } else {
        Cow::Borrowed(text)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(all(test, feature = "quality"))]
mod tests {
    use super::*;
    use ahash::AHashMap;

    #[test]
    fn test_calculate_quality_score_empty_text() {
        assert_eq!(calculate_quality_score("", None), 0.0);
        assert_eq!(calculate_quality_score("   ", None), 0.0);
        assert_eq!(calculate_quality_score("\n\n\n", None), 0.0);
    }

    #[test]
    fn test_calculate_quality_score_short_text() {
        let text = "Hello";
        let score = calculate_quality_score(text, None);
        assert_eq!(score, 0.1);
    }

    #[test]
    fn test_calculate_quality_score_normal_text() {
        let text =
            "This is a normal sentence with proper punctuation. It has multiple sentences. And proper structure.";
        let score = calculate_quality_score(text, None);
        assert!(score > 0.5);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_clean_extracted_text_empty() {
        assert_eq!(clean_extracted_text(""), "");
        assert_eq!(clean_extracted_text("   "), "");
    }

    #[test]
    fn test_clean_extracted_text_removes_scripts() {
        let text = "Before <script>alert('test');</script> After";
        let cleaned = clean_extracted_text(text);
        assert!(!cleaned.contains("<script"));
        assert!(cleaned.contains("Before"));
        assert!(cleaned.contains("After"));
    }

    #[test]
    fn test_normalize_spaces_empty() {
        assert_eq!(normalize_spaces(""), "");
        assert_eq!(normalize_spaces("   "), "");
    }

    #[test]
    fn test_normalize_spaces_single_paragraph() {
        let text = "This  is   a   test";
        let normalized = normalize_spaces(text);
        assert_eq!(normalized, "This is a test");
    }

    #[test]
    fn test_calculate_quality_score_with_metadata() {
        let text = "This is a normal text with proper structure.";
        let mut metadata = AHashMap::new();
        metadata.insert("title".to_string(), "Test Title".to_string());
        metadata.insert("author".to_string(), "Test Author".to_string());

        let score = calculate_quality_score(text, Some(&metadata));
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_clean_extracted_text_removes_styles() {
        let text = "Before <style>.class { color: red; }</style> After";
        let cleaned = clean_extracted_text(text);
        assert!(!cleaned.contains("<style"));
        assert!(cleaned.contains("Before"));
        assert!(cleaned.contains("After"));
    }

    #[test]
    fn test_clean_extracted_text_ocr_artifacts() {
        let text = "Text with   excessive    spaces";
        let cleaned = clean_extracted_text(text);
        assert!(!cleaned.contains("   "));
    }

    #[test]
    fn test_clean_extracted_text_navigation() {
        let text = "Content Skip to main content more content";
        let cleaned = clean_extracted_text(text);
        assert!(cleaned.contains("Content"));
        assert!(cleaned.contains("more content"));
    }

    #[test]
    fn test_clean_repeated_punctuation_ascii_helper() {
        let input = "Wow!!! Really??? Sure...";
        let cleaned = clean_repeated_punctuation_ascii(input).expect("Should collapse punctuation");
        assert_eq!(cleaned, "Wow! Really? Sure.");
    }

    #[test]
    fn test_clean_repeated_punctuation_non_ascii_passthrough() {
        assert!(clean_repeated_punctuation_ascii("¿Qué tal?").is_none());
    }

    #[test]
    fn test_normalize_spaces_multiple_paragraphs() {
        let text = "First paragraph.\n\nSecond paragraph.";
        let normalized = normalize_spaces(text);
        assert!(normalized.contains("\n\n"));
    }

    #[test]
    fn test_normalize_spaces_preserves_paragraphs() {
        let text = "Para 1\n\n\n\nPara 2";
        let normalized = normalize_spaces(text);
        assert_eq!(normalized, "Para 1\n\nPara 2");
    }

    #[test]
    fn test_clean_dashes_preserve_tables_simple() {
        let text = Cow::Borrowed("| Col1 |\n|------|\n| Data |");
        let result = clean_dashes_preserve_tables(text);
        assert!(result.contains("|------"));
    }

    #[test]
    fn test_clean_dashes_preserve_tables_replaces_non_table() {
        let text = Cow::Borrowed("Text with --- dashes");
        let result = clean_dashes_preserve_tables(text);
        assert!(result.contains("..."));
        assert!(!result.contains("---"));
    }

    #[test]
    fn test_quality_score_large_text_with_ocr_issues() {
        let text = "a".repeat(2000) + "   " + &"b".repeat(2000);
        let score = calculate_quality_score(&text, None);
        assert!(score >= 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_quality_score_clamped_to_range() {
        let perfect_text = "This is perfect text. ".repeat(100);
        let score = calculate_quality_score(&perfect_text, None);
        assert!(score >= 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_clean_extracted_text_scattered_chars() {
        let text = "a  b  c scattered";
        let cleaned = clean_extracted_text(text);
        assert!(!cleaned.is_empty());
    }

    #[cfg_attr(coverage, ignore = "coverage instrumentation perturbs ASCII fast path heuristics")]
    #[test]
    fn test_collapse_scattered_ascii_trigger() {
        let original = "S p a c e d";
        let collapsed = collapse_scattered_ascii(original).expect("fast path should trigger");
        assert_eq!(collapsed.trim(), "spaced");
    }

    #[test]
    fn test_collapse_scattered_ascii_non_ascii() {
        assert!(collapse_scattered_ascii("מ ש ה ו").is_none());
    }

    #[test]
    fn test_normalize_whitespace_ascii_spaces() {
        let input = "Hello   \tWorld\rWelcome";
        let normalized = normalize_whitespace_ascii(input).expect("ascii fast path should trigger");
        assert_eq!(normalized, "Hello World Welcome");
    }

    #[test]
    fn test_normalize_whitespace_ascii_newlines() {
        let input = "Line1\n  \n\n \nLine2";
        let normalized = normalize_whitespace_ascii(input).expect("ascii fast path should trigger");
        assert_eq!(normalized, "Line1\n\nLine2");
    }

    #[test]
    fn test_normalize_whitespace_ascii_no_change() {
        assert!(normalize_whitespace_ascii("Clean text").is_none());
    }

    #[test]
    fn test_normalize_whitespace_ascii_non_ascii() {
        assert!(normalize_whitespace_ascii("שלום שלום").is_none());
    }

    #[test]
    fn test_normalize_spaces_ascii_fast_path() {
        let input = "Hello   world\n\nSecond   line";
        let normalized = normalize_spaces(input);
        assert_eq!(normalized, "Hello world\n\nSecond line");
    }

    #[test]
    fn test_normalize_whitespace_cow_no_changes() {
        let text = Cow::Borrowed("normaltext");
        let result = normalize_whitespace_cow(text);
        assert_eq!(&*result, "normaltext");
    }

    #[test]
    fn test_normalize_whitespace_cow_with_changes() {
        let text = Cow::Borrowed("text   with   spaces");
        let result = normalize_whitespace_cow(text);
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_clean_scripts_no_scripts() {
        let text = Cow::Borrowed("clean text");
        let result = clean_scripts(text);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_clean_scripts_with_script_tag() {
        let text = Cow::Borrowed("<script>code</script>");
        let result = clean_scripts(text);
        assert!(!result.contains("<script"));
    }
}
