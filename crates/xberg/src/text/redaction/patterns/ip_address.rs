//! IPv4 and IPv6 address detection.
//!
//! IPv4 uses dotted-quad with each octet in 0-255.
//! IPv6 supports the full and double-colon (`::`) shortened forms.

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_IPV4: Lazy<Regex> = Lazy::new(|| {
    // Each octet 0-255 anchored on word boundaries.
    Regex::new(r"\b(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}\b")
        .expect("ipv4 regex compiles")
});

static RE_IPV6: Lazy<Regex> = Lazy::new(|| {
    // Conservative IPv6: 2+ groups separated by ':' with at least one '::' shortcut or 8 groups.
    Regex::new(
        r"(?xi)
        (?:
            [0-9A-F]{1,4}(?::[0-9A-F]{1,4}){7}                 # full 8 groups
            |
            (?:[0-9A-F]{1,4}:){1,7}:                            # leading :: shortcut
            |
            :(?::[0-9A-F]{1,4}){1,7}                            # trailing :: shortcut
            |
            (?:[0-9A-F]{1,4}:){1,6}:[0-9A-F]{1,4}               # middle :: shortcut
        )
        ",
    )
    .expect("ipv6 regex compiles")
});

/// Find all IPv4 and IPv6 address spans in `text`.
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    let mut matches = Vec::new();

    for m in RE_IPV4.find_iter(text) {
        matches.push(PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::IpAddress,
            text: m.as_str().to_string(),
        });
    }
    for m in RE_IPV6.find_iter(text) {
        let raw = m.as_str();
        // Filter out obvious garbage: must contain at least one ':' and not be all-zeros.
        if raw.matches(':').count() < 2 {
            continue;
        }
        matches.push(PatternMatch {
            start: m.start(),
            end: m.end(),
            category: PiiCategory::IpAddress,
            text: raw.to_string(),
        });
    }
    matches
}
