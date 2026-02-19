//! Citation format extractors for RIS, PubMed/MEDLINE, and EndNote XML.
//!
//! Extracts and parses citation files in various formats, providing structured access
//! to bibliography entries, metadata, and author information.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::{ExtractionResult, Metadata};
use ahash::AHashMap;
use async_trait::async_trait;
use std::borrow::Cow;
use std::collections::HashSet;

#[cfg(feature = "office")]
use biblib::{CitationParser, EndNoteXmlParser, PubMedParser, RisParser};

/// Citation format extractor for RIS, PubMed/MEDLINE, and EndNote XML formats.
///
/// Parses citation files and extracts structured bibliography data including
/// entries, authors, publication years, and format-specific metadata.
pub struct CitationExtractor;

impl CitationExtractor {
    /// Create a new citation extractor.
    pub fn new() -> Self {
        Self
    }
}

impl Default for CitationExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for CitationExtractor {
    fn name(&self) -> &str {
        "citation-extractor"
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
        "Extracts and parses citation files (RIS, PubMed/MEDLINE, EndNote XML) with structured metadata"
    }

    fn author(&self) -> &str {
        "Kreuzberg Team"
    }
}

#[cfg(feature = "office")]
#[async_trait]
impl DocumentExtractor for CitationExtractor {
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
        let citation_str = String::from_utf8_lossy(content);

        let mut citations_vec = Vec::new();
        let mut authors_set = HashSet::new();
        let mut years_set = HashSet::new();
        let mut dois_vec = Vec::new();
        let mut keywords_set = HashSet::new();
        let mut formatted_content = String::new();

        // Parse based on MIME type
        let (parse_result, format_string) = match mime_type {
            "application/x-research-info-systems" => (RisParser::new().parse(&citation_str), "RIS"),
            "application/x-pubmed" => (PubMedParser::new().parse(&citation_str), "PubMed"),
            "application/x-endnote+xml" => (EndNoteXmlParser::new().parse(&citation_str), "EndNote XML"),
            _ => {
                // Fallback: return raw content if MIME type is unexpected
                let mut additional: AHashMap<Cow<'static, str>, serde_json::Value> = AHashMap::new();
                additional.insert(Cow::Borrowed("citation_count"), serde_json::json!(0));
                additional.insert(Cow::Borrowed("format"), serde_json::json!("Unknown"));

                return Ok(ExtractionResult {
                    content: citation_str.to_string(),
                    mime_type: mime_type.to_string().into(),
                    metadata: Metadata {
                        additional,
                        ..Default::default()
                    },
                    pages: None,
                    tables: vec![],
                    detected_languages: None,
                    chunks: None,
                    images: None,
                    djot_content: None,
                    elements: None,
                    ocr_elements: None,
                    document: None,
                    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
                    extracted_keywords: None,
                    quality_score: None,
                    processing_warnings: Vec::new(),
                    annotations: None,
                });
            }
        };

        match parse_result {
            Ok(citations) => {
                for citation in &citations {
                    citations_vec.push(citation.title.clone());

                    // Collect authors
                    for author in &citation.authors {
                        let author_name = if let Some(given) = &author.given_name {
                            format!("{} {}", given, author.name)
                        } else {
                            author.name.clone()
                        };
                        if !author_name.is_empty() {
                            authors_set.insert(author_name);
                        }
                    }

                    // Collect years
                    if let Some(date) = &citation.date
                        && date.year > 0
                    {
                        years_set.insert(date.year as u32);
                    }

                    // Collect DOIs
                    if let Some(doi) = &citation.doi
                        && !doi.is_empty()
                    {
                        dois_vec.push(doi.clone());
                    }

                    // Collect keywords
                    for keyword in &citation.keywords {
                        if !keyword.is_empty() {
                            keywords_set.insert(keyword.clone());
                        }
                    }

                    // Format citation as readable text
                    if !citation.title.is_empty() {
                        formatted_content.push_str(&format!("Title: {}\n", citation.title));
                    }

                    if !citation.authors.is_empty() {
                        let author_strings: Vec<String> = citation
                            .authors
                            .iter()
                            .map(|a| {
                                if let Some(given) = &a.given_name {
                                    format!("{} {}", given, a.name)
                                } else {
                                    a.name.clone()
                                }
                            })
                            .collect();
                        formatted_content.push_str(&format!("Authors: {}\n", author_strings.join(", ")));
                    }

                    if let Some(journal) = &citation.journal {
                        formatted_content.push_str(&format!("Journal: {}\n", journal));
                    }

                    if let Some(date) = &citation.date {
                        formatted_content.push_str(&format!("Year: {}\n", date.year));
                    }

                    if let Some(volume) = &citation.volume {
                        formatted_content.push_str(&format!("Volume: {}", volume));
                        if let Some(issue) = &citation.issue {
                            formatted_content.push_str(&format!(", Issue: {}", issue));
                        }
                        if let Some(pages) = &citation.pages {
                            formatted_content.push_str(&format!(", Pages: {}", pages));
                        }
                        formatted_content.push('\n');
                    }

                    if let Some(doi) = &citation.doi {
                        formatted_content.push_str(&format!("DOI: {}\n", doi));
                    }

                    if let Some(pmid) = &citation.pmid {
                        formatted_content.push_str(&format!("PMID: {}\n", pmid));
                    }

                    if let Some(abstract_text) = &citation.abstract_text
                        && !abstract_text.is_empty()
                    {
                        formatted_content.push_str(&format!("Abstract: {}\n", abstract_text));
                    }

                    if !citation.keywords.is_empty() {
                        formatted_content.push_str(&format!("Keywords: {}\n", citation.keywords.join(", ")));
                    }

                    formatted_content.push_str("---\n");
                }
            }
            Err(_err) => {
                #[cfg(feature = "otel")]
                tracing::warn!("Citation parsing failed, returning raw content: {}", _err);
                formatted_content = citation_str.to_string();
            }
        }

