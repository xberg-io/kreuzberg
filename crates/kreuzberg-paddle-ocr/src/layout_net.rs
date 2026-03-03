//! PicoDet layout detection model.
//!
//! Implements PP-DocLayout-M for detecting layout regions in document images.
//! Detects 23 classes including table (class_id=8).
//! Table regions can be cropped and passed to SLANet for table structure recognition.

use crate::{base_net::BaseNet, ocr_error::OcrError};
use ndarray::Array4;
use ort::{inputs, session::Session, value::Tensor};

/// Input dimensions for PP-DocLayout-M: 640x640 square.
const INPUT_SIZE: u32 = 640;

/// ImageNet RGB normalization constants.
/// PP-DocLayout-M expects RGB channel order with ImageNet normalization:
///   output = (pixel / 255.0 - mean) / std
const MEAN_RGB: [f32; 3] = [0.485, 0.456, 0.406];
const STD_RGB: [f32; 3] = [0.229, 0.224, 0.225];

/// Default score threshold for filtering detections.
const DEFAULT_SCORE_THRESHOLD: f32 = 0.5;

/// Default IoU threshold for NMS.
const DEFAULT_NMS_THRESHOLD: f32 = 0.5;

/// A detected layout region with bounding box and class label.
#[derive(Debug, Clone)]
pub struct LayoutDetection {
    /// Bounding box in original image coordinates: [x1, y1, x2, y2].
    pub bbox: [f32; 4],
    /// Class ID.
    pub class_id: u32,
    /// Detection confidence score.
    pub score: f32,
    /// Class label string.
    pub label: &'static str,
}

/// Result of layout detection on an image.
#[derive(Debug, Clone)]
pub struct LayoutResult {
    /// Detected layout regions.
    pub detections: Vec<LayoutDetection>,
}

/// PicoDet layout detection model.
#[derive(Debug)]
pub struct LayoutNet {
    session: Option<Session>,
    input_names: Vec<String>,
    score_threshold: f32,
    nms_threshold: f32,
}

/// PP-DocLayout-M class labels (23 classes).
/// Table detection uses class_id=8 ("table").
const LAYOUT_LABELS: &[&str] = &[
    "paragraph_title", // 0
    "image",           // 1
    "text",            // 2
    "number",          // 3
    "abstract",        // 4
    "content",         // 5
    "figure_title",    // 6
    "formula",         // 7
    "table",           // 8
    "table_title",     // 9
    "reference",       // 10
    "doc_title",       // 11
    "footnote",        // 12
    "header",          // 13
    "algorithm",       // 14
    "footer",          // 15
    "seal",            // 16
    "chart_title",     // 17
    "chart",           // 18
    "formula_number",  // 19
    "header_image",    // 20
    "footer_image",    // 21
    "aside_text",      // 22
];

impl BaseNet for LayoutNet {
    fn new() -> Self {
        Self {
            session: None,
            input_names: Vec::new(),
            score_threshold: DEFAULT_SCORE_THRESHOLD,
            nms_threshold: DEFAULT_NMS_THRESHOLD,
        }
    }

    fn set_input_names(&mut self, input_names: Vec<String>) {
        self.input_names = input_names;
    }

    fn set_session(&mut self, session: Option<Session>) {
        self.session = session;
    }
}

