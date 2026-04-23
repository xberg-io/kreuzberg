//! PP-LCNet table classifier for wired vs wireless table detection.
//!
//! Classifies a cropped table image as either wired (bordered) or wireless
//! (borderless), routing to the appropriate SLANeXT model variant.
//!
//! Model: PP-LCNet_x1_0_table_cls
//! Input:  `x` shape `[batch, 3, 224, 224]` f32 (fixed size, ImageNet normalization)
//! Output: `fetch_name_0` shape `[batch, 2]` f32 — [wired_score, wireless_score]

use image::RgbImage;
use ndarray::Array4;
use ort::{inputs, session::Session, value::Tensor};

use crate::layout::error::LayoutError;
use crate::layout::session::build_session;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// PP-LCNet fixed input dimensions.
const INPUT_SIZE: u32 = 224;

/// ImageNet normalization mean, applied in BGR channel order.
///
/// PaddleOCR uses OpenCV (BGR) convention: B=0.485, G=0.456, R=0.406.
const IMAGENET_MEAN_BGR: [f32; 3] = [0.485, 0.456, 0.406];

/// ImageNet normalization std, applied in BGR channel order.
const IMAGENET_STD_BGR: [f32; 3] = [0.229, 0.224, 0.225];

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Table type classification result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableType {
    /// Bordered table with visible gridlines.
    Wired,
    /// Borderless table without visible gridlines.
    Wireless,
}

impl TableType {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Wired => "wired",
            Self::Wireless => "wireless",
        }
    }
}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// PP-LCNet table classifier model.
pub struct TableClassifier {
    session: Session,
    input_name: String,
}

impl TableClassifier {
    /// Load the table classifier ONNX model from a file path.
    pub(crate) fn from_file(
        path: &str,
        accel: Option<&crate::core::config::acceleration::AccelerationConfig>,
    ) -> Result<Self, LayoutError> {
        let budget = crate::core::config::concurrency::resolve_thread_budget(None);
        let session = match build_session(path, accel, budget) {
            Ok(s) => s,
            Err(first_err) => {
                tracing::warn!("TableClassifier: platform EP failed ({first_err}), retrying CPU-only");
                Self::build_cpu_session(path, budget)?
            }
        };
        let input_name = session.inputs()[0].name().to_string();
        Ok(Self { session, input_name })
    }

    fn build_cpu_session(path: &str, thread_budget: usize) -> Result<Session, LayoutError> {
        use ort::session::builder::GraphOptimizationLevel;
        let mut builder = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::All)
            .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
            .with_intra_threads(thread_budget)
            .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?
            .with_inter_threads(1)
            .map_err(|e| LayoutError::Ort(ort::Error::new(e.message())))?;
        Ok(builder.commit_from_file(path)?)
    }

    /// Classify a cropped table image as wired or wireless.
    pub(crate) fn classify(&mut self, table_img: &RgbImage) -> Result<TableType, LayoutError> {
        tracing::trace!(
            input_width = table_img.width(),
            input_height = table_img.height(),
            "TableClassifier: starting classification"
        );

        let input_tensor = preprocess_lcnet(table_img);
        let tensor = Tensor::from_array(input_tensor)?;

        let inference_start = std::time::Instant::now();
        let outputs = self.session.run(inputs![
            self.input_name.as_str() => tensor
        ])?;
        let inference_ms = inference_start.elapsed().as_secs_f64() * 1000.0;

        tracing::trace!(
            inference_ms = format!("{:.1}", inference_ms),
            "TableClassifier: inference complete"
        );

        // Output shape: [1, 2] — raw logits [wired, wireless].
        // Apply softmax to get probabilities before comparing.
        for (_name, value) in outputs.iter() {
            if let Ok(view) = value.try_extract_tensor::<f32>() {
                let data = view.1;
                if data.len() >= 2 {
                    let raw_wired = data[0];
                    let raw_wireless = data[1];

                    // Softmax: exp(x - max) / sum(exp(x - max))
                    let max_val = raw_wired.max(raw_wireless);
                    let exp_wired = (raw_wired - max_val).exp();
                    let exp_wireless = (raw_wireless - max_val).exp();
                    let sum_exp = exp_wired + exp_wireless;
                    let prob_wired = exp_wired / sum_exp;
                    let prob_wireless = exp_wireless / sum_exp;

                    let result = if prob_wired >= prob_wireless {
                        TableType::Wired
                    } else {
                        TableType::Wireless
                    };
                    tracing::debug!(
                        raw_wired = format!("{:.3}", raw_wired),
                        raw_wireless = format!("{:.3}", raw_wireless),
                        prob_wired = format!("{:.3}", prob_wired),
                        prob_wireless = format!("{:.3}", prob_wireless),
                        result = result.name(),
                        "Table classification result"
                    );
                    return Ok(result);
                }
            }
        }

        // Fallback to wireless if output parsing fails
        tracing::warn!("TableClassifier: could not parse output, defaulting to wireless");
        Ok(TableType::Wireless)
    }
}

