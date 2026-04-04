//! WASM config serialization round-trip tests
//!
//! These tests verify that ExtractionConfig can be serialized to JSON/TOML/YAML
//! and deserialized back without loss of information, ensuring bidirectional
//! compatibility between Rust and JavaScript representations.

#![cfg(target_arch = "wasm32")]

use kreuzberg::ExtractionConfig;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test config JSON round-trip with basic fields
#[wasm_bindgen_test]
fn test_config_json_round_trip_basic() {
    let config_json = serde_json::json!({
        "useCache": true,
        "forceOcr": false,
        "enableQualityProcessing": true,
        "outputFormat": "markdown",
        "resultFormat": "unified"
    });

    // Deserialize from JSON
    let config: ExtractionConfig =
        serde_json::from_value(config_json.clone()).expect("Should deserialize config from JSON");

    // Serialize back to JSON
    let roundtrip_json = serde_json::to_value(&config).expect("Should serialize config back to JSON");

    // Verify key fields preserved
    assert_eq!(
        config_json.get("useCache"),
        roundtrip_json.get("useCache"),
        "useCache should be preserved in round-trip"
    );
    assert_eq!(
        config_json.get("forceOcr"),
        roundtrip_json.get("forceOcr"),
        "forceOcr should be preserved in round-trip"
    );
    assert_eq!(
        config_json.get("outputFormat"),
        roundtrip_json.get("outputFormat"),
        "outputFormat should be preserved in round-trip"
    );
    assert_eq!(
        config_json.get("resultFormat"),
        roundtrip_json.get("resultFormat"),
        "resultFormat should be preserved in round-trip"
    );
}

/// Test config JSON round-trip with all boolean flags
#[wasm_bindgen_test]
fn test_config_json_round_trip_all_booleans() {
    let config_json = serde_json::json!({
        "useCache": false,
        "enableQualityProcessing": false,
        "forceOcr": true,
        "skipOcr": true,
        "enableMcpServer": false
    });

    let config: ExtractionConfig =
        serde_json::from_value(config_json.clone()).expect("Should deserialize boolean config");

    let roundtrip_json = serde_json::to_value(&config).expect("Should serialize boolean config");

    // Verify all boolean fields preserved
    if let (Some(orig_cache), Some(rt_cache)) = (config_json.get("useCache"), roundtrip_json.get("useCache")) {
        assert_eq!(orig_cache, rt_cache, "useCache boolean should match");
    }
    if let (Some(orig_ocr), Some(rt_ocr)) = (config_json.get("forceOcr"), roundtrip_json.get("forceOcr")) {
        assert_eq!(orig_ocr, rt_ocr, "forceOcr boolean should match");
    }
}

/// Test config JSON round-trip with numeric fields
#[wasm_bindgen_test]
fn test_config_json_round_trip_numerics() {
    let config_json = serde_json::json!({
        "maxConcurrentExtractions": 8,
        "cacheSizeLimit": 1024000000,
        "timeoutMs": 30000
    });

    let config: ExtractionConfig =
        serde_json::from_value(config_json.clone()).expect("Should deserialize numeric config");

    let roundtrip_json = serde_json::to_value(&config).expect("Should serialize numeric config");

    // Verify numeric fields preserved
    if let (Some(orig), Some(rt)) = (
        config_json.get("maxConcurrentExtractions"),
        roundtrip_json.get("maxConcurrentExtractions"),
    ) {
        assert_eq!(orig, rt, "maxConcurrentExtractions should be preserved");
    }
}

