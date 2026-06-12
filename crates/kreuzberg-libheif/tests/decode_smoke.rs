//! Decode-side smoke tests for the vendored libheif bindings.
//!
//! Confirms the bindings still produce sensible RGBA pixel data and dimensions
//! for HEIC / HEIF / AVIF after the Rust 2024 modernisation. Requires the system
//! `libheif` (with libde265 + libaom) to be installed at runtime.
//!
//! Fixtures live in the repository-root `test_documents/` submodule. No HEIF
//! binaries are stored in the kreuzberg codebase itself.

use kreuzberg_libheif::{ColorSpace, HeifContext, LibHeif, RgbChroma};

const TEST_HEIC: &[u8] = include_bytes!("../../../test_documents/images/test.heic");
const TEST_HEIF: &[u8] = include_bytes!("../../../test_documents/images/test.heif");
const TEST_AVIF: &[u8] = include_bytes!("../../../test_documents/images/test.avif");
const TEST_HEIF_ALPHA: &[u8] = include_bytes!("../../../test_documents/images/alpha.heif");

fn decode_to_rgba(bytes: &[u8]) -> (u32, u32, usize) {
    let lib = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(bytes).expect("HeifContext::read_from_bytes");
    let handle = ctx.primary_image_handle().expect("primary_image_handle");
    let image = lib
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)
        .expect("LibHeif::decode");
    let planes = image.planes();
    let plane = planes.interleaved.expect("interleaved RGBA plane");
    (image.width(), image.height(), plane.data.len())
}

#[test]
fn decodes_heic_to_rgba() {
    let (w, h, len) = decode_to_rgba(TEST_HEIC);
    assert!(w > 0 && h > 0, "expected positive dimensions, got {w}x{h}");
    assert!(
        len >= (w as usize) * (h as usize) * 4,
        "interleaved plane shorter than declared dimensions"
    );
}

#[test]
fn decodes_heif_to_rgba() {
    let (w, h, _) = decode_to_rgba(TEST_HEIF);
    assert!(w > 0 && h > 0);
}

#[test]
fn decodes_avif_to_rgba() {
    let (w, h, _) = decode_to_rgba(TEST_AVIF);
    assert!(w > 0 && h > 0);
}

#[test]
fn decodes_heif_with_alpha_to_rgba() {
    let (w, h, _) = decode_to_rgba(TEST_HEIF_ALPHA);
    assert!(w > 0 && h > 0);
}

#[test]
fn returns_clear_error_for_non_heif_bytes() {
    let result = HeifContext::read_from_bytes(b"not a heif file");
    assert!(result.is_err(), "non-HEIF bytes should not parse");
}
