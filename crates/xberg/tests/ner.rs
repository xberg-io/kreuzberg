#![cfg(feature = "ner")]

use xberg::types::entity::EntityCategory;

#[test]
fn entity_category_from_string() {
    assert_eq!(EntityCategory::from("person".to_string()), EntityCategory::Person);
    assert_eq!(
        EntityCategory::from("organization".to_string()),
        EntityCategory::Organization
    );
    assert_eq!(EntityCategory::from("location".to_string()), EntityCategory::Location);
    assert_eq!(EntityCategory::from("date".to_string()), EntityCategory::Date);
    assert_eq!(EntityCategory::from("time".to_string()), EntityCategory::Time);
    assert_eq!(EntityCategory::from("money".to_string()), EntityCategory::Money);
    assert_eq!(EntityCategory::from("percent".to_string()), EntityCategory::Percent);
    assert_eq!(EntityCategory::from("email".to_string()), EntityCategory::Email);
    assert_eq!(EntityCategory::from("phone".to_string()), EntityCategory::Phone);
    assert_eq!(EntityCategory::from("url".to_string()), EntityCategory::Url);
}

#[test]
fn entity_category_custom_fallback() {
    let custom = EntityCategory::from("treatment".to_string());
    match custom {
        EntityCategory::Custom(label) => assert_eq!(label, "treatment"),
        _ => panic!("Expected Custom variant"),
    }
}

#[test]
fn entity_category_default() {
    let default = EntityCategory::default();
    match default {
        EntityCategory::Custom(label) => assert_eq!(label, ""),
        _ => panic!("Expected Custom variant with empty string"),
    }
}

#[test]
fn entity_construction() {
    let entity = xberg::Entity {
        category: EntityCategory::Person,
        text: "Alice".to_string(),
        start: 0,
        end: 5,
        confidence: Some(0.95),
    };

    assert_eq!(entity.category, EntityCategory::Person);
    assert_eq!(entity.text, "Alice");
    assert_eq!(entity.start, 0);
    assert_eq!(entity.end, 5);
    assert_eq!(entity.confidence, Some(0.95));
}

#[test]
fn entity_confidence_optional() {
    let entity = xberg::Entity {
        category: EntityCategory::Organization,
        text: "Acme".to_string(),
        start: 10,
        end: 14,
        confidence: None,
    };

    assert_eq!(entity.confidence, None);
}

#[cfg(feature = "ner-llm")]
#[test]
fn llm_backend_construction() {
    use xberg::LlmBackend;
    use xberg::core::config::LlmConfig;

    let config = LlmConfig::default();
    let backend = LlmBackend::new(config);
    let _backend_ref: &dyn xberg::text::ner::NerBackend = &backend;
}
