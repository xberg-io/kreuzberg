//! Cross-language serialization integration tests.
//!
//! These tests validate that ExtractionConfig serializes correctly
//! and that the serialized output can be used for cross-language comparison.

use kreuzberg::core::config::ExtractionConfig;

#[test]
fn test_extraction_config_minimal_serialization() {
    let config = ExtractionConfig::default();
    let json = serde_json::to_value(&config).expect("Failed to serialize config");

    // Validate that all expected fields are present
    assert!(json.get("use_cache").is_some(), "Missing use_cache field");
    assert!(
        json.get("enable_quality_processing").is_some(),
        "Missing enable_quality_processing field"
    );
    assert!(json.get("force_ocr").is_some(), "Missing force_ocr field");
}

#[test]
fn test_extraction_config_serialization_round_trip() {
    let original = ExtractionConfig {
        use_cache: true,
        enable_quality_processing: false,
        force_ocr: true,
        ..Default::default()
    };

    // Serialize to JSON
    let json = serde_json::to_value(&original).expect("Failed to serialize");

    // Deserialize back
    let restored: ExtractionConfig = serde_json::from_value(json).expect("Failed to deserialize");

    // Validate that key fields are preserved
    assert_eq!(original.use_cache, restored.use_cache, "use_cache field not preserved");
    assert_eq!(
        original.enable_quality_processing, restored.enable_quality_processing,
        "enable_quality_processing field not preserved"
    );
    assert_eq!(original.force_ocr, restored.force_ocr, "force_ocr field not preserved");
}

#[test]
fn test_extraction_config_nested_serialization() {
    let config = ExtractionConfig {
        use_cache: true,
        enable_quality_processing: true,
        force_ocr: false,
        // Note: Nested fields like ocr, chunking, etc. would be set here
        // This test focuses on the basic serialization structure
        ..Default::default()
    };

    let json = serde_json::to_value(&config).expect("Failed to serialize");

    // Ensure it's a proper JSON object
    assert!(json.is_object(), "Serialized output should be a JSON object");

    // Validate that core fields are present
    assert!(json.get("use_cache").is_some());
    assert!(json.get("enable_quality_processing").is_some());
    assert!(json.get("force_ocr").is_some());
}

#[test]
fn test_extraction_config_json_format() {
    let config = ExtractionConfig::default();
    let json_string = serde_json::to_string(&config).expect("Failed to serialize to string");

    // Validate that output is valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json_string).expect("Invalid JSON output");
    assert!(parsed.is_object(), "JSON should be an object");
}

#[test]
fn test_extraction_config_pretty_print() {
    let config = ExtractionConfig::default();
    let pretty_json = serde_json::to_string_pretty(&config).expect("Failed to serialize");

    // Validate that pretty-printed JSON is parseable
    let _parsed: serde_json::Value = serde_json::from_str(&pretty_json).expect("Invalid pretty-printed JSON");

    // Pretty JSON should have newlines
    assert!(pretty_json.contains('\n'), "Pretty JSON should have newlines");
}

#[test]
fn test_extraction_config_field_consistency() {
    let configs = vec![
        ExtractionConfig::default(),
        ExtractionConfig {
            use_cache: true,
            ..Default::default()
        },
        ExtractionConfig {
            enable_quality_processing: false,
            ..Default::default()
        },
    ];

    for config in configs {
        let json = serde_json::to_value(&config).expect("Failed to serialize");

        // All configs should have the same set of top-level fields
        assert!(json.get("use_cache").is_some());
        assert!(json.get("enable_quality_processing").is_some());
        assert!(json.get("force_ocr").is_some());
    }
}
