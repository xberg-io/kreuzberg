//! Quality scoring functions
//!
//! This module provides functions to calculate quality scores and penalties
//! based on various text characteristics.

use super::patterns::*;
use ahash::AHashMap;
use memchr::memmem;
use regex::Regex;

// ============================================================================
// Scoring Constants and Weights
// ============================================================================

pub(crate) const OCR_PENALTY_WEIGHT: f64 = 0.3;
pub(crate) const SCRIPT_PENALTY_WEIGHT: f64 = 0.2;
pub(crate) const NAV_PENALTY_WEIGHT: f64 = 0.1;
pub(crate) const STRUCTURE_BONUS_WEIGHT: f64 = 0.2;
pub(crate) const METADATA_BONUS_WEIGHT: f64 = 0.1;

pub(crate) const MIN_TEXT_LENGTH: usize = 10;
pub(crate) const LARGE_TEXT_LENGTH: usize = 1000;

// ============================================================================
// Helper Functions
// ============================================================================

/// Sums the total length of all regex matches in the text
#[inline]
pub(crate) fn sum_match_lengths(text: &str, pattern: &Regex) -> usize {
    pattern.find_iter(text).map(|m| m.len()).sum()
}

// ============================================================================
// Penalty Calculation Functions
// ============================================================================

/// Calculate penalty based on OCR artifacts in the text
#[inline]
pub(crate) fn calculate_ocr_penalty(text: &str, total_chars: f64) -> f64 {
    if total_chars == 0.0 {
        return 0.0;
    }

    if memmem::find(text.as_bytes(), b"  ").is_none() && memmem::find(text.as_bytes(), b"...").is_none() {
        return 0.0;
    }

    let artifact_chars = sum_match_lengths(text, &SCATTERED_CHARS_PATTERN)
        + sum_match_lengths(text, &REPEATED_PUNCT_PATTERN)
        + count_non_table_dash_artifacts(text)
        + sum_match_lengths(text, &ISOLATED_PUNCT_PATTERN)
        + sum_match_lengths(text, &MALFORMED_WORDS_PATTERN)
        + sum_match_lengths(text, &EXCESSIVE_WHITESPACE_PATTERN);

    (artifact_chars as f64 / total_chars).min(1.0)
}

/// Count dash artifacts while preserving table separators
#[inline]
pub(crate) fn count_non_table_dash_artifacts(text: &str) -> usize {
    let mut artifact_count = 0;

    for line in text.lines() {
        let trimmed = line.trim();
        let is_table_separator = trimmed.starts_with('|')
            && trimmed.ends_with('|')
            && trimmed
                .chars()
                .all(|c| c == '|' || c == '-' || c.is_whitespace() || c == ':');

        if !is_table_separator {
            for m in DASH_PATTERN.find_iter(line) {
                artifact_count += m.len();
            }
        }
    }

    artifact_count
}

/// Calculate penalty based on embedded scripts and code
#[inline]
pub(crate) fn calculate_script_penalty(text: &str, total_chars: f64) -> f64 {
    if total_chars == 0.0 {
        return 0.0;
    }

    if memmem::find(text.as_bytes(), b"function").is_none()
        && memmem::find(text.as_bytes(), b"<script").is_none()
        && memmem::find(text.as_bytes(), b"<style").is_none()
    {
        return 0.0;
    }

    let script_chars = sum_match_lengths(text, &JS_FUNCTION_PATTERN)
        + sum_match_lengths(text, &CSS_RULES_PATTERN)
        + sum_match_lengths(text, &SCRIPT_TAG_PATTERN)
        + sum_match_lengths(text, &STYLE_TAG_PATTERN);

    (script_chars as f64 / total_chars).min(1.0)
}

/// Calculate penalty based on navigation elements
#[inline]
pub(crate) fn calculate_navigation_penalty(text: &str, total_chars: f64) -> f64 {
    if total_chars == 0.0 {
        return 0.0;
    }

    let nav_chars = sum_match_lengths(text, &NAV_WORDS_PATTERN)
        + sum_match_lengths(text, &BREADCRUMB_PATTERN)
        + sum_match_lengths(text, &PAGINATION_PATTERN);

    (nav_chars as f64 / total_chars).min(1.0)
}

// ============================================================================
// Bonus Calculation Functions
// ============================================================================

/// Calculate bonus based on document metadata quality
#[inline]
pub(crate) fn calculate_metadata_bonus(metadata: &AHashMap<String, String>) -> f64 {
    const IMPORTANT_FIELDS: &[&str] = &["title", "author", "subject", "description", "keywords"];

    let present_fields = IMPORTANT_FIELDS
        .iter()
        .filter(|&&field| metadata.contains_key(field))
        .count();

    present_fields as f64 / IMPORTANT_FIELDS.len() as f64
}

/// Compute a heuristic score (0.0–1.0) describing how clean the extracted text is.
///
/// The scoring pipeline rewards well-structured prose while penalising OCR artefacts,
/// embedded scripts, and navigation chrome. Supplying document metadata allows the
/// function to include contextual bonuses.
///
/// ```rust
/// use ahash::AHashMap;
/// use kreuzberg::utils::quality::calculate_quality_score;
///
/// let text = "Executive Summary\n===================\nKreuzberg extracts documents quickly.";
/// let score = calculate_quality_score(text, None);
/// assert!(score > 0.7);
/// ```
pub fn calculate_quality_score(text: &str, metadata: Option<&AHashMap<String, String>>) -> f64 {
    if text.is_empty() || text.trim().is_empty() {
        return 0.0;
    }

    let total_chars = text.len() as f64;

    if text.len() < MIN_TEXT_LENGTH {
        return 0.1;
    }

    let mut score = 1.0;

    if text.len() > LARGE_TEXT_LENGTH {
        let ocr_penalty = calculate_ocr_penalty(text, total_chars);
        let script_penalty = calculate_script_penalty(text, total_chars);
        let nav_penalty = calculate_navigation_penalty(text, total_chars);
        let structure_bonus = super::heuristics::calculate_structure_bonus(text);

        score -= ocr_penalty * OCR_PENALTY_WEIGHT;
        score -= script_penalty * SCRIPT_PENALTY_WEIGHT;
        score -= nav_penalty * NAV_PENALTY_WEIGHT;
        score += structure_bonus * STRUCTURE_BONUS_WEIGHT;
    } else {
        score -= calculate_ocr_penalty(text, total_chars) * OCR_PENALTY_WEIGHT;
        score += super::heuristics::calculate_structure_bonus(text) * STRUCTURE_BONUS_WEIGHT;
    }

    if let Some(metadata) = metadata {
        score += calculate_metadata_bonus(metadata) * METADATA_BONUS_WEIGHT;
    }

    score.clamp(0.0, 1.0)
}
