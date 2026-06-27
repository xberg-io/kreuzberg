//! Integration test: SVG embedded in HTML is rasterised to PNG by the
//! `image-encode` pipeline when `output_format = Png`.
//!
//! The HTML extractor captures inline `<svg>` elements as `ExtractedImage`
//! entries with `format = "svg"`.  The re-encode pass then rasterises them
//! through `resvg` + `tiny-skia` and replaces the bytes with the PNG output.
//!
//! Requires both `html` (inline image extraction) and `svg` (rasterisation)
//! features.

#![cfg(all(feature = "html", feature = "svg", feature = "image-encode"))]

mod helpers;
use helpers::extract_bytes_document_blocking;

use xberg::core::config::ExtractionConfig;
use xberg::core::config::extraction::{ImageExtractionConfig, ImageOutputFormat};

/// PNG magic: `\x89PNG\r\n\x1a\n` (8 bytes).
const PNG_MAGIC: &[u8] = b"\x89PNG\r\n\x1a\n";

/// Minimal SVG with a gradient fill and a rect.  Declares explicit viewBox
/// `0 0 100 100` so the rasteriser allocates a well-defined pixel buffer.
///
/// Embedded inline inside an HTML document so the HTML extractor picks it up
/// as `InlineImageFormat::Svg`.
const HTML_WITH_INLINE_SVG: &[u8] = br##"<!DOCTYPE html>
<html>
<body>
<p>Document with an inline SVG below.</p>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
  <defs>
    <linearGradient id="g">
      <stop offset="0%" stop-color="red"/>
      <stop offset="100%" stop-color="blue"/>
    </linearGradient>
  </defs>
  <rect width="100" height="100" fill="url(#g)"/>
  <path d="M10,10 L90,90" stroke="white" stroke-width="2"/>
</svg>
</body>
</html>"##;

fn config_with_output_format(output_format: ImageOutputFormat) -> ExtractionConfig {
    ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            output_format,
            ..Default::default()
        }),
        disable_ocr: true,
        use_cache: false,
        ..Default::default()
    }
}

/// Driving `output_format = Png` over an HTML document containing an inline
/// SVG must:
///
/// 1. Return `Ok`.
/// 2. Produce at least one `ExtractedImage`.
/// 3. Each image must report `format == "png"`.
/// 4. Each image's raw bytes must start with the PNG magic signature.
#[test]
fn svg_inline_in_html_rasterised_to_png() {
    let config = config_with_output_format(ImageOutputFormat::Png);
    let result =
        extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config).expect("extraction must succeed");

    let images = result
        .images
        .as_ref()
        .expect("images must be Some when extract_images=true");

    assert!(
        !images.is_empty(),
        "HTML with inline SVG must yield at least one ExtractedImage; \
         got 0. processing_warnings: {:?}",
        result.processing_warnings
    );

    for img in images {
        assert_eq!(
            img.format.as_ref(),
            "png",
            "image at index {} must report format=\"png\" after PNG re-encode; got \"{}\"",
            img.image_index,
            img.format
        );
        assert!(
            img.data.starts_with(PNG_MAGIC),
            "image at index {} must start with PNG magic bytes \
             (first {} bytes: {:02x?})",
            img.image_index,
            PNG_MAGIC.len().min(img.data.len()),
            &img.data[..PNG_MAGIC.len().min(img.data.len())]
        );
        assert!(
            !img.data.is_empty(),
            "re-encoded PNG image at index {} must have non-empty data",
            img.image_index
        );
    }
}

/// Re-encoding the same SVG twice must produce identical bytes (deterministic
/// rasterisation).
#[test]
fn svg_to_png_rasterisation_is_deterministic() {
    let config = config_with_output_format(ImageOutputFormat::Png);

    let result1 = extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config)
        .expect("first extraction must succeed");
    let result2 = extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config)
        .expect("second extraction must succeed");

    let images1 = result1.images.as_ref().expect("images must be present in first run");
    let images2 = result2.images.as_ref().expect("images must be present in second run");

    assert_eq!(
        images1.len(),
        images2.len(),
        "both runs must produce the same number of images"
    );

    for (i, (img1, img2)) in images1.iter().zip(images2.iter()).enumerate() {
        assert_eq!(
            img1.data.len(),
            img2.data.len(),
            "image at index {i}: byte length must be identical across two runs"
        );
        assert_eq!(
            img1.data, img2.data,
            "image at index {i}: bytes must be identical across two runs (non-deterministic rasterisation)"
        );
    }
}
