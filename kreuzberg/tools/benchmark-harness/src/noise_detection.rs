//! Noise and dirt detection for markdown extraction output.
//!
//! Detects common quality issues in extracted markdown such as HTML remnants,
//! garbled text, broken tables, page number artifacts, and other extraction
//! artifacts. All heuristics operate on the raw markdown string, line by line,
//! skipping content inside fenced code blocks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single noise issue found in the markdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseIssue {
    /// The kind of noise detected.
    pub kind: NoiseKind,
    /// 1-indexed line number where the issue was found.
    pub line: usize,
    /// ~80 char preview of the offending line.
    pub context: String,
    /// Severity of the issue.
    pub severity: Severity,
}

/// Categories of noise that can be detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoiseKind {
    /// HTML tags found outside code blocks.
    HtmlRemnant,
    /// Runs of 4+ consecutive blank lines.
    ExcessiveWhitespace,
    /// Lines with high non-ASCII ratio or consecutive punctuation.
    GarbledText,
    /// Heading markers with no content text.
    EmptyHeading,
    /// Pipe tables with inconsistent column counts.
    BrokenTable,
    /// List markers (`-`, `*`, `+`, `1.`) with no content.
    OrphanedListMarker,
    /// Standalone small numbers that look like page numbers.
    PageNumberArtifact,
    /// Lines repeated 10+ times at regular intervals in the document.
    HeaderFooterRepetition,
    /// Footnote references without matching definitions.
    DanglingReference,
    /// More headings than paragraphs (heading-heavy document).
    ExcessiveHeadingDensity,
    /// Unresolved HTML entities like `&#10;` or `&amp;` outside code blocks.
    UnresolvedHtmlEntity,
}

impl NoiseKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::HtmlRemnant => "HtmlRemnant",
            Self::ExcessiveWhitespace => "ExcessiveWhitespace",
            Self::GarbledText => "GarbledText",
            Self::EmptyHeading => "EmptyHeading",
            Self::BrokenTable => "BrokenTable",
            Self::OrphanedListMarker => "OrphanedListMarker",
            Self::PageNumberArtifact => "PageNumberArtifact",
            Self::HeaderFooterRepetition => "HeaderFooterRepetition",
            Self::DanglingReference => "DanglingReference",
            Self::ExcessiveHeadingDensity => "ExcessiveHeadingDensity",
            Self::UnresolvedHtmlEntity => "UnresolvedHtmlEntity",
        }
    }
}

/// Severity levels for noise issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    /// Informational — minor cosmetic issues.
    Info,
    /// Warning — likely extraction artifacts.
    Warning,
    /// Error — definite extraction failures.
    Error,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }
}

/// Full diagnostic report for a markdown document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    /// All noise issues found.
    pub issues: Vec<NoiseIssue>,
    /// Aggregated summary.
    pub summary: NoiseSummary,
}

/// Aggregated summary of noise issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseSummary {
    /// Total number of issues found.
    pub total_issues: usize,
    /// Issue counts grouped by kind.
    pub by_kind: HashMap<String, usize>,
    /// Issue counts grouped by severity.
    pub by_severity: HashMap<String, usize>,
    /// Overall noise score: 0.0 = clean, 1.0 = extremely noisy.
    pub noise_score: f64,
}

/// Represents a range of lines inside a fenced code block.
#[derive(Debug, Clone, Copy)]
struct CodeRange {
    start: usize, // inclusive, 0-indexed
    end: usize,   // inclusive, 0-indexed
}

/// Returns true if the given 0-indexed line is inside any code range.
fn in_code_block(line_idx: usize, code_ranges: &[CodeRange]) -> bool {
    code_ranges.iter().any(|r| line_idx >= r.start && line_idx <= r.end)
}

/// Identifies fenced code block ranges (``` or ~~~) using a simple state machine.
/// Also detects indented code blocks (4+ space or 1+ tab indentation on
/// consecutive lines, preceded by a blank line).
fn find_code_ranges(lines: &[&str]) -> Vec<CodeRange> {
    let mut ranges = Vec::new();
    let mut in_fence = false;
    let mut fence_start = 0;

    // First pass: fenced code blocks
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            if in_fence {
                ranges.push(CodeRange {
                    start: fence_start,
                    end: i,
                });
                in_fence = false;
            } else {
                fence_start = i;
                in_fence = true;
            }
        }
    }

    // Second pass: indented code blocks (4 spaces or 1 tab, preceded by blank line)
    let mut i = 0;
    while i < lines.len() {
        // Skip lines already inside fenced code blocks
        if in_code_block(i, &ranges) {
            i += 1;
            continue;
        }

        let is_indented_code = lines[i].starts_with("    ") || lines[i].starts_with('\t');
        if is_indented_code {
            // Check that it's preceded by a blank line or start of document
            let preceded_by_blank = i == 0 || lines[i - 1].trim().is_empty();
            if preceded_by_blank {
                let block_start = i;
                // Consume all consecutive indented or blank lines
                while i < lines.len()
                    && (lines[i].starts_with("    ") || lines[i].starts_with('\t') || lines[i].trim().is_empty())
                {
                    i += 1;
                }
                // Only mark as code if we had at least one indented line
                // (block_start was already verified as indented)
                let block_end = i.saturating_sub(1);
                ranges.push(CodeRange {
                    start: block_start,
                    end: block_end,
                });
                continue;
            }
        }
        i += 1;
    }

    ranges
}

/// Truncates a string to approximately `max_len` characters for context previews.
/// Uses char boundaries to avoid panicking on multi-byte UTF-8 sequences.
fn truncate_context(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        // Find a valid char boundary at or before max_len
        let end = s.floor_char_boundary(max_len);
        format!("{}...", &s[..end])
    }
}

/// Returns true if ALL occurrences of `tag` in `lower` are preceded by a backslash,
/// indicating they are markdown-escaped and not real HTML.
fn all_tag_occurrences_escaped(lower: &str, tag: &str) -> bool {
    let mut start = 0;
    let mut found_any = false;
    while let Some(pos) = lower[start..].find(tag) {
        let abs_pos = start + pos;
        found_any = true;
        // Check if preceded by backslash
        if abs_pos == 0 || lower.as_bytes()[abs_pos - 1] != b'\\' {
            return false;
        }
        start = abs_pos + tag.len();
    }
    found_any // should always be true since caller verified contains()
}

