//! Integration test: SVG sanitization pass strips XSS payloads while
//! preserving safe structure.
//!
//! When `ImageExtractionConfig.svg.sanitize = true` (the default) and
//! `output_format = Native`, the pipeline passes SVG images through `usvg`
//! re-serialisation.  This strips:
//!   - `<script>` elements (JS event handlers)
//!   - External `href`/`xlink:href` image references (SSRF probe vectors)
//!   - `foreignObject` elements (HTML injection)
//!
//! Safe elements (rects, paths, circles) must survive the sanitization pass.
//!
//! Requires both `html` (inline image extraction) and `svg` (sanitisation)
//! features.

#![cfg(all(feature = "html", feature = "svg", feature = "image-encode"))]

mod helpers;
use helpers::extract_bytes_document_blocking;

use xberg::core::config::ExtractionConfig;
use xberg::core::config::extraction::{ImageExtractionConfig, ImageOutputFormat, SvgOptions};

/// HTML document containing an inline SVG with XSS payloads:
///
/// - `<script>alert(1)</script>` — should be stripped.
/// - `<image href="https://attacker.example/probe">` — external image ref,
///   should be stripped or emptied by the resolver that returns `None`.
/// - A `<rect>` that must survive (safe structure).
const HTML_WITH_XSS_SVG: &[u8] = br##"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     width="100" height="100" viewBox="0 0 100 100">
  <script>alert(1)</script>
  <image href="https://attacker.example/probe" width="100" height="100"/>
  <rect x="10" y="10" width="80" height="80" fill="steelblue"/>
</svg>
</body>
</html>"##;

/// Build a config that enables image extraction with sanitize=true and
/// `output_format = Native`.  This triggers the SVG sanitize pass inside
/// `apply_output_format_pass` when the `svg` feature is active.
fn sanitize_config() -> ExtractionConfig {
    ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            output_format: ImageOutputFormat::Native,
            svg: SvgOptions {
                sanitize: true,
                render_dpi: 96.0,
            },
            ..Default::default()
        }),
        disable_ocr: true,
        use_cache: false,
        ..Default::default()
    }
}

/// Build a config with sanitize=false so we can verify the XSS content would
/// otherwise be present (i.e., the test is meaningful and not vacuous).
fn no_sanitize_config() -> ExtractionConfig {
    ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            output_format: ImageOutputFormat::Native,
            svg: SvgOptions {
                sanitize: false,
                render_dpi: 96.0,
            },
            ..Default::default()
        }),
        disable_ocr: true,
        use_cache: false,
        ..Default::default()
    }
}

/// Sanitize pass must strip `<script>` and the external image `href` while
/// leaving the output format as `"svg"`.
#[test]
fn svg_sanitize_strips_script_and_external_href() {
    let result = extract_bytes_document_blocking(HTML_WITH_XSS_SVG, "text/html", &sanitize_config())
        .expect("extraction must succeed");

    let images = result
        .images
        .as_ref()
        .expect("images must be Some when extract_images=true");

    assert!(
        !images.is_empty(),
        "HTML with inline SVG must yield at least one ExtractedImage"
    );

    for img in images {
        assert_eq!(
            img.format.as_ref(),
            "svg",
            "sanitize pass must leave format as \"svg\"; got \"{}\"",
            img.format
        );

        let svg_text = std::str::from_utf8(&img.data).expect("sanitized SVG output must be valid UTF-8");

        assert!(
            !svg_text.contains("<script"),
            "sanitized SVG must not contain <script; got:\n{}",
            svg_text
        );
        assert!(
            !svg_text.contains("attacker.example"),
            "sanitized SVG must not contain the external attacker.example href; got:\n{}",
            svg_text
        );
    }
}

/// Without sanitization (`sanitize=false`), the pipeline is a no-op for
/// `Native` output, so the image bytes are returned untouched.
///
/// This is a control test: if it fails the raw fixture doesn't contain the
/// XSS payload to begin with, making `svg_sanitize_strips_script_and_external_href`
/// vacuous.
#[test]
fn svg_without_sanitize_preserves_original_bytes_on_native_target() {
    let result = extract_bytes_document_blocking(HTML_WITH_XSS_SVG, "text/html", &no_sanitize_config())
        .expect("extraction must succeed");

    let images = result
        .images
        .as_ref()
        .expect("images must be Some when extract_images=true");

    assert!(
        !images.is_empty(),
        "HTML with inline SVG must yield at least one ExtractedImage"
    );

    for img in images {
        assert_eq!(
            img.format.as_ref(),
            "svg",
            "format must stay \"svg\" in no-sanitize pass-through mode"
        );
        // Without sanitization and Native target the pipeline must not modify bytes.
        // The original SVG contains "script" — verify it is still present.
        let svg_text = std::str::from_utf8(&img.data).expect("original SVG must be valid UTF-8");
        assert!(
            svg_text.contains("script") || svg_text.contains("alert"),
            "unsanitized SVG must still contain the original script payload; \
             the test fixture may have been processed unexpectedly. svg:\n{}",
            svg_text
        );
    }
}

/// After sanitization the output must be non-empty and parseable as UTF-8 XML.
/// The SVG root element marker `<svg` must survive.
#[test]
fn svg_sanitize_output_is_valid_xml_structure() {
    let result = extract_bytes_document_blocking(HTML_WITH_XSS_SVG, "text/html", &sanitize_config())
        .expect("extraction must succeed");

    let images = result
        .images
        .as_ref()
        .expect("images must be Some when extract_images=true");

    for img in images {
        assert!(!img.data.is_empty(), "sanitized SVG must not be empty");
        let svg_text = std::str::from_utf8(&img.data).expect("sanitized SVG must be valid UTF-8");
        assert!(
            svg_text.contains("<svg"),
            "sanitized SVG must retain the <svg root element; got:\n{}",
            svg_text
        );
    }
}
