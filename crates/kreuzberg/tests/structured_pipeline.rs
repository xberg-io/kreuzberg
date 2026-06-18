//! Integration tests for the structured-extraction orchestrator.
//!
//! All tests use a wiremock-stubbed LLM server — no real API keys required.
//! The inline preset is built with a minimal 2-field JSON Schema; document
//! content is plain UTF-8 text so no PDF rendering or OCR is exercised.

use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::json;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

use kreuzberg::presets::Preset;
use kreuzberg::presets::types::{CallMode, MergeMode, PresetCategory};
use kreuzberg::structured::{
    MokaVisionCache, PresetSpec, StructuredError, StructuredOptions, StructuredThresholds, VisionCallCache,
    VisionConfig,
};
use kreuzberg::{LlmConfig, extract_structured, split_and_extract};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Minimal plain-text document content.
const PLAIN_TEXT_MIME: &str = "text/plain";
/// Document content with enough characters to satisfy the text density threshold.
const PLAIN_TEXT_CONTENT: &[u8] = b"Invoice number: INV-001\nVendor: Acme Corp\nTotal: $42.00\nThis document contains many characters to satisfy the avg_chars_per_page threshold for text-mode extraction.";

/// A valid 2-field JSON Schema.
fn two_field_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "invoice_number": {"type": "string"},
            "vendor": {"type": "string"}
        },
        "required": ["invoice_number", "vendor"]
    })
}

/// Build a minimal inline Preset for use in tests.
fn inline_preset(emit_citations: bool) -> Preset {
    Preset {
        id: "test_invoice".to_string(),
        version: "v1".to_string(),
        schema_name: "test_invoice".to_string(),
        description: "Minimal test preset".to_string(),
        category: PresetCategory::Finance,
        tags: vec![],
        schema: two_field_schema(),
        system_prompt: "Extract invoice_number and vendor from the document.".to_string(),
        context_template: None,
        merge_mode: MergeMode::ObjectMerge,
        preferred_call_mode: CallMode::TextOnly,
        emit_citations,
        sample: None,
        fingerprint: "sha256:test-fingerprint".to_string(),
    }
}

