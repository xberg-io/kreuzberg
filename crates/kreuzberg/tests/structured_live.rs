//! Live integration tests for the structured-extraction engine.
//!
//! These tests hit real provider APIs and require API keys in the workspace
//! `.env` (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`). Each
//! test skips gracefully when its required key is missing.
//!
//! Run with:
//!
//! ```text
//! cargo test -p kreuzberg --features structured --test structured_live -- --nocapture --test-threads=1
//! ```
//!
//! `--test-threads=1` keeps concurrent provider calls below rate limits.

#![cfg(all(feature = "structured", not(target_arch = "wasm32")))]

use serde_json::json;

use kreuzberg::heuristics::StructuredCallMode;
use kreuzberg::presets::Preset;
use kreuzberg::presets::types::{CallMode, MergeMode, PresetCategory};
use kreuzberg::structured::{PresetSpec, StructuredOptions, VisionConfig};
use kreuzberg::{LlmConfig, extract_structured, extract_structured_json, split_and_extract};

// ── Fixture paths (relative to the crate dir `crates/kreuzberg/`) ─────────────

const FAKE_MEMO_PDF: &str = "../../test_documents/pdf/fake_memo.pdf";
const RECEIPT_PDF: &str = "../../test_documents/vendored/markitdown/pdf/RECEIPT-2024-TXN-98765_retail_purchase.pdf";
const INVOICE_IMAGE_PNG: &str = "../../test_documents/images/invoice_image.png";
const REPAIR_PDF: &str = "../../test_documents/vendored/markitdown/pdf/REPAIR-2022-INV-001_multipage.pdf";

// ── Macros ────────────────────────────────────────────────────────────────────

