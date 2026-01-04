//! Djot markup extractor with YAML frontmatter support.
//!
//! This extractor provides:
//! - Djot parsing using the jotdown crate
//! - YAML frontmatter metadata extraction (same as Markdown)
//! - Table extraction as structured data
//! - Heading structure preservation
//! - Code block and link extraction
//!
//! Djot is a modern markup language with simpler parsing rules than CommonMark.
//! See https://djot.net for the specification.
//!
//! Requires the `office` feature.

#[cfg(feature = "office")]
use crate::Result;
#[cfg(feature = "office")]
use crate::core::config::ExtractionConfig;
#[cfg(feature = "office")]
use crate::plugins::{DocumentExtractor, Plugin};
#[cfg(feature = "office")]
use crate::types::{ExtractionResult, Metadata, Table};
#[cfg(feature = "office")]
use async_trait::async_trait;
#[cfg(feature = "office")]
use jotdown::{Container, Event, Parser};
#[cfg(feature = "office")]
use serde_yaml_ng::Value as YamlValue;

/// Djot markup extractor with metadata and table support.
///
/// Parses Djot documents with YAML frontmatter, extracting:
/// - Metadata from YAML frontmatter
/// - Plain text content
/// - Tables as structured data
/// - Document structure (headings, links, code blocks)
#[cfg(feature = "office")]
pub struct DjotExtractor;

#[cfg(feature = "office")]
impl DjotExtractor {
    /// Create a new Djot extractor.
    pub fn new() -> Self {
        Self
    }

    /// Extract YAML frontmatter from Djot content.
    ///
    /// Frontmatter is expected to be delimited by `---` at the start of the document.
    /// Returns the remaining content after frontmatter.
    fn extract_frontmatter(content: &str) -> (Option<YamlValue>, String) {
        if !content.starts_with("---") {
            return (None, content.to_string());
        }

        let rest = &content[3..];
        if let Some(end_pos) = rest.find("\n---") {
            let frontmatter_str = &rest[..end_pos];
            let remaining = &rest[end_pos + 4..];

            match serde_yaml_ng::from_str::<YamlValue>(frontmatter_str) {
                Ok(value) => (Some(value), remaining.to_string()),
                Err(_) => (None, content.to_string()),
            }
        } else {
            (None, content.to_string())
        }
    }

    /// Extract metadata from YAML frontmatter.
    fn extract_metadata_from_yaml(yaml: &YamlValue) -> Metadata {
        let mut metadata = Metadata::default();

        if let Some(title) = yaml.get("title").and_then(|v| v.as_str()) {
            metadata.additional.insert("title".to_string(), title.into());
        }

        if let Some(author) = yaml.get("author").and_then(|v| v.as_str()) {
            metadata.additional.insert("author".to_string(), author.into());
        }

        if let Some(date) = yaml.get("date").and_then(|v| v.as_str()) {
            metadata.created_at = Some(date.to_string());
        }

        if let Some(keywords) = yaml.get("keywords") {
            match keywords {
                YamlValue::String(s) => {
                    metadata.additional.insert("keywords".to_string(), s.clone().into());
                }
                YamlValue::Sequence(seq) => {
                    let keywords_str = seq.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ");
                    metadata.additional.insert("keywords".to_string(), keywords_str.into());
                }
                _ => {}
            }
        }

        if let Some(description) = yaml.get("description").and_then(|v| v.as_str()) {
            metadata.subject = Some(description.to_string());
        }

        if let Some(abstract_text) = yaml.get("abstract").and_then(|v| v.as_str()) {
            metadata.additional.insert("abstract".to_string(), abstract_text.into());
        }

        if let Some(subject) = yaml.get("subject").and_then(|v| v.as_str()) {
            metadata.subject = Some(subject.to_string());
        }

        if let Some(category) = yaml.get("category").and_then(|v| v.as_str()) {
            metadata.additional.insert("category".to_string(), category.into());
        }

        if let Some(tags) = yaml.get("tags") {
            match tags {
                YamlValue::String(s) => {
                    metadata.additional.insert("tags".to_string(), s.clone().into());
                }
                YamlValue::Sequence(seq) => {
                    let tags_str = seq.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ");
                    metadata.additional.insert("tags".to_string(), tags_str.into());
                }
                _ => {}
            }
        }

        if let Some(language) = yaml.get("language").and_then(|v| v.as_str()) {
            metadata.additional.insert("language".to_string(), language.into());
        }

        if let Some(version) = yaml.get("version").and_then(|v| v.as_str()) {
            metadata.additional.insert("version".to_string(), version.into());
        }

        metadata
    }

