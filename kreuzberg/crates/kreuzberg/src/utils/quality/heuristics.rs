//! Quality heuristics and text analysis
//!
//! This module provides heuristic checks for text quality, including
//! structure analysis and line-level checks.

use super::patterns::*;
use memchr::memmem;

// ============================================================================
// Structure Thresholds
// ============================================================================

const MIN_SENTENCE_WORDS: f64 = 10.0;
const MAX_SENTENCE_WORDS: f64 = 30.0;
const MIN_PARAGRAPH_WORDS: f64 = 50.0;
const MAX_PARAGRAPH_WORDS: f64 = 300.0;

// ============================================================================
// Structure Analysis
// ============================================================================

/// Calculate bonus based on text structure quality
#[inline]
pub(crate) fn calculate_structure_bonus(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }

    let sentence_count = SENTENCE_DETECT.find_iter(text).count() as f64;
    let paragraph_count = memmem::find_iter(text.as_bytes(), b"\n\n").count() as f64 + 1.0;
    let words = text.split_whitespace().count() as f64;

    if words == 0.0 {
        return 0.0;
    }

    let avg_words_per_sentence = words / sentence_count.max(1.0);
    let avg_words_per_paragraph = words / paragraph_count.max(1.0);

    let mut structure_score: f64 = 0.0;

    if (MIN_SENTENCE_WORDS..=MAX_SENTENCE_WORDS).contains(&avg_words_per_sentence) {
        structure_score += 0.3;
    }

    if (MIN_PARAGRAPH_WORDS..=MAX_PARAGRAPH_WORDS).contains(&avg_words_per_paragraph) {
        structure_score += 0.3;
    }

    if paragraph_count > 1.0 {
        structure_score += 0.2;
    }

    if PUNCTUATION_DETECT.is_match(text) {
        structure_score += 0.2;
    }

    structure_score.min(1.0)
}
