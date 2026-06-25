//! Shared types for markdown footnote parsing.

use serde::{Deserialize, Serialize};

/// A footnote anchor reference in markdown text.
///
/// Represents a `[^label]` use-site (not a definition).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct FootnoteAnchor {
    /// The label of the footnote reference (e.g., "1" in `[^1]`).
    pub label: String,

    /// Byte offset of the anchor in the markdown text.
    pub offset: usize,
}

/// A footnote definition from markdown text.
///
/// Represents `[^label]: content` declarations (including multi-line continuations).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct FootnoteDefinition {
    /// The label of the footnote (e.g., "1" in `[^1]: ...`).
    pub label: String,

    /// The full content of the footnote definition.
    pub content: String,

    /// Byte offset of the definition line in the markdown text.
    pub offset: usize,
}

/// A structured citation from a citation block.
///
/// Parsed from entries like:
/// `[^srcN]: source, locator, excerpt: "text"`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Citation {
    /// The label of the citation (e.g., "src1" in `[^src1]: ...`).
    pub label: String,

    /// The source reference (path, URL, or identifier).
    pub source: String,

    /// Optional locator within the source (e.g., "page 3" or "section 2.1").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,

    /// Optional excerpt — quoted text from the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}
