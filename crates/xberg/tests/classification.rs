#![cfg(feature = "classification")]

use xberg::ClassificationLabel;

#[test]
fn classification_label_with_confidence() {
    let label = ClassificationLabel {
        label: "invoice".to_string(),
        confidence: Some(0.85),
    };

    assert_eq!(label.label, "invoice");
    assert_eq!(label.confidence, Some(0.85));
}

#[test]
fn classification_label_without_confidence() {
    let label = ClassificationLabel {
        label: "memo".to_string(),
        confidence: None,
    };

    assert_eq!(label.label, "memo");
    assert_eq!(label.confidence, None);
}

#[test]
fn page_classification_structure() {
    use xberg::types::classification::PageClassification;

    let labels = vec![ClassificationLabel {
        label: "invoice".to_string(),
        confidence: Some(0.92),
    }];

    let classification = PageClassification { page_number: 1, labels };

    assert_eq!(classification.page_number, 1);
    assert_eq!(classification.labels.len(), 1);
    assert_eq!(classification.labels[0].label, "invoice");
}

#[test]
fn page_classification_config_creation() {
    use xberg::core::config::{LlmConfig, PageClassificationConfig};

    let config = PageClassificationConfig {
        labels: vec!["invoice".to_string(), "memo".to_string()],
        llm: LlmConfig::default(),
        prompt_template: None,
        multi_label: false,
    };

    assert_eq!(config.labels.len(), 2);
    assert_eq!(config.labels[0], "invoice");
    assert!(!config.multi_label);
}

#[test]
fn page_classification_multi_label_mode() {
    use xberg::core::config::{LlmConfig, PageClassificationConfig};

    let config = PageClassificationConfig {
        labels: vec!["invoice".to_string(), "memo".to_string(), "report".to_string()],
        llm: LlmConfig::default(),
        prompt_template: None,
        multi_label: true,
    };

    assert!(config.multi_label);
    assert_eq!(config.labels.len(), 3);
}

#[test]
fn classification_label_serialization() {
    let label = ClassificationLabel {
        label: "document".to_string(),
        confidence: Some(0.75),
    };

    let json = serde_json::to_string(&label).expect("serialization should succeed");
    assert!(json.contains("document"));
    assert!(json.contains("0.75"));

    let deserialized: ClassificationLabel = serde_json::from_str(&json).expect("deserialization should succeed");
    assert_eq!(deserialized.label, "document");
    assert_eq!(deserialized.confidence, Some(0.75));
}