    /// Extract plain text from Djot events.
    fn extract_text_from_events<'a>(events: impl Iterator<Item = Event<'a>>) -> String {
        let mut text = String::new();

        for event in events {
            match event {
                Event::Str(s) => {
                    text.push_str(&s);
                }
                Event::Softbreak | Event::Hardbreak | Event::Blankline => {
                    text.push('\n');
                }
                Event::NonBreakingSpace => {
                    text.push(' ');
                }
                Event::LeftSingleQuote | Event::RightSingleQuote => {
                    text.push('\'');
                }
                Event::LeftDoubleQuote | Event::RightDoubleQuote => {
                    text.push('"');
                }
                Event::Ellipsis => {
                    text.push_str("...");
                }
                Event::EnDash => {
                    text.push_str("--");
                }
                Event::EmDash => {
                    text.push_str("---");
                }
                Event::FootnoteReference(s) => {
                    text.push('[');
                    text.push_str(&s);
                    text.push(']');
                }
                Event::Symbol(s) => {
                    text.push(':');
                    text.push_str(&s);
                    text.push(':');
                }
                Event::ThematicBreak(_) => {
                    text.push_str("\n---\n");
                }
                Event::Start(_, _) | Event::End(_) | Event::Escape | Event::Attributes(_) => {}
            }
        }

        text
    }

    /// Extract tables from Djot events.
    fn extract_tables_from_events<'a>(events: impl Iterator<Item = Event<'a>>) -> Vec<Table> {
        let mut tables = Vec::new();
        let mut current_table: Option<(Vec<Vec<String>>, usize)> = None;
        let mut current_row: Vec<String> = Vec::new();
        let mut current_cell = String::new();
        let mut in_table_cell = false;
        let mut table_index = 0;

        for event in events {
            match event {
                Event::Start(Container::Table, _) => {
                    current_table = Some((Vec::new(), table_index));
                }
                Event::Start(Container::TableRow { .. }, _) => {
                    current_row = Vec::new();
                }
                Event::Start(Container::TableCell { .. }, _) => {
                    current_cell = String::new();
                    in_table_cell = true;
                }
                Event::Str(s) if in_table_cell => {
                    current_cell.push_str(&s);
                }
                Event::End(Container::TableCell { .. }) => {
                    if in_table_cell {
                        current_row.push(current_cell.trim().to_string());
                        current_cell = String::new();
                        in_table_cell = false;
                    }
                }
                Event::End(Container::TableRow { .. }) => {
                    if !current_row.is_empty()
                        && let Some((ref mut rows, _)) = current_table
                    {
                        rows.push(current_row.clone());
                    }
                    current_row = Vec::new();
                }
                Event::End(Container::Table) => {
                    if let Some((cells, idx)) = current_table.take()
                        && !cells.is_empty()
                    {
                        let markdown = Self::cells_to_markdown(&cells);
                        tables.push(Table {
                            cells,
                            markdown,
                            page_number: idx + 1,
                        });
                        table_index += 1;
                    }
                }
                _ => {}
            }
        }

        tables
    }

    /// Convert table cells to markdown format.
    fn cells_to_markdown(cells: &[Vec<String>]) -> String {
        if cells.is_empty() {
            return String::new();
        }

        let mut md = String::new();

        md.push('|');
        for cell in &cells[0] {
            md.push(' ');
            md.push_str(cell);
            md.push_str(" |");
        }
        md.push('\n');

        md.push('|');
        for _ in &cells[0] {
            md.push_str(" --- |");
        }
        md.push('\n');

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

    /// Extract first heading as title if not in frontmatter.
    fn extract_title_from_content(content: &str) -> Option<String> {
        for line in content.lines() {
            if let Some(heading) = line.strip_prefix("# ") {
                return Some(heading.trim().to_string());
            }
        }
        None
    }
}

