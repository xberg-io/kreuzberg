//! Djot document extractor with plugin integration.
//!
//! Implements the DocumentExtractor and Plugin traits for Djot markup files.

use super::parsing::{extract_complete_djot_content, extract_tables_from_events, extract_text_from_events};
use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::{ExtractionResult, Metadata};
use async_trait::async_trait;
use jotdown::{Event, Parser};

/// Djot markup extractor with metadata and table support.
///
/// Parses Djot documents with YAML frontmatter, extracting:
/// - Metadata from YAML frontmatter
/// - Plain text content
/// - Tables as structured data
/// - Document structure (headings, links, code blocks)
#[derive(Debug, Clone)]
pub struct DjotExtractor;

impl DjotExtractor {
    /// Create a new Djot extractor.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DjotExtractor {
    fn default() -> Self {
        Self::new()
    }
}

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

#[async_trait]
impl DocumentExtractor for DjotExtractor {
    #[cfg_attr(
        feature = "otel",
        tracing::instrument(
            skip(self, content, _config),
            fields(
                extractor.name = self.name(),
                content.size_bytes = content.len(),
            )
        )
    )]
    async fn extract_bytes(
        &self,
        content: &[u8],
        mime_type: &str,
        _config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining_content) = crate::extractors::frontmatter_utils::extract_frontmatter(&text);

        let mut metadata = if let Some(ref yaml_value) = yaml {
            crate::extractors::frontmatter_utils::extract_metadata_from_yaml(yaml_value)
        } else {
            Metadata::default()
        };

        if !metadata.additional.contains_key("title")
            && let Some(title) = crate::extractors::frontmatter_utils::extract_title_from_content(&remaining_content)
        {
            metadata.additional.insert("title".to_string(), title.into());
        }

        // Parse with jotdown and collect events once for extraction
        let parser = Parser::new(&remaining_content);
        let events: Vec<Event> = parser.collect();

        let extracted_text = extract_text_from_events(&events);
        let tables = extract_tables_from_events(&events);

        // Extract complete djot content with all features
        let djot_content = extract_complete_djot_content(&events, metadata.clone(), tables.clone());

        Ok(ExtractionResult {
            content: extracted_text,
            mime_type: mime_type.to_string(),
            metadata,
            tables,
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            djot_content: Some(djot_content),
            elements: None,
        })
    }

    fn supported_mime_types(&self) -> &[&str] {
        &["text/djot", "text/x-djot"]
    }

    fn priority(&self) -> i32 {
        50
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_djot_extractor_creation() {
        let extractor = DjotExtractor::new();
        assert_eq!(extractor.name(), "djot-extractor");
    }

    #[test]
    fn test_can_extract_djot_mime_types() {
        let extractor = DjotExtractor::new();
        let mime_types = extractor.supported_mime_types();

        assert!(mime_types.contains(&"text/djot"));
        assert!(mime_types.contains(&"text/x-djot"));
    }

    #[test]
    fn test_plugin_interface() {
        let extractor = DjotExtractor::new();
        assert_eq!(extractor.author(), "Kreuzberg Team");
        assert!(!extractor.version().is_empty());
        assert!(!extractor.description().is_empty());
    }

    #[tokio::test]
    async fn test_extract_simple_djot() {
        let content =
            b"# Header\n\nThis is a paragraph with *bold* and _italic_ text.\n\n## Subheading\n\nMore content here.";
        let extractor = DjotExtractor::new();
        let config = ExtractionConfig::default();

        let result = extractor.extract_bytes(content, "text/djot", &config).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.content.contains("Header"));
        assert!(result.content.contains("This is a paragraph"));
        assert!(result.content.contains("bold"));
        assert!(result.content.contains("italic"));
    }
}
