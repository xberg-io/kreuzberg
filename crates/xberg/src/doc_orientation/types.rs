//! Pure-Rust orientation result types — available without ONNX Runtime.

/// Document orientation detection result.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct OrientationResult {
    /// Detected orientation in degrees (0, 90, 180, or 270).
    pub degrees: u32,
    /// Confidence score (0.0-1.0).
    pub confidence: f32,
}