        let mut additional: AHashMap<Cow<'static, str>, serde_json::Value> = AHashMap::new();

        additional.insert(Cow::Borrowed("citation_count"), serde_json::json!(citations_vec.len()));

        let mut authors_list: Vec<String> = authors_set.into_iter().collect();
        authors_list.sort();
        additional.insert(Cow::Borrowed("authors"), serde_json::json!(authors_list));

        if !years_set.is_empty() {
            let min_year = years_set.iter().min().copied().unwrap_or(0);
            let max_year = years_set.iter().max().copied().unwrap_or(0);
            let mut years_sorted: Vec<u32> = years_set.into_iter().collect();
            years_sorted.sort_unstable();
            additional.insert(
                Cow::Borrowed("year_range"),
                serde_json::json!({
                    "min": min_year,
                    "max": max_year,
                    "years": years_sorted
                }),
            );
        }

        if !dois_vec.is_empty() {
            additional.insert(Cow::Borrowed("dois"), serde_json::json!(dois_vec));
        }

        let mut keywords_list: Vec<String> = keywords_set.into_iter().collect();
        keywords_list.sort();
        if !keywords_list.is_empty() {
            additional.insert(Cow::Borrowed("keywords"), serde_json::json!(keywords_list));
        }

        additional.insert(Cow::Borrowed("format"), serde_json::json!(format_string));

        Ok(ExtractionResult {
            content: formatted_content,
            mime_type: mime_type.to_string().into(),
            metadata: Metadata {
                additional,
                ..Default::default()
            },
            pages: None,
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            elements: None,
            ocr_elements: None,
            document: None,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            extracted_keywords: None,
            quality_score: None,
            processing_warnings: Vec::new(),
            annotations: None,
        })
    }

    fn supported_mime_types(&self) -> &[&str] {
        &[
            "application/x-research-info-systems",
            "application/x-pubmed",
            "application/x-endnote+xml",
        ]
    }

    fn priority(&self) -> i32 {
        60
    }
}