/// Detects HTML tags outside code blocks.
///
/// Skips tags that are backslash-escaped (e.g., `\<br\>`) since those are
/// literal text in markdown, not actual HTML remnants.
fn detect_html_remnants(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let html_tags = [
        "<table", "</table", "<tr", "</tr", "<td", "</td", "<th", "</th", "<div", "</div", "<span", "</span", "<p>",
        "</p>", "<p ", "<br", "<b>", "</b>", "<strong", "</strong", "<i>", "</i>", "<em", "</em", "<a ", "</a>", "<a>",
        "<img", "<pre", "</pre", "<code", "</code", "<ul", "</ul", "<ol", "</ol", "<li", "</li", "<h1", "</h1", "<h2",
        "</h2", "<h3", "</h3", "<h4", "</h4", "<h5", "</h5", "<h6", "</h6", "<sup", "</sup", "<sub", "</sub",
    ];

    let mut issues = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let lower = line.to_lowercase();
        for tag in &html_tags {
            if lower.contains(tag) && !all_tag_occurrences_escaped(&lower, tag) {
                issues.push(NoiseIssue {
                    kind: NoiseKind::HtmlRemnant,
                    line: i + 1,
                    context: truncate_context(line, 80),
                    severity: Severity::Warning,
                });
                break; // one issue per line
            }
        }
    }
    issues
}

/// Detects runs of 4+ consecutive blank lines.
fn detect_excessive_whitespace(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut issues = Vec::new();
    let mut blank_run_start: Option<usize> = None;
    let mut blank_count = 0;

    let flush_blank_run = |issues: &mut Vec<NoiseIssue>, count: usize, run_start: Option<usize>| {
        if let Some(start) = run_start
            && count >= 4
        {
            issues.push(NoiseIssue {
                kind: NoiseKind::ExcessiveWhitespace,
                line: start + 1,
                context: format!("{count} consecutive blank lines"),
                severity: Severity::Info,
            });
        }
    };

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            flush_blank_run(&mut issues, blank_count, blank_run_start);
            blank_count = 0;
            blank_run_start = None;
            continue;
        }

        if line.trim().is_empty() {
            if blank_run_start.is_none() {
                blank_run_start = Some(i);
            }
            blank_count += 1;
        } else {
            flush_blank_run(&mut issues, blank_count, blank_run_start);
            blank_count = 0;
            blank_run_start = None;
        }
    }

    // Handle trailing blank lines
    flush_blank_run(&mut issues, blank_count, blank_run_start);

    issues
}

/// Returns true if the line is a markdown table separator row (e.g., `|---|---|`).
fn is_table_separator_row(trimmed: &str) -> bool {
    trimmed.starts_with('|') && trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
}

/// Returns true if the line is a markdown horizontal rule (`---`, `***`, `===`, `___`).
fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < 3 {
        return false;
    }
    let first = trimmed.chars().next().unwrap_or(' ');
    matches!(first, '-' | '*' | '=' | '_') && trimmed.chars().all(|c| c == first || c == ' ')
}

/// Characters that commonly appear in markdown structural punctuation and should
/// NOT trigger the consecutive-punctuation garbled-text heuristic.
/// Covers both block-level (`-`, `|`, `*`, etc.) and inline syntax (`!`, `[`, `]`,
/// `(`, `)`, `\`, `.`, `/`) plus HTML entity delimiters (`&`, `;`).
const MARKDOWN_STRUCTURAL_PUNCT: &[char] = &[
    '-', '|', '*', '_', '=', '~', ':', '#', '>', // block-level
    '.', '/', '!', '[', ']', '(', ')', '\\', // inline syntax
    '{', '}', // LaTeX equations, e.g. \sum_{k=0}^{n}
    '&', ';', // HTML entities
    '\'', '"', // quotes (RST underlines, attribute values)
    '+', // list markers, diff markers
];

/// Returns true if the character belongs to a recognized non-Latin script that
/// commonly appears in multilingual documents: Arabic, CJK, Cyrillic, Greek,
/// Hebrew, Devanagari, Thai, Korean Hangul, Japanese Kana, etc.
///
/// This is intentionally broad to avoid flagging legitimate multilingual content.
fn is_known_script_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        // Latin Extended / accented (not garbled)
        0x00C0..=0x024F |
        // Greek and Coptic
        0x0370..=0x03FF |
        // Cyrillic
        0x0400..=0x04FF |
        // Armenian
        0x0530..=0x058F |
        // Hebrew
        0x0590..=0x05FF |
        // Arabic (including supplement, extended-A)
        0x0600..=0x06FF | 0x0750..=0x077F | 0x08A0..=0x08FF |
        // Devanagari
        0x0900..=0x097F |
        // Bengali, Gurmukhi, Gujarati, Oriya, Tamil, Telugu, Kannada, Malayalam
        0x0980..=0x0DFF |
        // Thai
        0x0E00..=0x0E7F |
        // Georgian
        0x10A0..=0x10FF |
        // Korean Hangul Jamo
        0x1100..=0x11FF |
        // General punctuation (em dash, en dash, bullets, etc.)
        0x2000..=0x206F |
        // Superscripts and subscripts
        0x2070..=0x209F |
        // Currency symbols
        0x20A0..=0x20CF |
        // Letterlike symbols
        0x2100..=0x214F |
        // Number forms (fractions)
        0x2150..=0x218F |
        // Arrows
        0x2190..=0x21FF |
        // Mathematical operators
        0x2200..=0x22FF |
        // Enclosed alphanumerics
        0x2460..=0x24FF |
        // Geometric shapes
        0x25A0..=0x25FF |
        // Miscellaneous symbols
        0x2600..=0x26FF |
        // Dingbats
        0x2700..=0x27BF |
        // CJK Unified Ideographs (+ extension A + compatibility)
        0x2E80..=0x9FFF |
        // Korean Hangul Syllables
        0xAC00..=0xD7AF |
        // CJK Compatibility Ideographs
        0xF900..=0xFAFF |
        // Arabic Presentation Forms
        0xFB50..=0xFDFF | 0xFE70..=0xFEFF |
        // CJK extension B+ (supplementary)
        0x20000..=0x2FA1F
    )
}

/// Returns true if the non-ASCII characters on this line are predominantly from
/// recognized scripts (not mojibake / encoding errors).
fn is_legitimate_multilingual(non_ws_chars: &[char]) -> bool {
    let non_ascii_chars: Vec<char> = non_ws_chars.iter().copied().filter(|c| !c.is_ascii()).collect();
    if non_ascii_chars.is_empty() {
        return true;
    }
    let known_count = non_ascii_chars.iter().filter(|c| is_known_script_char(**c)).count();
    // If 80%+ of non-ASCII chars are from known scripts, it's likely legit
    (known_count as f64 / non_ascii_chars.len() as f64) >= 0.8
}

