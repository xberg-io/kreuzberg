//! Filter IR and complexity validation for vector-store queries.
//!
//! A small, backend-neutral predicate language over document- and chunk-level
//! fields, with hard complexity caps so a malicious or accidental filter cannot
//! blow up a backend. Ported (de-tenanted) from the enterprise `vectorstore`
//! crate; Postgres-specific `*_tsv` columns are not part of the neutral surface.

use crate::error::{ComplexityKind, RagError, RagResult};
use serde::{Deserialize, Serialize};

/// Maximum nesting depth for filters.
pub const MAX_FILTER_DEPTH: u32 = 8;
/// Maximum total filter nodes (operators + predicates).
pub const MAX_FILTER_NODES: u32 = 64;
/// Maximum number of `text_match` predicates in a filter.
pub const MAX_TEXT_MATCH_PREDICATES: u32 = 4;
/// Maximum byte length of a `text_match` query string.
pub const MAX_TEXT_MATCH_QUERY_BYTES: u32 = 1024;

/// Namespace for a filter field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterNamespace {
    /// Document-level field (`doc.*`).
    Doc,
    /// Chunk-level field (`chunk.*`).
    Chunk,
}

/// A parsed filter field reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedField {
    /// Namespace (doc or chunk).
    pub namespace: FilterNamespace,
    /// Field path within the namespace.
    pub path: String,
}

/// A filter field identifier (`doc.title`, `chunk.content`, `doc.metadata.x`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterField(pub String);

impl FilterField {
    /// Parse and validate a field identifier into namespace + path.
    ///
    /// # Errors
    ///
    /// [`RagError::FilterUnknownField`] if the prefix is invalid or the field is
    /// not whitelisted.
    pub fn parse(&self) -> RagResult<ParsedField> {
        if let Some(path) = self.0.strip_prefix("doc.") {
            Self::validate_doc_field(path)?;
            Ok(ParsedField {
                namespace: FilterNamespace::Doc,
                path: path.to_string(),
            })
        } else if let Some(path) = self.0.strip_prefix("chunk.") {
            Self::validate_chunk_field(path)?;
            Ok(ParsedField {
                namespace: FilterNamespace::Chunk,
                path: path.to_string(),
            })
        } else {
            Err(RagError::FilterUnknownField { field: self.0.clone() })
        }
    }

    fn validate_doc_field(path: &str) -> RagResult<()> {
        const DOC_FIELDS: &[&str] = &[
            "full_text",
            "keywords",
            "labels",
            "entities",
            "title",
            "mime",
            "external_id",
            "source_uri",
            "ingested_at",
        ];
        if DOC_FIELDS.contains(&path) || path.starts_with("metadata.") {
            return Ok(());
        }
        Err(RagError::FilterUnknownField {
            field: format!("doc.{path}"),
        })
    }

    fn validate_chunk_field(path: &str) -> RagResult<()> {
        const CHUNK_FIELDS: &[&str] = &["ordinal", "external_id", "content", "chunk_metadata"];
        if CHUNK_FIELDS.contains(&path) || path.starts_with("chunk_metadata.") {
            return Ok(());
        }
        Err(RagError::FilterUnknownField {
            field: format!("chunk.{path}"),
        })
    }
}

/// A filter expression for constraining retrieval and deletion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// `Range` carries four optional jsonb bounds; the size is intrinsic to the wire
// shape and boxing it would force manual serde impls. Filters live in request
// scope so the carrying cost is negligible.
#[allow(clippy::large_enum_variant)]
pub enum Filter {
    /// Exact equality.
    Eq {
        /// Field to match.
        field: FilterField,
        /// Value to match.
        value: serde_json::Value,
    },
    /// Set membership.
    In {
        /// Field to match.
        field: FilterField,
        /// Allowed values.
        values: Vec<serde_json::Value>,
    },
    /// Range constraint.
    Range {
        /// Field to match.
        field: FilterField,
        /// Greater-than-or-equal bound.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        gte: Option<serde_json::Value>,
        /// Greater-than bound.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        gt: Option<serde_json::Value>,
        /// Less-than-or-equal bound.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        lte: Option<serde_json::Value>,
        /// Less-than bound.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        lt: Option<serde_json::Value>,
    },
    /// Array membership (the field's array contains the value).
    ArrayContains {
        /// Field to match.
        field: FilterField,
        /// Value to find within the array.
        value: serde_json::Value,
    },
    /// Full-text match.
    TextMatch {
        /// Field to search.
        field: FilterField,
        /// Query string.
        query: String,
    },
    /// Logical AND.
    And {
        /// Conjoined filters.
        filters: Vec<Filter>,
    },
    /// Logical OR.
    Or {
        /// Disjoined filters.
        filters: Vec<Filter>,
    },
    /// Logical NOT.
    Not {
        /// Negated filter.
        filter: Box<Filter>,
    },
}

impl Filter {
    /// Validate field references and complexity caps in one pass.
    ///
    /// # Errors
    ///
    /// [`RagError::FilterUnknownField`] for bad fields, or
    /// [`RagError::FilterComplexityExceeded`] when a cap is exceeded.
    pub fn validate(&self) -> RagResult<()> {
        let mut ctx = ComplexityContext::default();
        Self::check_recursive(self, &mut ctx, 0)
    }

