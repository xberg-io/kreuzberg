//! Regex patterns for quality detection
//!
//! This module contains all regex patterns used for detecting OCR artifacts,
//! script content, navigation elements, and text structure.

use once_cell::sync::Lazy;
use regex::Regex;

// ============================================================================
// OCR Artifact Patterns
// ============================================================================

/// Detects scattered characters with excessive spacing (e.g., "a  b  c")
pub(crate) static SCATTERED_CHARS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[a-zA-Z]\s{2,}[a-zA-Z]\s{2,}[a-zA-Z]\b")
        .expect("Scattered chars regex pattern is valid and should compile")
});

/// Detects repeated punctuation marks (3 or more dots or underscores)
pub(crate) static REPEATED_PUNCT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[.]{3,}|[_]{3,}").expect("Repeated punctuation regex pattern is valid and should compile")
});

/// Detects repeated dashes (3 or more)
pub(crate) static DASH_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[-]{3,}").expect("Dash pattern regex is valid and should compile"));

/// Detects isolated punctuation surrounded by spaces
pub(crate) static ISOLATED_PUNCT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s[.,;:!?]\s").expect("Isolated punctuation regex pattern is valid and should compile"));

/// Detects malformed words with mixed letters and numbers
pub(crate) static MALFORMED_WORDS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[a-zA-Z]+[0-9]+[a-zA-Z]+[a-zA-Z0-9]*\b")
        .expect("Malformed words regex pattern is valid and should compile")
});

/// Detects excessive whitespace (3 or more spaces)
pub(crate) static EXCESSIVE_WHITESPACE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s{3,}").expect("Excessive whitespace regex pattern is valid and should compile"));

// ============================================================================
// Script and Code Patterns
// ============================================================================

/// Detects JavaScript function declarations
pub(crate) static JS_FUNCTION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)function\s+\w+\s*\([^)]*\)\s*\{[^}]*\}")
        .expect("JavaScript function regex pattern is valid and should compile")
});

/// Detects CSS rules
pub(crate) static CSS_RULES_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\.[a-zA-Z][\w-]*\s*\{[^}]*\}").expect("CSS rules regex pattern is valid and should compile")
});

/// Detects HTML script tags
pub(crate) static SCRIPT_TAG_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?is)<script[^>]*>.*?</script>").expect("Script tag regex pattern is valid and should compile")
});

/// Detects HTML style tags
pub(crate) static STYLE_TAG_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?is)<style[^>]*>.*?</style>").expect("Style tag regex pattern is valid and should compile")
});

// ============================================================================
// Navigation Element Patterns
// ============================================================================

/// Detects common navigation words and phrases
pub(crate) static NAV_WORDS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(?:Skip to main content|Back to top|Main navigation|Site navigation)\b")
        .expect("Navigation words regex pattern is valid and should compile")
});

/// Detects breadcrumb navigation patterns
pub(crate) static BREADCRUMB_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:Home\s*[>»]\s*|[>»]\s*){2,}").expect("Breadcrumb regex pattern is valid and should compile")
});

/// Detects pagination text
pub(crate) static PAGINATION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(?:Page \d+ of \d+|First page|Last page|Previous page|Next page|^\d+ of \d+$)\b")
        .expect("Pagination regex pattern is valid and should compile")
});

// ============================================================================
// Text Structure Patterns
// ============================================================================

/// Detects sentence boundaries
pub(crate) static SENTENCE_DETECT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[.!?]\s+[A-Z]").expect("Sentence detection regex pattern is valid and should compile"));

/// Detects punctuation marks
pub(crate) static PUNCTUATION_DETECT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[.!?]").expect("Punctuation detection regex pattern is valid and should compile"));

// ============================================================================
// Whitespace Normalization Patterns
// ============================================================================

/// Normalizes various types of whitespace characters
pub(crate) static WHITESPACE_NORMALIZE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[ \t\f\v\r\xa0\u{2000}-\u{200b}\u{2028}\u{2029}\u{3000}]+")
        .expect("Whitespace normalization regex pattern is valid and should compile")
});

/// Normalizes multiple consecutive newlines
pub(crate) static NEWLINE_NORMALIZE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\n\s*\n\s*\n+").expect("Newline normalization regex pattern is valid and should compile")
});

/// Cleans up newline sequences
pub(crate) static NEWLINE_CLEANUP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\n+").expect("Newline cleanup regex pattern is valid and should compile"));