/// Detects garbled text: lines with >70% non-ASCII (unless from known scripts)
/// or 4+ consecutive non-structural punctuation.
fn detect_garbled_text(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut issues = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Skip table separator rows, horizontal rules, and markdown image references —
        // they are legitimate markdown, not garbled text.
        if is_table_separator_row(trimmed) || is_horizontal_rule(trimmed) || is_markdown_image(trimmed) {
            continue;
        }

        let non_ws_chars: Vec<char> = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
        if non_ws_chars.is_empty() {
            continue;
        }

        // Check non-ASCII ratio, but only flag if the non-ASCII characters are NOT
        // from recognized scripts (Arabic, CJK, Cyrillic, Greek, etc.)
        let non_ascii_count = non_ws_chars.iter().filter(|c| !c.is_ascii()).count();
        let ratio = non_ascii_count as f64 / non_ws_chars.len() as f64;
        if ratio > 0.7 && !is_legitimate_multilingual(&non_ws_chars) {
            issues.push(NoiseIssue {
                kind: NoiseKind::GarbledText,
                line: i + 1,
                context: truncate_context(line, 80),
                severity: Severity::Warning,
            });
            continue;
        }

        // Check for 4+ consecutive punctuation, excluding markdown structural characters.
        // Common markdown patterns like `---`, `***`, `|||`, `[^`, `$$` use structural
        // punctuation and should not be flagged.
        let mut consecutive_punct = 0;
        let mut has_punct_run = false;
        for ch in trimmed.chars() {
            if ch.is_ascii_punctuation() && !MARKDOWN_STRUCTURAL_PUNCT.contains(&ch) {
                consecutive_punct += 1;
                if consecutive_punct >= 4 {
                    has_punct_run = true;
                    break;
                }
            } else {
                consecutive_punct = 0;
            }
        }
        if has_punct_run {
            issues.push(NoiseIssue {
                kind: NoiseKind::GarbledText,
                line: i + 1,
                context: truncate_context(line, 80),
                severity: Severity::Warning,
            });
        }
    }

    issues
}

/// Detects empty headings (e.g., `# ` with no content).
fn detect_empty_headings(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut issues = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let trimmed = line.trim();
        // Match ^#{1,6}\s*$
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if (1..=6).contains(&hash_count) {
                let rest = &trimmed[hash_count..];
                if rest.trim().is_empty() {
                    issues.push(NoiseIssue {
                        kind: NoiseKind::EmptyHeading,
                        line: i + 1,
                        context: truncate_context(line, 80),
                        severity: Severity::Error,
                    });
                }
            }
        }
    }

    issues
}

/// Counts unescaped pipe characters in a table row.
///
/// Escaped pipes (`\|`) are literal pipe characters inside cell content and
/// should NOT be counted as column delimiters.
fn count_unescaped_pipes(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut count = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'|' && (i == 0 || bytes[i - 1] != b'\\') {
            count += 1;
        }
    }
    count
}

/// Detects broken pipe tables with inconsistent column counts.
///
/// Uses [`count_unescaped_pipes`] to ignore escaped pipes (`\|`) that appear
/// as literal content inside table cells.
fn detect_broken_tables(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut issues = Vec::new();

    let mut table_start: Option<usize> = None;
    let mut header_col_count: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            // End any open table
            table_start = None;
            header_col_count = None;
            continue;
        }

        let trimmed = line.trim();
        if trimmed.starts_with('|') {
            let col_count = count_unescaped_pipes(trimmed);
            if table_start.is_none() {
                // Start of a new table
                table_start = Some(i);
                header_col_count = Some(col_count);
            } else if let Some(expected) = header_col_count {
                // Skip separator rows (e.g., |---|---|)
                let is_separator = trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c == ' ');
                if !is_separator && col_count != expected {
                    issues.push(NoiseIssue {
                        kind: NoiseKind::BrokenTable,
                        line: i + 1,
                        context: truncate_context(line, 80),
                        severity: Severity::Warning,
                    });
                }
            }
        } else {
            // Non-table line ends the current table
            table_start = None;
            header_col_count = None;
        }
    }

    issues
}

/// Detects orphaned list markers with no content.
fn detect_orphaned_list_markers(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut issues = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let trimmed = line.trim();

        // Unordered: -, *, + with nothing after
        let is_orphaned_unordered = (trimmed == "-" || trimmed == "*" || trimmed == "+")
            || (trimmed.len() >= 2
                && (trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ "))
                && trimmed[2..].trim().is_empty());

        // Ordered: digits followed by . and nothing else
        let is_orphaned_ordered = if let Some(dot_pos) = trimmed.find('.') {
            let before_dot = &trimmed[..dot_pos];
            let after_dot = &trimmed[dot_pos + 1..];
            !before_dot.is_empty() && before_dot.chars().all(|c| c.is_ascii_digit()) && after_dot.trim().is_empty()
        } else {
            false
        };

        if is_orphaned_unordered || is_orphaned_ordered {
            issues.push(NoiseIssue {
                kind: NoiseKind::OrphanedListMarker,
                line: i + 1,
                context: truncate_context(line, 80),
                severity: Severity::Warning,
            });
        }
    }

    issues
}

/// Detects standalone small numbers that look like page number artifacts.
///
/// Only flags when at least 5 sequential or near-sequential standalone numbers
/// exist AND the sequential numbers span at least 20 lines apart (page-like
/// spacing). Clustered numbers (like table cells) are not flagged.
fn detect_page_number_artifacts(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    // Collect candidate lines: standalone numbers 1-9999
    let mut candidates: Vec<(usize, u32)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let trimmed = line.trim();
        if let Ok(num) = trimmed.parse::<u32>()
            && (1..=9999).contains(&num)
            && trimmed.len() <= 4
        {
            candidates.push((i, num));
        }
    }

    if candidates.len() < 5 {
        return Vec::new();
    }

    // Check for sequential/near-sequential values and track their line span
    let values: Vec<u32> = candidates.iter().map(|(_, v)| *v).collect();
    let line_indices: Vec<usize> = candidates.iter().map(|(i, _)| *i).collect();
    let mut sequential_count = 0;
    let mut sequential_min_line = usize::MAX;
    let mut sequential_max_line = 0usize;
    for (idx, window) in values.windows(2).enumerate() {
        let diff = window[1].saturating_sub(window[0]);
        if (1..=3).contains(&diff) {
            sequential_count += 1;
            sequential_min_line = sequential_min_line.min(line_indices[idx]);
            sequential_max_line = sequential_max_line.max(line_indices[idx + 1]);
        }
    }

    // Need at least 4 sequential pairs (5 sequential numbers)
    if sequential_count < 4 {
        return Vec::new();
    }

    // Require the sequential numbers to span at least 20 lines (page-like spacing).
    // Clustered numbers (e.g., table cells on adjacent lines) won't pass this check.
    let span = sequential_max_line.saturating_sub(sequential_min_line);
    if span < 20 {
        return Vec::new();
    }

    candidates
        .iter()
        .map(|(i, _)| NoiseIssue {
            kind: NoiseKind::PageNumberArtifact,
            line: i + 1,
            context: truncate_context(lines[*i], 80),
            severity: Severity::Info,
        })
        .collect()
}