#[cfg(all(test, feature = "office"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_can_extract_citation_mime_types() {
        let extractor = CitationExtractor::new();
        let supported = extractor.supported_mime_types();

        assert!(supported.contains(&"application/x-research-info-systems"));
        assert!(supported.contains(&"application/x-pubmed"));
        assert!(supported.contains(&"application/x-endnote+xml"));
        assert_eq!(supported.len(), 3);
    }

    #[tokio::test]
    async fn test_extract_simple_ris() {
        let extractor = CitationExtractor::new();
        let ris_content = br#"TY  - JOUR
TI  - Sample Title
AU  - Smith, John
PY  - 2023
ER  -"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(ris_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract valid RIS entry");

        assert!(result.content.contains("Sample Title"));
        assert!(result.content.contains("Smith"));

        let metadata = &result.metadata;
        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("citation_count")),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("format")),
            Some(&serde_json::json!("RIS"))
        );
    }

    #[tokio::test]
    async fn test_extract_multiple_ris_entries() {
        let extractor = CitationExtractor::new();
        let ris_content = br#"TY  - JOUR
TI  - First Paper
AU  - Author One
PY  - 2020
ER  -

TY  - JOUR
TI  - Second Paper
AU  - Author Two
PY  - 2021
ER  -"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(ris_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract multiple RIS entries");

        let metadata = &result.metadata;

        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("citation_count")),
            Some(&serde_json::json!(2))
        );

        if let Some(year_range) = metadata.additional.get("year_range") {
            assert_eq!(year_range.get("min"), Some(&serde_json::json!(2020)));
            assert_eq!(year_range.get("max"), Some(&serde_json::json!(2021)));
        }
    }

    #[tokio::test]
    async fn test_extract_ris_with_doi() {
        let extractor = CitationExtractor::new();
        let ris_content = br#"TY  - JOUR
TI  - Sample Article
AU  - Smith, John
DO  - 10.1234/example.doi
PY  - 2023
ER  -"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(ris_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract RIS with DOI");

        let metadata = &result.metadata;
        if let Some(dois) = metadata.additional.get("dois") {
            assert!(!dois.as_array().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn test_extract_empty_citation_file() {
        let extractor = CitationExtractor::new();
        let empty_content = b"";

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(empty_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should handle empty citation file");

        let metadata = &result.metadata;

        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("citation_count")),
            Some(&serde_json::json!(0))
        );
    }

    #[tokio::test]
    async fn test_extract_malformed_ris() {
        let extractor = CitationExtractor::new();
        let malformed_content = b"This is not valid RIS format\nJust some random text";

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(malformed_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract malformed as raw content");

        // When RIS parser encounters unparseable content, it may return empty results
        // Verify we get a result either way
        let metadata = &result.metadata;
        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("citation_count")),
            Some(&serde_json::json!(0))
        );
    }

    #[tokio::test]
    async fn test_citation_extractor_plugin_interface() {
        let extractor = CitationExtractor::new();
        assert_eq!(extractor.name(), "citation-extractor");
        assert_eq!(extractor.version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(extractor.priority(), 60);
        assert!(!extractor.supported_mime_types().is_empty());
    }

    #[test]
    fn test_citation_extractor_default() {
        let extractor = CitationExtractor;
        assert_eq!(extractor.name(), "citation-extractor");
    }

    #[tokio::test]
    async fn test_citation_extractor_initialize_shutdown() {
        let extractor = CitationExtractor::new();
        assert!(extractor.initialize().is_ok());
        assert!(extractor.shutdown().is_ok());
    }

    #[tokio::test]
    async fn test_extract_ris_with_keywords() {
        let extractor = CitationExtractor::new();
        let ris_content = br#"TY  - JOUR
TI  - Sample Article
AU  - Smith, John
KW  - keyword1
KW  - keyword2
KW  - keyword3
PY  - 2023
ER  -"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(ris_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract RIS with keywords");

        let metadata = &result.metadata;
        if let Some(keywords) = metadata.additional.get("keywords") {
            assert!(!keywords.as_array().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn test_extract_ris_with_multiple_authors() {
        let extractor = CitationExtractor::new();
        let ris_content = br#"TY  - JOUR
TI  - Collaborative Work
AU  - First Author
AU  - Second Author
AU  - Third Author
PY  - 2023
ER  -"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(ris_content, "application/x-research-info-systems", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract multiple authors");

        let metadata = &result.metadata;
        if let Some(authors) = metadata.additional.get("authors") {
            assert!(!authors.as_array().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn test_extract_pubmed_format() {
        let extractor = CitationExtractor::new();
        let pubmed_content = br#"PMID- 12345678
TI  - Sample PubMed Article
FAU - Smith, John
DP  - 2023"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(pubmed_content, "application/x-pubmed", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract PubMed format");

        let metadata = &result.metadata;
        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("format")),
            Some(&serde_json::json!("PubMed"))
        );
    }

    #[tokio::test]
    async fn test_extract_endnote_xml_format() {
        let extractor = CitationExtractor::new();
        let endnote_content = br#"<?xml version="1.0" encoding="UTF-8"?>
<xml>
  <records>
    <record>
      <titles>
        <title>Sample EndNote Article</title>
      </titles>
      <authors>
        <author>Smith, John</author>
      </authors>
    </record>
  </records>
</xml>"#;

        let config = ExtractionConfig::default();
        let result = extractor
            .extract_bytes(endnote_content, "application/x-endnote+xml", &config)
            .await;

        assert!(result.is_ok());
        let result = result.expect("Should extract EndNote XML format");

        let metadata = &result.metadata;
        assert_eq!(
            metadata.additional.get(&Cow::Borrowed("format")),
            Some(&serde_json::json!("EndNote XML"))
        );
    }
}
