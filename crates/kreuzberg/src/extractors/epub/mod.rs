//! Native EPUB extractor using permissive-licensed dependencies.
//!
//! This extractor provides native Rust-based EPUB extraction without GPL-licensed
//! dependencies, extracting:
//! - Metadata from OPF (Open Packaging Format) using Dublin Core standards
//! - Content from XHTML files in spine order
//! - Proper handling of EPUB2 and EPUB3 formats
//!
//! Uses only permissive-licensed crates:
//! - `zip` (MIT/Apache) - for reading EPUB container
//! - `roxmltree` (MIT) - for parsing XML
//! - `html-to-markdown-rs` (MIT) - for converting XHTML to Markdown/Djot when requested

mod content;
mod metadata;
mod parsing;

use crate::Result;
use crate::core::config::{ExtractionConfig, OutputFormat};
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::ExtractionResult;
use crate::types::Metadata;
use crate::types::ProcessingWarning;
use ahash::AHashMap;
use async_trait::async_trait;
use std::borrow::Cow;
use std::io::Cursor;
use zip::ZipArchive;

use content::extract_text_from_xhtml;
use content::read_body_documents;
use metadata::{build_additional_metadata, parse_opf};
use parsing::{parse_container_xml, read_file_from_zip};

/// EPUB format extractor using permissive-licensed dependencies.
///
/// Extracts content and metadata from EPUB files (both EPUB2 and EPUB3)
/// using native Rust parsing without GPL-licensed dependencies.
pub struct EpubExtractor;

impl EpubExtractor {
    /// Create a new EPUB extractor.
    pub fn new() -> Self {
        Self
    }
}

impl Default for EpubExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "office")]
struct RenderedSpineDocument {
    content_fragment: String,
    content_fully_converted: bool,
    document: Option<crate::types::document_structure::DocumentStructure>,
    warnings: Vec<ProcessingWarning>,
}

#[cfg(feature = "office")]
fn trim_trailing_newlines(s: &str) -> &str {
    s.trim_end_matches(['\n', '\r'])
}

#[cfg(feature = "office")]
impl EpubExtractor {
    fn build_fallback_document_structure(
        document: &content::EpubSpineDocument,
        index: usize,
    ) -> crate::types::document_structure::DocumentStructure {
        use crate::types::builder::DocumentStructureBuilder;

        let mut builder = DocumentStructureBuilder::new().source_format("epub");
        let chapter_title =
            extract_title_from_xhtml(&document.xhtml).unwrap_or_else(|| format!("Chapter {}", index + 1));
        builder.push_heading(1, &chapter_title, None, None);

        let text = extract_text_from_xhtml(&document.xhtml);
        for paragraph in text.split("\n\n") {
            let trimmed = paragraph.trim();
            if !trimmed.is_empty() {
                builder.push_paragraph(trimmed, vec![], None, None);
            }
        }

        builder.build()
    }

    /// Render a spine document once.
    fn render_spine_document(
        document: &content::EpubSpineDocument,
        index: usize,
        config: &ExtractionConfig,
    ) -> RenderedSpineDocument {
        let wants_markup = matches!(config.output_format, OutputFormat::Markdown | OutputFormat::Djot);
        let mut warnings = Vec::new();

        let (content_fragment, content_fully_converted) = if wants_markup {
            match crate::extraction::html::convert_html_to_markdown_with_metadata(
                &document.xhtml,
                config.html_options.clone(),
                Some(config.output_format),
            ) {
                Ok((converted, _)) => (trim_trailing_newlines(&converted).to_string(), true),
                Err(err) => {
                    warnings.push(ProcessingWarning {
                        source: std::borrow::Cow::Borrowed("epub"),
                        message: std::borrow::Cow::Owned(format!(
                            "XHTML conversion failed for spine item '{}'; falling back to plain text: {}",
                            document.file_path, err
                        )),
                    });
                    (extract_text_from_xhtml(&document.xhtml).trim_end().to_string(), false)
                }
            }
        } else {
            (extract_text_from_xhtml(&document.xhtml).trim_end().to_string(), true)
        };

        let document = if config.include_document_structure {
            let chapter_structure = crate::extraction::html::structure::build_document_structure(&document.xhtml);

            if chapter_structure.nodes.is_empty() {
                warnings.push(ProcessingWarning {
                    source: std::borrow::Cow::Borrowed("epub"),
                    message: std::borrow::Cow::Owned(format!(
                        "Document structure extraction produced no nodes for spine item '{}'; falling back to plain-text structure",
                        document.file_path
                    )),
                });
                Some(Self::build_fallback_document_structure(document, index))
            } else {
                Some(chapter_structure)
            }
        } else {
            None
        };

        RenderedSpineDocument {
            content_fragment,
            content_fully_converted,
            document,
            warnings,
        }
    }

