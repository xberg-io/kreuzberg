//! JATS (Journal Article Tag Suite) document extractor.
//!
//! This extractor handles JATS XML documents, the standard format for scientific journal articles
//! used by PubMed Central and major academic publishers.
//!
//! It extracts:
//! - Rich metadata (title, subtitle, authors with affiliations, DOI, PII, keywords, dates)
//! - Article abstract (regular and graphical)
//! - Section hierarchy and content (intro, methods, results, discussion)
//! - Paragraphs and text content
//! - Tables with captions
//! - Figures with captions
//! - Citations and references
//! - Supplementary material information

mod elements;
mod metadata;
mod parser;

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::{ExtractionResult, Metadata};
use async_trait::async_trait;
#[cfg(feature = "tokio-runtime")]
use std::path::Path;

use elements::extract_jats_all_in_one;

/// JATS document extractor.
///
/// Supports JATS (Journal Article Tag Suite) XML documents in various versions,
/// handling both the full article structure and minimal JATS subsets.
pub struct JatsExtractor;

impl Default for JatsExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl JatsExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for JatsExtractor {
    fn name(&self) -> &str {
        "jats-extractor"
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
}

#[async_trait]
impl DocumentExtractor for JatsExtractor {
    #[cfg_attr(
        feature = "otel",
        tracing::instrument(
            skip(self, content, config),
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
        config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        let _ = config;
        let jats_content = std::str::from_utf8(content)
            .map(|s| s.to_string())
            .unwrap_or_else(|_| String::from_utf8_lossy(content).to_string());

        let (jats_metadata, extracted_content, _title, tables) = extract_jats_all_in_one(&jats_content)?;

        let mut metadata = Metadata::default();
        let mut subject_parts = Vec::new();

        if !jats_metadata.title.is_empty() {
            metadata.subject = Some(jats_metadata.title.clone());
            subject_parts.push(format!("Title: {}", jats_metadata.title));
        }

        if let Some(subtitle) = &jats_metadata.subtitle {
            subject_parts.push(format!("Subtitle: {}", subtitle));
        }

        if !jats_metadata.authors.is_empty() {
            subject_parts.push(format!("Authors: {}", jats_metadata.authors.join("; ")));
        }

        if !jats_metadata.affiliations.is_empty() {
            subject_parts.push(format!("Affiliations: {}", jats_metadata.affiliations.join("; ")));
        }

        if let Some(doi) = &jats_metadata.doi {
            subject_parts.push(format!("DOI: {}", doi));
        }

        if let Some(pii) = &jats_metadata.pii {
            subject_parts.push(format!("PII: {}", pii));
        }

        if !jats_metadata.keywords.is_empty() {
            subject_parts.push(format!("Keywords: {}", jats_metadata.keywords.join("; ")));
        }

        if let Some(date) = &jats_metadata.publication_date {
            metadata.created_at = Some(date.clone());
            subject_parts.push(format!("Publication Date: {}", date));
        }

        if let Some(volume) = &jats_metadata.volume {
            subject_parts.push(format!("Volume: {}", volume));
        }

        if let Some(issue) = &jats_metadata.issue {
            subject_parts.push(format!("Issue: {}", issue));
        }

        if let Some(pages) = &jats_metadata.pages {
            subject_parts.push(format!("Pages: {}", pages));
        }

        if let Some(journal_title) = &jats_metadata.journal_title {
            subject_parts.push(format!("Journal: {}", journal_title));
        }

        if let Some(article_type) = &jats_metadata.article_type {
            subject_parts.push(format!("Article Type: {}", article_type));
        }

        if let Some(abstract_text) = &jats_metadata.abstract_text {
            subject_parts.push(format!("Abstract: {}", abstract_text));
        }

        if let Some(corresp_author) = &jats_metadata.corresponding_author {
            subject_parts.push(format!("Corresponding Author: {}", corresp_author));
        }

        if !subject_parts.is_empty() {
            metadata.subject = Some(subject_parts.join(" | "));
        }

        Ok(ExtractionResult {
            content: extracted_content,
            mime_type: mime_type.to_string().into(),
            metadata,
            tables,
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
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

    #[cfg(feature = "tokio-runtime")]
    #[cfg_attr(
        feature = "otel",
        tracing::instrument(
            skip(self, path, config),
            fields(
                extractor.name = self.name(),
            )
        )
    )]
    #[cfg(feature = "tokio-runtime")]
    async fn extract_file(&self, path: &Path, mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult> {
        let bytes = tokio::fs::read(path).await?;
        self.extract_bytes(&bytes, mime_type, config).await
    }

    fn supported_mime_types(&self) -> &[&str] {
        &["application/x-jats+xml", "text/jats"]
    }

    fn priority(&self) -> i32 {
        50
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use elements::extract_jats_all_in_one;

    #[test]
    fn test_jats_extractor_plugin_interface() {
        let extractor = JatsExtractor::new();
        assert_eq!(extractor.name(), "jats-extractor");
        assert!(extractor.initialize().is_ok());
        assert!(extractor.shutdown().is_ok());
    }

    #[test]
    fn test_jats_extractor_supported_mime_types() {
        let extractor = JatsExtractor::new();
        let mime_types = extractor.supported_mime_types();
        assert_eq!(mime_types.len(), 2);
        assert!(mime_types.contains(&"application/x-jats+xml"));
        assert!(mime_types.contains(&"text/jats"));
    }

    #[test]
    fn test_jats_extractor_priority() {
        let extractor = JatsExtractor::new();
        assert_eq!(extractor.priority(), 50);
    }

    #[test]
    fn test_parse_simple_jats_article() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-title>Test Article Title</article-title>
    </article-meta>
  </front>
  <body>
    <p>Test content paragraph.</p>
  </body>
</article>"#;

        let (metadata, content, title, _tables) = extract_jats_all_in_one(jats).expect("Parse failed");
        assert_eq!(title, "Test Article Title");
        assert_eq!(metadata.title, "Test Article Title");
        assert!(content.contains("Test Article Title"));
        assert!(content.contains("Test content"));
    }

    #[test]
    fn test_extract_jats_title() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-title>Effects of Caffeine on Human Health</article-title>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.title, "Effects of Caffeine on Human Health");
    }