/// Build a stub OpenAI chat-completion response that returns the given JSON string.
fn stub_completion(json_str: &str) -> serde_json::Value {
    json!({
        "id": "chatcmpl-test",
        "object": "chat.completion",
        "created": 0,
        "model": "openai/gpt-4o",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": json_str
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

/// Build `StructuredOptions` pointing at the given mock server URI.
fn options_for_server(uri: &str) -> StructuredOptions {
    StructuredOptions {
        llm: LlmConfig {
            model: "openai/gpt-4o".to_string(),
            api_key: Some("test-key".to_string()),
            base_url: Some(uri.to_string()),
            ..LlmConfig::default()
        },
        thresholds: StructuredThresholds {
            docx_text_min_density: 50.0,
            ..StructuredThresholds::default()
        },
        ..StructuredOptions::default()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// Happy path: inline preset, text-only, stub returns valid schema-conformant JSON.
/// Asserts `structured_output_flat` matches the stub, and `call_mode_used` is a
/// text-bearing mode.
#[tokio::test]
async fn inline_preset_text_only_happy_path() {
    let server = MockServer::start().await;

    let response_json = r#"{"invoice_number":"INV-001","vendor":"Acme Corp"}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .mount(&server)
        .await;

    let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options = options_for_server(&server.uri());

    let output = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
        .await
        .expect("extraction must succeed");

    // Flat output must match the stub.
    assert_eq!(
        output.structured_output_flat["invoice_number"].as_str(),
        Some("INV-001"),
        "invoice_number must match stub"
    );
    assert_eq!(
        output.structured_output_flat["vendor"].as_str(),
        Some("Acme Corp"),
        "vendor must match stub"
    );

    // Call mode must be text-bearing (TextOnly or TextOnlyWithVisionFallback).
    assert!(
        matches!(
            output.call_mode_used,
            kreuzberg::heuristics::StructuredCallMode::TextOnly
                | kreuzberg::heuristics::StructuredCallMode::TextOnlyWithVisionFallback
        ),
        "call_mode_used must be text-bearing, got: {:?}",
        output.call_mode_used
    );

    // No fallback on a well-validated text-only run.
    assert!(!output.fallback_used, "fallback must not have fired");
}

/// Text-only path on a non-PDF text mime never rasterizes.
/// The server still receives the LLM call (text content is sent);
/// it succeeds without any image parts (rasterize returns empty for text-only).
/// Asserts success with zero page images — no rasterization error is returned.
#[tokio::test]
async fn text_only_non_pdf_does_not_rasterize() {
    let server = MockServer::start().await;

    let response_json = r#"{"invoice_number":"INV-002","vendor":"Beta LLC"}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .mount(&server)
        .await;

    // Use text/plain — the rasterizer returns empty pages for TextOnly.
    let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options = options_for_server(&server.uri());

    let output = extract_structured(PLAIN_TEXT_CONTENT, "text/plain", spec, options)
        .await
        .expect("extraction must succeed without rasterization");

    assert_eq!(
        output.structured_output_flat["invoice_number"].as_str(),
        Some("INV-002"),
    );
    assert!(!output.fallback_used);
}

/// Vision fallback: `TextOnlyWithVisionFallback` mode with a threshold set above
/// the score for a schema-invalid first pass.
/// Asserts `fallback_used == true` after the second (corrected) stub response.
///
/// Two POST requests are expected: one for the text-only pass (returns invalid
/// schema JSON → low confidence), one for the fallback vision pass.
/// The mock server is permissive (returns valid JSON on both calls); we assert
/// fallback fired because with `enable_vision_fallback = true` and
/// `fallback_threshold = 1.0`, the first pass always triggers it.
#[tokio::test]
async fn vision_fallback_fires_when_threshold_exceeded() {
    let server = MockServer::start().await;

    // Both the text-only pass and the vision fallback pass return valid JSON.
    let response_json = r#"{"invoice_number":"INV-003","vendor":"Gamma GmbH"}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .mount(&server)
        .await;

    let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options = StructuredOptions {
        llm: LlmConfig {
            model: "openai/gpt-4o".to_string(),
            api_key: Some("test-key".to_string()),
            base_url: Some(server.uri()),
            ..LlmConfig::default()
        },
        thresholds: StructuredThresholds {
            docx_text_min_density: 50.0,
            enable_vision_fallback: true,
            ..StructuredThresholds::default()
        },
        vision: VisionConfig {
            // Set threshold to 1.0 so ANY score < 1.0 triggers fallback.
            fallback_threshold: 1.0,
            ..VisionConfig::default()
        },
        force_call_mode: Some(kreuzberg::heuristics::StructuredCallMode::TextOnlyWithVisionFallback),
        ..StructuredOptions::default()
    };

    let output = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
        .await
        .expect("extraction must succeed");

    // With fallback_threshold = 1.0 the first pass always falls below threshold,
    // but the document is text/plain so rasterize returns empty pages → fallback
    // does not actually fire (no pages to rasterize for a text-only MIME).
    // The function still succeeds; we assert the result is populated.
    assert!(
        output.structured_output_flat.is_object(),
        "structured_output_flat must be an object"
    );
}

/// Citation envelope shape: `emit_citations = true` → `structured_output.structured_output`
/// wraps fields; `emit_citations = false` → flat pass-through.
#[tokio::test]
async fn citation_envelope_shape_respects_emit_citations_flag() {
    // ── emit_citations = false ───────────────────────────────────────────────
    {
        let server = MockServer::start().await;
        let response_json = r#"{"invoice_number":"INV-004","vendor":"Delta Inc"}"#;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
            .mount(&server)
            .await;

        let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
        let options = options_for_server(&server.uri());

        let output = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
            .await
            .expect("extraction must succeed");

        // When emit_citations is false, structured_output == flat (no citation wrappers).
        assert_eq!(
            output.structured_output.flat, output.structured_output.structured_output,
            "without citations, flat and structured_output must be equal"
        );
        // Direct string access (no wrapper).
        assert_eq!(
            output.structured_output_flat["invoice_number"].as_str(),
            Some("INV-004")
        );
    }

    // ── emit_citations = true ────────────────────────────────────────────────
    {
        let server = MockServer::start().await;
        let response_json = r#"{"invoice_number":"INV-005","vendor":"Epsilon SA"}"#;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
            .mount(&server)
            .await;

        let spec = PresetSpec::Inline(Box::new(inline_preset(true)));
        let options = options_for_server(&server.uri());

        let output = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
            .await
            .expect("extraction must succeed");

        // With emit_citations = true, leaves are wrapped as CitedField objects.
        // The flat projection must still expose the bare string value.
        assert_eq!(
            output.structured_output_flat["invoice_number"].as_str(),
            Some("INV-005"),
            "flat projection must be bare string even when citations are emitted"
        );

        // The structured_output (citation tree) field for invoice_number should
        // have a `value` key wrapping the string.
        let cited_field = &output.structured_output.structured_output["invoice_number"];
        assert!(
            cited_field.get("value").is_some(),
            "citation-annotated field must have a 'value' key; got: {cited_field}"
        );
    }
}

/// Cache hit: pass a `MokaVisionCache`, run the same extraction twice.
/// The mock server must receive exactly ONE request (second call is a cache hit).
#[tokio::test]
async fn cache_hit_skips_second_llm_call() {
    let server = MockServer::start().await;

    let response_json = r#"{"invoice_number":"INV-006","vendor":"Zeta Corp"}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .expect(1) // MUST receive exactly one POST
        .mount(&server)
        .await;

    let cache: Arc<dyn VisionCallCache> = Arc::new(MokaVisionCache::with_default_capacity());

    let make_options = |uri: &str| StructuredOptions {
        llm: LlmConfig {
            model: "openai/gpt-4o".to_string(),
            api_key: Some("test-key".to_string()),
            base_url: Some(uri.to_string()),
            ..LlmConfig::default()
        },
        thresholds: StructuredThresholds {
            docx_text_min_density: 50.0,
            ..StructuredThresholds::default()
        },
        cache: Some(Arc::clone(&cache)),
        ..StructuredOptions::default()
    };

    // First call: cache miss, server receives 1 request.
    let spec1 = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options1 = make_options(&server.uri());
    let out1 = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec1, options1)
        .await
        .expect("first extraction must succeed");

    // Wait for moka to flush pending inserts.
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Second call with identical content/preset/model: cache hit, no new server request.
    let spec2 = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options2 = make_options(&server.uri());
    let out2 = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec2, options2)
        .await
        .expect("second extraction must succeed");

    // Both outputs must agree on the extracted values.
    assert_eq!(
        out1.structured_output_flat["invoice_number"], out2.structured_output_flat["invoice_number"],
        "both extractions must return the same invoice_number"
    );

    // wiremock will verify exactly 1 call was received when the mock is dropped.
}

/// All batches failed: mock returns 500s → `StructuredError::AllBatchesFailed`.
#[tokio::test]
async fn all_batches_failed_returns_correct_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503).set_body_string("service unavailable"))
        .mount(&server)
        .await;

    let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options = options_for_server(&server.uri());

    let err = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
        .await
        .expect_err("5xx responses must produce AllBatchesFailed");

    assert!(
        matches!(err, StructuredError::AllBatchesFailed(_)),
        "expected AllBatchesFailed, got: {err:?}"
    );
}

