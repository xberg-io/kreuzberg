//! Serde round-trip and wire-tag tests for [`ImageOutputFormat::Svg`].
//!
//! Mirrors `crates/xberg/src/core/config/extraction/types.rs`
//! `test_output_format_webp_wire_value_is_lowercase_webp` for the new SVG variant.
//! Also verifies `SvgOptions` serialises and round-trips via JSON.

#![cfg(feature = "svg")]

use xberg::core::config::extraction::{ImageOutputFormat, SvgOptions};

// ── ImageOutputFormat::Svg wire tag ─────────────────────────────────────────

/// The enum is tagged (`{"type": "..."}`, snake_case rename).  The `Svg`
/// variant (capital S, lower v) must serialise to `{"type":"svg"}`.
#[test]
fn svg_output_format_wire_value_is_lowercase_svg() {
    let fmt = ImageOutputFormat::Svg;
    let json = serde_json::to_string(&fmt).expect("serialisation must succeed");
    assert_eq!(json, r#"{"type":"svg"}"#);
}

/// Deserialising `{"type":"svg"}` must produce `ImageOutputFormat::Svg`.
#[test]
fn svg_output_format_deserialises_from_lowercase_type_tag() {
    let fmt: ImageOutputFormat = serde_json::from_str(r#"{"type":"svg"}"#).expect("deserialisation must succeed");
    assert_eq!(fmt, ImageOutputFormat::Svg);
}

/// Serialise → deserialise round-trip must be an identity.
#[test]
fn svg_output_format_round_trips_via_json() {
    let original = ImageOutputFormat::Svg;
    let json = serde_json::to_string(&original).expect("serialisation must succeed");
    let back: ImageOutputFormat = serde_json::from_str(&json).expect("deserialisation must succeed");
    assert_eq!(back, original);
}

// ── SvgOptions serde ─────────────────────────────────────────────────────────

/// Default `SvgOptions` must serialise to the canonical wire shape and
/// round-trip back to an equal value.
#[test]
fn svg_options_round_trips_via_json() {
    let opts = SvgOptions::default();
    let json = serde_json::to_string(&opts).expect("SvgOptions serialisation must succeed");
    let back: SvgOptions = serde_json::from_str(&json).expect("SvgOptions deserialisation must succeed");
    assert_eq!(back, opts);
}

/// `SvgOptions` with non-default fields must round-trip without loss.
#[test]
fn svg_options_non_default_values_round_trip() {
    let opts = SvgOptions {
        sanitize: false,
        render_dpi: 192.0,
    };
    let json = serde_json::to_string(&opts).expect("serialisation must succeed");
    let back: SvgOptions = serde_json::from_str(&json).expect("deserialisation must succeed");
    assert!(!back.sanitize);
    assert!(
        (back.render_dpi - 192.0_f32).abs() < f32::EPSILON,
        "render_dpi must survive round-trip"
    );
}

/// Default values for `SvgOptions` fields must match the documented defaults:
/// `sanitize = true`, `render_dpi = 96.0`.
#[test]
fn svg_options_defaults_are_documented_values() {
    let opts = SvgOptions::default();
    assert!(opts.sanitize, "sanitize must default to true");
    assert!(
        (opts.render_dpi - 96.0_f32).abs() < f32::EPSILON,
        "render_dpi must default to 96.0, got {}",
        opts.render_dpi
    );
}