    #[test]
    fn test_extract_jats_subtitle() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-title>Main Title</article-title>
      <subtitle>A Systematic Review</subtitle>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.title, "Main Title");
        assert_eq!(metadata.subtitle, Some("A Systematic Review".to_string()));
    }

    #[test]
    fn test_extract_jats_authors() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <contrib-group>
        <contrib contrib-type="author">
          <name>
            <surname>Smith</surname>
            <given-names>John A.</given-names>
          </name>
        </contrib>
        <contrib contrib-type="author">
          <name>
            <surname>Johnson</surname>
            <given-names>Jane B.</given-names>
          </name>
        </contrib>
      </contrib-group>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.authors.len(), 2);
        assert!(metadata.authors[0].contains("Smith"));
        assert!(metadata.authors[1].contains("Johnson"));
    }

    #[test]
    fn test_extract_jats_affiliations() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <aff id="aff1">Department of Medicine, Harvard University, Cambridge, MA</aff>
      <aff id="aff2">Center for Health Research, Boston Medical Center, Boston, MA</aff>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.affiliations.len(), 2);
        assert!(metadata.affiliations[0].contains("Harvard"));
        assert!(metadata.affiliations[1].contains("Boston"));
    }

    #[test]
    fn test_extract_jats_doi_and_pii() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-id pub-id-type="doi">10.1371/journal.pmed.0020124</article-id>
      <article-id pub-id-type="pii">05-PLME-RA-0071R2</article-id>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.doi, Some("10.1371/journal.pmed.0020124".to_string()));
        assert_eq!(metadata.pii, Some("05-PLME-RA-0071R2".to_string()));
    }

    #[test]
    fn test_extract_jats_keywords() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <kwd-group>
        <kwd>caffeine</kwd>
        <kwd>meta-analysis</kwd>
        <kwd>systematic review</kwd>
      </kwd-group>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.keywords.len(), 3);
        assert!(metadata.keywords.contains(&"caffeine".to_string()));
        assert!(metadata.keywords.contains(&"meta-analysis".to_string()));
    }

    #[test]
    fn test_extract_jats_publication_info() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <pub-date pub-type="epub">
        <day>18</day>
        <month>04</month>
        <year>2005</year>
      </pub-date>
      <volume>2</volume>
      <issue>4</issue>
      <fpage>e124</fpage>
      <lpage>e132</lpage>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert!(metadata.publication_date.is_some());
        assert_eq!(metadata.volume, Some("2".to_string()));
        assert_eq!(metadata.issue, Some("4".to_string()));
        assert!(metadata.pages.is_some());
    }

    #[test]
    fn test_extract_jats_abstract() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <abstract>
        <sec>
          <title>Background</title>
          <p>This is the background information of the study.</p>
        </sec>
        <sec>
          <title>Methods</title>
          <p>We used quantitative analysis to evaluate the hypothesis.</p>
        </sec>
      </abstract>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert!(metadata.abstract_text.is_some());
        let abstract_text = metadata.abstract_text.unwrap();
        assert!(abstract_text.contains("background"));
        assert!(abstract_text.contains("quantitative"));
    }

    #[test]
    fn test_extract_jats_tables_basic() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <body>
    <table-wrap id="tbl1">
      <table>
        <thead>
          <tr>
            <th>Study</th>
            <th>Year</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Study A</td>
            <td>2003</td>
          </tr>
          <tr>
            <td>Study B</td>
            <td>2004</td>
          </tr>
        </tbody>
      </table>
    </table-wrap>
  </body>