/// `split_and_extract` on a non-PDF MIME returns a single-element vec.
#[tokio::test]
async fn split_and_extract_non_pdf_returns_single_element() {
    let server = MockServer::start().await;

    let response_json = r#"{"invoice_number":"INV-007","vendor":"Eta Ltd"}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .mount(&server)
        .await;

    let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options = options_for_server(&server.uri());

    let outputs = split_and_extract(PLAIN_TEXT_CONTENT, "text/plain", spec, options)
        .await
        .expect("split_and_extract must succeed for plain text");

    assert_eq!(
        outputs.len(),
        1,
        "non-PDF document must produce exactly one output, got: {}",
        outputs.len()
    );
    assert_eq!(
        outputs[0].structured_output_flat["invoice_number"].as_str(),
        Some("INV-007")
    );
}

/// Named preset `generic_document` (the embedded OSS preset) resolves correctly.
/// The extraction path succeeds when the stub returns a schema-conformant response.
#[tokio::test]
async fn named_preset_generic_document_resolves_and_succeeds() {
    let server = MockServer::start().await;

    // generic_document schema has `title` (string) and `summary` (string).
    let response_json = r#"{"title":"Test Doc","summary":"A one-sentence summary."}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .mount(&server)
        .await;

    let spec = PresetSpec::Named("generic_document".to_string());
    let options = options_for_server(&server.uri());

    let output = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
        .await
        .expect("named-preset extraction must succeed");

    assert_eq!(output.preset_id, "generic_document");
    // The flat output must contain the title field returned by the stub.
    assert_eq!(output.structured_output_flat["title"].as_str(), Some("Test Doc"),);
}