// ---------------------------------------------------------------------------
// Preprocessing
// ---------------------------------------------------------------------------

/// Minimum edge length before center-crop.
const MIN_EDGE: u32 = 256;

/// Preprocess an image for PP-LCNet table classifier.
///
/// Matches MinerU's `paddle_table_cls.py` preprocessing exactly:
/// 1. Resize so shortest edge = 256 (aspect-preserving)
/// 2. Center-crop to 224×224
/// 3. Normalize in BGR channel order (PaddleOCR convention)
/// 4. Layout: NCHW `[1, 3, 224, 224]` f32
fn preprocess_lcnet(img: &RgbImage) -> Array4<f32> {
    let orig_w = img.width();
    let orig_h = img.height();

    // Step 1: Resize so shortest edge = 256 (aspect-preserving)
    let scale = MIN_EDGE as f32 / orig_w.min(orig_h) as f32;
    let new_w = (orig_w as f32 * scale).round().max(1.0) as u32;
    let new_h = (orig_h as f32 * scale).round().max(1.0) as u32;

    let resized = image::imageops::resize(img, new_w, new_h, image::imageops::FilterType::Triangle);

    // Step 2: Center-crop to 224×224
    let crop_x = (new_w.saturating_sub(INPUT_SIZE)) / 2;
    let crop_y = (new_h.saturating_sub(INPUT_SIZE)) / 2;
    let crop_w = INPUT_SIZE.min(new_w);
    let crop_h = INPUT_SIZE.min(new_h);
    let cropped = image::imageops::crop_imm(&resized, crop_x, crop_y, crop_w, crop_h).to_image();

    let w = INPUT_SIZE as usize;
    let h = INPUT_SIZE as usize;
    let hw = h * w;

    // Output tensor is BGR (channel 0=B, 1=G, 2=R).
    // Input pixels are RGB, so we swap R↔B in the write loop below.
    // Normalization: pixel * (scale/std) + (-mean/std)
    const INV_255: f32 = 1.0 / 255.0;
    let alpha_b = INV_255 / IMAGENET_STD_BGR[0]; // B channel: mean=0.485, std=0.229
    let alpha_g = INV_255 / IMAGENET_STD_BGR[1]; // G channel: mean=0.456, std=0.224
    let alpha_r = INV_255 / IMAGENET_STD_BGR[2]; // R channel: mean=0.406, std=0.225
    let beta_b = -IMAGENET_MEAN_BGR[0] / IMAGENET_STD_BGR[0];
    let beta_g = -IMAGENET_MEAN_BGR[1] / IMAGENET_STD_BGR[1];
    let beta_r = -IMAGENET_MEAN_BGR[2] / IMAGENET_STD_BGR[2];

    let mut data = vec![0.0f32; 3 * hw];
    let pixels = cropped.as_raw();

    // Output channel order: BGR (channel 0 = B, 1 = G, 2 = R)
    // Input pixel order: RGB (index 0 = R, 1 = G, 2 = B)
    for (i, pixel) in pixels.chunks_exact(3).enumerate() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        // Channel 0 = B
        data[i] = b * alpha_b + beta_b;
        // Channel 1 = G
        data[hw + i] = g * alpha_g + beta_g;
        // Channel 2 = R
        data[2 * hw + i] = r * alpha_r + beta_r;
    }

    Array4::from_shape_vec((1, 3, h, w), data).expect("shape mismatch in preprocess_lcnet")
}