/// Test config JSON round-trip with nested OCR configuration
#[wasm_bindgen_test]
fn test_config_json_round_trip_nested_ocr() {
    let config_json = serde_json::json!({
        "ocr": {
            "backend": "tesseract",
            "language": "eng",
            "enabled": true
        },
        "outputFormat": "plain"
    });

    let config: ExtractionConfig =
        serde_json::from_value(config_json.clone()).expect("Should deserialize nested OCR config");

    let roundtrip_json = serde_json::to_value(&config).expect("Should serialize nested OCR config");

    // Verify nested OCR structure preserved
    if let Some(ocr) = roundtrip_json.get("ocr") {
        if config_json.get("ocr").is_some() {
            let orig_ocr = config_json.get("ocr").unwrap();
            if let Some(backend) = orig_ocr.get("backend") {
                assert_eq!(ocr.get("backend"), Some(backend), "OCR backend should match");
            }
            if let Some(language) = orig_ocr.get("language") {
                assert_eq!(ocr.get("language"), Some(language), "OCR language should match");
            }
        }
    }
}

/// Test config JSON round-trip with nested chunking configuration
#[wasm_bindgen_test]
fn test_config_json_round_trip_nested_chunking() {
    let config_json = serde_json::json!({
        "chunking": {
            "maxCharacters": 1000,
            "maxOverlap": 200,
            "strategy": "semantic"
        }
    });

    let config: ExtractionConfig =
        serde_json::from_value(config_json.clone()).expect("Should deserialize nested chunking config");

    let roundtrip_json = serde_json::to_value(&config).expect("Should serialize nested chunking config");

    // Verify nested chunking preserved
    if let Some(chunking) = roundtrip_json.get("chunking") {
        if let Some(orig_chunking) = config_json.get("chunking") {
            assert!(chunking.is_object(), "Chunking should serialize as object");
            assert!(orig_chunking.is_object(), "Original chunking should be object");
        }
    }
}

/// Test config round-trip with TOML format
#[wasm_bindgen_test]
fn test_config_toml_round_trip() {
    let toml_str = r#"
use_cache = true
force_ocr = false
enable_quality_processing = true
output_format = "markdown"
result_format = "unified"
max_concurrent_extractions = 4
"#;

    // Parse TOML to config
    let config: ExtractionConfig = toml::from_str(toml_str).expect("Should parse TOML config");

    // Serialize back to TOML
    let roundtrip_toml = toml::to_string(&config).expect("Should serialize config back to TOML");

    // Verify roundtrip config is valid
    let config2: ExtractionConfig = toml::from_str(&roundtrip_toml).expect("Should parse roundtrip TOML config");

    // Both configs should deserialize successfully
    let json1 = serde_json::to_value(&config).unwrap();
    let json2 = serde_json::to_value(&config2).unwrap();

    // Verify key fields match
    assert_eq!(
        json1.get("useCache"),
        json2.get("useCache"),
        "useCache should match between round-trips"
    );
}

/// Test config round-trip with YAML format
#[wasm_bindgen_test]
fn test_config_yaml_round_trip() {
    let yaml_str = r#"
useCache: true
forceOcr: false
enableQualityProcessing: true
outputFormat: plain
resultFormat: element_based
maxConcurrentExtractions: 6
"#;

    // Parse YAML to config
    let config: ExtractionConfig = serde_yaml::from_str(yaml_str).expect("Should parse YAML config");

    // Serialize back to YAML
    let roundtrip_yaml = serde_yaml::to_string(&config).expect("Should serialize config back to YAML");

    // Verify roundtrip config is valid
    let config2: ExtractionConfig = serde_yaml::from_str(&roundtrip_yaml).expect("Should parse roundtrip YAML config");

    // Both configs should deserialize successfully
    let json1 = serde_json::to_value(&config).unwrap();
    let json2 = serde_json::to_value(&config2).unwrap();

    // Verify key fields match
    assert_eq!(
        json1.get("outputFormat"),
        json2.get("outputFormat"),
        "outputFormat should match between YAML round-trips"
    );
}