impl LayoutNet {
    /// Detect layout regions in an image.
    ///
    /// Returns bounding boxes for all detected layout regions in original image coordinates.
    pub fn detect(&mut self, img_src: &image::RgbImage) -> Result<LayoutResult, OcrError> {
        let orig_h = img_src.height();
        let orig_w = img_src.width();

        let input_tensor = Self::preprocess(img_src);
        let image_tensor = Tensor::from_array(input_tensor)?;

        // scale_factor: [scale_y, scale_x] where scale = target_dim / orig_dim
        let scale_y = INPUT_SIZE as f32 / orig_h as f32;
        let scale_x = INPUT_SIZE as f32 / orig_w as f32;
        let scale_array = ndarray::Array2::from_shape_vec((1, 2), vec![scale_y, scale_x])
            .map_err(|e| OcrError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        let scale_tensor = Tensor::from_array(scale_array)?;

        // Run inference
        let raw_detections = {
            let session = self.session.as_mut().ok_or(OcrError::SessionNotInitialized)?;
            let outputs = session.run(inputs!["image" => image_tensor, "scale_factor" => scale_tensor])?;

            // Output 0: [num_detections, 6] with [class_id, score, x1, y1, x2, y2]
            // Output 1: [1] with detection count
            let mut output_iter = outputs.iter();
            let (_, det_output) = output_iter.next().ok_or_else(|| {
                OcrError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "LayoutNet: missing detection output tensor",
                ))
            })?;

            let (shape, raw) = det_output.try_extract_tensor::<f32>()?;
            let num_dets = *shape.first().unwrap_or(&0) as usize;
            let stride = *shape.get(1).unwrap_or(&6) as usize;

            let mut dets = Vec::with_capacity(num_dets);
            for i in 0..num_dets {
                let offset = i * stride;
                if offset + 5 < raw.len() {
                    let class_id = raw[offset] as u32;
                    let score = raw[offset + 1];
                    // The model receives scale_factor and outputs coordinates
                    // already in original image space — no rescaling needed.
                    let x1 = raw[offset + 2].clamp(0.0, orig_w as f32);
                    let y1 = raw[offset + 3].clamp(0.0, orig_h as f32);
                    let x2 = raw[offset + 4].clamp(0.0, orig_w as f32);
                    let y2 = raw[offset + 5].clamp(0.0, orig_h as f32);

                    dets.push((class_id, score, [x1, y1, x2, y2]));
                }
            }
            dets
        };

        // Filter by score threshold
        let filtered: Vec<_> = raw_detections
            .into_iter()
            .filter(|(_, score, bbox)| {
                *score >= self.score_threshold && bbox[2] > bbox[0] && bbox[3] > bbox[1]
            })
            .collect();

        // Apply NMS per class
        let kept = self.nms(&filtered);

        let detections = kept
            .into_iter()
            .map(|(class_id, score, bbox)| {
                let label = LAYOUT_LABELS
                    .get(class_id as usize)
                    .copied()
                    .unwrap_or("unknown");
                LayoutDetection {
                    bbox,
                    class_id,
                    score,
                    label,
                }
            })
            .collect();

        Ok(LayoutResult { detections })
    }

    /// Preprocess an image for PP-DocLayout-M inference.
    ///
    /// Returns tensor `[1, 3, 640, 640]` in RGB channel order with ImageNet normalization.
    fn preprocess(img_src: &image::RgbImage) -> Array4<f32> {
        // Resize to exactly INPUT_SIZE x INPUT_SIZE (no aspect ratio preservation)
        let resized = image::imageops::resize(
            img_src,
            INPUT_SIZE,
            INPUT_SIZE,
            image::imageops::FilterType::Lanczos3,
        );

        // Normalize and convert to NCHW RGB
        let mut tensor = Array4::<f32>::zeros((1, 3, INPUT_SIZE as usize, INPUT_SIZE as usize));
        for y in 0..INPUT_SIZE as usize {
            for x in 0..INPUT_SIZE as usize {
                let pixel = resized.get_pixel(x as u32, y as u32);
                let r = pixel[0] as f32 / 255.0;
                let g = pixel[1] as f32 / 255.0;
                let b = pixel[2] as f32 / 255.0;

                // RGB channel order
                tensor[[0, 0, y, x]] = (r - MEAN_RGB[0]) / STD_RGB[0];
                tensor[[0, 1, y, x]] = (g - MEAN_RGB[1]) / STD_RGB[1];
                tensor[[0, 2, y, x]] = (b - MEAN_RGB[2]) / STD_RGB[2];
            }
        }

        tensor
    }

    /// Apply non-maximum suppression per class.
    fn nms(&self, detections: &[(u32, f32, [f32; 4])]) -> Vec<(u32, f32, [f32; 4])> {
        if detections.is_empty() {
            return Vec::new();
        }

        // Sort by score descending
        let mut sorted: Vec<_> = detections.to_vec();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut kept = Vec::new();
        let mut suppressed = vec![false; sorted.len()];

        for i in 0..sorted.len() {
            if suppressed[i] {
                continue;
            }
            kept.push(sorted[i]);

            for j in (i + 1)..sorted.len() {
                if suppressed[j] || sorted[j].0 != sorted[i].0 {
                    continue;
                }
                if iou(&sorted[i].2, &sorted[j].2) > self.nms_threshold {
                    suppressed[j] = true;
                }
            }
        }

        kept
    }
}

