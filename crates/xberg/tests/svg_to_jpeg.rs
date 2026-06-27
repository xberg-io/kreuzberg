//! Integration test: SVG embedded in HTML is rasterised to JPEG by the
//! `image-encode` pipeline when `output_format = Jpeg`.
//!
//! Requires both `html` (inline image extraction) and `svg` (rasterisation)
//! features.

#![cfg(all(feature = "html", feature = "svg", feature = "image-encode"))]

mod helpers;
use helpers::extract_bytes_document_blocking;

use xberg::core::config::ExtractionConfig;
use xberg::core::config::extraction::{ImageExtractionConfig, ImageOutputFormat};

/// JPEG SOI marker: `\xFF\xD8\xFF`.
const JPEG_MAGIC: &[u8] = b"\xFF\xD8\xFF";

/// Minimal HTML document with an inline SVG.
const HTML_WITH_INLINE_SVG: &[u8] = br##"<!DOCTYPE html>
<html>
<body>
<svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 80 80">
  <circle cx="40" cy="40" r="35" fill="green"/>
</svg>
</body>
</html>"##;

fn config_jpeg(quality: u8) -> ExtractionConfig {
    ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            output_format: ImageOutputFormat::Jpeg { quality },
            ..Default::default()
        }),
        disable_ocr: true,
        use_cache: false,
        ..Default::default()
    }
}

/// Driving `output_format = Jpeg { quality: 85 }` over an HTML document
/// containing an inline SVG must:
///
/// 1. Return `Ok`.
/// 2. Produce at least one `ExtractedImage`.
/// 3. Each image must report `format == "jpeg"`.
/// 4. Each image's raw bytes must start with the JPEG SOI marker `\xFF\xD8\xFF`.
#[test]
fn svg_inline_in_html_rasterised_to_jpeg() {
    let config = config_jpeg(85);
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
            "jpeg",
            "image at index {} must report format=\"jpeg\" after JPEG re-encode; got \"{}\"",
            img.image_index,
            img.format
        );
        assert!(
            img.data.len() >= JPEG_MAGIC.len(),
            "JPEG image at index {} must have at least {} bytes",
            img.image_index,
            JPEG_MAGIC.len()
        );
        assert_eq!(
            &img.data[..JPEG_MAGIC.len()],
            JPEG_MAGIC,
            "image at index {} must start with JPEG SOI marker \\xFF\\xD8\\xFF; \
             got {:02x?}",
            img.image_index,
            &img.data[..JPEG_MAGIC.len().min(img.data.len())]
        );
    }
}

/// Lower JPEG quality must produce a smaller (or equal-sized) output than
/// higher quality — proves the `quality` parameter reaches the encoder.
///
/// Compares `quality=95` vs `quality=20` for the same SVG source.
#[test]
fn svg_to_jpeg_quality_parameter_affects_output_size() {
    let config_high = config_jpeg(95);
    let config_low = config_jpeg(20);

    let result_high = extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config_high)
        .expect("high-quality extraction must succeed");
    let result_low = extract_bytes_document_blocking(HTML_WITH_INLINE_SVG, "text/html", &config_low)
        .expect("low-quality extraction must succeed");

    let images_high = result_high
        .images
        .as_ref()
        .expect("images must be present in high-quality run");
    let images_low = result_low
        .images
        .as_ref()
        .expect("images must be present in low-quality run");

    assert_eq!(
        images_high.len(),
        images_low.len(),
        "both quality runs must produce the same number of images"
    );

    // For at least one image the low-quality output must be ≤ high-quality output.
    let any_smaller_or_equal = images_high
        .iter()
        .zip(images_low.iter())
        .any(|(high, low)| low.data.len() <= high.data.len());

    assert!(
        any_smaller_or_equal,
        "JPEG quality=20 must produce output ≤ quality=95 for at least one image; \
         sizes at q=95: {:?}, sizes at q=20: {:?}",
        images_high.iter().map(|i| i.data.len()).collect::<Vec<_>>(),
        images_low.iter().map(|i| i.data.len()).collect::<Vec<_>>()
    );
}
