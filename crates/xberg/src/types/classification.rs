//! Page-classification output types.
//!
//! Produced by the page-classification post-processor
//! (`crates/xberg/src/text/classification/`) and attached to
//! [`ExtractedDocument::page_classifications`](super::extraction::ExtractedDocument::page_classifications).

use serde::{Deserialize, Serialize};

/// Classification result for a single page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct PageClassification {
    /// 1-indexed page number this classification belongs to.
    pub page_number: u32,
    /// Labels assigned to the page. Single-label classification yields exactly one
    /// entry; multi-label classification yields any subset of the configured label set.
    pub labels: Vec<ClassificationLabel>,
}

/// A single label + confidence pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct ClassificationLabel {
    /// Label name as configured in `PageClassificationConfig::labels`.
    pub label: String,
    /// Backend-reported confidence in `[0.0, 1.0]`. `None` when the backend (e.g. an LLM
    /// prompt without explicit confidence schema) did not report one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}
