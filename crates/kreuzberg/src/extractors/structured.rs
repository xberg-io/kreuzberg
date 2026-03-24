//! Structured data extractor (JSON, YAML, TOML).

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::{ExtractionResult, Metadata};
use ahash::AHashMap;
use async_trait::async_trait;
use std::borrow::Cow;
#[cfg(feature = "tokio-runtime")]
use std::path::Path;

/// Build a `DocumentStructure` from a structured data result.
///
/// For JSON objects: top-level keys become group headings, nested objects become
/// sub-groups, arrays become lists. Falls back to a code block if the content
/// is not a JSON object (e.g. YAML/TOML where structure is less uniform).
fn build_structured_document_structure(
    result: &crate::extraction::structured::StructuredDataResult,
    mime_type: &str,
) -> crate::types::document_structure::DocumentStructure {
    use crate::types::builder::DocumentStructureBuilder;

    let source_format = match mime_type {
        "application/json" | "text/json" | "application/csl+json" => "json",
        "application/x-ndjson" | "application/jsonl" | "application/x-jsonlines" => "jsonl",
        "application/yaml" | "application/x-yaml" | "text/yaml" | "text/x-yaml" => "yaml",
        "application/toml" | "text/toml" => "toml",
        _ => "structured",
    };

    let language = match source_format {
        "json" | "jsonl" => Some("json"),
        "yaml" => Some("yaml"),
        "toml" => Some("toml"),
        _ => None,
    };

    let mut builder = DocumentStructureBuilder::new().source_format(source_format);

    // Try to build structured document for JSON objects
    if source_format == "json"
        && let Ok(value) = serde_json::from_str::<serde_json::Value>(&result.content)
        && value.is_object()
    {
        build_json_structure(&value, &mut builder, 1);
        return builder.build();
    }

    // Fallback: code block
    builder.push_code(&result.content, language, None);
    builder.build()
}

/// Recursively build document structure from a JSON value.
fn build_json_structure(
    value: &serde_json::Value,
    builder: &mut crate::types::builder::DocumentStructureBuilder,
    depth: u8,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let level = depth.min(6);
                match val {
                    serde_json::Value::Object(_) => {
                        builder.push_heading(level, key, None, None);
                        build_json_structure(val, builder, depth + 1);
                    }
                    serde_json::Value::Array(arr) => {
                        builder.push_heading(level, key, None, None);
                        let list_idx = builder.push_list(false, None);
                        for item in arr {
                            let text = match item {
                                serde_json::Value::String(s) => s.clone(),
                                other => other.to_string(),
                            };
                            builder.push_list_item(list_idx, &text, None);
                        }
                    }
                    serde_json::Value::String(s) => {
                        builder.push_paragraph(&format!("{}: {}", key, s), vec![], None, None);
                    }
                    other => {
                        builder.push_paragraph(&format!("{}: {}", key, other), vec![], None, None);
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            let list_idx = builder.push_list(false, None);
            for item in arr {
                let text = match item {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                builder.push_list_item(list_idx, &text, None);
            }
        }
        serde_json::Value::String(s) => {
            builder.push_paragraph(s, vec![], None, None);
        }
        other => {
            builder.push_paragraph(&other.to_string(), vec![], None, None);
        }
    }
}

/// Structured data extractor supporting JSON, JSONL/NDJSON, YAML, and TOML.
pub struct StructuredExtractor;

impl Default for StructuredExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl StructuredExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for StructuredExtractor {
    fn name(&self) -> &str {
        "structured-extractor"
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

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl DocumentExtractor for StructuredExtractor {
    #[cfg_attr(feature = "otel", tracing::instrument(
        skip(self, content, config),
        fields(
            extractor.name = self.name(),
            content.size_bytes = content.len(),
        )
    ))]
    async fn extract_bytes(
        &self,
        content: &[u8],
        mime_type: &str,
        config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        let structured_result = match mime_type {
            "application/json" | "text/json" | "application/csl+json" => {
                crate::extraction::structured::parse_json(content, None)?
            }
            "application/x-ndjson" | "application/jsonl" | "application/x-jsonlines" => {
                crate::extraction::structured::parse_jsonl(content, None)?
            }
            "application/yaml" | "application/x-yaml" | "text/yaml" | "text/x-yaml" => {
                crate::extraction::structured::parse_yaml(content)?
            }
            "application/toml" | "text/toml" => crate::extraction::structured::parse_toml(content)?,
            _ => return Err(crate::KreuzbergError::UnsupportedFormat(mime_type.to_string())),
        };

        let document = if config.include_document_structure && !structured_result.content.trim().is_empty() {
            Some(build_structured_document_structure(&structured_result, mime_type))
        } else {
            None
        };

        let mut additional = AHashMap::new();
        additional.insert(
            Cow::Borrowed("field_count"),
            serde_json::json!(structured_result.text_fields.len()),
        );
        additional.insert(
            Cow::Borrowed("data_format"),
            serde_json::json!(structured_result.format),
        );

        for (key, value) in structured_result.metadata {
            additional.insert(Cow::Owned(key), serde_json::json!(value));
        }

        Ok(ExtractionResult {
            content: structured_result.content,
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
            elements: None,
            djot_content: None,
            ocr_elements: None,
            document,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            extracted_keywords: None,
            quality_score: None,
            processing_warnings: Vec::new(),
            annotations: None,
            children: None,
        })
    }

    #[cfg(feature = "tokio-runtime")]
    #[cfg_attr(feature = "otel", tracing::instrument(
        skip(self, path, config),
        fields(
            extractor.name = self.name(),
        )
    ))]
    async fn extract_file(&self, path: &Path, mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult> {
        let bytes = tokio::fs::read(path).await?;
        self.extract_bytes(&bytes, mime_type, config).await
    }

    fn supported_mime_types(&self) -> &[&str] {
        &[
            "application/json",
            "text/json",
            "application/csl+json",
            "application/x-ndjson",
            "application/jsonl",
            "application/x-jsonlines",
            "application/yaml",
            "application/x-yaml",
            "text/yaml",
            "text/x-yaml",
            "application/toml",
            "text/toml",
        ]
    }

    fn priority(&self) -> i32 {
        50
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_extractor_plugin_interface() {
        let extractor = StructuredExtractor::new();
        assert_eq!(extractor.name(), "structured-extractor");
        assert!(extractor.initialize().is_ok());
        assert!(extractor.shutdown().is_ok());
    }

    #[test]
    fn test_structured_extractor_supported_mime_types() {
        let extractor = StructuredExtractor::new();
        let mime_types = extractor.supported_mime_types();
        assert_eq!(mime_types.len(), 12);
        assert!(mime_types.contains(&"application/json"));
        assert!(mime_types.contains(&"application/x-ndjson"));
        assert!(mime_types.contains(&"application/jsonl"));
        assert!(mime_types.contains(&"application/x-jsonlines"));
        assert!(mime_types.contains(&"application/x-yaml"));
        assert!(mime_types.contains(&"application/toml"));
        assert!(mime_types.contains(&"application/csl+json"));
    }
}
