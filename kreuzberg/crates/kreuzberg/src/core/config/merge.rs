//! JSON-level configuration merging.
//!
//! Provides a unified merge function for combining a base `ExtractionConfig` with
//! JSON overrides. Used by both the CLI (`--config-json`) and MCP server to apply
//! partial configuration overrides without losing unspecified fields.

use super::ExtractionConfig;

/// Merge extraction configuration using JSON-level field override.
///
/// Serializes the base config to JSON, merges each field from the override JSON
/// (top-level only), and deserializes back. This correctly handles boolean fields
/// explicitly set to their default values — the override always wins for any field
/// present in `override_json`.
///
/// Fields **not** present in `override_json` are preserved from `base`.
///
/// # Errors
///
/// Returns `Err` if the base config cannot be serialized, or if the merged JSON
/// cannot be deserialized back into `ExtractionConfig` (e.g., wrong field types).
///
/// # Examples
///
/// ```rust,ignore
/// use kreuzberg::ExtractionConfig;
/// use serde_json::json;
///
/// let mut base = ExtractionConfig::default();
/// base.use_cache = true;
///
/// let overrides = json!({"force_ocr": true});
/// let merged = kreuzberg::core::config::merge::merge_config_json(&base, overrides).unwrap();
/// assert!(merged.use_cache);   // preserved from base
/// assert!(merged.force_ocr);   // applied from override
/// ```
pub fn merge_config_json(
    base: &ExtractionConfig,
    override_json: serde_json::Value,
) -> Result<ExtractionConfig, String> {
    let mut config_json =
        serde_json::to_value(base).map_err(|e| format!("Failed to serialize base config to JSON: {e}"))?;

    if let serde_json::Value::Object(json_obj) = override_json
        && let Some(config_obj) = config_json.as_object_mut()
    {
        for (key, value) in json_obj {
            config_obj.insert(key, value);
        }
    }

    serde_json::from_value(config_json).map_err(|e| format!("Failed to deserialize merged config: {e}"))
}

/// Build extraction config by optionally merging JSON overrides into a base config.
///
/// If `override_json` is `None`, returns a clone of `base`. Otherwise delegates
/// to [`merge_config_json`].
pub fn build_config_from_json(
    base: &ExtractionConfig,
    override_json: Option<serde_json::Value>,
) -> Result<ExtractionConfig, String> {
    match override_json {
        Some(json) => merge_config_json(base, json),
        None => Ok(base.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_preserves_unspecified_fields() {
        let base = ExtractionConfig {
            use_cache: false,
            enable_quality_processing: true,
            force_ocr: false,
            ..Default::default()
        };

        let merged = merge_config_json(&base, json!({"force_ocr": true})).unwrap();

        assert!(!merged.use_cache, "use_cache should be preserved from base");
        assert!(
            merged.enable_quality_processing,
            "enable_quality_processing should be preserved"
        );
        assert!(merged.force_ocr, "force_ocr should be overridden");
    }

    #[test]
    fn test_merge_override_to_default_value() {
        let base = ExtractionConfig {
            use_cache: false,
            ..Default::default()
        };

        let merged = merge_config_json(&base, json!({"use_cache": true})).unwrap();
        assert!(
            merged.use_cache,
            "Should use explicit override even if it matches the struct default"
        );
    }

    #[test]
    fn test_merge_multiple_fields() {
        let base = ExtractionConfig {
            use_cache: true,
            force_ocr: true,
            ..Default::default()
        };

        let merged = merge_config_json(&base, json!({"use_cache": false, "output_format": "markdown"})).unwrap();

        assert!(!merged.use_cache);
        assert!(merged.force_ocr, "force_ocr should be preserved");
        assert_eq!(
            merged.output_format,
            crate::core::config::formats::OutputFormat::Markdown,
        );
    }

    #[test]
    fn test_merge_invalid_field_type_returns_error() {
        let base = ExtractionConfig::default();
        let result = merge_config_json(&base, json!({"use_cache": "not_a_boolean"}));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to deserialize"));
    }

    #[test]
    fn test_build_config_from_json_none_returns_clone() {
        let base = ExtractionConfig {
            use_cache: false,
            ..Default::default()
        };
        let result = build_config_from_json(&base, None).unwrap();
        assert!(!result.use_cache);
    }

    #[test]
    fn test_build_config_from_json_some_merges() {
        let base = ExtractionConfig::default();
        let result = build_config_from_json(&base, Some(json!({"force_ocr": true}))).unwrap();
        assert!(result.force_ocr);
    }
}
