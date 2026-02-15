//! Shared frontmatter and metadata utilities for markup extractors.
//!
//! This module provides common functionality for extractors that process
//! documents with YAML frontmatter (Markdown, Djot, etc.).
//!
//! This is a core module used by the Djot extractor (always available) and
//! the enhanced Markdown extractor (requires `office` feature).

use crate::types::Metadata;

use serde_yaml_ng::Value as YamlValue;
use std::borrow::Cow;

/// Extract YAML frontmatter from document content.
///
/// Frontmatter is expected to be delimited by `---` or `...` at the start of the document.
/// This implementation properly handles edge cases:
/// - `---` appearing within YAML strings or arrays
/// - Both `---` and `...` as end delimiters (YAML spec compliant)
/// - Multiline YAML values containing dashes
///
/// Returns a tuple of (parsed YAML value, remaining content after frontmatter).
///
/// # Examples
///
/// ```rust,ignore
/// let content = "---\ntitle: Test\n---\n\n# Content";
/// let (yaml, remaining) = extract_frontmatter(content);
/// assert!(yaml.is_some());
/// assert!(remaining.contains("# Content"));
/// ```
pub fn extract_frontmatter(content: &str) -> (Option<YamlValue>, String) {
    // Frontmatter must start at the beginning of the document
    if !content.starts_with("---") {
        return (None, content.to_string());
    }

    // Skip opening delimiter
    let rest = &content[3..];

    // Find the closing delimiter
    // We need to find "---" or "..." on its own line (not embedded in YAML content)
    // The delimiter must be preceded by a newline and followed by newline or EOF
    let mut end_pos = None;
    let mut search_start = 0;

    while let Some(pos) = rest[search_start..].find('\n') {
        let absolute_pos = search_start + pos;
        let after_newline = absolute_pos + 1;

        if after_newline >= rest.len() {
            break;
        }

        // Check if we have "---" or "..." at the start of a line
        let remaining = &rest[after_newline..];
        if remaining.starts_with("---") || remaining.starts_with("...") {
            // Verify it's on its own line (followed by newline or EOF)
            let delimiter_end = after_newline + 3;
            if delimiter_end >= rest.len() || rest.as_bytes()[delimiter_end] == b'\n' {
                end_pos = Some(absolute_pos);
                break;
            }
        }

        search_start = after_newline;
    }

    if let Some(end) = end_pos {
        let frontmatter_str = &rest[..end];
        // Skip past the closing delimiter and any following newline
        let after_delimiter = end + 1; // Skip the newline before delimiter
        let remaining_start = if after_delimiter + 3 < rest.len() {
            // Skip "---" or "..."
            let after_delim = after_delimiter + 3;
            // Skip trailing newline after delimiter if present
            if after_delim < rest.len() && rest.as_bytes()[after_delim] == b'\n' {
                after_delim + 1
            } else {
                after_delim
            }
        } else {
            rest.len()
        };

        let remaining = if remaining_start < rest.len() {
            &rest[remaining_start..]
        } else {
            ""
        };

        // Try to parse the frontmatter as YAML
        match serde_yaml_ng::from_str::<YamlValue>(frontmatter_str) {
            Ok(value) => (Some(value), remaining.to_string()),
            Err(_) => (None, content.to_string()),
        }
    } else {
        // No closing delimiter found
        (None, content.to_string())
    }
}