    fn build_document_structure(
        rendered_documents: &[RenderedSpineDocument],
    ) -> Option<crate::types::document_structure::DocumentStructure> {
        use crate::types::builder::DocumentStructureBuilder;

        let mut builder = DocumentStructureBuilder::new().source_format("epub");
        let mut has_nodes = false;

        for rendered in rendered_documents {
            let Some(chapter_structure) = &rendered.document else {
                continue;
            };

            for node in &chapter_structure.nodes {
                has_nodes = true;
                builder.push_raw(
                    node.content.clone(),
                    None,
                    None,
                    node.content_layer,
                    node.annotations.clone(),
                );
            }
        }

        if has_nodes { Some(builder.build()) } else { None }
    }
}

/// Extract the first heading text from XHTML content.
#[cfg(feature = "office")]
fn extract_title_from_xhtml(xhtml: &str) -> Option<String> {
    let sanitized = content::normalize_xhtml(xhtml);
    let doc = roxmltree::Document::parse(&sanitized).ok()?;

    for node in doc.root().descendants() {
        if node.is_element() {
            let tag = node.tag_name().name().to_ascii_lowercase();
            if matches!(tag.as_str(), "h1" | "h2" | "h3") {
                let text: String = node
                    .descendants()
                    .filter(|n| n.is_text())
                    .filter_map(|n| n.text())
                    .collect::<Vec<_>>()
                    .join("");
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
        }
    }
    None
}

impl Plugin for EpubExtractor {
    fn name(&self) -> &str {
        "epub-extractor"
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
        "Extracts content and metadata from EPUB documents (native Rust implementation with permissive licenses)"
    }

    fn author(&self) -> &str {
        "Kreuzberg Team"
    }
}

#[cfg(feature = "office")]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl DocumentExtractor for EpubExtractor {
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
        let cursor = Cursor::new(content.to_vec());

        let mut archive = ZipArchive::new(cursor).map_err(|e| crate::KreuzbergError::Parsing {
            message: format!("Failed to open EPUB as ZIP: {}", e),
            source: None,
        })?;

        let container_xml = read_file_from_zip(&mut archive, "META-INF/container.xml")?;
        let opf_path = parse_container_xml(&container_xml)?;

        let manifest_dir = if let Some(last_slash) = opf_path.rfind('/') {
            opf_path[..last_slash].to_string()
        } else {
            String::new()
        };

        let opf_xml = read_file_from_zip(&mut archive, &opf_path)?;
        let (package, mut processing_warnings) = parse_opf(&opf_xml, &manifest_dir)?;
        let additional_metadata = build_additional_metadata(&package.metadata);
        let (documents, content_warnings) = read_body_documents(&mut archive, &package)?;
        processing_warnings.extend(content_warnings);
        let mut rendered_documents = Vec::with_capacity(documents.len());
        for (index, document) in documents.iter().enumerate() {
            let mut rendered = Self::render_spine_document(document, index, config);
            processing_warnings.append(&mut rendered.warnings);
            rendered_documents.push(rendered);
        }
        let mut extracted_content = String::new();
        for rendered in &rendered_documents {
            if !rendered.content_fragment.is_empty() {
                if !extracted_content.is_empty() && !extracted_content.ends_with('\n') {
                    extracted_content.push('\n');
                }
                extracted_content.push_str(&rendered.content_fragment);
                extracted_content.push('\n');
            }
        }
        let extracted_content = trim_trailing_newlines(&extracted_content).to_string();
        let fully_converted = matches!(config.output_format, OutputFormat::Markdown | OutputFormat::Djot)
            && rendered_documents
                .iter()
                .all(|rendered| rendered.content_fully_converted);
        let metadata_map: AHashMap<Cow<'static, str>, serde_json::Value> = additional_metadata
            .into_iter()
            .map(|(k, v)| (Cow::Owned(k), v))
            .collect();

        // Signal that the extractor already formatted the output so the pipeline
        // does not double-convert (mirrors HtmlExtractor behavior).
        let pre_formatted = if fully_converted {
            match config.output_format {
                crate::core::config::OutputFormat::Markdown => Some("markdown".to_string()),
                crate::core::config::OutputFormat::Djot => Some("djot".to_string()),
                _ => None,
            }
        } else {
            None
        };

        // Build document structure from spine chapters (only when requested)
        let document = if config.include_document_structure {
            Self::build_document_structure(&rendered_documents)
        } else {
            None
        };

        Ok(ExtractionResult {
            content: extracted_content,
            mime_type: mime_type.to_string().into(),
            metadata: Metadata {
                title: package.metadata.title,
                authors: package.metadata.creator.map(|c| vec![c]),
                language: package.metadata.language,
                created_at: package.metadata.date,
                additional: metadata_map,
                output_format: pre_formatted,
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
            document,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            extracted_keywords: None,
            quality_score: None,
            processing_warnings,
            annotations: None,
            children: None,
        })
    }

    fn supported_mime_types(&self) -> &[&str] {
        &[
            "application/epub+zip",
            "application/x-epub+zip",
            "application/vnd.epub+zip",
        ]
    }

    fn priority(&self) -> i32 {
        60
    }
}

