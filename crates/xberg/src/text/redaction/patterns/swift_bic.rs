//! SWIFT/BIC bank identifier detection.
//!
//! Format: 4-letter bank code + 2-letter ISO country code + 2-character
//! location code + optional 3-character branch code (total 8 or 11 chars).

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_SWIFT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}(?:[A-Z0-9]{3})?\b").expect("swift regex compiles"));

/// Find all SWIFT/BIC code spans in `text` (8 or 11 uppercase alphanumeric characters).
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    // Match directly on source text (no case-folding). Real SWIFT BICs are
    // always written in uppercase in financial documents; case-folding the
    // entire input caused false positives on any 8-letter lowercase English
    // word (e.g. "launches", "codename").
    RE_SWIFT
        .find_iter(text)
        .map(|m| PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::SwiftBic,
            text: text[m.start()..m.end()].to_string(),
        })
        .collect()
}