/// Extract metadata from YAML frontmatter.
///
/// Extracts the following YAML fields into Kreuzberg metadata:
/// - **Standard fields**: title, author, date, description (as subject)
/// - **Extended fields**: abstract, subject, category, tags, language, version
/// - **Array fields** (keywords, tags): converted to comma-separated strings
///
/// # Arguments
///
/// * `yaml` - The parsed YAML value from frontmatter
///
/// # Returns
///
/// A `Metadata` struct populated with extracted fields
///
/// # Examples
///
/// ```rust,ignore
/// let yaml = serde_yaml_ng::from_str("title: Test\nauthor: John").unwrap();
/// let metadata = extract_metadata_from_yaml(&yaml);
/// assert_eq!(metadata.additional.get("title"), Some(&"Test".into()));
/// ```
pub fn extract_metadata_from_yaml(yaml: &YamlValue) -> Metadata {
    let mut metadata = Metadata::default();

    // Title
    if let Some(title) = yaml.get("title").and_then(|v| v.as_str()) {
        if metadata.title.is_none() {
            metadata.title = Some(title.to_string());
        }
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata.additional.insert(Cow::Borrowed("title"), title.into());
    }

    // Author
    if let Some(author) = yaml.get("author").and_then(|v| v.as_str()) {
        if metadata.created_by.is_none() {
            metadata.created_by = Some(author.to_string());
        }
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata.additional.insert(Cow::Borrowed("author"), author.into());
    }

    // Date (map to created_at)
    if let Some(date) = yaml.get("date").and_then(|v| v.as_str()) {
        metadata.created_at = Some(date.to_string());
    }

    // Keywords (support both string and array)
    if let Some(keywords) = yaml.get("keywords") {
        match keywords {
            YamlValue::String(s) => {
                if metadata.keywords.is_none() {
                    metadata.keywords = Some(s.split(',').map(|k| k.trim().to_string()).collect());
                }
                // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                metadata.additional.insert(Cow::Borrowed("keywords"), s.clone().into());
            }
            YamlValue::Sequence(seq) => {
                let kw_vec: Vec<String> = seq.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect();
                if metadata.keywords.is_none() {
                    metadata.keywords = Some(kw_vec.clone());
                }
                let keywords_str = kw_vec.join(", ");
                // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                metadata
                    .additional
                    .insert(Cow::Borrowed("keywords"), keywords_str.into());
            }
            _ => {}
        }
    }

    // Description (map to subject)
    if let Some(description) = yaml.get("description").and_then(|v| v.as_str()) {
        metadata.subject = Some(description.to_string());
    }

    // Abstract
    if let Some(abstract_val) = yaml.get("abstract").and_then(|v| v.as_str()) {
        metadata.abstract_text = Some(abstract_val.to_string());
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata
            .additional
            .insert(Cow::Borrowed("abstract"), abstract_val.into());
    }

    // Subject (overrides description if both present)
    if let Some(subject) = yaml.get("subject").and_then(|v| v.as_str()) {
        metadata.subject = Some(subject.to_string());
    }

    // Category
    if let Some(category) = yaml.get("category").and_then(|v| v.as_str()) {
        metadata.category = Some(category.to_string());
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata.additional.insert(Cow::Borrowed("category"), category.into());
    }

    // Tags (support both string and array)
    if let Some(tags) = yaml.get("tags") {
        match tags {
            YamlValue::String(s) => {
                metadata.tags = Some(s.split(',').map(|t| t.trim().to_string()).collect());
                // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                metadata.additional.insert(Cow::Borrowed("tags"), s.clone().into());
            }
            YamlValue::Sequence(seq) => {
                let tags_vec: Vec<String> = seq.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect();
                metadata.tags = Some(tags_vec.clone());
                let tags_str = tags_vec.join(", ");
                // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                metadata.additional.insert(Cow::Borrowed("tags"), tags_str.into());
            }
            _ => {}
        }
    }

    // Language
    if let Some(language) = yaml.get("language").and_then(|v| v.as_str()) {
        if metadata.language.is_none() {
            metadata.language = Some(language.to_string());
        }
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata.additional.insert(Cow::Borrowed("language"), language.into());
    }

    // Version
    if let Some(version) = yaml.get("version").and_then(|v| v.as_str()) {
        metadata.document_version = Some(version.to_string());
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata.additional.insert(Cow::Borrowed("version"), version.into());
    }

    metadata
}

/// Extract first heading as title from content.
///
/// Searches for the first level-1 heading (# Title) in the content
/// and returns it as a potential title if no title was found in frontmatter.
///
/// # Arguments
///
/// * `content` - The document content to search
///
/// # Returns
///
/// Some(title) if a heading is found, None otherwise
///
/// # Examples
///
/// ```rust,ignore
/// let content = "# My Document\n\nContent here";
/// assert_eq!(extract_title_from_content(content), Some("My Document".to_string()));
/// ```
pub fn extract_title_from_content(content: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(heading) = line.strip_prefix("# ") {
            return Some(heading.trim().to_string());
        }
    }
    None
}