#[cfg(all(test, feature = "office"))]
mod tests {
    use super::*;

    #[test]
    fn test_epub_extractor_plugin_interface() {
        let extractor = EpubExtractor::new();
        assert_eq!(extractor.name(), "epub-extractor");
        assert_eq!(extractor.version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(extractor.priority(), 60);
        assert!(!extractor.supported_mime_types().is_empty());
    }

    #[test]
    fn test_epub_extractor_default() {
        let extractor = EpubExtractor;
        assert_eq!(extractor.name(), "epub-extractor");
    }

    #[tokio::test]
    async fn test_epub_extractor_initialize_shutdown() {
        let extractor = EpubExtractor::new();
        assert!(extractor.initialize().is_ok());
        assert!(extractor.shutdown().is_ok());
    }

    #[test]
    fn test_epub_extractor_supported_mime_types() {
        let extractor = EpubExtractor::new();
        let supported = extractor.supported_mime_types();
        assert!(supported.contains(&"application/epub+zip"));
        assert!(supported.contains(&"application/x-epub+zip"));
        assert!(supported.contains(&"application/vnd.epub+zip"));
    }

    #[test]
    fn test_epub_full_dublin_core_metadata() {
        let opf = r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>Test Book</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:language>en</dc:language>
    <dc:coverage>Worldwide</dc:coverage>
    <dc:format>application/epub+zip</dc:format>
    <dc:relation>http://example.com/related</dc:relation>
    <dc:source>Original Manuscript</dc:source>
    <dc:type>Text</dc:type>
    <dc:publisher>Test Publisher</dc:publisher>
    <dc:description>A test book</dc:description>
    <dc:rights>CC BY 4.0</dc:rights>
    <meta name="cover" content="cover-img"/>
  </metadata>
  <manifest>
    <item id="cover-img" href="images/cover.jpg" media-type="image/jpeg"/>
    <item id="ch1" href="ch1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
    <itemref idref="ch1"/>
  </spine>
</package>"#;

        let (package, _warnings) = metadata::parse_opf(opf, "").expect("Metadata parse failed");
        let epub_meta = package.metadata;
        let additional = metadata::build_additional_metadata(&epub_meta);
        assert_eq!(epub_meta.title, Some("Test Book".to_string()));
        assert_eq!(epub_meta.coverage, Some("Worldwide".to_string()));
        assert_eq!(epub_meta.format, Some("application/epub+zip".to_string()));
        assert_eq!(epub_meta.relation, Some("http://example.com/related".to_string()));
        assert_eq!(epub_meta.source, Some("Original Manuscript".to_string()));
        assert_eq!(epub_meta.dc_type, Some("Text".to_string()));
        assert_eq!(epub_meta.cover_image_href, Some("images/cover.jpg".to_string()));

        assert!(additional.contains_key("coverage"));
        assert!(additional.contains_key("format"));
        assert!(additional.contains_key("relation"));
        assert!(additional.contains_key("source"));
        assert!(additional.contains_key("type"));
        assert!(additional.contains_key("cover_image"));
    }
}
