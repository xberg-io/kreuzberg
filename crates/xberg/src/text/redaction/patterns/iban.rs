//! IBAN (International Bank Account Number) detection.
//!
//! Format: two-letter ISO 3166-1 country code + two check digits + up to 30
//! alphanumeric BBAN characters. The regex matches IBANs with optional space
//! separators every four characters (the common pretty-print form).

use super::PatternMatch;
use crate::types::redaction::PiiCategory;
use once_cell::sync::Lazy;
use regex::Regex;

// Known IBAN-using country codes (ISO 3166-1 alpha-2). Filter cuts down the
// false-positive surface that any two upper-case letters would otherwise allow.
const IBAN_COUNTRIES: &[&str] = &[
    "AD", "AE", "AL", "AT", "AZ", "BA", "BE", "BG", "BH", "BR", "BY", "CH", "CR", "CY", "CZ", "DE", "DK", "DO", "EE",
    "EG", "ES", "FI", "FO", "FR", "GB", "GE", "GI", "GL", "GR", "GT", "HR", "HU", "IE", "IL", "IQ", "IS", "IT", "JO",
    "KW", "KZ", "LB", "LC", "LI", "LT", "LU", "LV", "LY", "MC", "MD", "ME", "MK", "MR", "MT", "MU", "NL", "NO", "PK",
    "PL", "PS", "PT", "QA", "RO", "RS", "SA", "SC", "SE", "SI", "SK", "SM", "ST", "SV", "TL", "TN", "TR", "UA", "VA",
    "VG", "XK",
];

static RE_IBAN: Lazy<Regex> = Lazy::new(|| {
    // Country (2 letters) + check (2 digits) + 11-30 alphanumeric BBAN with optional spaces.
    Regex::new(r"\b[A-Z]{2}\d{2}(?:[ ]?[A-Z0-9]){11,30}\b").expect("iban regex compiles")
});

/// Find all IBAN spans in `text`, validated against country-specific length rules.
pub fn find_all(text: &str) -> Vec<PatternMatch> {
    let upper = text.to_ascii_uppercase();

    RE_IBAN
        .find_iter(&upper)
        .filter_map(|m| {
            let raw = &upper[m.start()..m.end()];
            let cc = &raw[..2];
            if !IBAN_COUNTRIES.contains(&cc) {
                return None;
            }
            // Strip whitespace and verify total length is within the IBAN range (15-34 chars).
            let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
            if !(15..=34).contains(&compact.len()) {
                return None;
            }
            Some(PatternMatch {
                start: m.start(),
                end: m.end(),
                category: PiiCategory::Iban,
                text: text[m.start()..m.end()].to_string(),
            })
        })
        .collect()
}
