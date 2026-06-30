//! Integration coverage for P5: the extraction pipeline populates the typed
//! [`extraction_confidence`] field on every extracted document.
//!
//! The per-signal math (recognition-mean `ocr_aggregate`, weight folding,
//! `combined` clamping) is unit-tested in `heuristics::confidence`. These tests
//! prove the *pipeline* sets the field — historically it was always `None` —
//! and that the extract-time defaults are the documented ones: `text_coverage`
//! is 1.0 (core does not yet compute per-page coverage), `schema_compliance` is
//! `AllValid` (no schema at extract time), and a text-only document with no OCR
//! has `ocr_aggregate == None`.
#![cfg(feature = "heuristics")]

use xberg::heuristics::confidence::SchemaCompliance;
use xberg::{ExtractInput, ExtractionConfig, extract};

#[tokio::test]
async fn extract_populates_extraction_confidence_with_extract_time_defaults() {
    let config = ExtractionConfig::default();
    let output = extract(
        ExtractInput::from_bytes(b"Hello, world.".to_vec(), "text/plain", None),
        &config,
    )
    .await
    .expect("text extraction succeeds");

    let doc = output.results.first().expect("one result");
    let confidence = doc
        .extraction_confidence
        .as_ref()
        .expect("extraction_confidence is populated (was always None before P5)");

    // Documented extract-time defaults.
    assert_eq!(
        confidence.text_coverage, 1.0,
        "text_coverage defaults to 1.0 at extract time"
    );
    assert_eq!(
        confidence.schema_compliance,
        SchemaCompliance::AllValid,
        "schema compliance is AllValid at extract time (no schema yet)"
    );
    // A plain-text document runs no OCR, so there is no recognition signal.
    assert!(
        confidence.ocr_aggregate.is_none(),
        "no OCR ran, so ocr_aggregate is None"
    );
    // `combined` is a real score in [0, 1]; with no OCR it folds the OCR weight
    // into text_coverage (1.0) and AllValid (1.0), so it must be 1.0.
    assert_eq!(
        confidence.combined, 1.0,
        "combined of full text coverage + AllValid is 1.0"
    );
}

#[tokio::test]
async fn extract_batch_populates_extraction_confidence_for_each_document() {
    let config = ExtractionConfig::default();
    let output = xberg::extract_batch(
        vec![
            ExtractInput::from_bytes(b"First document.".to_vec(), "text/plain", None),
            ExtractInput::from_bytes(b"Second document.".to_vec(), "text/plain", None),
        ],
        &config,
    )
    .await
    .expect("batch extraction succeeds");

    assert_eq!(output.results.len(), 2, "two documents extracted");
    for doc in &output.results {
        assert!(
            doc.extraction_confidence.is_some(),
            "every extracted document carries typed extraction_confidence"
        );
    }
}
