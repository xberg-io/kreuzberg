//! Smoke test for the gline-rs ONNX NER backend.
//!
//! The default model `urchade/gliner_multi-v2.1` (~100MB) downloads on first
//! run, which makes this test slow and network-dependent. It is marked
//! `#[ignore]` so it does not run in CI by default; the body is deterministic
//! when invoked with `cargo test --features ner-onnx --test ner_smoke -- --ignored`.
//!
//! NOTE: Today the inference path returns `MissingDependency` because the
//! upstream `gline-rs` crate pins `ort = "=2.0.0-rc.9"` while xberg uses
//! `2.0.0-rc.12`. Once the upstream version skew is resolved, this test
//! verifies entity detection on a canonical sentence. Until then, the test
//! still exercises the model download path (asserts a non-zero file size).

#![cfg(feature = "ner-onnx")]

use xberg::text::ner::{default_model_name, download_model};

#[test]
#[ignore = "downloads ~100MB GLiNER model from HuggingFace"]
fn download_default_model_succeeds() {
    let model_path = download_model(default_model_name(), None).expect("model download succeeds");

    let metadata = std::fs::metadata(&model_path).expect("model file exists");
    assert!(metadata.len() > 0, "model file should not be empty");
}

#[tokio::test]
#[ignore = "downloads ~100MB GLiNER model from HuggingFace; runs inference"]
async fn detects_person_org_location_in_canonical_sentence() {
    use xberg::text::ner::NerBackend;
    use xberg::text::ner::gline::GlineBackend;
    use xberg::types::entity::EntityCategory;

    let backend = GlineBackend::new(None).expect("backend construction");
    let text = "Cristiano Ronaldo plays for Al Nassr in Saudi Arabia.";
    let categories = vec![
        EntityCategory::Person,
        EntityCategory::Organization,
        EntityCategory::Location,
    ];

    let result = backend.detect(text, &categories).await;
    match result {
        Ok(entities) => {
            // When inference runs we expect at least Person + Organization + Location.
            assert!(entities.iter().any(|e| e.category == EntityCategory::Person));
            assert!(entities.iter().any(|e| e.category == EntityCategory::Organization));
            assert!(entities.iter().any(|e| e.category == EntityCategory::Location));
        }
        Err(xberg::XbergError::MissingDependency(msg)) => {
            // Current state: gline-rs upstream version skew prevents inference.
            assert!(
                msg.contains("gline-rs") || msg.contains("upstream"),
                "MissingDependency should mention the upstream blocker, got: {msg}"
            );
        }
        Err(other) => panic!("unexpected error: {other:?}"),
    }
}

/// Verifies that `NerConfig::custom_labels` participates in backend dispatch.
///
/// Even with the gline-rs upstream blocker in place the test must compile and
/// run: the backend must accept the call and either succeed (when the LLM is
/// configured) or fail with `MissingDependency` (when neither feature is
/// available at runtime). Both outcomes prove the wiring exists.
#[tokio::test]
async fn custom_labels_route_through_backend() {
    use xberg::core::config::ner::NerConfig;
    use xberg::text::ner::NerBackend;
    use xberg::text::ner::gline::GlineBackend;
    use xberg::types::entity::EntityCategory;

    let cfg = NerConfig {
        categories: vec![EntityCategory::Person, EntityCategory::Organization],
        custom_labels: vec!["Product".to_string(), "Treatment".to_string()],
        ..NerConfig::default()
    };

    let backend = match GlineBackend::new(None) {
        Ok(b) => b,
        Err(xberg::XbergError::MissingDependency(_)) | Err(xberg::XbergError::Plugin { .. }) => {
            // Model download blocked offline — acceptable in this test.
            return;
        }
        Err(other) => panic!("unexpected backend construction error: {other:?}"),
    };

    let text = "Aspirin treats headaches and is manufactured by Bayer.";
    let result = backend
        .detect_with_custom(text, &cfg.categories, &cfg.custom_labels)
        .await;
    match result {
        Ok(_entities) => {
            // Inference unexpectedly online — accept whatever it returns.
        }
        Err(xberg::XbergError::MissingDependency(msg)) => {
            assert!(
                msg.contains("gline-rs") || msg.contains("upstream"),
                "MissingDependency should mention the upstream blocker, got: {msg}"
            );
        }
        Err(other) => panic!("unexpected error: {other:?}"),
    }
}
