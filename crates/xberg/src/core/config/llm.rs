//! LLM configuration types for liter-llm integration.
//!
//! These types are always available (not feature-gated) since they are
//! pure configuration data with no runtime dependency on liter-llm.

use serde::{Deserialize, Serialize};

/// Configuration for an LLM provider/model via liter-llm.
///
/// Each feature (VLM OCR, VLM embeddings, structured extraction) carries
/// its own `LlmConfig`, allowing different providers per feature.
///
/// # Example
///
/// ```toml
/// [structured_extraction.llm]
/// model = "openai/gpt-4o"
/// api_key = "sk-..."  # or use XBERG_LLM_API_KEY env var
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct LlmConfig {
    /// Provider/model string using liter-llm routing format.
    ///
    /// Examples: `"openai/gpt-4o"`, `"anthropic/claude-sonnet-4-20250514"`,
    /// `"groq/llama-3.1-70b-versatile"`.
    pub model: String,

    /// API key for the provider. When `None`, liter-llm falls back to
    /// the provider's standard environment variable (e.g., `OPENAI_API_KEY`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Custom base URL override for the provider endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Request timeout in seconds (default: 60).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,

    /// Maximum retry attempts (default: 3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,

    /// Sampling temperature for generation tasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Maximum tokens to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
}

/// Configuration for LLM-based structured data extraction.
///
/// Sends extracted document content to a VLM with a JSON schema,
/// returning structured data that conforms to the schema.
///
/// # Example
///
/// ```toml
/// [structured_extraction]
/// schema_name = "invoice_data"
/// strict = true
///
/// [structured_extraction.schema]
/// type = "object"
/// properties.vendor = { type = "string" }
/// properties.total = { type = "number" }
/// required = ["vendor", "total"]
///
/// [structured_extraction.llm]
/// model = "openai/gpt-4o"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredExtractionConfig {
    /// JSON Schema defining the desired output structure.
    pub schema: serde_json::Value,

    /// Schema name passed to the LLM's structured output mode.
    #[serde(default = "default_schema_name")]
    pub schema_name: String,

    /// Optional schema description for the LLM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_description: Option<String>,

    /// Enable strict mode — output must exactly match the schema.
    #[serde(default)]
    pub strict: bool,

    /// Custom Jinja2 extraction prompt template. When `None`, a default template is used.
    ///
    /// Available template variables:
    /// - `{{ content }}` — The extracted document text.
    /// - `{{ schema }}` — The JSON schema as a formatted string.
    /// - `{{ schema_name }}` — The schema name.
    /// - `{{ schema_description }}` — The schema description (may be empty).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// LLM configuration for the extraction.
    pub llm: LlmConfig,
}

fn default_schema_name() -> String {
    "extraction".to_string()
}

/// How a structured-extraction preset is dispatched to the model.
///
/// This is the preset-facing call mode (the `preferred_call_mode` field of a
/// [`crate::presets::Preset`]). The structured pipeline has a richer
/// runtime-only decision enum with skip and fallback states; this 3-variant
/// type is the stable, serializable surface presets and bindings depend on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CallMode {
    /// Use the extracted text only.
    #[default]
    TextOnly,
    /// Use rasterized page images only.
    VisionOnly,
    /// Provide both extracted text and page images to the model.
    TextPlusVision,
}

/// How partial results from multiple model calls (e.g. per page batch) are combined.
///
/// Canonical home for the merge strategy referenced by presets and by the
/// structured pipeline's post-processing. There is intentionally only one merge
/// type across the crate — do not introduce a second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum MergeMode {
    /// Deep-merge JSON objects field by field (later calls fill missing fields).
    #[default]
    ObjectMerge,
    /// Concatenate top-level arrays across calls.
    ArrayConcat,
    /// Keep the first non-empty result; ignore subsequent calls.
    ObjectFirst,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test for https://github.com/xberg-io/xberg/issues/716
    ///
    /// `LlmConfig` must implement `Default` so callers can use the struct-update
    /// syntax documented in the VLM OCR guide:
    ///
    /// ```rust
    /// use xberg::core::config::LlmConfig;
    /// let cfg = LlmConfig {
    ///     model: "openai/gpt-4o-mini".to_string(),
    ///     ..Default::default()
    /// };
    /// ```
    #[test]
    fn test_llm_config_default_trait_is_satisfied() {
        let cfg = LlmConfig::default();
        assert!(cfg.model.is_empty(), "default model should be empty string");
        assert!(cfg.api_key.is_none());
        assert!(cfg.base_url.is_none());
        assert!(cfg.timeout_secs.is_none());
        assert!(cfg.max_retries.is_none());
        assert!(cfg.temperature.is_none());
        assert!(cfg.max_tokens.is_none());
    }

    /// Verify the struct-update pattern from the issue compiles and produces
    /// only the explicitly set field.
    #[test]
    fn test_llm_config_struct_update_syntax() {
        let cfg = LlmConfig {
            model: "openai/gpt-4o-mini".to_string(),
            ..Default::default()
        };
        assert_eq!(cfg.model, "openai/gpt-4o-mini");
        assert!(cfg.api_key.is_none());
        assert!(cfg.base_url.is_none());
        assert!(cfg.timeout_secs.is_none());
        assert!(cfg.max_retries.is_none());
        assert!(cfg.temperature.is_none());
        assert!(cfg.max_tokens.is_none());
    }

    #[test]
    fn test_call_mode_serde_round_trip() {
        for (mode, wire) in [
            (CallMode::TextOnly, "\"text_only\""),
            (CallMode::VisionOnly, "\"vision_only\""),
            (CallMode::TextPlusVision, "\"text_plus_vision\""),
        ] {
            let json = serde_json::to_string(&mode).expect("serialize");
            assert_eq!(json, wire);
            let decoded: CallMode = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(decoded, mode);
        }
        assert_eq!(CallMode::default(), CallMode::TextOnly);
    }

    #[test]
    fn test_merge_mode_serde_round_trip() {
        for (mode, wire) in [
            (MergeMode::ObjectMerge, "\"object_merge\""),
            (MergeMode::ArrayConcat, "\"array_concat\""),
            (MergeMode::ObjectFirst, "\"object_first\""),
        ] {
            let json = serde_json::to_string(&mode).expect("serialize");
            assert_eq!(json, wire);
            let decoded: MergeMode = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(decoded, mode);
        }
        assert_eq!(MergeMode::default(), MergeMode::ObjectMerge);
    }
}