/// Returns true if the line is a pipe table row (starts with `|`).
fn is_pipe_table_row(trimmed: &str) -> bool {
    trimmed.starts_with('|')
}

/// Returns true if the line is a markdown image reference like `![alt](url)` or `![]()`.
/// Matches lines that consist entirely of a markdown image pattern (possibly with
/// surrounding whitespace already trimmed).
fn is_image_placeholder(trimmed: &str) -> bool {
    trimmed.starts_with("![")
}

/// Returns true if the line is a markdown image reference `![...](...)`
/// or an escaped variant `\[...\](...)`. Used to skip garbled-text detection
/// on lines that are purely image/link markup.
fn is_markdown_image(trimmed: &str) -> bool {
    // Standard markdown image: ![...](...) possibly with trailing text
    if trimmed.starts_with("![") {
        return true;
    }
    // Escaped markdown link/image: \[...\](...) or \[\![...\](...) from Wikipedia extraction
    if trimmed.starts_with("\\[") || trimmed.starts_with("\\!") {
        return true;
    }
    false
}

/// Returns true if the line looks like an RST grid table border.
///
/// RST grid table borders start with `+` and contain only `+`, `-`, `=`, `|`,
/// and spaces. Examples: `+---+---+---+`, `+===+===+`.
fn is_rst_grid_table_border(trimmed: &str) -> bool {
    trimmed.starts_with('+') && trimmed.chars().all(|c| matches!(c, '+' | '-' | '=' | '|' | ' '))
}

/// Detects lines that repeat 10+ times in the document (header/footer repetition).
///
/// Skips pipe table rows, image placeholders, table separator rows, and lines
/// with fewer than 20 non-whitespace characters (too short to be a meaningful
/// header/footer candidate).
///
/// To reduce false positives from legitimate repetitive content (e.g., ISO
/// standard column headers, Wikipedia navbox rows), candidates must also pass
/// a **periodicity check**: their occurrences must be roughly evenly spaced
/// (std_dev / mean_gap <= 0.5). Real page headers/footers appear at regular
/// intervals corresponding to page breaks, while content repetition is
/// irregular.
///
/// Lines that look like table column headers (all words Title Case or
/// UPPERCASE, under 40 chars) are also excluded.
///
/// Results are capped at 30 issues per document to avoid inflating noise
/// counts.
fn detect_header_footer_repetition(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    const MIN_OCCURRENCES: usize = 10;
    const MIN_NON_WS_CHARS: usize = 20;
    const MAX_ISSUES: usize = 30;
    const MAX_PERIODICITY_RATIO: f64 = 0.5;
    const TABLE_HEADER_MAX_LEN: usize = 40;

    let mut line_counts: HashMap<&str, Vec<usize>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Skip pipe table rows (including separator rows like |---|---|)
        if is_pipe_table_row(trimmed) {
            continue;
        }

        // Skip image placeholders
        if is_image_placeholder(trimmed) {
            continue;
        }

        // Skip RST grid table borders (lines of +, -, =, |, spaces)
        if is_rst_grid_table_border(trimmed) {
            continue;
        }

        // Require minimum non-whitespace characters to be a header/footer candidate
        let non_ws_count = trimmed.chars().filter(|c| !c.is_whitespace()).count();
        if non_ws_count < MIN_NON_WS_CHARS {
            continue;
        }

        // Skip lines that look like table column headers: all words are Title Case
        // or UPPERCASE and the line is short.
        if trimmed.len() <= TABLE_HEADER_MAX_LEN && looks_like_table_header(trimmed) {
            continue;
        }

        line_counts.entry(trimmed).or_default().push(i);
    }

    let mut issues = Vec::new();
    for (content, positions) in &line_counts {
        if positions.len() >= MIN_OCCURRENCES && is_periodic(positions, MAX_PERIODICITY_RATIO) {
            for &pos in positions {
                issues.push(NoiseIssue {
                    kind: NoiseKind::HeaderFooterRepetition,
                    line: pos + 1,
                    context: truncate_context(content, 80),
                    severity: Severity::Warning,
                });
            }
        }
    }

    // Sort by line number for deterministic output
    issues.sort_by_key(|issue| issue.line);

    // Cap total issues to avoid inflating noise counts
    issues.truncate(MAX_ISSUES);
    issues
}

/// Returns true if `positions` (sorted line indices) are roughly evenly spaced.
///
/// Computes the coefficient of variation (std_dev / mean) of the gaps between
/// consecutive positions. A ratio <= `max_ratio` indicates periodic repetition
/// (like page headers). A higher ratio means the repetition is irregular (like
/// repeated table content).
///
/// Returns `true` (periodic) when there are fewer than 3 positions, since we
/// cannot meaningfully assess periodicity.
fn is_periodic(positions: &[usize], max_ratio: f64) -> bool {
    if positions.len() < 3 {
        return true;
    }

    let gaps: Vec<f64> = positions.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    let n = gaps.len() as f64;
    let mean = gaps.iter().sum::<f64>() / n;

    if mean < 1.0 {
        // All occurrences are adjacent — not a header/footer pattern
        return false;
    }

    let variance = gaps.iter().map(|g| (g - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();
    let cv = std_dev / mean;

    cv <= max_ratio
}

/// Returns true if the line looks like a table column header.
///
/// A table header line has ALL words either Title Case (first char uppercase,
/// rest lowercase) or fully UPPERCASE. This catches patterns like
/// "Item Content", "Remark", "Prerequisite", "TEST CASE ID".
fn looks_like_table_header(line: &str) -> bool {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.is_empty() {
        return false;
    }

    words.iter().all(|word| {
        let mut chars = word.chars();
        match chars.next() {
            Some(first) => {
                if !first.is_alphabetic() {
                    return false;
                }
                let rest: String = chars.collect();
                let is_title_case =
                    first.is_uppercase() && rest.chars().all(|c| !c.is_alphabetic() || c.is_lowercase());
                let is_upper = first.is_uppercase() && rest.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
                is_title_case || is_upper
            }
            None => true,
        }
    })
}

/// Detects footnote references `[^N]` without corresponding `[^N]:` definitions.
fn detect_dangling_references(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut references: Vec<(usize, String)> = Vec::new(); // (line_idx, label)
    let mut definitions: std::collections::HashSet<String> = std::collections::HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }

        let mut start = 0;
        while let Some(pos) = line[start..].find("[^") {
            let abs_pos = start + pos;
            let after = &line[abs_pos + 2..];
            if let Some(close) = after.find(']') {
                let label = after[..close].to_string();
                let after_close = &after[close + 1..];
                if after_close.starts_with(':') {
                    definitions.insert(label);
                } else if !label.is_empty() && !after_close.starts_with('(') {
                    // Skip empty labels and [^](url) which are regular links, not footnotes
                    references.push((i, label));
                }
                start = abs_pos + 2 + close + 1;
            } else {
                break;
            }
        }
    }

    references
        .into_iter()
        .filter(|(_, label)| !definitions.contains(label))
        .map(|(i, _)| NoiseIssue {
            kind: NoiseKind::DanglingReference,
            line: i + 1,
            context: truncate_context(lines[i], 80),
            severity: Severity::Warning,
        })
        .collect()
}

