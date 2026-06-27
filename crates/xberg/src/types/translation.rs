//! Translation output types.
//!
//! Produced by the translation post-processor
//! (`crates/xberg/src/text/translation/`) and attached to
//! [`ExtractedDocument::translation`](super::extraction::ExtractedDocument::translation).

use serde::{Deserialize, Serialize};

/// Translation of the extracted content.
///
/// Holds the translated rendition of `ExtractedDocument::content` and (when
/// `preserve_markup` was requested) the translated `formatted_content`. Chunks
/// are translated in place inside `ExtractedDocument::chunks[*].content` rather
/// than duplicated here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Translation {
    /// BCP-47 language tag the translation was produced into (e.g. `"de"`, `"fr-CA"`).
    pub target_lang: String,
    /// BCP-47 source language. `None` when the translation backend was asked to detect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<String>,
    /// Translated plain-text body. Matches the shape of `ExtractedDocument::content`.
    pub content: String,
    /// Translated markup body (Markdown / HTML / etc.) when `preserve_markup` was
    /// enabled on the config. `None` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_content: Option<String>,
}
