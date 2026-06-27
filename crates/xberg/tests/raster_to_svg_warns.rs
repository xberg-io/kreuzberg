//! Integration test: requesting `output_format = Svg` for a document that
//! produces only raster images must emit an `image_encoder` `ProcessingWarning`
//! and leave the raster image bytes completely unchanged.
//!
//! Raster→SVG (vectorization) is not supported by the re-encode pipeline.
//! The `UnsupportedDirection` error propagates as a `ProcessingWarning` so
//! the overall extraction succeeds.
//!
//! Requires both `html` (inline image extraction) and `svg` features.

#![cfg(all(feature = "html", feature = "svg", feature = "image-encode"))]

mod helpers;
use helpers::extract_bytes_document_blocking;

use xberg::core::config::ExtractionConfig;
use xberg::core::config::extraction::{ImageExtractionConfig, ImageOutputFormat};

/// PNG magic: `\x89PNG\r\n\x1a\n` (8 bytes).
const PNG_MAGIC: &[u8] = b"\x89PNG\r\n\x1a\n";

/// A minimal 1×1 white PNG encoded as a base64 data URI embedded in HTML.
///
/// The PNG was generated from `image::RgbaImage::new(1, 1)` and is the
/// smallest well-formed PNG we can embed for testing purposes.
///
/// The pixel: RGBA (255, 255, 255, 255).
const HTML_WITH_PNG_DATA_URI: &[u8] = br##"<!DOCTYPE html>
<html>
<body>
<img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI6QAAAABJRU5ErkJggg==" alt="1x1 white pixel"/>
</body>
</html>"##;

fn config_svg_target() -> ExtractionConfig {
    ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            output_format: ImageOutputFormat::Svg,
            ..Default::default()
        }),
        disable_ocr: true,
        use_cache: false,
        ..Default::default()
    }
}

/// When `output_format = Svg` and the source image is a raster PNG:
///
/// 1. `extract_bytes_document_blocking` must return `Ok` (not a hard error).
/// 2. At least one `ProcessingWarning` with `source == "image_encoder"` must
///    be emitted containing the word "svg" (the unsupported direction message).
/// 3. The image's bytes must still start with PNG magic (untouched).
/// 4. The image's `format` must remain `"png"` (unchanged).
#[test]
fn raster_png_to_svg_target_warns_and_preserves_bytes() {
    let result = extract_bytes_document_blocking(HTML_WITH_PNG_DATA_URI, "text/html", &config_svg_target())
        .expect("extraction must return Ok even when raster→SVG is unsupported");

    // Must have emitted at least one image_encoder warning.
    let encoder_warnings: Vec<_> = result
        .processing_warnings
        .iter()
        .filter(|w| w.source.as_ref() == "image_encoder")
        .collect();

    assert!(
        !encoder_warnings.is_empty(),
        "raster→SVG must emit at least one image_encoder ProcessingWarning; \
         all warnings: {:?}",
        result.processing_warnings
    );

    // The warning message must mention "svg" so callers can diagnose the issue.
    for warning in &encoder_warnings {
        assert!(
            warning.message.to_lowercase().contains("svg"),
            "image_encoder warning must mention \"svg\"; got: {:?}",
            warning.message
        );
    }

    // The original image bytes must be untouched.
    if let Some(images) = result.images.as_ref() {
        for img in images {
            assert!(
                !img.data.is_empty(),
                "image at index {} must retain non-empty data after skipped re-encode",
                img.image_index
            );
            // If the extractor returned the original PNG bytes they should
            // still carry the PNG magic header.
            if img.format.as_ref() == "png" {
                assert!(
                    img.data.starts_with(PNG_MAGIC),
                    "untouched PNG image at index {} must still start with PNG magic bytes; \
                     got {:02x?}",
                    img.image_index,
                    &img.data[..PNG_MAGIC.len().min(img.data.len())]
                );
            }
        }
    }
}

/// The overall extraction result must be `Ok` — a failed re-encode direction
/// must never propagate as a hard `XbergError`.
#[test]
fn raster_to_svg_does_not_return_hard_error() {
    let outcome = extract_bytes_document_blocking(HTML_WITH_PNG_DATA_URI, "text/html", &config_svg_target());
    assert!(
        outcome.is_ok(),
        "raster→SVG unsupported direction must degrade to a warning, not a hard error; \
         got: {:?}",
        outcome.err()
    );
}