#[cfg(feature = "office")]
impl Default for DjotExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "office")]
impl Plugin for DjotExtractor {
    fn name(&self) -> &str {
        "djot-extractor"
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    fn description(&self) -> &str {
        "Extracts content from Djot markup files with YAML frontmatter and table support"
    }

    fn author(&self) -> &str {
        "Kreuzberg Team"
    }
}

#[cfg(feature = "office")]
#[async_trait]
impl DocumentExtractor for DjotExtractor {
    #[cfg_attr(feature = "otel", tracing::instrument(
        skip(self, content, _config),
        fields(
            extractor.name = self.name(),
            content.size_bytes = content.len(),
        )
    ))]
    async fn extract_bytes(
        &self,
        content: &[u8],
        mime_type: &str,
        _config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining_content) = Self::extract_frontmatter(&text);

        let mut metadata = if let Some(ref yaml_value) = yaml {
            Self::extract_metadata_from_yaml(yaml_value)
        } else {
            Metadata::default()
        };

        if !metadata.additional.contains_key("title")
            && let Some(title) = Self::extract_title_from_content(&remaining_content)
        {
            metadata.additional.insert("title".to_string(), title.into());
        }

        // Parse with jotdown and collect events for text extraction
        let parser = Parser::new(&remaining_content);
        let events: Vec<Event> = parser.collect();

        let extracted_text = Self::extract_text_from_events(events.iter().cloned());

        // Parse again for table extraction (jotdown events are not Clone-able references)
        let parser = Parser::new(&remaining_content);
        let tables = Self::extract_tables_from_events(parser);

        Ok(ExtractionResult {
            content: extracted_text,
            mime_type: mime_type.to_string(),
            metadata,
            tables,
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
        })
    }

    fn supported_mime_types(&self) -> &[&str] {
        &["text/djot", "text/x-djot"]
    }

    fn priority(&self) -> i32 {
        50
    }
}

#[cfg(all(test, feature = "office"))]
mod tests {
    use super::*;

    #[test]
    fn test_can_extract_djot_mime_types() {
        let extractor = DjotExtractor::new();
        let mime_types = extractor.supported_mime_types();

        assert!(mime_types.contains(&"text/djot"));
        assert!(mime_types.contains(&"text/x-djot"));
    }

    #[test]
    fn test_extract_simple_djot() {
        let content = b"# Header\n\nThis is a paragraph with *bold* and _italic_ text.\n\n## Subheading\n\nMore content here.";
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = DjotExtractor::extract_frontmatter(&text);
        assert!(yaml.is_none());
        assert!(!remaining.is_empty());

        let parser = Parser::new(&remaining);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());