/// Detects excessive heading density (more than 2x headings vs paragraphs when heading count > 10).
fn detect_excessive_heading_density(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    let mut heading_count = 0usize;
    let mut paragraph_count = 0usize;

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if (1..=6).contains(&hash_count) {
                heading_count += 1;
                continue;
            }
        }

        // Skip list markers, table rows
        if trimmed.starts_with('|')
            || trimmed.starts_with("- ")
            || trimmed.starts_with("* ")
            || trimmed.starts_with("+ ")
            || (trimmed.len() >= 2 && trimmed.as_bytes()[0].is_ascii_digit() && trimmed.contains(". "))
        {
            continue;
        }

        paragraph_count += 1;
    }

    if heading_count > 2 * paragraph_count && heading_count > 10 {
        vec![NoiseIssue {
            kind: NoiseKind::ExcessiveHeadingDensity,
            line: 1,
            context: format!("{heading_count} headings vs {paragraph_count} paragraphs"),
            severity: Severity::Warning,
        }]
    } else {
        Vec::new()
    }
}

/// Detects unresolved HTML entities like `&#10;`, `&#x0A;`, `&amp;`, `&nbsp;` outside code blocks.
///
/// These are extraction artifacts where the HTML-to-markdown conversion failed to
/// decode character references.
fn detect_unresolved_html_entities(lines: &[&str], code_ranges: &[CodeRange]) -> Vec<NoiseIssue> {
    // Match numeric (&#123;) and named (&amp;) HTML entities.
    // We use a simple scan rather than pulling in the regex crate.
    let mut issues = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if in_code_block(i, code_ranges) {
            continue;
        }

        let bytes = line.as_bytes();
        let mut pos = 0;
        while pos < bytes.len() {
            if bytes[pos] == b'&' {
                let rest = &line[pos..];
                if let Some(semi) = rest.find(';') {
                    // Limit entity length to avoid matching across large spans
                    if semi <= 10 {
                        let entity = &rest[1..semi];
                        let is_numeric = entity.starts_with('#')
                            && entity.len() > 1
                            && entity[1..].chars().all(|c| c.is_ascii_digit() || c == 'x' || c == 'X');
                        let is_named = !entity.is_empty() && entity.chars().all(|c| c.is_ascii_alphanumeric());

                        if is_numeric || is_named {
                            issues.push(NoiseIssue {
                                kind: NoiseKind::UnresolvedHtmlEntity,
                                line: i + 1,
                                context: truncate_context(line, 80),
                                severity: Severity::Warning,
                            });
                            // One issue per line is enough
                            break;
                        }
                    }
                    pos += semi + 1;
                } else {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }
    }

    issues
}

/// Runs all noise detection heuristics and produces a diagnostic report.
pub fn detect_noise(markdown: &str) -> DiagnosticReport {
    let lines: Vec<&str> = markdown.lines().collect();
    let code_ranges = find_code_ranges(&lines);

    let mut issues = Vec::new();
    issues.extend(detect_html_remnants(&lines, &code_ranges));
    issues.extend(detect_excessive_whitespace(&lines, &code_ranges));
    issues.extend(detect_garbled_text(&lines, &code_ranges));
    issues.extend(detect_empty_headings(&lines, &code_ranges));
    issues.extend(detect_broken_tables(&lines, &code_ranges));
    issues.extend(detect_orphaned_list_markers(&lines, &code_ranges));
    issues.extend(detect_page_number_artifacts(&lines, &code_ranges));
    issues.extend(detect_header_footer_repetition(&lines, &code_ranges));
    issues.extend(detect_dangling_references(&lines, &code_ranges));
    issues.extend(detect_excessive_heading_density(&lines, &code_ranges));
    issues.extend(detect_unresolved_html_entities(&lines, &code_ranges));

    let total_lines = lines.len();
    let summary = build_summary(&issues, total_lines);

    DiagnosticReport { issues, summary }
}

/// Builds an aggregated summary from a list of issues.
fn build_summary(issues: &[NoiseIssue], total_lines: usize) -> NoiseSummary {
    let mut by_kind: HashMap<String, usize> = HashMap::new();
    let mut by_severity: HashMap<String, usize> = HashMap::new();

    let mut error_count = 0usize;
    let mut warning_count = 0usize;
    let mut info_count = 0usize;

    for issue in issues {
        *by_kind.entry(issue.kind.as_str().to_string()).or_insert(0) += 1;
        *by_severity.entry(issue.severity.as_str().to_string()).or_insert(0) += 1;

        match issue.severity {
            Severity::Error => error_count += 1,
            Severity::Warning => warning_count += 1,
            Severity::Info => info_count += 1,
        }
    }

    let weighted = error_count as f64 * 0.3 + warning_count as f64 * 0.1 + info_count as f64 * 0.02;
    let denominator = (total_lines / 50).max(1) as f64;
    let noise_score = (weighted / denominator).min(1.0);

    NoiseSummary {
        total_issues: issues.len(),
        by_kind,
        by_severity,
        noise_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_markdown() {
        let md = "\
# Hello World

This is a paragraph with some text.

## Section Two

Another paragraph here with more content.

- Item one
- Item two
- Item three
";
        let report = detect_noise(md);
        assert!(
            report.issues.is_empty(),
            "Expected 0 issues for clean markdown, got: {:?}",
            report.issues
        );
        assert_eq!(report.summary.noise_score, 0.0);
    }

    #[test]
    fn test_html_remnant_detection() {
        let md = "\
# Title

<div class=\"content\">Some text</div>

More text here.
";
        let report = detect_noise(md);
        let html_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HtmlRemnant)
            .collect();
        assert!(!html_issues.is_empty(), "Expected HTML remnant issues");
        assert_eq!(html_issues[0].severity, Severity::Warning);
    }

    #[test]
    fn test_empty_heading() {
        let md = "\
#

Some content here.
";
        let report = detect_noise(md);
        let heading_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::EmptyHeading)
            .collect();
        assert_eq!(heading_issues.len(), 1);
        assert_eq!(heading_issues[0].severity, Severity::Error);
        assert_eq!(heading_issues[0].line, 1);
    }

    #[test]
    fn test_broken_table() {
        let md = "\
| Col1 | Col2 | Col3 |
|------|------|------|
| a | b | c |
| d | e |
";
        let report = detect_noise(md);
        let table_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::BrokenTable)
            .collect();
        assert!(!table_issues.is_empty(), "Expected broken table issues");
        assert_eq!(table_issues[0].severity, Severity::Warning);
    }

    #[test]
    fn test_code_block_skipped() {
        let md = "\
# Title

```html
<div>This should not be flagged</div>
<table><tr><td>Also not flagged</td></tr></table>
```

Normal paragraph.
";
        let report = detect_noise(md);
        let html_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HtmlRemnant)
            .collect();
        assert!(
            html_issues.is_empty(),
            "HTML inside code blocks should not be flagged, got: {:?}",
            html_issues
        );
    }

    #[test]
    fn test_garbled_text() {
        // Use private-use area characters and replacement chars to simulate
        // real mojibake, which should still be flagged. Latin-Extended chars
        // (like a-umlaut, o-umlaut) are legitimate and should NOT be flagged.
        let md = "\
# Title

\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}

