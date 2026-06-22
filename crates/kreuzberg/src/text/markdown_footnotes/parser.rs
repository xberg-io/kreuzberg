//! Parsing logic for markdown footnotes and citations.

use super::types::{Citation, FootnoteAnchor, FootnoteDefinition};
use regex::Regex;
use std::sync::LazyLock;

// Regex patterns for footnote parsing
static FOOTNOTE_ANCHOR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\^([a-zA-Z0-9_-]+)\]").expect("Valid regex"));

static FOOTNOTE_DEF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\[\^([a-zA-Z0-9_-]+)\]:\s*(.+)$").expect("Valid regex"));

static INFERENCE_MARKER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[\*inference\*\]").expect("Valid regex"));

static CITATION_BLOCK_START: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^---\s*$").expect("Valid regex"));

static CITATION_COMMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!--\s*citations\s*.*?-->").expect("Valid regex"));

static CITATION_ENTRY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\[\^([a-zA-Z0-9_-]+)\]:\s*(.+)$").expect("Valid regex"));

/// Find all footnote anchor references (use-sites) in markdown.
///
/// Definition sites (`[^label]:`) are excluded — a match immediately followed by `:` is a
/// definition, not a reference.
pub(crate) fn find_footnote_anchors(markdown: &str) -> Vec<FootnoteAnchor> {
    FOOTNOTE_ANCHOR_RE
        .captures_iter(markdown)
        .filter_map(|caps| {
            let whole = caps.get(0)?;
            // Skip definition sites: `[^label]:` is a definition, not a reference.
            if markdown[whole.end()..].starts_with(':') {
                return None;
            }
            let label = caps.get(1)?;
            Some(FootnoteAnchor {
                label: label.as_str().to_string(),
                offset: whole.start(),
            })
        })
        .collect()
}

