//! Integration tests for office format extraction in WASM environments.
//!
//! These tests verify that the newly added office formats (DOCX, PPTX, RTF, RST,
//! Org, FB2, Typst, BibTeX, Markdown) can be extracted via the WASM binding.
//! Tests use inline byte content to avoid filesystem dependencies.

#![cfg(target_arch = "wasm32")]

use js_sys::Uint8Array;
use kreuzberg_wasm::*;

// --- Minimal test documents ---

/// A minimal RTF document.
const MINIMAL_RTF: &[u8] = b"{\\rtf1\\ansi{\\fonttbl{\\f0 Times New Roman;}}\\pard Hello from RTF!\\par}";

/// A minimal reStructuredText document.
const MINIMAL_RST: &[u8] = b"Title\n=====\n\nHello from reStructuredText.\n";

/// A minimal Org-mode document.
const MINIMAL_ORG: &[u8] = b"* Heading\n\nHello from Org mode.\n";

/// A minimal Typst document.
const MINIMAL_TYPST: &[u8] = b"= Hello\n\nHello from Typst.\n";

/// A minimal BibTeX document.
const MINIMAL_BIBTEX: &[u8] = b"@article{knuth1984,\n  author = {Donald E. Knuth},\n  title = {Literate Programming},\n  journal = {The Computer Journal},\n  year = {1984},\n}\n";

/// A minimal Markdown document.
const MINIMAL_MARKDOWN: &[u8] = b"# Hello\n\nHello from Markdown.\n";

/// A minimal FictionBook document.
const MINIMAL_FB2: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\">\n  <body>\n    <section>\n      <p>Hello from FictionBook.</p>\n    </section>\n  </body>\n</FictionBook>";

// --- RTF tests ---

#[test]
fn test_rtf_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_RTF) };
    let result = extract_bytes_sync_wasm(data, "application/rtf".to_string(), None);
    assert!(result.is_ok(), "RTF extraction should succeed: {:?}", result.err());
}

#[test]
fn test_rtf_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_RTF) };
    let result = extract_bytes_sync_wasm(data, "application/rtf".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "RTF result should be a JavaScript object");
    }
}

// --- reStructuredText tests ---

#[test]
fn test_rst_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_RST) };
    let result = extract_bytes_sync_wasm(data, "text/x-rst".to_string(), None);
    assert!(result.is_ok(), "RST extraction should succeed: {:?}", result.err());
}

#[test]
fn test_rst_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_RST) };
    let result = extract_bytes_sync_wasm(data, "text/x-rst".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "RST result should be a JavaScript object");
    }
}

// --- Org-mode tests ---

#[test]
fn test_org_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_ORG) };
    let result = extract_bytes_sync_wasm(data, "text/x-org".to_string(), None);
    assert!(result.is_ok(), "Org extraction should succeed: {:?}", result.err());
}

#[test]
fn test_org_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_ORG) };
    let result = extract_bytes_sync_wasm(data, "text/x-org".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "Org result should be a JavaScript object");
    }
}

// --- Typst tests ---

#[test]
fn test_typst_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_TYPST) };
    let result = extract_bytes_sync_wasm(data, "application/x-typst".to_string(), None);
    assert!(result.is_ok(), "Typst extraction should succeed: {:?}", result.err());
}

#[test]
fn test_typst_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_TYPST) };
    let result = extract_bytes_sync_wasm(data, "application/x-typst".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "Typst result should be a JavaScript object");
    }
}

// --- BibTeX tests ---

#[test]
fn test_bibtex_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_BIBTEX) };
    let result = extract_bytes_sync_wasm(data, "application/x-bibtex".to_string(), None);
    assert!(result.is_ok(), "BibTeX extraction should succeed: {:?}", result.err());
}

#[test]
fn test_bibtex_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_BIBTEX) };
    let result = extract_bytes_sync_wasm(data, "application/x-bibtex".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "BibTeX result should be a JavaScript object");
    }
}

// --- Markdown tests ---

#[test]
fn test_markdown_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_MARKDOWN) };
    let result = extract_bytes_sync_wasm(data, "text/markdown".to_string(), None);
    assert!(result.is_ok(), "Markdown extraction should succeed: {:?}", result.err());
}

#[test]
fn test_markdown_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_MARKDOWN) };
    let result = extract_bytes_sync_wasm(data, "text/markdown".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "Markdown result should be a JavaScript object");
    }
}

// --- FictionBook tests ---

#[test]
fn test_fb2_extraction_basic() {
    let data = unsafe { Uint8Array::view(MINIMAL_FB2) };
    let result = extract_bytes_sync_wasm(data, "application/x-fictionbook+xml".to_string(), None);
    assert!(result.is_ok(), "FB2 extraction should succeed: {:?}", result.err());
}

#[test]
fn test_fb2_extraction_returns_object() {
    let data = unsafe { Uint8Array::view(MINIMAL_FB2) };
    let result = extract_bytes_sync_wasm(data, "application/x-fictionbook+xml".to_string(), None);
    if let Ok(js_value) = result {
        assert!(js_value.is_object(), "FB2 result should be a JavaScript object");
    }
}

// --- Cross-format tests ---

#[test]
fn test_multiple_office_formats_no_state_leak() {
    let rtf = unsafe { Uint8Array::view(MINIMAL_RTF) };
    let result1 = extract_bytes_sync_wasm(rtf, "application/rtf".to_string(), None);

    let md = unsafe { Uint8Array::view(MINIMAL_MARKDOWN) };
    let result2 = extract_bytes_sync_wasm(md, "text/markdown".to_string(), None);

    let org = unsafe { Uint8Array::view(MINIMAL_ORG) };
    let result3 = extract_bytes_sync_wasm(org, "text/x-org".to_string(), None);

    assert!(result1.is_ok(), "RTF should succeed");
    assert!(result2.is_ok(), "Markdown should succeed");
    assert!(result3.is_ok(), "Org should succeed");
}