</article>"#;

        let (_metadata, _content, _title, tables) = extract_jats_all_in_one(jats).expect("Table extraction failed");
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].cells.len(), 3);
    }

    #[test]
    fn test_extract_jats_corresponding_author() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <author-notes>
        <corresp id="cor1">To whom correspondence should be addressed. E-mail: rwilliams@yale.edu</corresp>
      </author-notes>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert!(metadata.corresponding_author.is_some());
        let corresp = metadata.corresponding_author.unwrap();
        assert!(corresp.contains("rwilliams"));
    }

    #[test]
    fn test_extract_jats_section_hierarchy() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-title>Article Title</article-title>
    </article-meta>
  </front>
  <body>
    <sec id="s1">
      <title>Introduction</title>
      <p>Intro content.</p>
    </sec>
    <sec id="s2">
      <title>Methods</title>
      <p>Methods content.</p>
    </sec>
    <sec id="s3">
      <title>Results</title>
      <p>Results content.</p>
    </sec>
  </body>
</article>"#;

        let (_metadata, content, _title, _tables) = extract_jats_all_in_one(jats).expect("Parse failed");
        assert!(content.contains("Introduction"));
        assert!(content.contains("Methods"));
        assert!(content.contains("Results"));
    }

    #[test]
    fn test_jats_extractor_full_metadata_extraction() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-title>Sample Article</article-title>
      <contrib-group>
        <contrib contrib-type="author">
          <name>
            <surname>Smith</surname>
            <given-names>John</given-names>
          </name>
        </contrib>
      </contrib-group>
      <article-id pub-id-type="doi">10.1234/test</article-id>
      <kwd-group>
        <kwd>test</kwd>
      </kwd-group>
      <abstract>
        <p>Test abstract.</p>
      </abstract>
    </article-meta>
  </front>
  <body>
    <p>Sample content.</p>
  </body>
</article>"#;

        let (metadata_extracted, _content, _title, _tables) =
            extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata_extracted.title, "Sample Article");
        assert_eq!(metadata_extracted.authors.len(), 1);
        assert_eq!(metadata_extracted.doi, Some("10.1234/test".to_string()));
        assert_eq!(metadata_extracted.keywords.len(), 1);
        assert!(metadata_extracted.abstract_text.is_some());
    }

    #[test]
    fn test_jats_extractor_empty_article() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
    </article-meta>
  </front>
  <body>
  </body>
</article>"#;

        let (metadata, content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert!(metadata.title.is_empty());
        assert!(content.is_empty() || content.trim().is_empty());
    }

    #[test]
    fn test_extract_jats_journal_title() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-title>Test Article</article-title>
      <journal-title>Nature Medicine</journal-title>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.journal_title, Some("Nature Medicine".to_string()));
    }

    #[test]
    fn test_extract_jats_article_type() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article article-type="research-article">
  <front>
    <article-meta>
      <article-title>Test Article</article-title>
    </article-meta>
  </front>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");
        assert_eq!(metadata.article_type, Some("research-article".to_string()));
    }

    #[test]
    fn test_extract_all_13_metadata_fields() {
        let jats = r#"<?xml version="1.0" encoding="UTF-8"?>
<article article-type="research-article">
  <front>
    <article-meta>
      <article-title>Full Metadata Test</article-title>
      <subtitle>A Complete Example</subtitle>
      <contrib-group>
        <contrib contrib-type="author">
          <name>
            <surname>Author</surname>
            <given-names>First</given-names>
          </name>
        </contrib>
      </contrib-group>
      <aff>Department of Testing, Test University</aff>
      <article-id pub-id-type="doi">10.1234/full-test</article-id>
      <article-id pub-id-type="pii">TEST-001</article-id>
      <kwd-group>
        <kwd>testing</kwd>
        <kwd>metadata</kwd>
      </kwd-group>
      <pub-date pub-type="epub">
        <year>2024</year>
      </pub-date>
      <volume>5</volume>
      <issue>3</issue>
      <fpage>100</fpage>
      <lpage>110</lpage>
      <journal-title>Test Journal</journal-title>
      <abstract>
        <p>This is a test abstract for all metadata fields.</p>
      </abstract>
      <author-notes>
        <corresp>Correspondence: test@example.com</corresp>
      </author-notes>
    </article-meta>
  </front>
  <body>
    <p>Test content.</p>
  </body>
</article>"#;

        let (metadata, _content, _title, _tables) = extract_jats_all_in_one(jats).expect("Metadata extraction failed");

        assert_eq!(metadata.title, "Full Metadata Test");
        assert_eq!(metadata.subtitle, Some("A Complete Example".to_string()));
        assert_eq!(metadata.authors.len(), 1);
        assert!(metadata.authors[0].contains("Author"));
        assert_eq!(metadata.affiliations.len(), 1);
        assert!(metadata.affiliations[0].contains("Testing"));
        assert_eq!(metadata.doi, Some("10.1234/full-test".to_string()));
        assert_eq!(metadata.pii, Some("TEST-001".to_string()));
        assert_eq!(metadata.keywords.len(), 2);
        assert!(metadata.publication_date.is_some());
        assert_eq!(metadata.volume, Some("5".to_string()));
        assert_eq!(metadata.issue, Some("3".to_string()));
        assert!(metadata.pages.is_some());
        assert_eq!(metadata.journal_title, Some("Test Journal".to_string()));
        assert_eq!(metadata.article_type, Some("research-article".to_string()));
        assert!(metadata.abstract_text.is_some());
        assert!(metadata.corresponding_author.is_some());
    }
}
