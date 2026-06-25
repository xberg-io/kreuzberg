//! QR-code detection output types.
//!
//! Produced by the QR post-processor (`crates/xberg/src/extractors/qr.rs`) and
//! attached to
//! [`ExtractedImage::qr_codes`](super::extraction::ExtractedImage::qr_codes).

use serde::{Deserialize, Serialize};

/// One QR code decoded from an extracted image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct QrCode {
    /// Decoded payload (text, URL, vCard string, …).
    pub payload: String,
    /// Detector-reported confidence in `[0.0, 1.0]`. `None` when the decoder
    /// does not expose confidence (the default `rqrr` backend always reports
    /// `Some` because successful decode implies high confidence).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// Bounding box of the QR code inside the source image, in pixel coordinates
    /// (`x`, `y` of the top-left corner; `width`, `height` of the rectangle).
    /// `None` if the decoder did not report a bounding box.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<QrBoundingBox>,
}

/// Pixel-space bounding box of a QR code inside its source image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct QrBoundingBox {
    /// Horizontal pixel offset of the bounding box top-left corner.
    pub x: u32,
    /// Vertical pixel offset of the bounding box top-left corner.
    pub y: u32,
    /// Width of the bounding box in pixels.
    pub width: u32,
    /// Height of the bounding box in pixels.
    pub height: u32,
}
