//! Equivalence tests: the Rust-only `Engine::new_default()` must produce
//! results identical to the free `xberg::extract` / `xberg::extract_batch`
//! functions, which delegate to a process-global default engine.
//!
//! This is the verification anchor for the P1 pure refactor that introduced
//! `xberg::engine::Engine`.

#![cfg(feature = "tokio-runtime")]

use std::fs::File;
use std::io::Write;

use tempfile::tempdir;

use xberg::core::config::{ExtractInput, ExtractionConfig};
use xberg::engine::Engine;

/// Compare two extraction results by their serialized JSON form. The single
/// `extract` path is deterministic (no per-item timing metadata is attached),
/// so byte-for-byte JSON equality is a strong equivalence assertion.
fn assert_results_equal(lhs: &xberg::ExtractionResult, rhs: &xberg::ExtractionResult) {
    let lhs_json = serde_json::to_value(lhs).expect("serialize free-function result");
    let rhs_json = serde_json::to_value(rhs).expect("serialize engine result");
    assert_eq!(lhs_json, rhs_json, "engine and free-function results must be identical");
}

#[tokio::test]
async fn engine_default_matches_free_function_for_bytes() {
    let config = ExtractionConfig::default();

    let free = xberg::extract(
        ExtractInput::from_bytes(b"hello engine equivalence".to_vec(), "text/plain", None),
        &config,
    )
    .await
    .expect("free-function extract");

    let engine_output = Engine::new_default()
        .extract(
            ExtractInput::from_bytes(b"hello engine equivalence".to_vec(), "text/plain", None),
            &config,
        )
        .await
        .expect("engine extract");

    assert_eq!(free.results.len(), 1);
    assert_results_equal(&free, &engine_output);
}

#[tokio::test]
async fn engine_default_matches_free_function_for_file() {
    let dir = tempdir().expect("create tempdir");
    let path = dir.path().join("doc.txt");
    File::create(&path)
        .expect("create temp file")
        .write_all(b"hello file equivalence")
        .expect("write temp file");

    let config = ExtractionConfig::default();

    let free = xberg::extract(ExtractInput::from_uri(path.to_string_lossy()), &config)
        .await
        .expect("free-function extract");

    let engine_output = Engine::new_default()
        .extract(ExtractInput::from_uri(path.to_string_lossy()), &config)
        .await
        .expect("engine extract");

    assert_eq!(free.results.len(), 1);
    assert_results_equal(&free, &engine_output);
}

#[tokio::test]
async fn engine_default_matches_free_function_for_batch() {
    let dir = tempdir().expect("create tempdir");
    let path = dir.path().join("doc.txt");
    File::create(&path)
        .expect("create temp file")
        .write_all(b"hello batch equivalence")
        .expect("write temp file");

    let config = ExtractionConfig::default();

    let inputs = || {
        vec![
            ExtractInput::from_bytes(b"hello batch bytes".to_vec(), "text/plain", None),
            ExtractInput::from_uri(path.to_string_lossy()),
        ]
    };

    let free = xberg::extract_batch(inputs(), &config)
        .await
        .expect("free-function extract_batch");

    let engine_output = Engine::new_default()
        .extract_batch(inputs(), &config)
        .await
        .expect("engine extract_batch");

    // The batch path attaches per-item `extraction_duration_ms`, which varies
    // between runs, so compare structural fields rather than exact JSON.
    assert_eq!(free.results.len(), engine_output.results.len());
    assert_eq!(free.summary.inputs, engine_output.summary.inputs);
    assert_eq!(free.summary.results, engine_output.summary.results);
    assert_eq!(free.summary.errors, engine_output.summary.errors);
    assert_eq!(free.errors.len(), engine_output.errors.len());
    for (lhs, rhs) in free.results.iter().zip(engine_output.results.iter()) {
        assert_eq!(lhs.content, rhs.content);
        assert_eq!(lhs.mime_type, rhs.mime_type);
    }
}