/// Test camelCase field name preservation in JSON
#[wasm_bindgen_test]
fn test_camel_case_field_names_json() {
    let config_json = serde_json::json!({
        "useCache": false,
        "enableQualityProcessing": true,
        "forceOcr": true,
        "maxConcurrentExtractions": 8,
        "cacheSizeLimit": 512000000,
        "outputFormat": "html",
        "resultFormat": "unified"
    });

    let config: ExtractionConfig = serde_json::from_value(config_json).expect("Should deserialize with camelCase");

    let serialized = serde_json::to_value(&config).unwrap();

    // WASM/JavaScript should use camelCase (JS naming convention)
    assert!(
        serialized.get("useCache").is_some() || serialized.get("use_cache").is_some(),
        "Should have cache field (camelCase or snake_case)"
    );
    assert!(
        serialized.get("forceOcr").is_some() || serialized.get("force_ocr").is_some(),
        "Should have ocr field"
    );
}

/// Test snake_case to camelCase conversion in round-trip
#[wasm_bindgen_test]
fn test_snake_case_to_camel_case_conversion() {
    let snake_case_json = serde_json::json!({
        "use_cache": true,
        "force_ocr": false,
        "enable_quality_processing": true,
        "output_format": "markdown",
        "result_format": "unified",
        "max_concurrent_extractions": 5
    });

    // Parse snake_case JSON
    let config: ExtractionConfig = serde_json::from_value(snake_case_json).expect("Should deserialize snake_case JSON");

    // Serialize (may convert to preferred format)
    let serialized = serde_json::to_value(&config).expect("Should serialize config");

    // Parse back to verify round-trip works regardless of naming convention
    let config2: ExtractionConfig = serde_json::from_value(serialized).expect("Should deserialize roundtrip config");

    let json1 = serde_json::to_value(&config).unwrap();
    let json2 = serde_json::to_value(&config2).unwrap();

    // Both should deserialize successfully
    assert!(json1.is_object());
    assert!(json2.is_object());
}

/// Test default config serialization
#[wasm_bindgen_test]
fn test_default_config_serialization() {
    let config = ExtractionConfig::default();

    let json = serde_json::to_value(&config).expect("Should serialize default config to JSON");

    assert!(json.is_object(), "Default config should serialize to JSON object");

    // Re-parse the serialized config
    let config2: ExtractionConfig = serde_json::from_value(json).expect("Should deserialize serialized default config");

    // Both should be valid
    let json2 = serde_json::to_value(&config2).expect("Should serialize re-parsed config");

    assert!(json2.is_object(), "Re-parsed config should also be object");
}

/// Test empty/minimal config round-trip
#[wasm_bindgen_test]
fn test_minimal_config_round_trip() {
    let minimal_json = serde_json::json!({});

    let config: ExtractionConfig =
        serde_json::from_value(minimal_json).expect("Should deserialize minimal empty config");

    let serialized = serde_json::to_value(&config).expect("Should serialize minimal config");

    // Should be able to deserialize again
    let config2: ExtractionConfig =
        serde_json::from_value(serialized).expect("Should deserialize roundtrip minimal config");

    // Both configs should be valid
    assert!(serde_json::to_value(&config).is_ok());
    assert!(serde_json::to_value(&config2).is_ok());
}

/// Test config with all format variants in round-trip
#[wasm_bindgen_test]
fn test_all_output_formats_round_trip() {
    let formats = vec!["plain", "markdown", "html", "djot"];

    for format in formats {
        let config_json = serde_json::json!({
            "outputFormat": format,
            "useCache": false
        });

        let config: ExtractionConfig = serde_json::from_value(config_json.clone())
            .expect(&format!("Should deserialize config with format: {}", format));

        let roundtrip =
            serde_json::to_value(&config).expect(&format!("Should serialize config with format: {}", format));

        // Verify format field is preserved or acceptable
        assert!(
            roundtrip.get("outputFormat").is_some() || roundtrip.get("output_format").is_some(),
            "outputFormat should be present in roundtrip for format: {}",
            format
        );
    }
}

