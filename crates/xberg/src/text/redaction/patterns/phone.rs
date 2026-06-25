//! Phone number detection.
//!
//! Matches international E.164 form and North American formats with
//! optional country code, area-code grouping, and common separators
//! (space, dot, hyphen). Length is constrained to 7–15 digits so that
//! generic 4-digit numbers and credit-card chunks are excluded.

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_PHONE: Lazy<Regex> = Lazy::new(|| {
    // (?:\+\d{1,3}[\s.-]?)?(?:\(?\d{2,4}\)?[\s.-]?)?\d{3,4}[\s.-]?\d{3,4}
    // Anchored with word boundaries to avoid grabbing inside longer digit runs.
    Regex::new(
        r"(?x)
        \b
        (?:\+\d{1,3}[\s.\-]?)?       # optional country code
        (?:\(?\d{2,4}\)?[\s.\-]?)?   # optional area code (may be parenthesised)
        \d{3,4}[\s.\-]?\d{3,4}       # subscriber digits
        \b
        ",
    )
    .expect("phone regex compiles")
});

/// Find all phone number spans in `text` (international E.164 and North American formats).
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    RE_PHONE
        .find_iter(text)
        .filter_map(|m| {
            let raw = m.as_str();
            let digit_count = raw.chars().filter(|c| c.is_ascii_digit()).count();
            if !(7..=15).contains(&digit_count) {
                return None;
            }
            Some(PatternMatch {
                start: m.start(),
                end: m.end(),
                category: PiiCategory::Phone,
                text: raw.to_string(),
            })
        })
        .collect()
}
