//! Regression test for issue #1116.
//!
//! `ImageExtractionConfig::classify` must default to `false` so that users
//! who do not explicitly request image classification do not incur ML overhead.

use xberg::ImageExtractionConfig;

#[test]
fn classify_defaults_to_false_in_default_impl() {
    let config = ImageExtractionConfig::default();
    assert!(
        !config.classify,
        "ImageExtractionConfig::default().classify must be false (regression: #1116)"
    );
}

#[test]
fn classify_defaults_to_false_when_deserializing_empty_json() {
    let config: ImageExtractionConfig = serde_json::from_str("{}").unwrap();
    assert!(
        !config.classify,
        "absent `classify` field in JSON must deserialize to false (regression: #1116)"
    );
}

#[test]
fn classify_can_be_enabled_explicitly() {
    let json = r#"{"classify": true}"#;
    let config: ImageExtractionConfig = serde_json::from_str(json).unwrap();
    assert!(config.classify, "explicit classify=true must be honoured");
}
