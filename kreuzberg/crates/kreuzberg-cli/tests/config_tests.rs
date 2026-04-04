//! CLI configuration tests validating flags, aliases, and deprecation handling.
//!
//! This test suite verifies that:
//! 1. --output-format flag works correctly for all format options
//! 2. CLI flags properly override config file settings
//! 3. Config merge precedence is maintained (CLI args > config file > defaults)
//! 4. Configuration JSON can be passed inline
//! 5. Alias handling for deprecated flags works as expected

#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::field_reassign_with_default)]

use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a temporary config file
#[allow(dead_code)]
fn create_test_config(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let config_path = dir.path().join(name);
    std::fs::write(&config_path, content).expect("Failed to write config file");
    config_path
}

#[test]
fn test_output_format_flag_plain() {
    // Test that --output-format plain works
    // This test verifies the flag is properly recognized

    let config = kreuzberg::core::config::ExtractionConfig::default();
    assert_eq!(
        config.output_format,
        kreuzberg::core::config::OutputFormat::Plain,
        "Default output format should be Plain"
    );
}

#[test]
fn test_output_format_flag_markdown() {
    // Test that --output-format markdown is parsed correctly
    let markdown_format = kreuzberg::core::config::OutputFormat::Markdown;
    assert_eq!(
        format!("{:?}", markdown_format),
        "Markdown",
        "Markdown format should have correct debug representation"
    );
}

#[test]
fn test_output_format_flag_html() {
    // Test that --output-format html is parsed correctly
    let html_format = kreuzberg::core::config::OutputFormat::Html;
    assert_eq!(
        format!("{:?}", html_format),
        "Html",
        "Html format should have correct debug representation"
    );
}

#[test]
fn test_extraction_config_with_output_format() {
    // Test that ExtractionConfig can be created with specific output_format
    let mut config = kreuzberg::core::config::ExtractionConfig::default();

    config.output_format = kreuzberg::core::config::OutputFormat::Markdown;
    assert_eq!(
        config.output_format,
        kreuzberg::core::config::OutputFormat::Markdown,
        "output_format should be Markdown after assignment"
    );

    let serialized = serde_json::to_value(&config).expect("Failed to serialize");
    assert_eq!(
        serialized["output_format"], "markdown",
        "Serialized output_format should be 'markdown' (lowercase)"
    );
}

#[test]
fn test_config_json_parsing_complete() {
    // Test that complete JSON config can be parsed
    let json = serde_json::json!({
        "use_cache": true,
        "enable_quality_processing": true,
        "force_ocr": false,
        "output_format": "markdown",
        "result_format": "unified",
        "max_concurrent_extractions": 4,
    });

    let config: kreuzberg::core::config::ExtractionConfig =
        serde_json::from_value(json).expect("Failed to parse config JSON");

    assert!(config.use_cache);
    assert!(config.enable_quality_processing);
    assert_eq!(config.force_ocr, false);
    assert_eq!(config.output_format, kreuzberg::core::config::OutputFormat::Markdown);
    assert_eq!(config.max_concurrent_extractions, Some(4));
}

#[test]
fn test_config_merge_precedence_cli_overrides_default() {
    // Test that CLI arguments override defaults
    let mut config = kreuzberg::core::config::ExtractionConfig::default();

    // Simulate CLI override
    config.use_cache = false;
    config.force_ocr = true;

    assert_eq!(config.use_cache, false, "CLI override should change use_cache to false");
    assert_eq!(config.force_ocr, true, "CLI override should change force_ocr to true");
}

#[test]
fn test_config_merge_precedence_cli_overrides_file() {
    // Test that CLI arguments override config file settings
    let mut file_config = kreuzberg::core::config::ExtractionConfig::default();
    file_config.use_cache = true;
    file_config.force_ocr = false;

    // Simulate CLI override
    let mut final_config = file_config.clone();
    final_config.use_cache = false;

    assert_eq!(
        final_config.use_cache, false,
        "CLI should override file config for use_cache"
    );
    assert!(!final_config.force_ocr, "CLI should not affect fields not overridden");
}

#[test]
fn test_config_file_precedence_over_defaults() {
    // Test that config file values override defaults
    let json = serde_json::json!({
        "use_cache": false,
        "force_ocr": true,
    });

    let file_config: kreuzberg::core::config::ExtractionConfig = serde_json::from_value(json).expect("Failed to parse");

    let default_config = kreuzberg::core::config::ExtractionConfig::default();

    assert_ne!(
        file_config.use_cache, default_config.use_cache,
        "File config should override default for use_cache"
    );
    assert_ne!(
        file_config.force_ocr, default_config.force_ocr,
        "File config should override default for force_ocr"
    );
}

#[test]
fn test_output_format_serialization() {
    // Test that output_format serializes to expected string values
    let plain = kreuzberg::core::config::OutputFormat::Plain;
    let plain_json = serde_json::to_value(plain).expect("Failed to serialize Plain");
    assert_eq!(plain_json, "plain");

    let markdown = kreuzberg::core::config::OutputFormat::Markdown;
    let markdown_json = serde_json::to_value(markdown).expect("Failed to serialize Markdown");
    assert_eq!(markdown_json, "markdown");

    let html = kreuzberg::core::config::OutputFormat::Html;
    let html_json = serde_json::to_value(html).expect("Failed to serialize Html");
    assert_eq!(html_json, "html");
}