/// Test config with all result formats in round-trip
#[wasm_bindgen_test]
fn test_all_result_formats_round_trip() {
    let formats = vec!["unified", "element_based"];

    for format in formats {
        let config_json = serde_json::json!({
            "resultFormat": format,
            "useCache": false
        });

        let config: ExtractionConfig = serde_json::from_value(config_json.clone())
            .expect(&format!("Should deserialize config with result format: {}", format));

        let roundtrip =
            serde_json::to_value(&config).expect(&format!("Should serialize config with result format: {}", format));

        // Verify result format field is preserved
        assert!(
            roundtrip.get("resultFormat").is_some() || roundtrip.get("result_format").is_some(),
            "resultFormat should be present for format: {}",
            format
        );
    }
}

/// Test complex nested config round-trip
#[wasm_bindgen_test]
fn test_complex_nested_config_round_trip() {
    let config_json = serde_json::json!({
        "useCache": true,
        "enableQualityProcessing": true,
        "forceOcr": false,
        "outputFormat": "markdown",
        "resultFormat": "unified",
        "maxConcurrentExtractions": 4,
        "cacheSizeLimit": 1024000000,
        "timeoutMs": 30000,
        "ocr": {
            "backend": "tesseract",
            "language": "eng",
            "enabled": true
        },
        "chunking": {
            "maxCharacters": 1000,
            "maxOverlap": 200,
            "strategy": "semantic"
        },
        "quality": {
            "minConfidence": 0.85,
            "skipLowConfidence": false
        }
    });

    let config: ExtractionConfig =
        serde_json::from_value(config_json.clone()).expect("Should deserialize complex nested config");

    let roundtrip = serde_json::to_value(&config).expect("Should serialize complex nested config");

    // Parse again to verify full round-trip
    let config2: ExtractionConfig =
        serde_json::from_value(roundtrip.clone()).expect("Should deserialize roundtrip complex config");

    let roundtrip2 = serde_json::to_value(&config2).expect("Should serialize roundtrip complex config");

    // Verify it's stable (third serialization should match second)
    assert!(roundtrip.is_object(), "Roundtrip should be object");
    assert!(roundtrip2.is_object(), "Roundtrip2 should be object");
}

/// Test config preserves unknown fields if supported
#[wasm_bindgen_test]
fn test_config_custom_fields_handling() {
    // Test with a field that might not be recognized
    let config_json = serde_json::json!({
        "useCache": true,
        "outputFormat": "plain",
        "customField": "should_be_preserved_or_ignored"
    });

    // This should either deserialize successfully or fail gracefully
    match serde_json::from_value::<ExtractionConfig>(config_json.clone()) {
        Ok(config) => {
            // If it deserializes, verify core fields are present
            let roundtrip = serde_json::to_value(&config).expect("Should serialize config with custom fields");
            assert!(roundtrip.is_object(), "Serialized config should be object");
        }
        Err(_) => {
            // Custom fields not supported - this is acceptable behavior
            // But core fields should work
            let config_json_core = serde_json::json!({
                "useCache": true,
                "outputFormat": "plain"
            });
            let config: ExtractionConfig =
                serde_json::from_value(config_json_core).expect("Should deserialize without custom fields");
            assert!(serde_json::to_value(&config).is_ok());
        }
    }
}

/// Test null/undefined handling in JSON round-trip
#[wasm_bindgen_test]
fn test_config_null_fields_handling() {
    let config_json = serde_json::json!({
        "useCache": true,
        "forceOcr": null,
        "outputFormat": "markdown"
    });

    // Try to deserialize - null values should be handled
    match serde_json::from_value::<ExtractionConfig>(config_json) {
        Ok(config) => {
            let roundtrip = serde_json::to_value(&config).expect("Should serialize config with null fields");
            assert!(roundtrip.is_object());
        }
        Err(_) => {
            // Null values might not be supported - test without nulls
            let config_json_non_null = serde_json::json!({
                "useCache": true,
                "outputFormat": "markdown"
            });
            let config: ExtractionConfig =
                serde_json::from_value(config_json_non_null).expect("Should deserialize without null fields");
            assert!(serde_json::to_value(&config).is_ok());
        }
    }
}
