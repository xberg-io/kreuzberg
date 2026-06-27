//! Public structured-extraction integration tests.
//!
//! These tests use the unified public `extract(ExtractInput, &ExtractionConfig)`
//! API with a wiremock-stubbed OpenAI-compatible endpoint. No provider API keys
//! or live network calls are required.

use serde_json::json;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

use xberg::{ExtractInput, ExtractionConfig, ExtractionResult, LlmConfig, StructuredExtractionConfig, extract};

const PLAIN_TEXT_MIME: &str = "text/plain";
const PLAIN_TEXT_CONTENT: &[u8] = b"Invoice number: INV-001
Vendor: Acme Corp
Total: $42.00
This document contains enough text for the public extraction pipeline.";

fn invoice_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "invoice_number": { "type": "string" },
            "vendor": { "type": "string" },
            "total": { "type": "string" }
        },
        "required": ["invoice_number", "vendor", "total"],
        "additionalProperties": false
    })
}

fn document_stats_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "word_count": { "type": "integer" },
            "language": { "type": "string" }
        },
        "required": ["word_count", "language"],
        "additionalProperties": false
    })
}

fn stub_completion(content: &str) -> serde_json::Value {
    json!({
        "id": "chatcmpl-test",
        "object": "chat.completion",
        "created": 0,
        "model": "openai/gpt-4o-mini",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": content
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 100,
            "completion_tokens": 20,
            "total_tokens": 120
        }
    })
}

fn structured_config(server_uri: &str, schema: serde_json::Value, prompt: Option<String>) -> ExtractionConfig {
    ExtractionConfig {
        structured_extraction: Some(StructuredExtractionConfig {
            schema,
            schema_name: "test_schema".to_string(),
            schema_description: Some("Deterministic test schema".to_string()),
            strict: true,
            prompt,
            llm: LlmConfig {
                model: "openai/gpt-4o-mini".to_string(),
                api_key: Some("test-key".to_string()),
                base_url: Some(server_uri.to_string()),
                timeout_secs: Some(10),
                max_retries: Some(0),
                ..Default::default()
            },
        }),
        ..Default::default()
    }
}

async fn extract_public_structured(config: &ExtractionConfig) -> ExtractionResult {
    extract(
        ExtractInput::from_bytes(
            PLAIN_TEXT_CONTENT.to_vec(),
            PLAIN_TEXT_MIME,
            Some("invoice.txt".to_string()),
        ),
        config,
    )
    .await
    .expect("public extraction must succeed")
}

fn assert_successful_single_result(output: &ExtractionResult) {
    assert_eq!(output.summary.inputs, 1);
    assert_eq!(output.summary.results, 1);
    assert_eq!(output.summary.errors, 0);
    assert!(
        output.errors.is_empty(),
        "unexpected public extraction errors: {:?}",
        output.errors
    );
    assert_eq!(output.results.len(), 1);
}

#[tokio::test]
async fn public_structured_extraction_populates_result_summary_and_errors() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(
            r#"{"invoice_number":"INV-001","vendor":"Acme Corp","total":"$42.00"}"#,
        )))
        .mount(&server)
        .await;

    let config = structured_config(&server.uri(), invoice_schema(), None);
    let output = extract_public_structured(&config).await;

    assert_successful_single_result(&output);
    let result = &output.results[0];
    let structured_output = result
        .structured_output
        .as_ref()
        .expect("structured_output should be populated");
    assert_eq!(structured_output["invoice_number"].as_str(), Some("INV-001"));
    assert_eq!(structured_output["vendor"].as_str(), Some("Acme Corp"));
    assert_eq!(structured_output["total"].as_str(), Some("$42.00"));

    let usage = result
        .llm_usage
        .as_ref()
        .expect("structured extraction should record LLM usage");
    assert!(
        usage.iter().any(|entry| entry.source == "structured_extraction"),
        "expected structured_extraction usage entry, got {:?}",
        usage.iter().map(|entry| entry.source.as_str()).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn public_structured_extraction_accepts_custom_prompt_template() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(r#"{"word_count":12,"language":"en"}"#)))
        .mount(&server)
        .await;

    let prompt = Some(
        "Analyze the document and return JSON only.

Document:
{{ content }}

Schema:
{{ schema }}"
            .to_string(),
    );
    let config = structured_config(&server.uri(), document_stats_schema(), prompt);
    let output = extract_public_structured(&config).await;

    assert_successful_single_result(&output);
    let structured_output = output.results[0]
        .structured_output
        .as_ref()
        .expect("structured_output should be populated");
    assert_eq!(structured_output["word_count"].as_i64(), Some(12));
    assert_eq!(structured_output["language"].as_str(), Some("en"));
}

#[tokio::test]
async fn public_structured_extraction_failure_is_reported_as_processing_warning() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion("not valid JSON")))
        .mount(&server)
        .await;

    let config = structured_config(&server.uri(), invoice_schema(), None);
    let output = extract_public_structured(&config).await;

    assert_successful_single_result(&output);
    let result = &output.results[0];
    assert!(
        result.structured_output.is_none(),
        "invalid LLM JSON should not populate structured_output"
    );
    assert!(
        result.processing_warnings.iter().any(|warning| {
            warning.source == "structured_extraction"
                && warning.message.contains("Structured extraction failed")
                && warning.message.contains("invalid JSON")
        }),
        "expected structured extraction processing warning, got {:?}",
        result.processing_warnings
    );
}