/// Convert table cells to markdown format.
///
/// Takes a 2D array of cell values and formats them as a markdown table
/// with header row, separator row, and data rows.
///
/// # Arguments
///
/// * `cells` - A 2D array where cells[0] is the header row
///
/// # Returns
///
/// A string containing the markdown-formatted table
///
/// # Examples
///
/// ```rust,ignore
/// let cells = vec![
///     vec!["Name".to_string(), "Age".to_string()],
///     vec!["Alice".to_string(), "30".to_string()],
/// ];
/// let markdown = cells_to_markdown(&cells);
/// assert!(markdown.contains("| Name | Age |"));
/// ```
pub fn cells_to_markdown(cells: &[Vec<String>]) -> String {
    if cells.is_empty() {
        return String::new();
    }

    let mut md = String::new();

    // Header row
    md.push('|');
    for cell in &cells[0] {
        md.push(' ');
        md.push_str(cell);
        md.push_str(" |");
    }
    md.push('\n');

    // Separator row
    md.push('|');
    for _ in &cells[0] {
        md.push_str(" --- |");
    }
    md.push('\n');

    // Data rows
    for row in &cells[1..] {
        md.push('|');
        for cell in row {
            md.push(' ');
            md.push_str(cell);
            md.push_str(" |");
        }
        md.push('\n');
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_basic() {
        let content = "---\ntitle: Test\n---\n\n# Content";
        let (yaml, remaining) = extract_frontmatter(content);

        assert!(yaml.is_some());
        assert!(remaining.contains("# Content"));

        let metadata = extract_metadata_from_yaml(&yaml.unwrap());
        assert_eq!(metadata.additional.get("title").and_then(|v| v.as_str()), Some("Test"));
    }

    #[test]
    fn test_frontmatter_with_dashes_in_content() {
        let content = "---\ntitle: Test\ndescription: |\n  This has ---\n  in the middle\n---\n\n# Body";
        let (yaml, remaining) = extract_frontmatter(content);

        assert!(yaml.is_some());
        assert!(remaining.contains("# Body"));
    }

    #[test]
    fn test_frontmatter_with_dots_terminator() {
        let content = "---\ntitle: Test\nauthor: John\n...\n\n# Content";
        let (yaml, remaining) = extract_frontmatter(content);

        assert!(yaml.is_some());
        assert!(remaining.contains("# Content"));

        let metadata = extract_metadata_from_yaml(&yaml.unwrap());
        assert_eq!(metadata.additional.get("title").and_then(|v| v.as_str()), Some("Test"));
    }

    #[test]
    fn test_frontmatter_with_triple_dash_in_string() {
        let content = "---\ntitle: \"Before --- After\"\nauthor: John\n---\n\n# Content";
        let (yaml, remaining) = extract_frontmatter(content);

        assert!(yaml.is_some());
        assert!(remaining.contains("# Content"));

        let metadata = extract_metadata_from_yaml(&yaml.unwrap());
        assert_eq!(
            metadata.additional.get("title").and_then(|v| v.as_str()),
            Some("Before --- After")
        );
    }

    #[test]
    fn test_frontmatter_multiline_string_with_dashes() {
        let content = "---\ntitle: Test\ndescription: |\n  Line 1\n  ---\n  Line 2\n---\n\n# Body";
        let (yaml, remaining) = extract_frontmatter(content);

        assert!(yaml.is_some());
        assert!(remaining.contains("# Body"));

        let metadata = extract_metadata_from_yaml(&yaml.unwrap());
        assert_eq!(metadata.additional.get("title").and_then(|v| v.as_str()), Some("Test"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Title\n\nContent without frontmatter";
        let (yaml, remaining) = extract_frontmatter(content);

        assert!(yaml.is_none());
        assert_eq!(remaining, content);
    }

    #[test]
    fn test_incomplete_frontmatter() {
        let content = "---\ntitle: Test\nauthor: John\n\n# Content";
        let (yaml, remaining) = extract_frontmatter(content);

        // No closing delimiter, should return None
        assert!(yaml.is_none());
        assert_eq!(remaining, content);
    }

    #[test]
    fn test_extract_title_from_content() {
        let content = "# My Document\n\nContent here";
        assert_eq!(extract_title_from_content(content), Some("My Document".to_string()));
    }

    #[test]
    fn test_extract_title_from_content_no_heading() {
        let content = "Content without heading";
        assert_eq!(extract_title_from_content(content), None);
    }

    #[test]
    fn test_extract_title_from_content_level_2() {
        let content = "## Subheading\n\nContent";
        assert_eq!(extract_title_from_content(content), None);
    }

    #[test]
    fn test_cells_to_markdown() {
        let cells = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];

        let markdown = cells_to_markdown(&cells);
        assert!(markdown.contains("| Name | Age |"));
        assert!(markdown.contains("| Alice | 30 |"));
        assert!(markdown.contains("| Bob | 25 |"));
        assert!(markdown.contains("| --- | --- |"));
    }

    #[test]
    fn test_cells_to_markdown_empty() {
        let cells: Vec<Vec<String>> = vec![];
        let markdown = cells_to_markdown(&cells);
        assert_eq!(markdown, "");
    }

    #[test]
    fn test_metadata_from_yaml_all_fields() {
        let yaml_str = r#"
title: Test Document
author: John Doe
date: 2024-01-15
keywords:
  - rust
  - testing
description: A test document
abstract: This is an abstract
subject: Test Subject
category: Documentation
tags:
  - tag1
  - tag2
language: en
version: 1.0
"#;

        let yaml: YamlValue = serde_yaml_ng::from_str(yaml_str).unwrap();
        let metadata = extract_metadata_from_yaml(&yaml);

        assert_eq!(
            metadata.additional.get("title").and_then(|v| v.as_str()),
            Some("Test Document")
        );
        assert_eq!(
            metadata.additional.get("author").and_then(|v| v.as_str()),
            Some("John Doe")
        );
        assert_eq!(metadata.created_at, Some("2024-01-15".to_string()));
        assert!(metadata.additional.contains_key("keywords"));
        assert_eq!(metadata.subject, Some("Test Subject".to_string()));
        assert!(metadata.additional.contains_key("tags"));
    }

    #[test]
    fn test_metadata_from_yaml_string_arrays() {
        let yaml_str = r#"
keywords: "single, keyword, string"
tags: "tag1, tag2"
"#;

        let yaml: YamlValue = serde_yaml_ng::from_str(yaml_str).unwrap();
        let metadata = extract_metadata_from_yaml(&yaml);

        assert_eq!(
            metadata.additional.get("keywords").and_then(|v| v.as_str()),
            Some("single, keyword, string")
        );
    }
}
