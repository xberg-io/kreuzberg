//! US Social Security Number detection.
//!
//! Format: `AAA-GG-SSSS` where:
//! - Area number (AAA) is 001-665 or 667-899 (666 and 9xx historically reserved)
//! - Group number (GG) is 01-99
//! - Serial number (SSSS) is 0001-9999
//!
//! Spaces and dashes are accepted between groups. The format check filters out
//! obvious non-SSNs like `000-00-0000` or `666-12-3456`.

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_SSN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(\d{3})[\s\-](\d{2})[\s\-](\d{4})\b").expect("ssn regex compiles"));

/// Find all US Social Security Number spans in `text` (format: NNN-NN-NNNN).
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    RE_SSN
        .captures_iter(text)
        .filter_map(|cap| {
            let area: u16 = cap.get(1)?.as_str().parse().ok()?;
            let group: u16 = cap.get(2)?.as_str().parse().ok()?;
            let serial: u16 = cap.get(3)?.as_str().parse().ok()?;

            // SSA exclusions: 000, 666, 900-999 areas; 00 group; 0000 serial.
            if area == 0 || area == 666 || (900..=999).contains(&area) {
                return None;
            }
            if group == 0 || serial == 0 {
                return None;
            }

            let full = cap.get(0)?;
            Some(PatternMatch {
                start: full.start(),
                end: full.end(),
                category: PiiCategory::Ssn,
                text: full.as_str().to_string(),
            })
        })
        .collect()
}