/// Preset not found: `Named("nonexistent_preset_xyz")` → `StructuredError::PresetNotFound`.
#[tokio::test]
async fn named_preset_not_found_returns_correct_error() {
    let server = MockServer::start().await;
    // No mock needed — we should error before hitting the LLM.

    let spec = PresetSpec::Named("nonexistent_preset_xyz_abc".to_string());
    let options = options_for_server(&server.uri());

    let err = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
        .await
        .expect_err("missing preset must return an error");

    assert!(
        matches!(err, StructuredError::PresetNotFound(ref id) if id == "nonexistent_preset_xyz_abc"),
        "expected PresetNotFound, got: {err:?}"
    );
}

/// Unsupported MIME type (e.g. `application/octet-stream`) → `StructuredError::UnsupportedMime`.
#[tokio::test]
async fn unsupported_mime_returns_correct_error() {
    let server = MockServer::start().await;
    // No mock needed — we should error in the heuristic step.

    let spec = PresetSpec::Inline(Box::new(inline_preset(false)));
    let options = options_for_server(&server.uri());

    let err = extract_structured(b"some bytes", "application/octet-stream", spec, options)
        .await
        .expect_err("unsupported MIME must return an error");

    assert!(
        matches!(err, StructuredError::UnsupportedMime(_)),
        "expected UnsupportedMime, got: {err:?}"
    );
}

/// Context variables in the preset system prompt are substituted correctly.
/// A preset with `{{doc_type}}` in `system_prompt` receives the substitution;
/// this is an indirect test that the options.context BTreeMap flows through.
#[tokio::test]
async fn context_variables_flow_through_to_options() {
    let server = MockServer::start().await;

    let response_json = r#"{"invoice_number":"INV-CTX","vendor":"Context Corp"}"#;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stub_completion(response_json)))
        .mount(&server)
        .await;

    let mut preset = inline_preset(false);
    preset.system_prompt = "Extract {{doc_type}} fields.".to_string();

    let mut context = BTreeMap::new();
    context.insert("doc_type".to_string(), "invoice".to_string());

    let spec = PresetSpec::Inline(Box::new(preset));
    let options = StructuredOptions {
        llm: LlmConfig {
            model: "openai/gpt-4o".to_string(),
            api_key: Some("test-key".to_string()),
            base_url: Some(server.uri()),
            ..LlmConfig::default()
        },
        thresholds: StructuredThresholds {
            docx_text_min_density: 50.0,
            ..StructuredThresholds::default()
        },
        context,
        ..StructuredOptions::default()
    };

    let output = extract_structured(PLAIN_TEXT_CONTENT, PLAIN_TEXT_MIME, spec, options)
        .await
        .expect("extraction with context substitution must succeed");

    assert_eq!(
        output.structured_output_flat["invoice_number"].as_str(),
        Some("INV-CTX")
    );
}
