//! Document summarisation output types.
//!
//! Produced by the summarisation post-processor
//! (`crates/xberg/src/text/summarization/`) and attached to
//! [`ExtractedDocument::summary`](super::extraction::ExtractedDocument::summary).

use serde::{Deserialize, Serialize};

/// Summary of an extracted document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct DocumentSummary {
    /// Summary text (plain prose).
    pub text: String,
    /// Strategy that produced this summary.
    pub strategy: SummaryStrategy,
    /// Approximate token count of the summary, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_count: Option<u32>,
}

/// Summarisation strategy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum SummaryStrategy {
    /// Pure-Rust extractive summary (TextRank over the chunk graph). Deterministic,
    /// fast, no external service required.
    #[default]
    Extractive,
    /// Abstractive summary produced by liter-llm. Requires `liter-llm` feature and
    /// a configured `LlmConfig`. Token usage is captured in
    /// [`ExtractedDocument::llm_usage`](super::extraction::ExtractedDocument::llm_usage).
    Abstractive,
}

impl std::fmt::Display for SummaryStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Extractive => f.write_str("extractive"),
            Self::Abstractive => f.write_str("abstractive"),
        }
    }
}
