//! JSON parsing and validation for ExtractionConfig
//!
//! Handles deserialization from JSON strings with comprehensive validation.

use kreuzberg::core::config::ExtractionConfig;

type FfiResult<T> = std::result::Result<T, String>;

/// Parse an ExtractionConfig from a JSON string.
///
/// This is the core parsing logic shared by all FFI functions that deal with
/// JSON configuration. It handles:
/// - JSON deserialization
/// - All validation rules
/// - Type conversions
/// - HTML options parsing (delegated to html module)
///
/// The error messages are user-friendly and include guidance on what went wrong.
pub fn parse_extraction_config_from_json(json_str: &str) -> FfiResult<ExtractionConfig> {
    let json_value: serde_json::Value = serde_json::from_str(json_str).map_err(|e| format!("Invalid JSON: {}", e))?;

    let mut config: ExtractionConfig =
        serde_json::from_value(json_value.clone()).map_err(|e| format!("Invalid configuration structure: {}", e))?;

    // Parse HTML options if present (complex nested structure)
    if let Some(html_opts_val) = json_value.get("html_options") {
        config.html_options = Some(super::html::parse_html_options(html_opts_val)?);
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_config() {
        let json = "{}";
        let result = parse_extraction_config_from_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_config_with_use_cache() {
        let json = r#"{"use_cache": true}"#;
        let result = parse_extraction_config_from_json(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.use_cache);
    }

    #[test]
    fn test_parse_config_with_ocr() {
        let json = r#"{"ocr": {"backend": "tesseract", "language": "eng"}}"#;
        let result = parse_extraction_config_from_json(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.ocr.is_some());
        let ocr = config.ocr.unwrap();
        assert_eq!(ocr.backend, "tesseract");
        assert_eq!(ocr.language, "eng");
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = "{invalid json}";
        let result = parse_extraction_config_from_json(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_complex_config() {
        let json = r#"{
            "use_cache": true,
            "enable_quality_processing": true,
            "force_ocr": false,
            "ocr": {
                "backend": "tesseract",
                "language": "eng"
            },
            "chunking": {
                "max_chars": 1024,
                "max_overlap": 128
            },
            "max_concurrent_extractions": 4
        }"#;
        let result = parse_extraction_config_from_json(json);
        assert!(result.is_ok());
    }
}
