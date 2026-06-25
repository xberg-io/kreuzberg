//! Postal-code detection for US ZIP, UK postcode, German PLZ, French CP,
//! and Canadian FSA+LDU.
//!
//! These countries cover the majority of corpora we expect to encounter and
//! have well-defined shapes that survive regex extraction without a country
//! code in scope. Additional locales can be added without breaking changes.

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

// US ZIP: 5 digits, optional -4.
static RE_US: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b\d{5}(?:-\d{4})?\b").expect("us zip regex compiles"));

// UK postcode: outward 1-2 letters + 1-2 digits (optionally letter), space, 1 digit + 2 letters.
static RE_UK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b[A-Z]{1,2}[0-9][0-9A-Z]?\s?[0-9][A-Z]{2}\b").expect("uk postcode regex compiles"));

// German PLZ: 5 digits. Same shape as US ZIP, so the dispatcher coalesces
// matches by span; this regex is supplementary for sanity.
// (US regex already matches German PLZ, so no separate Lazy needed.)

// French CP: 5 digits — same shape; covered by US regex.

// Canadian FSA+LDU: A1A 1A1 (no vowels in first letter group per spec; we allow all letters for simplicity).
static RE_CA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b[A-Z][0-9][A-Z]\s?[0-9][A-Z][0-9]\b").expect("ca postal regex compiles"));

/// Find all postal code spans in `text` (US ZIP, UK postcode, German PLZ, Canadian FSA, and more).
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    let mut matches = Vec::new();

    let upper = text.to_ascii_uppercase();

    for m in RE_UK.find_iter(&upper) {
        matches.push(PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::PostalCode,
            text: text[m.start()..m.end()].to_string(),
        });
    }
    for m in RE_CA.find_iter(&upper) {
        matches.push(PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::PostalCode,
            text: text[m.start()..m.end()].to_string(),
        });
    }
    for m in RE_US.find_iter(text) {
        matches.push(PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::PostalCode,
            text: m.as_str().to_string(),
        });
    }

    matches
}