    fn check_recursive(filter: &Filter, ctx: &mut ComplexityContext, depth: u32) -> RagResult<()> {
        if depth > MAX_FILTER_DEPTH {
            return Err(RagError::FilterComplexityExceeded {
                kind: ComplexityKind::Depth,
                cap: MAX_FILTER_DEPTH,
                observed: depth,
            });
        }

        ctx.node_count += 1;
        if ctx.node_count > MAX_FILTER_NODES {
            return Err(RagError::FilterComplexityExceeded {
                kind: ComplexityKind::NodeCount,
                cap: MAX_FILTER_NODES,
                observed: ctx.node_count,
            });
        }

        match filter {
            Filter::Eq { field, .. } | Filter::ArrayContains { field, .. } => {
                field.parse()?;
                Ok(())
            }
            Filter::Range { field, .. } => {
                field.parse()?;
                Ok(())
            }
            Filter::In { field, values } => {
                field.parse()?;
                ctx.node_count += values.len() as u32;
                if ctx.node_count > MAX_FILTER_NODES {
                    return Err(RagError::FilterComplexityExceeded {
                        kind: ComplexityKind::NodeCount,
                        cap: MAX_FILTER_NODES,
                        observed: ctx.node_count,
                    });
                }
                Ok(())
            }
            Filter::TextMatch { field, query } => {
                field.parse()?;
                ctx.text_match_count += 1;
                if ctx.text_match_count > MAX_TEXT_MATCH_PREDICATES {
                    return Err(RagError::FilterComplexityExceeded {
                        kind: ComplexityKind::TextMatchCount,
                        cap: MAX_TEXT_MATCH_PREDICATES,
                        observed: ctx.text_match_count,
                    });
                }
                let bytes = query.len() as u32;
                if bytes > MAX_TEXT_MATCH_QUERY_BYTES {
                    return Err(RagError::FilterComplexityExceeded {
                        kind: ComplexityKind::TextMatchQueryBytes,
                        cap: MAX_TEXT_MATCH_QUERY_BYTES,
                        observed: bytes,
                    });
                }
                Ok(())
            }
            Filter::And { filters } | Filter::Or { filters } => {
                for f in filters {
                    Self::check_recursive(f, ctx, depth + 1)?;
                }
                Ok(())
            }
            Filter::Not { filter } => Self::check_recursive(filter, ctx, depth + 1),
        }
    }
}

#[derive(Default)]
struct ComplexityContext {
    node_count: u32,
    text_match_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_whitelisted_doc_field() {
        let parsed = FilterField("doc.full_text".to_string()).parse().unwrap();
        assert_eq!(parsed.namespace, FilterNamespace::Doc);
        assert_eq!(parsed.path, "full_text");
    }

    #[test]
    fn parses_nested_metadata_path() {
        let parsed = FilterField("doc.metadata.foo.bar".to_string()).parse().unwrap();
        assert_eq!(parsed.path, "metadata.foo.bar");
    }

    #[test]
    fn rejects_unknown_chunk_field() {
        assert!(FilterField("chunk.arbitrary".to_string()).parse().is_err());
    }

    #[test]
    fn rejects_sql_injection_shaped_field() {
        assert!(
            FilterField("chunk.id; DROP TABLE chunks--".to_string())
                .parse()
                .is_err()
        );
    }

    #[test]
    fn rejects_invalid_prefix() {
        assert!(FilterField("unknown.field".to_string()).parse().is_err());
    }

    #[test]
    fn validate_flat_filter_ok() {
        let filter = Filter::And {
            filters: vec![
                Filter::Eq {
                    field: FilterField("doc.title".to_string()),
                    value: serde_json::json!("x"),
                },
                Filter::TextMatch {
                    field: FilterField("chunk.content".to_string()),
                    query: "search".to_string(),
                },
            ],
        };
        assert!(filter.validate().is_ok());
    }

    #[test]
    fn validate_rejects_unknown_field() {
        let filter = Filter::Eq {
            field: FilterField("doc.not_a_field".to_string()),
            value: serde_json::json!(1),
        };
        assert!(matches!(filter.validate(), Err(RagError::FilterUnknownField { .. })));
    }

    #[test]
    fn validate_rejects_excess_nodes() {
        let filters = (0..65)
            .map(|i| Filter::Eq {
                field: FilterField("doc.title".to_string()),
                value: serde_json::json!(i),
            })
            .collect();
        assert!(matches!(
            Filter::And { filters }.validate(),
            Err(RagError::FilterComplexityExceeded {
                kind: ComplexityKind::NodeCount,
                ..
            })
        ));
    }

    #[test]
    fn validate_rejects_oversize_text_match() {
        let filter = Filter::TextMatch {
            field: FilterField("chunk.content".to_string()),
            query: "x".repeat(1025),
        };
        assert!(matches!(
            filter.validate(),
            Err(RagError::FilterComplexityExceeded {
                kind: ComplexityKind::TextMatchQueryBytes,
                ..
            })
        ));
    }

    #[test]
    fn filter_round_trips_through_json() {
        let original = Filter::And {
            filters: vec![
                Filter::ArrayContains {
                    field: FilterField("doc.labels".to_string()),
                    value: serde_json::json!("10-K"),
                },
                Filter::Not {
                    filter: Box::new(Filter::TextMatch {
                        field: FilterField("doc.full_text".to_string()),
                        query: "confidential".to_string(),
                    }),
                },
            ],
        };
        let json = serde_json::to_value(&original).unwrap();
        assert!(json.get("and").is_some());
        let back: Filter = serde_json::from_value(json).unwrap();
        assert_eq!(original, back);
    }
}
