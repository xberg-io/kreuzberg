//! Layout detection configuration.

use serde::{Deserialize, Serialize};

/// Layout detection configuration.
///
/// Controls layout detection behavior in the extraction pipeline.
/// When set on [`ExtractionConfig`](super::ExtractionConfig), layout detection
/// is enabled for PDF extraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDetectionConfig {
    /// Preset for model selection. Currently only `"accurate"` (RT-DETR) is supported.
    #[serde(default = "default_preset")]
    pub preset: String,

    /// Confidence threshold override (None = use model default).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f32>,

    /// Whether to apply postprocessing heuristics (default: true).
    #[serde(default = "default_true")]
    pub apply_heuristics: bool,

    /// Table structure recognition model.
    ///
    /// Controls which model is used for table cell detection within layout-detected
    /// table regions. Options:
    /// - `"tatr"` (default): TATR (Table Transformer), 30MB, DETR-based row/column detection
    /// - `"slanet_wired"`: SLANeXT wired variant, 365MB, optimized for bordered tables
    /// - `"slanet_wireless"`: SLANeXT wireless variant, 365MB, optimized for borderless tables
    /// - `"slanet_plus"`: SLANet-plus lightweight, 7.78MB, general-purpose
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_model: Option<String>,
}

impl Default for LayoutDetectionConfig {
    fn default() -> Self {
        Self {
            preset: default_preset(),
            confidence_threshold: None,
            apply_heuristics: true,
            table_model: None,
        }
    }
}

fn default_preset() -> String {
    "accurate".to_string()
}

fn default_true() -> bool {
    true
}
