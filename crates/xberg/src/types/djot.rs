//! Djot document types.
//!
//! This module defines types for representing Djot document structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import Metadata and Table from parent module
use super::Table;
use super::metadata::Metadata;

/// Comprehensive Djot document structure with semantic preservation.
///
/// This type captures the full richness of Djot markup, including:
/// - Block-level structures (headings, lists, blockquotes, code blocks, etc.)
/// - Inline formatting (emphasis, strong, highlight, subscript, superscript, etc.)
/// - Attributes (classes, IDs, key-value pairs)
/// - Links, images, footnotes
/// - Math expressions (inline and display)
/// - Tables with full structure
///
/// Available when the `djot` feature is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "api", schema(no_recursion))]
pub struct DjotContent {
    /// Plain text representation for backwards compatibility
    pub plain_text: String,

    /// Structured block-level content
    pub blocks: Vec<FormattedBlock>,

    /// Metadata from YAML frontmatter
    pub metadata: Metadata,

    /// Extracted tables as structured data
    pub tables: Vec<Table>,

    /// Extracted images with metadata
    pub images: Vec<DjotImage>,

    /// Extracted links with URLs
    pub links: Vec<DjotLink>,

    /// Footnote definitions
    pub footnotes: Vec<Footnote>,

    /// Attributes mapped by element identifier (if present)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    #[cfg_attr(feature = "api", schema(value_type = serde_json::Value))]
    #[cfg_attr(alef, alef(skip))]
    pub attributes: Vec<(String, Attributes)>,
}

/// Block-level element in a Djot document.
///
/// Represents structural elements like headings, paragraphs, lists, code blocks, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "api", schema(no_recursion))]
pub struct FormattedBlock {
    /// Type of block element
    pub block_type: BlockType,

    /// Heading level (1-6) for headings, or nesting level for lists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<usize>,

    /// Inline content within the block
    pub inline_content: Vec<InlineElement>,

    /// Element attributes (classes, IDs, key-value pairs)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(alef, alef(skip))]
    pub attributes: Option<Attributes>,

    /// Language identifier for code blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Raw code content for code blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Nested blocks for containers (blockquotes, list items, divs)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<FormattedBlock>,
}

/// Types of block-level elements in Djot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum BlockType {
    /// Standard prose paragraph.
    Paragraph,
    /// Section heading (level stored in `FormattedBlock::level`).
    Heading,
    /// Block quotation container.
    Blockquote,
    /// Fenced or indented code block.
    CodeBlock,
    /// Individual item within a list.
    ListItem,
    /// Numbered (ordered) list container.
    OrderedList,
    /// Unnumbered (bullet) list container.
    BulletList,
    /// Task / checkbox list container.
    TaskList,
    /// Definition list container.
    DefinitionList,
    /// Term part of a definition list entry.
    DefinitionTerm,
    /// Description / definition part of a definition list entry.
    DefinitionDescription,
    /// Generic `div` container with optional attributes.
    Div,
    /// Logical section container, often associated with a heading.
    Section,
    /// Horizontal rule / thematic break.
    ThematicBreak,
    /// Raw content block in a specified format (e.g. HTML, LaTeX).
    RawBlock,
    /// Display-mode mathematical expression.
    MathDisplay,
}

/// Inline element within a block.
///
/// Represents text with formatting, links, images, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct InlineElement {
    /// Type of inline element
    pub element_type: InlineType,

    /// Text content
    pub content: String,

    /// Element attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(alef, alef(skip))]
    pub attributes: Option<Attributes>,

    /// Additional metadata (e.g., href for links, src/alt for images)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Types of inline elements in Djot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum InlineType {
    /// Plain text run.
    Text,
    /// Bold / strong emphasis.
    Strong,
    /// Italic / regular emphasis.
    Emphasis,
    /// Highlighted text (marker pen).
    Highlight,
    /// Subscript text.
    Subscript,
    /// Superscript text.
    Superscript,
    /// Inserted text (tracked change).
    Insert,
    /// Deleted text (tracked change).
    Delete,
    /// Inline code span.
    Code,
    /// Hyperlink with URL.
    Link,
    /// Inline image reference.
    Image,
    /// Generic inline span with optional attributes.
    Span,
    /// Inline mathematical expression.
    Math,
    /// Raw inline content in a specified format.
    RawInline,
    /// Footnote reference marker.
    FootnoteRef,
    /// Named symbol or emoji shortcode.
    Symbol,
}

/// Element attributes in Djot.
///
/// Represents the attributes attached to elements using {.class #id key="value"} syntax.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Attributes {
    /// Element ID (#identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// CSS classes (.class1 .class2)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub classes: Vec<String>,

    /// Key-value pairs (key="value")
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    #[cfg_attr(feature = "api", schema(value_type = Vec<[String; 2]>))]
    pub key_values: Vec<(String, String)>,
}

/// Image element in Djot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct DjotImage {
    /// Image source URL or path
    pub src: String,

    /// Alternative text
    pub alt: String,

    /// Optional title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Element attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(alef, alef(skip))]
    pub attributes: Option<Attributes>,
}

/// Link element in Djot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct DjotLink {
    /// Link URL
    pub url: String,

    /// Link text content
    pub text: String,

    /// Optional title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Element attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(alef, alef(skip))]
    pub attributes: Option<Attributes>,
}

/// Footnote in Djot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Footnote {
    /// Footnote label
    pub label: String,

    /// Footnote content blocks
    pub content: Vec<FormattedBlock>,
}
