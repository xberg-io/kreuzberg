//! Markdown heading-aware chunking with hierarchy path prepending.
//!
//! Splits markdown at heading boundaries (H1-H6) and prepends the full
//! heading hierarchy path to each chunk, giving LLMs section context.
//!
//! Inspired by Anthropic's Contextual Retrieval research (2024), which
//! showed that adding document context to chunks reduces retrieval
//! failures by 49%. This chunker achieves a lightweight version of
//! contextual enrichment without requiring an extra LLM call per chunk.

use crate::error::Result;
use crate::types::{Chunk, ChunkMetadata, HeadingLevel, PageBoundary};

use super::config::{ChunkingConfig, ChunkingResult};
use super::headings::{build_heading_map, resolve_heading_context};

/// Split markdown at heading boundaries with heading path prepending.
///
/// Each chunk is a section under a heading, prefixed with the full
/// heading hierarchy (e.g., "# Setup > ## Installation\n\nContent...").
///
/// Sections that exceed `max_characters` are split further using
/// paragraph-boundary-aware splitting while retaining the heading prefix.
///
/// Falls back to standard markdown chunking if no headings are found.
pub fn chunk_markdown_by_headings(
    text: &str,
    config: &ChunkingConfig,
    page_boundaries: Option<&[PageBoundary]>,
) -> Result<ChunkingResult> {
    if text.is_empty() {
        return Ok(ChunkingResult {
            chunks: vec![],
            chunk_count: 0,
        });
    }

    let heading_map = build_heading_map(text);
    if heading_map.is_empty() {
        // Fall back to standard Markdown chunking (not MarkdownHeading,
        // which would recurse back here via core::chunk_text).
        let fallback_config = ChunkingConfig {
            chunker_type: super::config::ChunkerType::Markdown,
            ..config.clone()
        };
        return super::core::chunk_text(text, &fallback_config, page_boundaries);
    }

    let sections = split_at_headings(text, &heading_map);
    let mut chunks: Vec<Chunk> = Vec::new();

    for section in &sections {
        let prefix = format_heading_path(&section.heading_path);
        let content = if prefix.is_empty() {
            section.body.to_string()
        } else {
            format!("{}\n\n{}", prefix, section.body)
        };

        if content.len() <= config.max_characters || config.max_characters == 0 {
            chunks.push(Chunk {
                content,
                embedding: None,
                metadata: ChunkMetadata {
                    byte_start: section.byte_start,
                    byte_end: section.byte_end,
                    token_count: None,
                    chunk_index: 0,
                    total_chunks: 0,
                    first_page: None,
                    last_page: None,
                    heading_context: resolve_heading_context(section.byte_start, &heading_map),
                },
            });
        } else {
            let sub_chunks = split_large_section(section.body, config.max_characters, &prefix);
            let mut byte_offset = section.byte_start;
            for sub in &sub_chunks {
                let body_len = if prefix.is_empty() {
                    sub.len()
                } else {
                    sub.len().saturating_sub(prefix.len() + 2)
                };
                let sub_end = (byte_offset + body_len).min(section.byte_end);
                chunks.push(Chunk {
                    content: sub.clone(),
                    embedding: None,
                    metadata: ChunkMetadata {
                        byte_start: byte_offset,
                        byte_end: sub_end,
                        token_count: None,
                        chunk_index: 0,
                        total_chunks: 0,
                        first_page: None,
                        last_page: None,
                        heading_context: resolve_heading_context(byte_offset, &heading_map),
                    },
                });
                byte_offset = sub_end;
            }
        }
    }

    let total = chunks.len();
    for (i, chunk) in chunks.iter_mut().enumerate() {
        chunk.metadata.chunk_index = i;
        chunk.metadata.total_chunks = total;
    }

    Ok(ChunkingResult {
        chunk_count: total,
        chunks,
    })
}

struct Section<'a> {
    heading_path: Vec<HeadingLevel>,
    body: &'a str,
    byte_start: usize,
    byte_end: usize,
}