Normal text here.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            !garbled.is_empty(),
            "Expected garbled text detection for mojibake / replacement characters"
        );
    }

    #[test]
    fn test_german_umlauts_not_flagged_as_garbled() {
        // German text with umlauts is legitimate multilingual content
        let md = "\
# Title

\u{00e4}\u{00f6}\u{00fc}\u{00e4}\u{00f6}\u{00fc}\u{00e4}\u{00f6}\u{00fc}\u{00e4}\u{00f6}\u{00fc}\u{00e4}\u{00f6}\u{00fc}\u{00e4}\u{00f6}\u{00fc}\u{00e4}\u{00f6}\u{00fc}

Normal text here.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "German umlauts should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_page_numbers() {
        // Need 5+ sequential numbers spanning 20+ lines to trigger detection
        let mut lines = vec!["# Title".to_string(), String::new()];
        for page in 1..=6 {
            lines.push(format!("Page {page} content with enough text to fill the space."));
            lines.push(String::new());
            lines.push(String::new());
            lines.push(String::new());
            lines.push(page.to_string());
            lines.push(String::new());
        }
        lines.push("Final text.".to_string());
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let page_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::PageNumberArtifact)
            .collect();
        assert!(
            !page_issues.is_empty(),
            "Expected page number artifact detection for sequential standalone numbers"
        );
        assert_eq!(page_issues.len(), 6);
    }

    #[test]
    fn test_clustered_numbers_not_flagged_as_page_numbers() {
        // Numbers on adjacent lines (table-like data) should NOT be flagged
        let md = "\
# Table Data

1
2
3
4
5
6
7

Some text after.
";
        let report = detect_noise(md);
        let page_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::PageNumberArtifact)
            .collect();
        assert!(
            page_issues.is_empty(),
            "Clustered sequential numbers (table data) should not be flagged as page numbers, got: {:?}",
            page_issues
        );
    }

    #[test]
    fn test_dangling_footnote() {
        let md = "\
# Title

This has a reference[^1] and another[^2].

[^1]: This is defined.
";
        let report = detect_noise(md);
        let dangling: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::DanglingReference)
            .collect();
        assert!(!dangling.is_empty(), "Expected dangling reference for [^2]");
        // [^1] should not be flagged since it has a definition
        assert!(
            dangling.iter().all(|i| {
                let line = &md.lines().collect::<Vec<_>>()[i.line - 1];
                line.contains("[^2]")
            }),
            "Only [^2] should be flagged as dangling"
        );
    }

    #[test]
    fn test_empty_input() {
        let report = detect_noise("");
        assert!(report.issues.is_empty());
        assert_eq!(report.summary.total_issues, 0);
        assert_eq!(report.summary.noise_score, 0.0);
    }

    // ---- False-positive regression tests ----

    #[test]
    fn test_pipe_table_rows_not_flagged_as_header_footer() {
        // Pipe table rows like `|  |  |  |` should NOT be flagged as repetition
        let md = "\
| Col1 | Col2 | Col3 |
|------|------|------|
|  |  |  |
|  |  |  |
|  |  |  |
|  |  |  |
|  |  |  |
|  |  |  |
";
        let report = detect_noise(md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "Pipe table rows should not be flagged as header/footer repetition, got: {:?}",
            rep_issues
        );
    }

    #[test]
    fn test_image_placeholders_not_flagged_as_header_footer() {
        let md = "\
# Gallery

![](image1.png)
![](image2.png)
![](image3.png)
![](image4.png)
![](image5.png)
![](image6.png)
";
        let report = detect_noise(md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "Image placeholders should not be flagged as header/footer repetition, got: {:?}",
            rep_issues
        );
    }

    #[test]
    fn test_table_separator_not_flagged_as_garbled() {
        let md = "\
| Col1 | Col2 |
|------|------|
| a    | b    |
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Table separator rows should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_horizontal_rule_not_flagged_as_garbled() {
        let md = "\
# Section

---

More text.

***

Even more text.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Horizontal rules should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_arabic_text_not_flagged_as_garbled() {
        // Arabic text is 100% non-ASCII but is legitimate multilingual content.
        // At 70% threshold it would be flagged, but we want to verify that
        // mixed content below 70% is NOT flagged.
        let md = "\
# Document

Some English text mixed with \u{0645}\u{0631}\u{062d}\u{0628}\u{0627} Arabic words in a sentence.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Mixed Arabic/English text below 70% non-ASCII should not be flagged, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_cjk_text_below_threshold_not_flagged() {
        // CJK text mixed with enough ASCII to stay below 70%
        let md = "\
# Document

This line has some CJK chars \u{4f60}\u{597d} mixed with English text here.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Mixed CJK/English text below 70% non-ASCII should not be flagged, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_truly_garbled_text_still_flagged() {
        // Mojibake using Private Use Area and misc symbols that don't belong
        // to any recognized script. These should be flagged as garbled.
        let md = "\
# Title

\u{FFFD}\u{E000}\u{FFFD}\u{E001}\u{FFFD}\u{E002}\u{FFFD}\u{E003}\u{FFFD}\u{E004}\u{FFFD}\u{E005}\u{FFFD}\u{E006}\u{FFFD}\u{E007}\u{FFFD}\u{E008}\u{FFFD}\u{E009}

Normal text here.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            !garbled.is_empty(),
            "Truly garbled text (Private Use Area / replacement chars) should still be flagged"
        );
    }

    #[test]
    fn test_markdown_structural_punct_not_flagged_as_garbled() {
        // Lines with markdown structural punctuation like `---`, `***`, `|||`, `$$`
        let md = "\
# Title

Some text with --- dashes in it.

Text with **bold** and ~~strike~~ formatting.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Markdown structural punctuation should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_short_repeated_lines_not_flagged_as_header_footer() {
        // Short lines (< 20 non-ws chars) repeated many times should not be flagged
        let md = "\
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text
Hello world text

Some other text here.
";
        let report = detect_noise(md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "Short repeated lines (< 20 non-ws chars) should not be flagged, got: {:?}",
            rep_issues
        );
    }

    #[test]
    fn test_genuine_header_footer_repetition_still_flagged() {
        // Genuine header/footer text repeated 10+ times at regular intervals should be flagged
        let md = "\
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 1
Some content here.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 2
More content here.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 3
Even more content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 4
Still more content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 5
Additional content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 6
Further content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 7
Yet more content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 8
Content eight.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 9
Content nine.
Copyright 2024 Acme Corporation All Rights Reserved
";
        let report = detect_noise(md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            !rep_issues.is_empty(),
            "Genuine header/footer repetition (10+ times, periodic) should be flagged"
        );
        assert_eq!(rep_issues.len(), 10);
    }

    #[test]
    fn test_nine_repetitions_no_longer_flagged() {
        // 9 repetitions should NOT be flagged (threshold is now 10)
        let md = "\
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 1
Some content here.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 2
More content here.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 3
Even more content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 4
Still more content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 5
Additional content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 6
Further content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 7
Yet more content.
Copyright 2024 Acme Corporation All Rights Reserved
# Chapter 8
Content eight.
Copyright 2024 Acme Corporation All Rights Reserved
";
        let report = detect_noise(md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "9 repetitions should not be flagged (threshold is 10), got: {:?}",
            rep_issues
        );
    }

    // ---- GarbledText false-positive regression tests ----

    #[test]
    fn test_empty_image_link_not_flagged_as_garbled() {
        // `![]()` contains `!`, `[`, `]`, `(`, `)` — should NOT be flagged
        let md = "\
# Gallery

![](image1.png)
![]()
![alt text](http://example.com/img.jpg)

Normal text.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Empty image links ![]() should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_escaped_markdown_links_not_flagged_as_garbled() {
        // Wikipedia extraction: `\[Big Machine Records\](/wiki/Big_Machine_Records)`
        let md = "\
# Wikipedia Article

\\[Big Machine Records\\](/wiki/Big_Machine_Records)
\\[Taylor Swift\\](/wiki/Taylor_Swift)

Normal text here.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Escaped markdown links should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_toc_dot_leaders_not_flagged_as_garbled() {
        // Table of contents with dot leaders
        let md = "\
# Table of Contents

Foreword .............. v
Chapter 1 ............ 1
Chapter 2 ............ 15
Appendix ............. 200
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "TOC dot leaders should not be flagged as garbled text, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_truly_garbled_punct_still_flagged() {
        // Truly garbled punctuation that is NOT markdown structural
        let md = "\
# Title

Some text with @@@@garbled content here.

Normal text.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            !garbled.is_empty(),
            "Non-structural consecutive punctuation (@@@@) should still be flagged as garbled text"
        );
    }

    // ---- UnresolvedHtmlEntity tests ----

    #[test]
    fn test_numeric_html_entity_detected() {
        let md = "\
# Document

This has an unresolved entity&#10;in the middle.

Normal text.
";
        let report = detect_noise(md);
        let entity_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::UnresolvedHtmlEntity)
            .collect();
        assert!(
            !entity_issues.is_empty(),
            "Numeric HTML entity &#10; should be detected as UnresolvedHtmlEntity"
        );
    }

    #[test]
    fn test_named_html_entity_detected() {
        let md = "\
# Document

This has &amp; and &nbsp; entities.

Normal text.
";
        let report = detect_noise(md);
        let entity_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::UnresolvedHtmlEntity)
            .collect();
        assert!(
            !entity_issues.is_empty(),
            "Named HTML entities &amp; and &nbsp; should be detected"
        );
    }

    #[test]
    fn test_html_entity_in_code_block_not_detected() {
        let md = "\
# Document

```html
This has &amp; entities in code.
```

Normal text.
";
        let report = detect_noise(md);
        let entity_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::UnresolvedHtmlEntity)
            .collect();
        assert!(
            entity_issues.is_empty(),
            "HTML entities inside code blocks should not be flagged, got: {:?}",
            entity_issues
        );
    }

    // ---- HeaderFooterRepetition heuristic regression tests ----

    #[test]
    fn test_iso_column_headers_not_flagged_as_header_footer() {
        // ISO-style column headers like "Item Content Description" appearing 200x
        // at irregular intervals should NOT be flagged — they are legitimate table
        // content, not page headers/footers. The table header filter catches Title
        // Case lines under 40 chars, and the min-char filter catches shorter ones.
        let mut lines = Vec::new();
        for i in 0..200 {
            lines.push(format!("## Test Case {i}"));
            lines.push("Item Content Description".to_string());
            lines.push("Prerequisite Condition".to_string());
            lines.push("Expected Test Result".to_string());
            // Irregular spacing: add extra lines for some entries
            if i % 3 == 0 {
                lines.push("Additional Notes Section".to_string());
                lines.push(String::new());
            }
            lines.push(format!("Test step {i}: verify the output is correct."));
            lines.push(String::new());
        }
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "ISO-style column headers at irregular intervals should not be flagged, got {} issues",
            rep_issues.len()
        );
    }

    #[test]
    fn test_periodic_real_headers_are_flagged() {
        // Real page headers appearing every ~50 lines (periodic) should be flagged.
        let mut lines = Vec::new();
        for page in 0..12 {
            lines.push("ACME Corporation - Internal Document - Confidential Draft".to_string());
            for j in 0..49 {
                lines.push(format!("Content line {j} of page {page} with enough text."));
            }
        }
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            !rep_issues.is_empty(),
            "Periodic real headers (every ~50 lines, 12 occurrences) should be flagged"
        );
        assert_eq!(rep_issues.len(), 12);
    }

    #[test]
    fn test_header_footer_cap_at_30_issues() {
        // Even with many periodic occurrences, results are capped at 30 issues.
        let mut lines = Vec::new();
        for page in 0..50 {
            lines.push("ACME Corporation - Internal Document - Confidential Draft".to_string());
            for j in 0..49 {
                lines.push(format!("Content line {j} of page {page} with enough text."));
            }
        }
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.len() <= 30,
            "Header/footer issues should be capped at 30, got {}",
            rep_issues.len()
        );
    }

    #[test]
    fn test_irregular_repetition_not_flagged() {
        // Content repeated many times but at completely irregular intervals should
        // NOT be flagged (fails periodicity check).
        let mut lines = Vec::new();
        let repeated = "This particular content line appears many times in the document";
        // Insert at irregular positions: clustered at the start and then sparse
        let positions = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 50, 200];
        let total_lines = 300;
        for i in 0..total_lines {
            if positions.contains(&i) {
                lines.push(repeated.to_string());
            } else {
                lines.push(format!("Regular content line number {i} in the document."));
            }
        }
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "Irregular (non-periodic) repetition should not be flagged, got {} issues",
            rep_issues.len()
        );
    }

    #[test]
    fn test_table_header_words_excluded() {
        // Lines where all words are Title Case and under 40 chars should not be flagged
        let mut lines = Vec::new();
        for i in 0..15 {
            lines.push("Test Case Identifier".to_string());
            for j in 0..4 {
                lines.push(format!("Content line {j} of section {i} with text."));
            }
        }
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "Title Case table headers under 40 chars should not be flagged, got {} issues",
            rep_issues.len()
        );
    }

    // ---- False-positive regression tests for escaped HTML, indented code, escaped pipes ----

    #[test]
    fn test_escaped_html_tags_not_flagged() {
        // Backslash-escaped HTML tags like \<br\> are literal text, not HTML remnants
        let md = "\
# Wikipedia Infobox

| Traded as | \\<br\\>Nasdaq: MSFT |
|---|---|
| Brands | \\<br\\>Windows, Xbox |
";
        let report = detect_noise(md);
        let html_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HtmlRemnant)
            .collect();
        assert!(
            html_issues.is_empty(),
            "Backslash-escaped HTML tags should not be flagged, got: {:?}",
            html_issues
        );
    }

    #[test]
    fn test_real_html_still_flagged_alongside_escaped() {
        // A line with both escaped AND real HTML should still be flagged
        let md = "\
# Title

\\<br\\> but also <div>real html</div> here.
";
        let report = detect_noise(md);
        let html_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HtmlRemnant)
            .collect();
        assert!(
            !html_issues.is_empty(),
            "Line with real (non-escaped) HTML tags should still be flagged"
        );
    }

    #[test]
    fn test_indented_code_block_html_not_flagged() {
        // Indented code blocks (4 spaces) should not have their HTML flagged
        let md = "\
# Title

Here's a code example:

    <div>
        <p>This is inside an indented code block</p>
    </div>

Normal paragraph after.
";
        let report = detect_noise(md);
        let html_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HtmlRemnant)
            .collect();
        assert!(
            html_issues.is_empty(),
            "HTML inside indented code blocks should not be flagged, got: {:?}",
            html_issues
        );
    }

    #[test]
    fn test_escaped_pipes_not_counted_in_broken_table() {
        // Escaped pipes \| in cell content should not affect column count
        let md = "\
| Col1 | Col2 | Col3 |
|------|------|------|
| a\\|b | c | d |
| e | f\\|g | h |
";
        let report = detect_noise(md);
        let table_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::BrokenTable)
            .collect();
        assert!(
            table_issues.is_empty(),
            "Escaped pipes in table cells should not cause BrokenTable, got: {:?}",
            table_issues
        );
    }

    #[test]
    fn test_pure_arabic_text_not_flagged_as_garbled() {
        // Pure Arabic text (>70% non-ASCII) from a recognized script should NOT be flagged
        let md = "\
# Document

\u{062a}\u{062d}\u{0633}\u{064a}\u{0646} \u{0627}\u{0644}\u{0625}\u{0646}\u{062a}\u{0627}\u{062c}\u{064a}\u{0629} \u{0648}\u{062d}\u{0644} \u{0627}\u{0644}\u{0645}\u{0634}\u{0643}\u{0644}\u{0627}\u{062a}
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Pure Arabic text should not be flagged as garbled, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_pure_chinese_text_not_flagged_as_garbled() {
        // Pure Chinese text (>70% non-ASCII) should NOT be flagged
        let md = "\
# Document

\u{80a1}\u{7968}\u{4ee3}\u{7801}\u{ff1a}\u{4e09}\u{96f6}\u{4e8c}\u{4e00}\u{516b} \u{80a1}\u{7968}\u{7b80}\u{79f0}\u{ff1a}\u{5b89}\u{5229}\u{80a1}\u{4efd}
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Pure Chinese text should not be flagged as garbled, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_pure_cyrillic_text_not_flagged_as_garbled() {
        // Pure Cyrillic/Russian text should NOT be flagged
        let md = "\
# Document

\u{0420}\u{0430}\u{0437}\u{0434}\u{0435}\u{043b}\u{003a} \u{0424}\u{0438}\u{043d}\u{0430}\u{043b}\u{044c}\u{043d}\u{044b}\u{0439} \u{044d}\u{043a}\u{0437}\u{0430}\u{043c}\u{0435}\u{043d}
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Pure Cyrillic text should not be flagged as garbled, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_math_operators_not_flagged_as_garbled() {
        // Mathematical operators like ∑, ∏, √, ∞, ≤, ≥ are legitimate
        let md = "\
# Math

\u{2211}\u{2208}\u{2209}\u{221A}\u{221E}\u{2264}\u{2265}\u{2260}\u{2261}\u{2248}\u{2202}\u{222B}\u{220F}\u{2200}\u{2203}\u{2207}\u{2211}\u{2208}\u{2209}\u{221A}
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "Mathematical operators should not be flagged as garbled, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_latex_braces_not_flagged_as_garbled() {
        // LaTeX equations like \sum_{k=0}^{n} use { and } which should not trigger garbled
        let md = "\
# Equation

The formula is \\sum_{k=0}^{n} C{k}{n} x^{k}.
";
        let report = detect_noise(md);
        let garbled: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::GarbledText)
            .collect();
        assert!(
            garbled.is_empty(),
            "LaTeX brace patterns should not be flagged as garbled, got: {:?}",
            garbled
        );
    }

    #[test]
    fn test_rst_grid_table_borders_not_flagged_as_header_footer() {
        // RST grid table borders like +---+---+ should not be flagged
        let mut lines = Vec::new();
        for i in 0..15 {
            lines.push("+-----+-----+-----+".to_string());
            lines.push(format!("| a{i}  | b{i}  | c{i}  |"));
        }
        lines.push("+-----+-----+-----+".to_string());
        let md = lines.join("\n");
        let report = detect_noise(&md);
        let rep_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::HeaderFooterRepetition)
            .collect();
        assert!(
            rep_issues.is_empty(),
            "RST grid table borders should not be flagged as header/footer repetition, got: {:?}",
            rep_issues
        );
    }

    #[test]
    fn test_excessive_heading_density_raised_threshold() {
        // 8 headings vs 3 paragraphs used to be flagged (>5, > paragraphs)
        // but with new threshold (>10, > 2*paragraphs) this should NOT be flagged
        let md = "\
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6
# Heading 7
## Heading 8

Paragraph one.

Paragraph two.

Paragraph three.
";
        let report = detect_noise(md);
        let density_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.kind == NoiseKind::ExcessiveHeadingDensity)
            .collect();
        assert!(
            density_issues.is_empty(),
            "8 headings vs 3 paragraphs should not be flagged with raised threshold, got: {:?}",
            density_issues
        );
    }
}
