//! Integration test: SVG embedded in HTML is rasterised to WebP by the
//! `image-encode` pipeline when `output_format = Webp`.
//!
//! Requires both `html` (inline image extraction) and `svg` (rasterisation)
//! features.

#![cfg(all(feature = "html", feature = "svg", feature = "image-encode"))]

mod helpers;
use helpers::extract_bytes_document_blocking;

use xberg::core::config::ExtractionConfig;
use xberg::core::config::extraction::{ImageExtractionConfig, ImageOutputFormat};

/// Returns `true` when `data` starts with the RIFF/WEBP container signature.
///
/// A valid WebP file starts with `RIFF` at offset 0 and `WEBP` at offset 8.
fn is_webp(data: &[u8]) -> bool {
    data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP"
}

/// Minimal HTML document with an inline SVG.
const HTML_WITH_INLINE_SVG: &[u8] = br##"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 64 64">
  <rect x="8" y="8" width="48" height="48" rx="8" fill="#4a90d9"/>
  <text x="32" y="36" text-anchor="middle" font-size="16" fill="white">K</text>
</svg>
</body>
</html>"##;

fn config_webp(quality: u8) -> ExtractionConfig {
    ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            output_format: ImageOutputFormat::Webp { quality },
            ..Default::default()
        }),
        disable_ocr: true,
        use_cache: false,
        ..Default::default()
    }
}

/// Driving `output_format = Webp { quality: 80 }` over an HTML document
/// containing an inline SVG must:
///
/// 1. Return `Ok`.
/// 2. Produce at least one `ExtractedImage`.
/// 3. Each image must report `format == "webp"`.
/// 4. Each image's raw bytes must match the RIFF/WEBP container signature.
#[test]
fn svg_inline_in_html_rasterised_to_webp() {
    let config = config_webp(80);
    let result =
        extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config).expect("extraction must succeed");

    let images = result
        .images
        .as_ref()
        .expect("images must be Some when extract_images=true");

    assert!(
        !images.is_empty(),
        "HTML with inline SVG must yield at least one ExtractedImage; \
         processing_warnings: {:?}",
        result.processing_warnings
    );

    for img in images {
        assert_eq!(
            img.format.as_ref(),
            "webp",
            "image at index {} must report format=\"webp\" after WebP re-encode; got \"{}\"",
            img.image_index,
            img.format
        );
        assert!(
            is_webp(&img.data),
            "image at index {} must have RIFF/WEBP signature; \
             first 12 bytes: {:02x?}",
            img.image_index,
            &img.data[..12.min(img.data.len())]
        );
    }
}

/// The no-warning path: rasterising an SVG to WebP must not emit any
/// `ProcessingWarning` with `source == "image_encoder"`.
#[test]
fn svg_to_webp_emits_no_image_encoder_warnings() {
    let config = config_webp(80);
    let result =
        extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config).expect("extraction must succeed");

    let encoder_warnings: Vec<_> = result
        .processing_warnings
        .iter()
        .filter(|w| w.source.as_ref() == "image_encoder")
        .collect();

    assert!(
        encoder_warnings.is_empty(),
        "SVG → WebP rasterisation must emit no image_encoder warnings; got: {:?}",
        encoder_warnings
    );
}