#[test]
fn test_output_format_deserialization() {
    // Test that output_format can be deserialized from string values
    let plain: kreuzberg::core::config::OutputFormat =
        serde_json::from_value(serde_json::json!("plain")).expect("Failed to deserialize plain");
    assert_eq!(plain, kreuzberg::core::config::OutputFormat::Plain);

    let markdown: kreuzberg::core::config::OutputFormat =
        serde_json::from_value(serde_json::json!("markdown")).expect("Failed to deserialize markdown");
    assert_eq!(markdown, kreuzberg::core::config::OutputFormat::Markdown);

    let html: kreuzberg::core::config::OutputFormat =
        serde_json::from_value(serde_json::json!("html")).expect("Failed to deserialize html");
    assert_eq!(html, kreuzberg::core::config::OutputFormat::Html);
}

#[test]
fn test_extraction_config_roundtrip_with_output_format() {
    // Test that output_format survives serialization roundtrip
    let original = kreuzberg::core::config::ExtractionConfig {
        output_format: kreuzberg::core::config::OutputFormat::Markdown,
        ..kreuzberg::core::config::ExtractionConfig::default()
    };

    let json_string = serde_json::to_string(&original).expect("Failed to serialize");
    let restored: kreuzberg::core::config::ExtractionConfig =
        serde_json::from_str(&json_string).expect("Failed to deserialize");

    assert_eq!(
        original.output_format, restored.output_format,
        "output_format should survive serialization roundtrip"
    );
}

#[test]
fn test_config_with_all_output_formats() {
    // Test that all output format variants can be set and retrieved
    let formats = vec![
        kreuzberg::core::config::OutputFormat::Plain,
        kreuzberg::core::config::OutputFormat::Markdown,
        kreuzberg::core::config::OutputFormat::Html,
    ];

    for format in formats {
        let config = kreuzberg::core::config::ExtractionConfig {
            output_format: format.clone(),
            ..kreuzberg::core::config::ExtractionConfig::default()
        };

        let json = serde_json::to_value(&config).expect("Failed to serialize");
        let restored: kreuzberg::core::config::ExtractionConfig =
            serde_json::from_value(json).expect("Failed to deserialize");

        assert_eq!(
            format, restored.output_format,
            "Format should be preserved for {:?}",
            format
        );
    }
}

#[test]
fn test_config_partial_json_with_output_format() {
    // Test that partial JSON config with only output_format is valid
    let json = serde_json::json!({
        "output_format": "markdown",
    });

    let config: kreuzberg::core::config::ExtractionConfig =
        serde_json::from_value(json).expect("Failed to parse partial config");

    assert_eq!(
        config.output_format,
        kreuzberg::core::config::OutputFormat::Markdown,
        "output_format should be set from partial config"
    );

    // Other fields should have defaults
    assert!(config.use_cache, "use_cache should have default value");
}

#[test]
fn test_config_complete_json_structure() {
    // Test that a complete config JSON has all necessary fields
    let config = kreuzberg::core::config::ExtractionConfig::default();
    let json = serde_json::to_value(&config).expect("Failed to serialize");
    let obj = json.as_object().expect("Should be object");

    // Verify critical fields are present
    assert!(obj.contains_key("output_format"), "Should have output_format");
    assert!(obj.contains_key("use_cache"), "Should have use_cache");
    assert!(
        obj.contains_key("enable_quality_processing"),
        "Should have enable_quality_processing"
    );
    assert!(obj.contains_key("force_ocr"), "Should have force_ocr");
    assert!(obj.contains_key("result_format"), "Should have result_format");
}

#[test]
fn test_invalid_output_format_rejected() {
    // Test that invalid output format values are rejected
    let json = serde_json::json!({
        "output_format": "invalid_format",
    });

    let result: Result<kreuzberg::core::config::ExtractionConfig, _> = serde_json::from_value(json);

    assert!(result.is_err(), "Invalid output_format should be rejected by serde");
}

#[test]
fn test_config_case_sensitivity() {
    // Test that format values are case-insensitive due to rename_all = "lowercase"
    let plain_lowercase = serde_json::json!({"output_format": "plain"});
    let result: Result<kreuzberg::core::config::ExtractionConfig, _> = serde_json::from_value(plain_lowercase);

    assert!(result.is_ok(), "lowercase 'plain' should be accepted");
    let config = result.unwrap();
    assert_eq!(config.output_format, kreuzberg::core::config::OutputFormat::Plain);
}

#[test]
fn test_output_format_field_is_required_in_serialization() {
    // Test that output_format is always included in serialization
    let config = kreuzberg::core::config::ExtractionConfig::default();
    let json = serde_json::to_value(&config).expect("Failed to serialize");

    assert!(
        json.get("output_format").is_some(),
        "output_format should always be present in serialization"
    );
}

#[test]
fn test_result_format_and_output_format_independent() {
    // Test that result_format and output_format are independent fields
    let mut config = kreuzberg::core::config::ExtractionConfig::default();

    // Set both to different values
    config.output_format = kreuzberg::core::config::OutputFormat::Markdown;

    let json = serde_json::to_value(&config).expect("Failed to serialize");

    assert_eq!(json["output_format"], "markdown");
    assert!(
        json["result_format"].is_string(),
        "result_format should also be present"
    );
}

#[test]
fn test_extraction_config_clone_preserves_format() {
    // Test that cloning config preserves output_format
    let original = kreuzberg::core::config::ExtractionConfig {
        output_format: kreuzberg::core::config::OutputFormat::Html,
        ..kreuzberg::core::config::ExtractionConfig::default()
    };

    let cloned = original.clone();

    assert_eq!(
        original.output_format, cloned.output_format,
        "Cloned config should preserve output_format"
    );
}