fn split_at_headings<'a>(text: &'a str, heading_map: &[(usize, u8, String)]) -> Vec<Section<'a>> {
    let mut sections: Vec<Section<'a>> = Vec::new();
    let mut heading_stack: Vec<HeadingLevel> = Vec::new();

    if let Some(&(first_offset, _, _)) = heading_map.first() {
        if first_offset > 0 {
            let preamble = &text[..first_offset];
            if !preamble.trim().is_empty() {
                sections.push(Section {
                    heading_path: vec![],
                    body: preamble.trim(),
                    byte_start: 0,
                    byte_end: first_offset,
                });
            }
        }
    }

    for (i, &(offset, level, ref heading_text)) in heading_map.iter().enumerate() {
        while heading_stack.last().is_some_and(|h| h.level >= level) {
            heading_stack.pop();
        }
        heading_stack.push(HeadingLevel {
            level,
            text: heading_text.clone(),
        });

        let section_end = heading_map
            .get(i + 1)
            .map(|&(next_offset, _, _)| next_offset)
            .unwrap_or(text.len());

        let heading_end = text[offset..section_end]
            .find('\n')
            .map(|pos| offset + pos + 1)
            .unwrap_or(section_end);
        let body = &text[heading_end..section_end];

        if !body.trim().is_empty() {
            sections.push(Section {
                heading_path: heading_stack.clone(),
                body: body.trim(),
                byte_start: offset,
                byte_end: section_end,
            });
        }
    }

    sections
}

fn format_heading_path(path: &[HeadingLevel]) -> String {
    path.iter()
        .map(|h| format!("{} {}", "#".repeat(h.level as usize), h.text))
        .collect::<Vec<_>>()
        .join(" > ")
}

fn split_large_section(body: &str, max_chars: usize, prefix: &str) -> Vec<String> {
    let prefix_overhead = if prefix.is_empty() { 0 } else { prefix.len() + 2 };
    let available = max_chars.saturating_sub(prefix_overhead).max(1);

    let mut sub_chunks = Vec::new();
    let mut start = 0;

    while start < body.len() {
        let actual_end = find_split_point(body, start, available);
        let sub = body[start..actual_end].trim();
        if !sub.is_empty() {
            sub_chunks.push(prepend_prefix(prefix, sub));
        }
        start = actual_end;
    }

    if sub_chunks.is_empty() && !body.trim().is_empty() {
        sub_chunks.push(prepend_prefix(prefix, body.trim()));
    }

    sub_chunks
}

/// Find the best split point starting from `start`, respecting paragraph
/// boundaries and UTF-8 char boundaries. Always advances at least one char.
fn find_split_point(body: &str, start: usize, available: usize) -> usize {
    let tentative_end = (start + available).min(body.len());
    let safe_end = floor_char_boundary(body, tentative_end);

    let break_at = body[start..safe_end]
        .rfind("\n\n")
        .map(|pos| start + pos + 2)
        .unwrap_or(safe_end);

    let end = if break_at > start { break_at } else { safe_end };
    if end == start { next_char_boundary(body, start) } else { end }
}

fn prepend_prefix(prefix: &str, content: &str) -> String {
    if prefix.is_empty() {
        content.to_string()
    } else {
        format!("{}\n\n{}", prefix, content)
    }
}