/// Compute intersection-over-union of two axis-aligned bounding boxes.
fn iou(a: &[f32; 4], b: &[f32; 4]) -> f32 {
    let x1 = a[0].max(b[0]);
    let y1 = a[1].max(b[1]);
    let x2 = a[2].min(b[2]);
    let y2 = a[3].min(b[3]);

    let inter = (x2 - x1).max(0.0) * (y2 - y1).max(0.0);
    let area_a = (a[2] - a[0]) * (a[3] - a[1]);
    let area_b = (b[2] - b[0]) * (b[3] - b[1]);
    let union = area_a + area_b - inter;

    if union <= 0.0 {
        0.0
    } else {
        inter / union
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_shape() {
        let img = image::RgbImage::new(200, 100);
        let tensor = LayoutNet::preprocess(&img);
        assert_eq!(tensor.shape(), &[1, 3, 640, 640]);
    }

    #[test]
    fn test_preprocess_normalization() {
        // White image (255, 255, 255)
        let img = image::RgbImage::from_pixel(10, 10, image::Rgb([255, 255, 255]));
        let tensor = LayoutNet::preprocess(&img);

        // For white pixel: (1.0 - mean) / std, RGB order
        let expected_r = (1.0 - MEAN_RGB[0]) / STD_RGB[0];
        let expected_g = (1.0 - MEAN_RGB[1]) / STD_RGB[1];
        let expected_b = (1.0 - MEAN_RGB[2]) / STD_RGB[2];

        assert!((tensor[[0, 0, 0, 0]] - expected_r).abs() < 0.01);
        assert!((tensor[[0, 1, 0, 0]] - expected_g).abs() < 0.01);
        assert!((tensor[[0, 2, 0, 0]] - expected_b).abs() < 0.01);
    }

    #[test]
    fn test_iou_identical() {
        let a = [10.0, 10.0, 50.0, 50.0];
        assert!((iou(&a, &a) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_iou_no_overlap() {
        let a = [0.0, 0.0, 10.0, 10.0];
        let b = [20.0, 20.0, 30.0, 30.0];
        assert!((iou(&a, &b)).abs() < 0.001);
    }

    #[test]
    fn test_iou_partial() {
        let a = [0.0, 0.0, 20.0, 20.0];
        let b = [10.0, 10.0, 30.0, 30.0];
        // Intersection: 10x10 = 100, Union: 400 + 400 - 100 = 700
        assert!((iou(&a, &b) - 100.0 / 700.0).abs() < 0.001);
    }

    #[test]
    fn test_nms_filters_overlapping() {
        let net = LayoutNet::new();
        let dets = vec![
            (0, 0.9, [10.0, 10.0, 50.0, 50.0]),
            (0, 0.7, [12.0, 12.0, 52.0, 52.0]), // overlaps heavily with first
            (0, 0.6, [200.0, 200.0, 300.0, 300.0]), // no overlap
        ];
        let kept = net.nms(&dets);
        assert_eq!(kept.len(), 2);
        assert!((kept[0].1 - 0.9).abs() < 0.001);
        assert!((kept[1].1 - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_layout_labels_count() {
        assert_eq!(LAYOUT_LABELS.len(), 23);
        assert_eq!(LAYOUT_LABELS[8], "table");
    }

    #[test]
    fn test_layout_result_empty() {
        let result = LayoutResult {
            detections: Vec::new(),
        };
        assert!(result.detections.is_empty());
    }
}
