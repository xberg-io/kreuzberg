//! Acceleration configuration for ONNX Runtime execution providers.

use serde::{Deserialize, Serialize};

/// Hardware acceleration configuration for ONNX Runtime models.
///
/// Controls which execution provider (CPU, CoreML, CUDA, TensorRT) is used
/// for inference in layout detection and embedding generation.
///
/// # Example
///
/// ```rust
/// use kreuzberg::AccelerationConfig;
///
/// // Auto-select: CoreML on macOS, CUDA on Linux, CPU elsewhere
/// let config = AccelerationConfig::default();
///
/// // Force CPU only
/// let config = AccelerationConfig {
///     provider: kreuzberg::ExecutionProviderType::Cpu,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccelerationConfig {
    /// Execution provider to use for ONNX inference.
    #[serde(default)]
    pub provider: ExecutionProviderType,

    /// GPU device ID (for CUDA/TensorRT). Ignored for CPU/CoreML/Auto.
    #[serde(default)]
    pub device_id: u32,
}

/// ONNX Runtime execution provider type.
///
/// Determines which hardware backend is used for model inference.
/// `Auto` (default) selects the best available provider per platform.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionProviderType {
    /// Auto-select: CoreML on macOS, CUDA on Linux, CPU elsewhere.
    #[default]
    Auto,
    /// CPU execution provider (always available).
    Cpu,
    /// Apple CoreML (macOS/iOS Neural Engine + GPU).
    #[serde(alias = "coreml")]
    CoreMl,
    /// NVIDIA CUDA GPU acceleration.
    Cuda,
    /// NVIDIA TensorRT (optimized CUDA inference).
    #[serde(alias = "tensorrt")]
    TensorRt,
}