macro_rules! require_env {
    ($var:expr) => {
        match std::env::var($var) {
            Ok(v) if !v.is_empty() => v,
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

// ── Helpers ───────────────────────────────────────────────────────────────────

fn make_llm_config(model: &str, api_key: String) -> LlmConfig {
    LlmConfig {
        model: model.into(),
        api_key: Some(api_key),
        timeout_secs: Some(120),
        max_retries: Some(2),
        ..Default::default()
    }
}

/// Build a minimal inline invoice Preset (3 fields: invoice_number, vendor, total).
fn inline_invoice_preset() -> Preset {
    Preset {
        id: "live_invoice".to_string(),
        version: "v1".to_string(),
        schema_name: "live_invoice".to_string(),
        description: "Live test invoice preset".to_string(),
        category: PresetCategory::Finance,
        tags: vec![],
        schema: json!({
            "type": "object",
            "properties": {
                "invoice_number": {"type": "string"},
                "vendor": {"type": "string"},
                "total": {"type": "string"}
            },
            "required": ["invoice_number", "vendor", "total"]
        }),
        system_prompt: "Extract invoice_number, vendor, and total.".to_string(),
        context_template: None,
        merge_mode: MergeMode::ObjectMerge,
        preferred_call_mode: CallMode::TextOnly,
        emit_citations: false,
        sample: None,
        fingerprint: "sha256:live-invoice-test-fingerprint".to_string(),
    }
}

fn make_structured_options(
    model: &str,
    api_key: String,
    force_call_mode: Option<StructuredCallMode>,
) -> StructuredOptions {
    StructuredOptions {
        llm: make_llm_config(model, api_key),
        force_call_mode,
        vision: VisionConfig {
            max_output_tokens: 512,
            ..Default::default()
        },
        ..Default::default()
    }
}

// ── (a) generic_document named preset on fake_memo.pdf ────────────────────────

async fn run_generic_document_on_memo(model: &str, api_key: String) {
    let bytes = std::fs::read(FAKE_MEMO_PDF).unwrap_or_else(|e| panic!("failed to read {FAKE_MEMO_PDF}: {e}"));
    let spec = PresetSpec::Named("generic_document".into());
    let options = make_structured_options(model, api_key, Some(StructuredCallMode::TextOnly));

    let output = extract_structured(&bytes, "application/pdf", spec, options)
        .await
        .expect("generic_document extraction must succeed");

    assert_eq!(
        output.preset_id, "generic_document",
        "preset_id must be generic_document"
    );
    assert_eq!(output.preset_version, "v1", "preset_version must be v1");
    assert_eq!(
        output.call_mode_used,
        StructuredCallMode::TextOnly,
        "call_mode_used must be TextOnly"
    );

    let title = output.structured_output_flat["title"].as_str();
    assert!(
        title.is_some() && !title.unwrap().is_empty(),
        "title must be a non-empty string, got: {:?}",
        output.structured_output_flat["title"]
    );
    let summary = output.structured_output_flat["summary"].as_str();
    assert!(
        summary.is_some() && !summary.unwrap().is_empty(),
        "summary must be a non-empty string, got: {:?}",
        output.structured_output_flat["summary"]
    );

    assert!(!output.llm_usage.is_empty(), "llm_usage must be non-empty");
    assert!(
        output.llm_usage.iter().any(|u| u.source == "structured_extraction"),
        "expected at least one usage entry with source=structured_extraction, got: {:?}",
        output.llm_usage.iter().map(|u| u.source.as_str()).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_generic_document_memo_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");
    run_generic_document_on_memo("openai/gpt-4o-mini", api_key).await;
}

#[tokio::test]
async fn test_generic_document_memo_anthropic() {
    init();
    let api_key = require_env!("ANTHROPIC_API_KEY");
    run_generic_document_on_memo("anthropic/claude-haiku-4-5-20251001", api_key).await;
}

#[tokio::test]
async fn test_generic_document_memo_gemini() {
    init();
    let api_key = require_env!("GEMINI_API_KEY");
    run_generic_document_on_memo("gemini/gemini-2.5-flash", api_key).await;
}

// ── (b) inline invoice preset on receipt PDF ──────────────────────────────────

async fn run_inline_invoice_on_receipt(model: &str, api_key: String) {
    let bytes = std::fs::read(RECEIPT_PDF).unwrap_or_else(|e| panic!("failed to read {RECEIPT_PDF}: {e}"));
    let spec = PresetSpec::Inline(Box::new(inline_invoice_preset()));
    let options = make_structured_options(model, api_key, Some(StructuredCallMode::TextOnly));

    let output = extract_structured(&bytes, "application/pdf", spec, options)
        .await
        .expect("inline invoice extraction on receipt must succeed");

    let flat = &output.structured_output_flat;

    let vendor = flat["vendor"].as_str();
    assert!(
        vendor.is_some() && !vendor.unwrap().is_empty(),
        "vendor must be a non-empty string, got: {:?}",
        flat["vendor"]
    );
    assert!(
        flat.get("invoice_number").is_some(),
        "invoice_number field must be present, got: {flat}"
    );
    assert!(flat.get("total").is_some(), "total field must be present, got: {flat}");
}

#[tokio::test]
async fn test_inline_invoice_receipt_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");
    run_inline_invoice_on_receipt("openai/gpt-4o-mini", api_key).await;
}

#[tokio::test]
async fn test_inline_invoice_receipt_anthropic() {
    init();
    let api_key = require_env!("ANTHROPIC_API_KEY");
    run_inline_invoice_on_receipt("anthropic/claude-haiku-4-5-20251001", api_key).await;
}

#[tokio::test]
async fn test_inline_invoice_receipt_gemini() {
    init();
    let api_key = require_env!("GEMINI_API_KEY");
    run_inline_invoice_on_receipt("gemini/gemini-2.5-flash", api_key).await;
}

// ── (c) VisionOnly on invoice_image.png ──────────────────────────────────────

async fn run_vision_only_invoice_image(model: &str, api_key: String) {
    let bytes = std::fs::read(INVOICE_IMAGE_PNG).unwrap_or_else(|e| panic!("failed to read {INVOICE_IMAGE_PNG}: {e}"));
    let spec = PresetSpec::Inline(Box::new(inline_invoice_preset()));
    let options = make_structured_options(model, api_key, Some(StructuredCallMode::VisionOnly));

    let output = extract_structured(&bytes, "image/png", spec, options)
        .await
        .expect("VisionOnly extraction on invoice image must succeed");

    assert_eq!(
        output.call_mode_used,
        StructuredCallMode::VisionOnly,
        "call_mode_used must be VisionOnly"
    );
    assert!(
        output.structured_output_flat.is_object(),
        "structured_output_flat must be an object, got: {:?}",
        output.structured_output_flat
    );
    assert!(
        !output.structured_output_flat.as_object().unwrap().is_empty(),
        "structured_output_flat must be non-empty"
    );
    assert!(
        !output.llm_usage.is_empty(),
        "llm_usage must be populated for a vision call"
    );
}

#[tokio::test]
async fn test_vision_only_invoice_image_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");
    run_vision_only_invoice_image("openai/gpt-4o-mini", api_key).await;
}

#[tokio::test]
async fn test_vision_only_invoice_image_anthropic() {
    init();
    let api_key = require_env!("ANTHROPIC_API_KEY");
    run_vision_only_invoice_image("anthropic/claude-haiku-4-5-20251001", api_key).await;
}

#[tokio::test]
async fn test_vision_only_invoice_image_gemini() {
    init();
    let api_key = require_env!("GEMINI_API_KEY");
    run_vision_only_invoice_image("gemini/gemini-2.5-flash", api_key).await;
}

// ── (c+1) OpenAI-only vision fallback: PDF + fallback_threshold=1.0 ──────────

#[tokio::test]
async fn test_vision_fallback_fires_on_pdf_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");

    let bytes = std::fs::read(FAKE_MEMO_PDF).unwrap_or_else(|e| panic!("failed to read {FAKE_MEMO_PDF}: {e}"));
    let spec = PresetSpec::Named("generic_document".into());
    let options = StructuredOptions {
        llm: make_llm_config("openai/gpt-4o-mini", api_key),
        force_call_mode: Some(StructuredCallMode::TextOnlyWithVisionFallback),
        vision: VisionConfig {
            max_output_tokens: 512,
            fallback_threshold: 1.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let output = extract_structured(&bytes, "application/pdf", spec, options)
        .await
        .expect("TextOnlyWithVisionFallback extraction must succeed");

    assert!(
        output.fallback_used,
        "fallback_used must be true when fallback_threshold=1.0"
    );
    assert!(
        output.llm_usage.len() >= 2,
        "at least 2 LLM calls expected (text-only + vision fallback), got: {}",
        output.llm_usage.len()
    );
}

// ── (d) split_and_extract on REPAIR multi-page PDF ───────────────────────────

async fn run_split_and_extract_repair(model: &str, api_key: String) {
    let bytes = std::fs::read(REPAIR_PDF).unwrap_or_else(|e| panic!("failed to read {REPAIR_PDF}: {e}"));
    let spec = PresetSpec::Named("generic_document".into());
    let options = StructuredOptions {
        llm: make_llm_config(model, api_key),
        force_call_mode: Some(StructuredCallMode::TextOnly),
        vision: VisionConfig {
            max_output_tokens: 512,
            ..Default::default()
        },
        max_parallel_calls: 2,
        ..Default::default()
    };

    let outputs = split_and_extract(&bytes, "application/pdf", spec, options)
        .await
        .expect("split_and_extract on REPAIR PDF must succeed");

    assert!(!outputs.is_empty(), "split_and_extract must return at least 1 segment");

    for (i, output) in outputs.iter().enumerate() {
        assert_eq!(
            output.preset_id, "generic_document",
            "segment {i}: preset_id must be generic_document"
        );
        assert!(
            output.structured_output_flat.is_object(),
            "segment {i}: structured_output_flat must be an object"
        );
        assert!(!output.llm_usage.is_empty(), "segment {i}: llm_usage must be non-empty");
    }
}

#[tokio::test]
async fn test_split_and_extract_repair_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");
    run_split_and_extract_repair("openai/gpt-4o-mini", api_key).await;
}

#[tokio::test]
async fn test_split_and_extract_repair_anthropic() {
    init();
    let api_key = require_env!("ANTHROPIC_API_KEY");
    run_split_and_extract_repair("anthropic/claude-haiku-4-5-20251001", api_key).await;
}

#[tokio::test]
async fn test_split_and_extract_repair_gemini() {
    init();
    let api_key = require_env!("GEMINI_API_KEY");
    run_split_and_extract_repair("gemini/gemini-2.5-flash", api_key).await;
}

// ── (e) extract_structured_json bridge (OpenAI, plain #[test]) ───────────────

#[test]
fn test_extract_structured_json_bridge_openai() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");

    let bytes = std::fs::read(FAKE_MEMO_PDF).unwrap_or_else(|e| panic!("failed to read {FAKE_MEMO_PDF}: {e}"));

    let preset_spec_json = json!({"named": "generic_document"}).to_string();
    let options_json = json!({
        "llm": {
            "model": "openai/gpt-4o-mini",
            "api_key": api_key
        },
        "force_call_mode": "text_only",
        "vision": {
            "max_output_tokens": 512
        }
    })
    .to_string();

    let result_str = extract_structured_json(&bytes, "application/pdf", &preset_spec_json, &options_json)
        .expect("extract_structured_json must succeed");

    let parsed: serde_json::Value = serde_json::from_str(&result_str).expect("result must be valid JSON");

    assert_eq!(
        parsed["preset_id"].as_str(),
        Some("generic_document"),
        "preset_id must be generic_document"
    );
    assert_eq!(
        parsed["preset_version"].as_str(),
        Some("v1"),
        "preset_version must be v1"
    );

    let title = parsed["structured_output_flat"]["title"].as_str();
    assert!(
        title.is_some() && !title.unwrap().is_empty(),
        "structured_output_flat.title must be a non-empty string, got: {:?}",
        parsed["structured_output_flat"]["title"]
    );

    let llm_usage = parsed["llm_usage"].as_array().expect("llm_usage must be an array");
    assert!(!llm_usage.is_empty(), "llm_usage must be non-empty");
}