        assert!(extracted.contains("Header"));
        assert!(extracted.contains("This is a paragraph"));
        assert!(extracted.contains("bold"));
        assert!(extracted.contains("italic"));
    }

    #[test]
    fn test_extract_frontmatter_metadata() {
        let content = b"---\ntitle: My Document\nauthor: John Doe\ndate: 2024-01-15\nkeywords: rust, djot, extraction\ndescription: A test document\n---\n\n# Content\n\nBody text.";

        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml_opt, remaining) = DjotExtractor::extract_frontmatter(&text);
        assert!(yaml_opt.is_some());
        assert!(remaining.contains("# Content"));

        let yaml = yaml_opt.expect("Should extract YAML frontmatter");
        let metadata = DjotExtractor::extract_metadata_from_yaml(&yaml);

        assert_eq!(
            metadata.additional.get("title").and_then(|v| v.as_str()),
            Some("My Document")
        );
        assert_eq!(
            metadata.additional.get("author").and_then(|v| v.as_str()),
            Some("John Doe")
        );
        assert_eq!(metadata.created_at, Some("2024-01-15".to_string()));
        assert!(metadata.subject.is_some());
        assert!(
            metadata
                .subject
                .as_ref()
                .expect("Should have subject description")
                .contains("test document")
        );
    }

    #[test]
    fn test_extract_frontmatter_metadata_array_keywords() {
        let content = b"---\ntitle: Document\nkeywords:\n  - rust\n  - djot\n  - parsing\n---\n\nContent";

        let text = String::from_utf8_lossy(content).into_owned();
        let (yaml_opt, _remaining) = DjotExtractor::extract_frontmatter(&text);

        assert!(yaml_opt.is_some());
        let yaml = yaml_opt.expect("Should extract YAML frontmatter");
        let metadata = DjotExtractor::extract_metadata_from_yaml(&yaml);

        let keywords = metadata.additional.get("keywords").and_then(|v| v.as_str());
        assert!(keywords.is_some());
        let keywords_str = keywords.expect("Should extract keywords from metadata");
        assert!(keywords_str.contains("rust"));
        assert!(keywords_str.contains("djot"));
    }

    #[tokio::test]
    async fn test_extract_tables() {
        let content = b"# Tables Example\n\n| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |\n| Cell 3   | Cell 4   |";

        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(content, "text/djot", &ExtractionConfig::default())
            .await
            .expect("Should extract djot with tables");

        assert!(!result.tables.is_empty());
        let table = &result.tables[0];
        assert!(!table.cells.is_empty());
        assert_eq!(table.cells[0].len(), 2);
        assert!(!table.markdown.is_empty());
    }

    #[test]
    fn test_extract_without_frontmatter() {
        let content = b"# Main Title\n\nSome content\n\nMore text";
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = DjotExtractor::extract_frontmatter(&text);
        assert!(yaml.is_none());
        assert_eq!(remaining, text);

        let title = DjotExtractor::extract_title_from_content(&remaining);
        assert_eq!(title, Some("Main Title".to_string()));
    }

    #[test]
    fn test_empty_document() {
        let content = b"";
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = DjotExtractor::extract_frontmatter(&text);
        assert!(yaml.is_none());
        assert!(remaining.is_empty());

        let parser = Parser::new(&remaining);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());
        assert!(extracted.is_empty());
    }

    #[test]
    fn test_unicode_content() {
        let content = "# 日本語のタイトル\n\nこれは日本語の内容です。\n\n## Español\n\nEste es un documento en español.\n\n## Русский\n\nЭто русский текст.".as_bytes();

        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = DjotExtractor::extract_frontmatter(&text);
        assert!(yaml.is_none());

        let parser = Parser::new(&remaining);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());

        assert!(extracted.contains("日本語"));
        assert!(extracted.contains("Español"));
        assert!(extracted.contains("Русский"));
    }

    #[tokio::test]
    async fn test_full_extraction_with_frontmatter_and_tables() {
        let content = b"---\ntitle: Complete Document\nauthor: Test Author\ndate: 2024-01-20\n---\n\n# Document\n\nIntroduction text.\n\n| Name | Value |\n|------|-------|\n| A    | 1     |\n| B    | 2     |";

        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(content, "text/x-djot", &ExtractionConfig::default())
            .await
            .expect("Should extract djot with frontmatter and tables");

        assert_eq!(result.mime_type, "text/x-djot");
        assert!(result.content.contains("Introduction text"));
        assert_eq!(
            result.metadata.additional.get("title").and_then(|v| v.as_str()),
            Some("Complete Document")
        );
        assert_eq!(
            result.metadata.additional.get("author").and_then(|v| v.as_str()),
            Some("Test Author")
        );
        assert!(!result.tables.is_empty());
    }

    #[test]
    fn test_plugin_interface() {
        let extractor = DjotExtractor::new();
        assert_eq!(extractor.name(), "djot-extractor");
        assert_eq!(extractor.version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(extractor.priority(), 50);
        assert!(extractor.supported_mime_types().contains(&"text/djot"));
    }

    #[test]
    fn test_cells_to_markdown() {
        let cells = vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string()],
            vec!["Data 3".to_string(), "Data 4".to_string()],
        ];

        let markdown = DjotExtractor::cells_to_markdown(&cells);
        assert!(markdown.contains("Header 1"));
        assert!(markdown.contains("Data 1"));
        assert!(markdown.contains("---"));
        let lines: Vec<&str> = markdown.lines().collect();
        assert!(lines.len() >= 4);
    }

    #[test]
    fn test_extract_djot_with_links() {
        let content = b"# Page\n\nCheck [Google](https://google.com) and [Rust](https://rust-lang.org).";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());

        assert!(extracted.contains("Google"));
        assert!(extracted.contains("Rust"));
    }

    #[test]
    fn test_extract_djot_with_code_blocks() {
        let content = b"# Code Example\n\n``` rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());

        assert!(extracted.contains("main"));
        assert!(extracted.contains("println"));
    }

    #[test]
    fn test_djot_emphasis_syntax() {
        // Djot uses _ for emphasis and * for strong (opposite of Markdown)
        let content = b"This has _emphasis_ and *strong* text.";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());

        assert!(extracted.contains("emphasis"));
        assert!(extracted.contains("strong"));
    }

    #[test]
    fn test_djot_smart_punctuation() {
        let content = b"He said \"Hello\" and she said 'Hi'... That's nice---really.";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(events.into_iter());

        // Smart quotes and dashes should be converted to ASCII equivalents
        assert!(extracted.contains("Hello"));
        assert!(extracted.contains("Hi"));
    }
}
