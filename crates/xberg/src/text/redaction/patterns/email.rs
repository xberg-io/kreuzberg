//! Email address detection.

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

// RFC-5322 inspired but pragmatic: local part allows alphanumerics, dots,
// plus, hyphen, underscore; domain requires at least one dot and a 2+ letter
// TLD. Tightening removes pathological matches like `foo@bar` from prose.
static RE_EMAIL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b").expect("email regex compiles"));

/// Find all email address spans in `text`.
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    RE_EMAIL
        .find_iter(text)
        .map(|m| PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::Email,
            text: m.as_str().to_string(),
        })
        .collect()
}