/// Parse footnote definitions from markdown.
pub(crate) fn parse_footnote_definitions(markdown: &str) -> Vec<FootnoteDefinition> {
    let mut definitions = Vec::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if let Some(caps) = FOOTNOTE_DEF_RE.captures(line) {
            if let (Some(label_match), Some(content_match)) = (caps.get(1), caps.get(2)) {
                let label = label_match.as_str().to_string();
                let mut content = content_match.as_str().to_string();

                let def_offset = markdown.find(&format!("[^{}]:", label)).unwrap_or(0);

                // Collect continuation lines: blank lines or indented text. A new definition or a
                // flush-left non-empty line ends this definition.
                let mut j = i + 1;
                while j < lines.len() {
                    let next_line = lines[j];
                    if FOOTNOTE_DEF_RE.is_match(next_line) {
                        break;
                    }
                    if next_line.is_empty() || next_line.starts_with(char::is_whitespace) {
                        if !next_line.is_empty() {
                            content.push('\n');
                            content.push_str(next_line.trim());
                        }
                        j += 1;
                    } else {
                        break;
                    }
                }

                definitions.push(FootnoteDefinition {
                    label,
                    content,
                    offset: def_offset,
                });

                i = j;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    definitions
}

/// Find all inference markers in markdown.
pub(crate) fn find_inference_markers(markdown: &str) -> Vec<usize> {
    INFERENCE_MARKER_RE.find_iter(markdown).map(|m| m.start()).collect()
}

/// Find unmarked claims in markdown.
pub(crate) fn find_unmarked_claims(markdown: &str) -> Vec<String> {
    let mut claims = Vec::new();

    // Find citation block boundaries
    let citation_block_start = find_citation_block_start(markdown);

    let inference_offsets = find_inference_markers(markdown);
    let footnote_anchors = find_footnote_anchors(markdown);

    // Build a set of line ranges that have citations or inferences
    let mut lines_with_markers = std::collections::HashSet::new();
    for anchor in &footnote_anchors {
        if let Some(line_num) = char_offset_to_line(markdown, anchor.offset) {
            lines_with_markers.insert(line_num);
        }
    }
    for offset in &inference_offsets {
        if let Some(line_num) = char_offset_to_line(markdown, *offset) {
            lines_with_markers.insert(line_num);
        }
    }

    for (line_num, line) in markdown.lines().enumerate() {
        // Skip if in citation block
        if citation_block_start.is_some_and(|block_start_line| line_num >= block_start_line) {
            continue;
        }

        // Skip if has a citation or inference marker
        if lines_with_markers.contains(&line_num) {
            continue;
        }

        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Skip headings
        if trimmed.starts_with('#') {
            continue;
        }

        // Skip footnote/citation definition lines (`[^label]: ...`) — they are not claims.
        if FOOTNOTE_DEF_RE.is_match(trimmed) {
            continue;
        }

        // Skip lines that are only markup/lists
        if is_markup_only(trimmed) {
            continue;
        }

        // Check if line looks like a claim: has alphabetic chars and ends with punctuation
        if is_claim_like(trimmed) {
            claims.push(trimmed.to_string());
        }
    }

    claims
}

/// Find the start line number of the citation block.
fn find_citation_block_start(markdown: &str) -> Option<usize> {
    let lines: Vec<&str> = markdown.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if CITATION_BLOCK_START.is_match(line) {
            // Look for citation comment in next lines
            if i + 1 < lines.len() && CITATION_COMMENT_RE.is_match(lines[i + 1]) {
                return Some(i);
            }
        }
    }

    None
}

/// Convert a byte offset to a line number.
fn char_offset_to_line(text: &str, offset: usize) -> Option<usize> {
    if offset > text.len() {
        return None;
    }

    let mut current_offset = 0;

    for (line_num, line) in text.lines().enumerate() {
        if current_offset + line.len() >= offset {
            return Some(line_num);
        }
        current_offset += line.len() + 1; // +1 for newline
    }

    None
}

/// Check if a line is markup-only (no substantial content).
fn is_markup_only(line: &str) -> bool {
    let trimmed = line.trim();

    // List items (but only if no alphanumeric after the marker)
    if (trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('+')) && trimmed.len() <= 2 {
        return true;
    }

    // Ordered list start with no content after the marker (e.g. "1.")
    if trimmed.starts_with(|c: char| c.is_ascii_digit())
        && let Some(dot_pos) = trimmed.find('.')
        && dot_pos < 3
        && trimmed[dot_pos + 1..].trim().is_empty()
    {
        return true;
    }

    // Blockquote with no content
    if trimmed.starts_with('>') && trimmed.len() <= 1 {
        return true;
    }

    false
}

/// Check if a line looks like a claim (has alphabetic chars, ends with punctuation).
fn is_claim_like(line: &str) -> bool {
    let trimmed = line.trim();

    // Must have at least one alphabetic character
    if !trimmed.chars().any(|c| c.is_alphabetic()) {
        return false;
    }

    // Must end with sentence-ending punctuation
    trimmed.ends_with(['.', '!', '?', ':', ';'])
}

/// Parse citations from the citation block.
pub(crate) fn parse_citations(markdown: &str) -> Vec<Citation> {
    let mut citations = Vec::new();

    // Find the citation block
    let lines: Vec<&str> = markdown.lines().collect();

    let mut in_citation_block = false;
    for line in lines {
        // Look for thematic break (---)
        if CITATION_BLOCK_START.is_match(line) {
            in_citation_block = true;
            continue;
        }

        // If in the citation block, look for citation comment
        if in_citation_block && CITATION_COMMENT_RE.is_match(line) {
            continue;
        }

        // Parse citation entries
        if in_citation_block
            && let Some(caps) = CITATION_ENTRY_RE.captures(line)
            && let (Some(label_match), Some(content_match)) = (caps.get(1), caps.get(2))
        {
            let label = label_match.as_str().to_string();
            let content = content_match.as_str();

            // Parse: source, locator?, excerpt?
            let (source, rest) = parse_citation_content(content);
            let (locator, excerpt) = match rest {
                Some(rest_str) => parse_citation_locator_and_excerpt(&rest_str),
                None => (None, None),
            };

            citations.push(Citation {
                label,
                source,
                locator,
                excerpt,
            });
        }
    }

    citations
}

/// Parse the source part of a citation content line.
///
/// Extracts the first comma-separated part as the source,
/// and returns the rest (if any) for locator/excerpt parsing.
fn parse_citation_content(content: &str) -> (String, Option<String>) {
    let content = content.trim();

    // Find the first comma to separate source from the rest (locator and/or excerpt).
    if let Some(first_comma) = content.find(',') {
        let source = content[..first_comma].trim().to_string();
        let after_source = content[first_comma + 1..].trim();
        if after_source.is_empty() {
            (source, None)
        } else {
            (source, Some(after_source.to_string()))
        }
    } else {
        // No comma; entire content is the source.
        (content.to_string(), None)
    }
}

/// Parse locator and excerpt from the rest of the citation.
fn parse_citation_locator_and_excerpt(rest: &str) -> (Option<String>, Option<String>) {
    // rest looks like: "page 3, excerpt: "text"" or "excerpt: "text""
    if let Some(excerpt_start) = rest.find("excerpt:") {
        // Extract the excerpt text (between quotes)
        let after_excerpt = rest[excerpt_start + 8..].trim();

        let excerpt = after_excerpt.strip_prefix('"').map(|inner| match inner.find('"') {
            Some(end_quote) => inner[..end_quote].to_string(),
            None => inner.trim_end_matches('"').to_string(),
        });

        // Extract locator (everything before "excerpt:")
        let before_excerpt = rest[..excerpt_start].trim().trim_end_matches(',').trim();
        let locator = if before_excerpt.is_empty() {
            None
        } else {
            Some(before_excerpt.to_string())
        };

        (locator, excerpt)
    } else {
        // No excerpt; treat the rest as locator
        let locator = rest.trim_end_matches(',').trim();
        if locator.is_empty() {
            (None, None)
        } else {
            (Some(locator.to_string()), None)
        }
    }
}

/// Verify that an excerpt appears in source text.
pub(crate) fn verify_excerpt(excerpt: &str, source_text: &str) -> bool {
    if excerpt.is_empty() {
        return true;
    }

    // Exact match first
    if source_text.contains(excerpt) {
        return true;
    }

    // Try whitespace-normalized matching
    let normalized_excerpt = normalize_whitespace(excerpt);
    let normalized_source = normalize_whitespace(source_text);

    normalized_source.contains(&normalized_excerpt)
}

/// Normalize whitespace by collapsing runs to single spaces.
fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
