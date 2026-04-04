use ort::session::{Session, builder::GraphOptimizationLevel};

use crate::core::config::acceleration::{AccelerationConfig, ExecutionProviderType};
use crate::layout::error::LayoutError;

/// Build an optimized ORT session from an ONNX model file.
///
/// `thread_budget` controls the number of intra-op threads for this session.
/// Pass the result of [`crate::core::config::concurrency::resolve_thread_budget`]
/// to respect the user's `ConcurrencyConfig`.
///
/// When `accel` is `None` or `Auto`, uses platform defaults:
/// - macOS: CoreML (Neural Engine / GPU)
/// - Linux: CUDA (GPU)
/// - Others: CPU only
///
/// ORT silently falls back to CPU if the requested EP is unavailable.
pub fn build_session(
    path: &str,
    accel: Option<&AccelerationConfig>,
    thread_budget: usize,
) -> Result<Session, LayoutError> {
    let builder = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)
        .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
        .with_intra_threads(thread_budget)
        .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
        .with_inter_threads(1)
        .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?;

    let provider = accel.map(|a| &a.provider).unwrap_or(&ExecutionProviderType::Auto);
    let device_id = accel.map(|a| a.device_id).unwrap_or(0);

    let mut builder = match provider {
        ExecutionProviderType::Cpu => {
            tracing::debug!("ORT session: CPU execution provider (explicit)");
            builder
        }
        ExecutionProviderType::CoreMl => {
            tracing::debug!("ORT session: CoreML execution provider requested");
            builder
                .with_execution_providers([ort::ep::CoreML::default().build()])
                .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
        }
        ExecutionProviderType::Cuda => {
            tracing::debug!(device_id, "ORT session: CUDA execution provider requested");
            builder
                .with_execution_providers([ort::ep::CUDA::default().with_device_id(device_id as i32).build()])
                .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
        }
        ExecutionProviderType::TensorRt => {
            tracing::debug!(device_id, "ORT session: TensorRT execution provider requested");
            builder
                .with_execution_providers([ort::ep::TensorRT::default().with_device_id(device_id as i32).build()])
                .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
        }
        ExecutionProviderType::Auto => {
            // Platform defaults
            #[cfg(target_os = "macos")]
            let builder = {
                tracing::debug!("ORT session: auto-selected CoreML (macOS)");
                builder
                    .with_execution_providers([ort::ep::CoreML::default().build()])
                    .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
            };
            #[cfg(target_os = "linux")]
            let builder = {
                tracing::debug!("ORT session: auto-selected CUDA (Linux)");
                builder
                    .with_execution_providers([ort::ep::CUDA::default().build()])
                    .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
            };
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            let builder = {
                tracing::debug!("ORT session: auto-selected CPU (no platform EP)");
                builder
            };
            builder
        }
    };

    let session = builder.commit_from_file(path)?;

    Ok(session)
}