/// Largest byte index <= `i` that is on a UTF-8 char boundary.
fn floor_char_boundary(s: &str, i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    let mut pos = i;
    while pos > 0 && !s.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

/// Smallest byte index > `i` that is on a UTF-8 char boundary.
fn next_char_boundary(s: &str, i: usize) -> usize {
    let mut pos = i + 1;
    while pos < s.len() && !s.is_char_boundary(pos) {
        pos += 1;
    }
    pos.min(s.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunking::config::ChunkerType;

    fn make_config(max_chars: usize) -> ChunkingConfig {
        ChunkingConfig {
            max_characters: max_chars,
            overlap: 0,
            trim: true,
            chunker_type: ChunkerType::MarkdownHeading,
            ..Default::default()
        }
    }

    #[test]
    fn test_single_heading_section() {
        let md = "# Title\n\nSome content here.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 1);
        assert!(result.chunks[0].content.starts_with("# Title"));
        assert!(result.chunks[0].content.contains("Some content here."));
    }

    #[test]
    fn test_multiple_sections() {
        let md = "# Intro\n\nIntro text.\n\n## Setup\n\nSetup text.\n\n## Usage\n\nUsage text.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 3);
        assert!(result.chunks[0].content.contains("# Intro"));
        assert!(result.chunks[1].content.contains("# Intro > ## Setup"));
        assert!(result.chunks[2].content.contains("# Intro > ## Usage"));
    }

    #[test]
    fn test_nested_hierarchy() {
        let md = "# A\n\nA body\n\n## B\n\nB body\n\n### C\n\nC body\n\n## D\n\nD body";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 4);
        let c_chunk = result.chunks.iter().find(|c| c.content.contains("C body")).unwrap();
        assert!(c_chunk.content.contains("# A > ## B > ### C"));
        let d_chunk = result.chunks.iter().find(|c| c.content.contains("D body")).unwrap();
        assert!(d_chunk.content.contains("# A > ## D"));
        assert!(!d_chunk.content.contains("## B"));
    }

    #[test]
    fn test_preamble_before_first_heading() {
        let md = "Preamble text.\n\n# First Heading\n\nSection content.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 2);
        assert_eq!(result.chunks[0].content, "Preamble text.");
        assert!(result.chunks[1].content.contains("# First Heading"));
    }

    #[test]
    fn test_no_headings_falls_back() {
        let md = "Just plain text without any headings.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 1);
        assert!(result.chunks[0].content.contains("Just plain text"));
    }

    #[test]
    fn test_empty_text() {
        let result = chunk_markdown_by_headings("", &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 0);
    }

    #[test]
    fn test_chunk_indices() {
        let md = "# A\n\nText A.\n\n## B\n\nText B.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        for (i, chunk) in result.chunks.iter().enumerate() {
            assert_eq!(chunk.metadata.chunk_index, i);
            assert_eq!(chunk.metadata.total_chunks, result.chunk_count);
        }
    }

    #[test]
    fn test_heading_context_populated() {
        let md = "# Title\n\n## Section\n\nContent.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        let section_chunk = result.chunks.iter().find(|c| c.content.contains("Content")).unwrap();
        let ctx = section_chunk.metadata.heading_context.as_ref().unwrap();
        assert_eq!(ctx.headings.len(), 2);
        assert_eq!(ctx.headings[0].text, "Title");
        assert_eq!(ctx.headings[1].text, "Section");
    }

    #[test]
    fn test_format_heading_path() {
        let path = vec![
            HeadingLevel { level: 1, text: "Setup".to_string() },
            HeadingLevel { level: 2, text: "Installation".to_string() },
            HeadingLevel { level: 3, text: "Dependencies".to_string() },
        ];
        assert_eq!(
            format_heading_path(&path),
            "# Setup > ## Installation > ### Dependencies"
        );
    }

    #[test]
    fn test_format_heading_path_empty() {
        assert_eq!(format_heading_path(&[]), "");
    }

    #[test]
    fn test_large_section_split() {
        let long_body = "Word ".repeat(200);
        let md = format!("# Title\n\n{}", long_body);
        let result = chunk_markdown_by_headings(&md, &make_config(100), None).unwrap();
        assert!(result.chunk_count > 1);
        for chunk in &result.chunks {
            assert!(chunk.content.starts_with("# Title"));
        }
    }

    #[test]
    fn test_empty_sections_skipped() {
        let md = "# A\n\n## B\n\n## C\n\nActual content.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        for chunk in &result.chunks {
            assert!(!chunk.content.trim().is_empty());
        }
    }

    #[test]
    fn test_all_heading_levels() {
        let md = "# H1\n\nH1 body\n\n## H2\n\nH2 body\n\n### H3\n\nH3 body\n\n\
                  #### H4\n\nH4 body\n\n##### H5\n\nH5 body\n\n###### H6\n\nH6 body";
        let result = chunk_markdown_by_headings(md, &make_config(10000), None).unwrap();
        assert_eq!(result.chunk_count, 6);
        let h6_chunk = result.chunks.iter().find(|c| c.content.contains("H6 body")).unwrap();
        assert!(h6_chunk.content.contains("# H1 > ## H2 > ### H3 > #### H4 > ##### H5 > ###### H6"));
    }

    #[test]
    fn test_heading_level_reset() {
        let md = "# Root\n\nRoot body\n\n## A\n\nA body\n\n### A1\n\nA1 body\n\n## B\n\nB body\n\n### B1\n\nB1 body";
        let result = chunk_markdown_by_headings(md, &make_config(10000), None).unwrap();
        let b1_chunk = result.chunks.iter().find(|c| c.content.contains("B1 body")).unwrap();
        assert!(b1_chunk.content.contains("# Root > ## B > ### B1"));
        assert!(!b1_chunk.content.contains("## A"));
    }

    #[test]
    fn test_multibyte_content() {
        let md = "# \u{65e5}\u{672c}\u{8a9e}\n\n\u{3053}\u{308c}\u{306f}\u{65e5}\u{672c}\u{8a9e}\u{306e}\u{30c6}\u{30b9}\u{30c8}\u{3067}\u{3059}\u{3002}";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 1);
    }

    #[test]
    fn test_emoji_content() {
        let md = "# Emoji Test \u{1f389}\n\n\u{1f680} Rockets and \u{1f30d} globes.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 1);
        assert!(result.chunks[0].content.contains("\u{1f680}"));
    }

    #[test]
    fn test_multibyte_forced_split() {
        // Force splitting inside multi-byte content
        let body = "\u{65e5}\u{672c}\u{8a9e}".repeat(50);
        let md = format!("# T\n\n{}", body);
        let result = chunk_markdown_by_headings(&md, &make_config(30), None).unwrap();
        assert!(result.chunk_count >= 1);
        for chunk in &result.chunks {
            assert!(!chunk.content.is_empty());
        }
    }

    #[test]
    fn test_mixed_scripts() {
        let md = "# \u{0395}\u{03bb}\u{03bb}\u{03b7}\u{03bd}\u{03b9}\u{03ba}\u{03ac}\n\nGreek text.\n\n## \u{4e2d}\u{6587}\n\nChinese text.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 2);
    }

    #[test]
    fn test_heading_with_no_body() {
        let md = "# Title\n\n## Empty Section\n\n## Has Content\n\nSome text.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        for chunk in &result.chunks {
            assert!(!chunk.content.trim().is_empty());
        }
    }

    #[test]
    fn test_single_heading_no_body() {
        let md = "# Only a heading";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert!(result.chunk_count <= 1);
    }

    #[test]
    fn test_prefix_exceeds_max_chars() {
        let md = "# Very Long Heading That Is Quite Lengthy\n\n## Another Long Sub Heading Here\n\nBody text.";
        let result = chunk_markdown_by_headings(md, &make_config(10), None).unwrap();
        assert!(result.chunk_count >= 1);
    }

    #[test]
    fn test_floor_char_boundary() {
        let s = "a\u{00e9} b"; // 'e\u{0301}' as precomposed e-acute is 2 bytes
        assert_eq!(floor_char_boundary(s, 0), 0);
        assert_eq!(floor_char_boundary(s, 1), 1);
        assert_eq!(floor_char_boundary(s, 2), 1); // inside multi-byte char
        assert_eq!(floor_char_boundary(s, 3), 3);
        assert_eq!(floor_char_boundary(s, 100), s.len());
    }

    #[test]
    fn test_next_char_boundary() {
        let s = "a\u{00e9} b";
        assert_eq!(next_char_boundary(s, 0), 1);
        assert_eq!(next_char_boundary(s, 1), 3); // past 2-byte char
        assert_eq!(next_char_boundary(s, 3), 4);
    }

    #[test]
    fn test_byte_offsets_within_source() {
        let md = "# Section A\n\nContent A.\n\n## Section B\n\nContent B.";
        let result = chunk_markdown_by_headings(md, &make_config(10000), None).unwrap();
        for chunk in &result.chunks {
            assert!(chunk.metadata.byte_start <= chunk.metadata.byte_end);
            assert!(chunk.metadata.byte_end <= md.len());
        }
    }

    #[test]
    fn test_code_in_heading() {
        let md = "# Using `grep` effectively\n\nSome tips about grep.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 1);
        assert!(result.chunks[0].content.contains("grep"));
    }

    #[test]
    fn test_consecutive_same_level_headings() {
        let md = "## A\n\nA body.\n\n## B\n\nB body.\n\n## C\n\nC body.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 3);
        assert!(result.chunks[0].content.starts_with("## A"));
        assert!(result.chunks[1].content.starts_with("## B"));
        assert!(result.chunks[2].content.starts_with("## C"));
    }

    #[test]
    fn test_deeply_nested_skipping_levels() {
        let md = "# Top\n\nTop body.\n\n#### Deep\n\nDeep body.";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 2);
        let deep_chunk = result.chunks.iter().find(|c| c.content.contains("Deep body")).unwrap();
        assert!(deep_chunk.content.contains("# Top > #### Deep"));
    }

    #[test]
    fn test_four_byte_emoji_split() {
        // Family emoji is 25 bytes (7 code points with ZWJ)
        let emoji = "\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467}\u{200d}\u{1f466}";
        let body = format!("{} {}", emoji, emoji);
        let md = format!("# E\n\n{}", body);
        let result = chunk_markdown_by_headings(&md, &make_config(20), None).unwrap();
        assert!(result.chunk_count >= 1);
        for chunk in &result.chunks {
            assert!(!chunk.content.is_empty());
        }
    }

    #[test]
    fn test_max_characters_zero_no_split() {
        // max_characters == 0 means unlimited — every section fits in one chunk.
        let md = "# Title\n\nA very long body that would normally exceed a small limit.";
        let result = chunk_markdown_by_headings(md, &make_config(0), None).unwrap();
        assert_eq!(result.chunk_count, 1);
        assert!(result.chunks[0].content.contains("A very long body"));
    }

    #[test]
    fn test_large_preamble_split_empty_prefix() {
        // A long preamble (no headings path) that exceeds max_characters triggers
        // the large-section split with an empty prefix.
        let preamble = "word ".repeat(100);
        let md = format!("{}\n\n# H\n\nBody.", preamble);
        let result = chunk_markdown_by_headings(&md, &make_config(50), None).unwrap();
        // Preamble should be split into multiple chunks, each without a heading prefix.
        let preamble_chunks: Vec<_> = result
            .chunks
            .iter()
            .filter(|c| !c.content.contains("# H"))
            .collect();
        assert!(preamble_chunks.len() > 1, "preamble should be split into multiple chunks");
        for pc in &preamble_chunks {
            assert!(!pc.content.starts_with('#'), "preamble chunks should have no heading prefix");
        }
    }

    #[test]
    fn test_find_split_point_advances_past_multibyte_boundary() {
        // When available=1 and the body starts with a multi-byte char,
        // safe_end == start, so find_split_point must advance via next_char_boundary.
        let body = "\u{00e9}abc"; // e-acute (2 bytes) followed by ASCII
        let end = find_split_point(body, 0, 1);
        assert!(end > 0, "must advance past start");
        assert!(body.is_char_boundary(end), "must land on char boundary");
    }

    #[test]
    fn test_next_char_boundary_at_end() {
        let s = "abc";
        // Calling next_char_boundary at the last valid index should return s.len().
        assert_eq!(next_char_boundary(s, 2), 3);
        // Calling next_char_boundary past the end should clamp to s.len().
        assert_eq!(next_char_boundary(s, 5), 3);
    }

    #[test]
    fn test_split_large_section_empty_prefix() {
        // Directly test split_large_section with an empty prefix to cover
        // the prefix_overhead == 0 path and body_len == sub.len() path.
        let body = "Hello world. This is a test of splitting without a prefix.";
        let result = split_large_section(body, 20, "");
        assert!(result.len() >= 1);
        for chunk in &result {
            // No prefix should be prepended.
            assert!(!chunk.starts_with('#'));
        }
    }

    #[test]
    fn test_split_large_section_fallback_non_empty_body() {
        // When body is entirely whitespace except for a tiny non-empty part,
        // and the loop produces empty trimmed subs, the fallback pushes body.trim().
        // This happens when body is all whitespace followed by content that
        // gets picked up as one sub. Actually the fallback is near-impossible
        // with the current code because find_split_point always advances.
        // But we can trigger it by testing that non-empty body always produces output.
        let body = "x";
        let result = split_large_section(body, 1000, "pfx");
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("pfx"));
        assert!(result[0].contains("x"));
    }

    #[test]
    fn test_prepend_prefix_empty() {
        assert_eq!(prepend_prefix("", "content"), "content");
    }

    #[test]
    fn test_prepend_prefix_non_empty() {
        assert_eq!(prepend_prefix("# H", "body"), "# H\n\nbody");
    }

    #[test]
    fn test_floor_char_boundary_at_zero() {
        // When i == 0, should return 0 (the while loop condition pos > 0 is false).
        let s = "\u{00e9}abc";
        assert_eq!(floor_char_boundary(s, 0), 0);
    }

    #[test]
    fn test_heading_only_at_end_no_newline() {
        // The last heading has no trailing newline, so find('\n') returns None
        // and heading_end == section_end. Body is empty, so section is skipped.
        let md = "# A\n\nContent.\n\n## B";
        let result = chunk_markdown_by_headings(md, &make_config(1000), None).unwrap();
        assert_eq!(result.chunk_count, 1);
        assert!(result.chunks[0].content.contains("Content."));
    }
}
