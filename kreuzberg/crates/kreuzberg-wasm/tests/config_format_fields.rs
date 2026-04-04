//! Tests for output_format and result_format fields in ExtractionConfig
//!
//! These tests verify that the TypeScript WASM bindings correctly expose
//! and handle the output_format and result_format configuration fields
//! for controlling extraction output structure and content formatting.

#![cfg(target_arch = "wasm32")]

use js_sys::Uint8Array;
use kreuzberg_wasm::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const VALID_TEXT: &[u8] = b"This is a test document with some content.\nIt has multiple lines.\nAnd some structure.";

const SIMPLE_PDF: &[u8] = b"%PDF-1.4\n\
1 0 obj\n\
<< /Type /Catalog /Pages 2 0 R >>\n\
endobj\n\
2 0 obj\n\
<< /Type /Pages /Kids [3 0 R] /Count 1 >>\n\
endobj\n\
3 0 obj\n\
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\n\
endobj\n\
4 0 obj\n\
<< /Length 44 >>\n\
stream\n\
BT\n\
/F1 12 Tf\n\
100 700 Td\n\
(Test) Tj\n\
ET\n\
endstream\n\
endobj\n\
5 0 obj\n\
<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\n\
endobj\n\
xref\n\
0 6\n\
0000000000 65535 f\n\
0000000009 00000 n\n\
0000000058 00000 n\n\
0000000115 00000 n\n\
0000000244 00000 n\n\
0000000337 00000 n\n\
trailer\n\
<< /Size 6 /Root 1 0 R >>\n\
startxref\n\
417\n\
%%EOF";

/// Test extraction with default output format (Plain)
#[wasm_bindgen_test]
fn test_extract_with_default_output_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with default output format should succeed");
}

/// Test extraction with explicit Plain output format
#[wasm_bindgen_test]
fn test_extract_with_plain_output_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "outputFormat": "plain"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with plain output format should succeed");
}

/// Test extraction with Markdown output format
#[wasm_bindgen_test]
fn test_extract_with_markdown_output_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "outputFormat": "markdown"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with markdown output format should succeed");
}

/// Test extraction with HTML output format
#[wasm_bindgen_test]
fn test_extract_with_html_output_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "outputFormat": "html"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with html output format should succeed");
}

/// Test extraction with Djot output format
#[wasm_bindgen_test]
fn test_extract_with_djot_output_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "outputFormat": "djot"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with djot output format should succeed");
}

/// Test extraction with default result format (Unified)
#[wasm_bindgen_test]
fn test_extract_with_default_result_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with default result format should succeed");
}

/// Test extraction with unified result format (explicit)
#[wasm_bindgen_test]
fn test_extract_with_unified_result_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "resultFormat": "unified"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(result.is_ok(), "Extraction with unified result format should succeed");
}

/// Test extraction with element_based result format
#[wasm_bindgen_test]
fn test_extract_with_element_based_result_format() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "resultFormat": "element_based"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(
        result.is_ok(),
        "Extraction with element_based result format should succeed"
    );
}

/// Test extraction with both outputFormat and resultFormat together
#[wasm_bindgen_test]
fn test_extract_with_both_formats() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "outputFormat": "markdown",
        "resultFormat": "unified"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(
        result.is_ok(),
        "Extraction with both output and result formats should succeed"
    );
}

/// Test extraction with element_based result format and markdown output
#[wasm_bindgen_test]
fn test_extract_with_element_format_and_markdown_output() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": false,
        "outputFormat": "markdown",
        "resultFormat": "element_based"
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(
        result.is_ok(),
        "Extraction with element format and markdown output should succeed"
    );
}

/// Test extraction with other config options and format fields
#[wasm_bindgen_test]
fn test_extract_with_complex_config() {
    let data = unsafe { Uint8Array::view(SIMPLE_PDF) };
    let config = serde_json::json!({
        "useCache": true,
        "enableQualityProcessing": true,
        "forceOcr": false,
        "outputFormat": "plain",
        "resultFormat": "unified",
        "maxConcurrentExtractions": 4
    });

    let js_config = serde_wasm_bindgen::to_value(&config).ok();
    let result = extract_bytes_sync_wasm(data, "application/pdf".to_string(), js_config);

    assert!(
        result.is_ok(),
        "Extraction with complex config including format fields should succeed"
    );
}

/// Test config loading from TOML string with outputFormat
#[wasm_bindgen_test]
fn test_load_config_from_toml_with_output_format() {
    let toml_config = r#"
use_cache = true
output_format = "markdown"
result_format = "unified"
"#;

    let result = load_config_from_string(toml_config.to_string(), "toml".to_string());
    assert!(
        result.is_ok(),
        "Config loading from TOML with format fields should succeed"
    );
}

/// Test config loading from JSON string with outputFormat
#[wasm_bindgen_test]
fn test_load_config_from_json_with_formats() {
    let json_config = r#"{"useCache": true, "outputFormat": "markdown", "resultFormat": "element_based"}"#;

    let result = load_config_from_string(json_config.to_string(), "json".to_string());
    assert!(
        result.is_ok(),
        "Config loading from JSON with format fields should succeed"
    );
}

/// Test config loading from YAML string with format fields
#[wasm_bindgen_test]
fn test_load_config_from_yaml_with_formats() {
    let yaml_config = r#"
use_cache: true
output_format: "plain"
result_format: "unified"
"#;

    let result = load_config_from_string(yaml_config.to_string(), "yaml".to_string());
    assert!(
        result.is_ok(),
        "Config loading from YAML with format fields should succeed"
    );
}
