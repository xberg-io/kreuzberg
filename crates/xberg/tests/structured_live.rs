//! Live integration tests for public structured extraction.
//!
//! These tests hit real provider APIs and require API keys in the workspace
//! `.env` (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`). Each
//! test skips gracefully when its required key is missing.
//!
//! Run with:
//!
//! ```text
//! cargo test -p xberg --features "structured,pdf" --test structured_live -- --nocapture --test-threads=1
//! ```
//!
//! `--test-threads=1` keeps concurrent provider calls below rate limits.

#![cfg(all(feature = "structured", feature = "pdf", not(target_arch = "wasm32")))]

use serde_json::json;

use xberg::{ExtractInput, ExtractionConfig, LlmConfig, StructuredExtractionConfig, extract};

const FAKE_MEMO_PDF: &str = "../../test_documents/pdf/fake_memo.pdf";
const PDF_MIME: &str = "application/pdf";

macro_rules! require_env {
    ($var:expr) => {
        match std::env::var($var) {
            Ok(value) if !value.is_empty() => value,
            _ => {
                eprintln!("SKIP: {} not set, skipping live test", $var);
                return;
            }
        }
    };
}

fn init() {
    let _ = dotenvy::dotenv();
}

fn make_llm_config(model: &str, api_key: String) -> LlmConfig {
    LlmConfig {
        model: model.to_string(),
        api_key: Some(api_key),
        timeout_secs: Some(120),
        max_retries: Some(2),
        temperature: Some(0.0),
        max_tokens: Some(512),
        ..Default::default()
    }
}

fn memo_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "title": { "type": "string" },
            "date": { "type": "string" },
            "summary": { "type": "string" }
        },
        "required": ["title", "date", "summary"],
        "additionalProperties": false
    })
}

fn structured_config(model: &str, api_key: String, strict: bool) -> ExtractionConfig {
    ExtractionConfig {
        structured_extraction: Some(StructuredExtractionConfig {
            schema: memo_schema(),
            schema_name: "memo_data".to_string(),
            schema_description: Some("Extract memo metadata".to_string()),
            strict,
            prompt: None,
            llm: make_llm_config(model, api_key),
        }),
        ..Default::default()
    }
}

async fn run_structured_memo(model: &str, api_key: String, strict: bool) {
    let bytes = std::fs::read(FAKE_MEMO_PDF).unwrap_or_else(|error| panic!("failed to read {FAKE_MEMO_PDF}: {error}"));
    let config = structured_config(model, api_key, strict);

    let output = extract(
        ExtractInput::from_bytes(bytes, PDF_MIME, Some("fake_memo.pdf".to_string())),
        &config,
    )
    .await
    .expect("public structured extraction must succeed");

    assert_eq!(output.summary.inputs, 1);
    assert_eq!(output.summary.results, 1);
    assert_eq!(output.summary.errors, 0);
    assert!(
        output.errors.is_empty(),
        "structured extraction returned public errors: {:?}",
        output.errors
    );

    let result = output.results.first().expect("expected one extraction result");
    let structured_output = result
        .structured_output
        .as_ref()
        .expect("structured_output must be populated");
    assert!(
        structured_output.is_object(),
        "structured_output must be an object, got: {structured_output}"
    );
    assert!(
        structured_output
            .get("title")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|title| !title.is_empty()),
        "title must be a non-empty string, got: {structured_output}"
    );
    assert!(
        structured_output
            .get("summary")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|summary| !summary.is_empty()),
        "summary must be a non-empty string, got: {structured_output}"
    );

    let usage = result
        .llm_usage
        .as_ref()
        .expect("structured extraction should record LLM usage");
    assert!(!usage.is_empty(), "llm_usage must be non-empty");
    assert!(
        usage.iter().any(|entry| entry.source == "structured_extraction"),
        "expected structured_extraction usage entry, got {:?}",
        usage.iter().map(|entry| entry.source.as_str()).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn structured_extraction_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");
    run_structured_memo("openai/gpt-4o-mini", api_key, true).await;
}

#[tokio::test]
async fn structured_extraction_anthropic() {
    init();
    let api_key = require_env!("ANTHROPIC_API_KEY");
    run_structured_memo("anthropic/claude-haiku-4-5-20251001", api_key, false).await;
}

#[tokio::test]
async fn structured_extraction_gemini() {
    init();
    let api_key = require_env!("GEMINI_API_KEY");
    run_structured_memo("gemini/gemini-2.5-flash", api_key, false).await;
}
