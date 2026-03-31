//! Threshold constants for PDF-to-Markdown spatial analysis.

/// Maximum word count for a paragraph to qualify as a heading.
pub(super) const MAX_HEADING_WORD_COUNT: usize = 20;
/// Maximum distance multiplier relative to average inter-cluster gap for heading assignment.
pub(super) const MAX_HEADING_DISTANCE_MULTIPLIER: f32 = 2.0;
/// Minimum ratio of heading font size to body font size (heading must be this much larger).
/// 1.15 captures LaTeX \subsection (12pt vs 10pt body = 1.2 ratio).
pub(super) const MIN_HEADING_FONT_RATIO: f32 = 1.15;
/// Minimum absolute font-size difference (in points) between heading and body.
/// 1.5pt captures academic sub-headings (11.5pt vs 10pt body).
pub(super) const MIN_HEADING_FONT_GAP: f32 = 1.5;
/// Fraction of page height to exclude from top (page headers).
pub(super) const PAGE_TOP_MARGIN_FRACTION: f32 = 0.06;
/// Fraction of page height to exclude from bottom (page footers/numbers).
pub(super) const PAGE_BOTTOM_MARGIN_FRACTION: f32 = 0.05;
/// Minimum font size (in points) for a segment to be included in analysis.
/// Segments below this size are likely artifacts (embedded images, symbols, noise).
pub(super) const MIN_FONT_SIZE: f32 = 4.0;
/// Maximum word count for a bold paragraph to be promoted to a section heading.
pub(super) const MAX_BOLD_HEADING_WORD_COUNT: usize = 15;
/// Fraction of the maximum right edge that a line must reach to be considered "full"
/// (used for dehyphenation to avoid false joins on short/indented lines).
pub(super) const FULL_LINE_FRACTION: f32 = 0.85;
