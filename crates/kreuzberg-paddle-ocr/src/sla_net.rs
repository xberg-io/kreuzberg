//! SLANet table structure recognition model.
//!
//! Implements PaddleOCR's SLANet-plus for detecting table structures in images.
//! Takes an image as input and outputs HTML structure tokens with cell bounding boxes.

use crate::{base_net::BaseNet, ocr_error::OcrError, ocr_utils::OcrUtils};
use ndarray::{Array4, s};
use ort::{inputs, session::Session, value::Tensor};

/// Maximum input dimension for SLANet preprocessing.
const MAX_LEN: u32 = 488;

/// ImageNet normalization — same constants as DbNet.
const MEAN_VALUES: [f32; 3] = [0.485_f32 * 255_f32, 0.456_f32 * 255_f32, 0.406_f32 * 255_f32];
const NORM_VALUES: [f32; 3] = [
    1.0_f32 / 0.229_f32 / 255.0_f32,
    1.0_f32 / 0.224_f32 / 255.0_f32,
    1.0_f32 / 0.225_f32 / 255.0_f32,
];

/// Tokens that represent table cell positions (where bbox predictions apply).
const TD_TOKENS: &[&str] = &["<td>", "<td", "<td></td>"];

/// Result of SLANet table structure recognition.
#[derive(Debug, Clone)]
pub struct TableStructureResult {
    /// HTML structure tokens (e.g., `["<html>", "<body>", "<table>", "<tr>", "<td>", ...]`).
    pub html_tokens: Vec<String>,
    /// Cell bounding boxes in original image coordinates `[x1, y1, x2, y2, x3, y3, x4, y4]`.
    /// One per `<td>` token.
    pub cell_bboxes: Vec<[f32; 8]>,
    /// Mean confidence score of structure predictions.
    pub confidence: f32,
}

/// SLANet-plus table structure recognition model.
#[derive(Debug)]
pub struct SlaNet {
    session: Option<Session>,
    input_names: Vec<String>,
    /// Vocabulary with "sos" prepended and "eos" appended.
    vocabulary: Vec<String>,
    /// Index of "eos" in vocabulary.
    eos_index: usize,
    /// Index of "sos" in vocabulary.
    sos_index: usize,
}

impl BaseNet for SlaNet {
    fn new() -> Self {
        Self {
            session: None,
            input_names: Vec::new(),
            vocabulary: Vec::new(),
            eos_index: 0,
            sos_index: 0,
        }
    }

    fn set_input_names(&mut self, input_names: Vec<String>) {
        self.input_names = input_names;
    }

    fn set_session(&mut self, session: Option<Session>) {
        self.session = session;
    }
}

impl SlaNet {
    /// Initialize the model and load vocabulary from ONNX metadata.
    pub fn init_model(
        &mut self,
        path: &str,
        num_thread: usize,
        builder_fn: Option<fn(ort::session::builder::SessionBuilder) -> Result<ort::session::builder::SessionBuilder, ort::Error>>,
    ) -> Result<(), OcrError> {
        BaseNet::init_model(self, path, num_thread, builder_fn)?;
        self.load_vocabulary()?;
        Ok(())
    }

    /// Load vocabulary from ONNX model metadata.
    fn load_vocabulary(&mut self) -> Result<(), OcrError> {
        let session = self.session.as_ref().ok_or(OcrError::SessionNotInitialized)?;

        let metadata = session.metadata()?;
        let char_str = metadata.custom("character").ok_or_else(|| {
            OcrError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "SLANet character vocabulary not found in model metadata",
            ))
        })?;

        let mut dict: Vec<String> = char_str.split('\n').map(|s| s.to_string()).collect();

        // merge_no_span_structure: ensure "<td></td>" exists, remove standalone "<td>"
        if !dict.iter().any(|s| s == "<td></td>") {
            dict.push("<td></td>".to_string());
        }
        dict.retain(|s| s != "<td>");

        // Prepend "sos", append "eos"
        dict.insert(0, "sos".to_string());
        dict.push("eos".to_string());

        self.sos_index = 0;
        self.eos_index = dict.len() - 1;
        self.vocabulary = dict;

        Ok(())
    }

    /// Detect table structure in an image.
    ///
    /// Returns `None` if no table structure is found (all tokens are eos).
    pub fn detect(&mut self, img_src: &image::RgbImage) -> Result<Option<TableStructureResult>, OcrError> {
        let orig_h = img_src.height();
        let orig_w = img_src.width();

        // Preprocessing: resize → normalize → pad → CHW
        let (input_tensor, ratio) = Self::preprocess(img_src);
        let tensor = Tensor::from_array(input_tensor)?;

        // Run inference — extract all data we need, then drop session borrow
        let (bbox_data, struct_data, seq_len, vocab_size, bbox_dim) = {
            let session = self.session.as_mut().ok_or(OcrError::SessionNotInitialized)?;
            let input_name = self.input_names.first().cloned().ok_or_else(|| {
                OcrError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "SLANet: no input names configured",
                ))
            })?;

            let outputs = session.run(inputs![input_name => tensor])?;

            let mut output_iter = outputs.iter();
            let (_, bbox_output) = output_iter.next().ok_or_else(|| {
                OcrError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "SLANet: missing bbox output tensor",
                ))
            })?;
            let (_, structure_output) = output_iter.next().ok_or_else(|| {
                OcrError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "SLANet: missing structure output tensor",
                ))
            })?;

            let (bbox_shape, bbox_raw) = bbox_output.try_extract_tensor::<f32>()?;
            let (struct_shape, struct_raw) = structure_output.try_extract_tensor::<f32>()?;

            (
                bbox_raw.to_vec(),
                struct_raw.to_vec(),
                *struct_shape.get(1).unwrap_or(&0) as usize,
                *struct_shape.get(2).unwrap_or(&0) as usize,
                *bbox_shape.get(2).unwrap_or(&0) as usize,
            )
        };

        // Decode structure tokens and extract cell bboxes
        let result = self.decode(
            &struct_data, &bbox_data,
            seq_len, vocab_size, bbox_dim,
            orig_h, orig_w, ratio,
        );

        // Only return HTML wrapper tokens means no real structure found
        if result.html_tokens.len() <= 6 {
            return Ok(None);
        }

        Ok(Some(result))
    }

    /// Preprocess an image for SLANet inference.
    ///
    /// Returns `(tensor [1, 3, 488, 488], resize_ratio)`.
    fn preprocess(img_src: &image::RgbImage) -> (Array4<f32>, f32) {
        let h = img_src.height();
        let w = img_src.width();
        let ratio = MAX_LEN as f32 / (h.max(w) as f32);
        let new_h = (h as f32 * ratio) as u32;
        let new_w = (w as f32 * ratio) as u32;

        // Resize maintaining aspect ratio
        let resized = image::imageops::resize(
            img_src,
            new_w,
            new_h,
            image::imageops::FilterType::Triangle,
        );

        // Normalize and pad to MAX_LEN x MAX_LEN, in CHW format
        // SLANet uses: (pixel / 255.0 - mean) / std
        // Which is equivalent to: pixel * (1/(255*std)) - mean/std
        // But we use the same OcrUtils::substract_mean_normalize pattern as DbNet:
        // data = pixel * norm_val - mean_val * norm_val
        // where mean_val = mean * 255, norm_val = 1/(std * 255)
        let normalized = OcrUtils::substract_mean_normalize(&resized, &MEAN_VALUES, &NORM_VALUES);

        // Pad: create 488x488 zero-filled tensor, copy normalized into top-left
        let mut padded = Array4::<f32>::zeros((1, 3, MAX_LEN as usize, MAX_LEN as usize));
        padded
            .slice_mut(s![.., .., ..new_h as usize, ..new_w as usize])
            .assign(&normalized.slice(s![.., .., .., ..]));

        (padded, ratio)
    }

    /// Decode SLANet outputs into structure tokens and cell bounding boxes.
    fn decode(
        &self,
        struct_data: &[f32],
        bbox_data: &[f32],
        seq_len: usize,
        vocab_size: usize,
        bbox_dim: usize,
        orig_h: u32,
        orig_w: u32,
        ratio: f32,
    ) -> TableStructureResult {
        let mut structure_list = Vec::new();
        let mut bbox_list: Vec<[f32; 8]> = Vec::new();
        let mut score_sum = 0.0_f32;
        let mut score_count = 0usize;

        for idx in 0..seq_len {
            // argmax over vocab dimension
            let start = idx * vocab_size;
            let end = start + vocab_size;
            let slice = &struct_data[start..end.min(struct_data.len())];

            let (max_index, max_value) =
                slice
                    .iter()
                    .enumerate()
                    .fold((0, f32::MIN), |(max_idx, max_val), (i, &val)| {
                        if val > max_val { (i, val) } else { (max_idx, max_val) }
                    });

            // Stop at eos
            if idx > 0 && max_index == self.eos_index {
                break;
            }

            // Skip sos and eos tokens
            if max_index == self.sos_index || max_index == self.eos_index {
                continue;
            }

            if max_index >= self.vocabulary.len() {
                continue;
            }

            let text = &self.vocabulary[max_index];

            // Extract bbox for td tokens
            if TD_TOKENS.iter().any(|t| text.starts_with(t)) {
                let bbox_start = idx * bbox_dim;
                let bbox_end = bbox_start + bbox_dim;
                if bbox_end <= bbox_data.len() && bbox_dim >= 8 {
                    let raw_bbox = &bbox_data[bbox_start..bbox_end];
                    let bbox = self.decode_bbox(raw_bbox, orig_h, orig_w, ratio);
                    bbox_list.push(bbox);
                }
            }

            structure_list.push(text.clone());
            score_sum += max_value;
            score_count += 1;
        }

        // Wrap with HTML structure
        let mut html_tokens = vec![
            "<html>".to_string(),
            "<body>".to_string(),
            "<table>".to_string(),
        ];
        html_tokens.extend(structure_list);
        html_tokens.push("</table>".to_string());
        html_tokens.push("</body>".to_string());
        html_tokens.push("</html>".to_string());

        // Filter out zero bboxes
        bbox_list.retain(|bbox| !bbox.iter().all(|&v| v == 0.0));

        let confidence = if score_count > 0 {
            score_sum / score_count as f32
        } else {
            0.0
        };

        TableStructureResult {
            html_tokens,
            cell_bboxes: bbox_list,
            confidence,
        }
    }

    /// Decode and denormalize a single cell bounding box.
    ///
    /// SLANet outputs 8 values per cell (4 corner points: x1,y1,x2,y2,x3,y3,x4,y4)
    /// normalized to [0,1] range. Following PaddleOCR's `TableLabelDecode`:
    ///   x_coords *= original_width
    ///   y_coords *= original_height
    fn decode_bbox(&self, raw: &[f32], orig_h: u32, orig_w: u32, _ratio: f32) -> [f32; 8] {
        let h = orig_h as f32;
        let w = orig_w as f32;

        let mut bbox = [0.0_f32; 8];
        for i in 0..8 {
            if i < raw.len() {
                if i % 2 == 0 {
                    bbox[i] = raw[i] * w;
                } else {
                    bbox[i] = raw[i] * h;
                }
            }
        }
        bbox
    }
}

/// Convert SLANet HTML tokens and cell bboxes into a 2D table cell grid.
///
/// Uses OCR text blocks to fill cell content by spatial overlap with cell bounding boxes.
pub fn html_to_table_cells(
    html_tokens: &[String],
    cell_bboxes: &[[f32; 8]],
    text_blocks: &[(String, f32, f32)], // (text, center_x, center_y)
) -> Vec<Vec<String>> {
    // Parse HTML tokens to determine table grid structure
    let mut rows: Vec<Vec<usize>> = Vec::new(); // rows[r] = vec of cell_bbox indices
    let mut current_row: Vec<usize> = Vec::new();
    let mut cell_idx = 0;

    for token in html_tokens {
        match token.as_str() {
            "<tr>" => {
                current_row = Vec::new();
            }
            "</tr>" => {
                rows.push(current_row.clone());
            }
            "<td></td>" | "<td>" => {
                if cell_idx < cell_bboxes.len() {
                    current_row.push(cell_idx);
                    cell_idx += 1;
                }
            }
            t if t.starts_with("<td") && !t.starts_with("<td>") && !t.starts_with("<td>") => {
                // Handle <td with colspan/rowspan attributes
                // The ">" token follows separately
                if cell_idx < cell_bboxes.len() {
                    current_row.push(cell_idx);
                    cell_idx += 1;
                }
            }
            _ => {}
        }
    }

    // Pre-compute axis-aligned bounding rects for each cell
    let cell_rects: Vec<(f32, f32, f32, f32)> = cell_bboxes
        .iter()
        .map(|bbox| {
            let min_x = bbox[0].min(bbox[2]).min(bbox[4]).min(bbox[6]);
            let max_x = bbox[0].max(bbox[2]).max(bbox[4]).max(bbox[6]);
            let min_y = bbox[1].min(bbox[3]).min(bbox[5]).min(bbox[7]);
            let max_y = bbox[1].max(bbox[3]).max(bbox[5]).max(bbox[7]);
            (min_x, max_x, min_y, max_y)
        })
        .collect();

    // Map text blocks into cells in two passes:
    // Pass 1: Exact containment — text center inside cell bbox (closest center wins)
    // Pass 2: For unmapped text, allow downward Y tolerance only — catches text
    //         just below tight cell bboxes (common with padded SLANet input)
    //         without pulling in text from above (header info leaking into table cells)
    let mut cell_texts: Vec<Vec<String>> = vec![Vec::new(); cell_bboxes.len()];
    let mut unmapped: Vec<usize> = Vec::new();

    for (ti, (text, cx, cy)) in text_blocks.iter().enumerate() {
        let mut best_idx = None;
        let mut best_dist = f32::MAX;
        for (i, &(min_x, max_x, min_y, max_y)) in cell_rects.iter().enumerate() {
            if *cx >= min_x && *cx <= max_x && *cy >= min_y && *cy <= max_y {
                let cell_cx = (min_x + max_x) / 2.0;
                let cell_cy = (min_y + max_y) / 2.0;
                let dist = (*cx - cell_cx).powi(2) + (*cy - cell_cy).powi(2);
                if dist < best_dist {
                    best_dist = dist;
                    best_idx = Some(i);
                }
            }
        }
        if let Some(idx) = best_idx {
            cell_texts[idx].push(text.clone());
        } else {
            unmapped.push(ti);
        }
    }

    // Pass 2: For unmapped text below cell bboxes, find nearest cell by Y.
    // Only allow text that is BELOW max_y (not above min_y) to avoid pulling
    // header text down into table body cells.
    for &ti in &unmapped {
        let (text, cx, cy) = &text_blocks[ti];
        let mut best_idx = None;
        let mut best_dist = f32::MAX;
        for (i, &(min_x, max_x, _min_y, max_y)) in cell_rects.iter().enumerate() {
            if *cx >= min_x && *cx <= max_x && *cy > max_y {
                let y_dist = *cy - max_y;
                let cell_h = (max_y - _min_y).max(1.0);
                if y_dist <= cell_h && y_dist < best_dist {
                    best_dist = y_dist;
                    best_idx = Some(i);
                }
            }
        }
        if let Some(idx) = best_idx {
            cell_texts[idx].push(text.clone());
        }
    }

    // Build the 2D grid
    let mut result: Vec<Vec<String>> = Vec::new();
    for row_indices in &rows {
        let mut row: Vec<String> = Vec::new();
        for &ci in row_indices {
            if ci < cell_texts.len() {
                row.push(cell_texts[ci].join(" "));
            } else {
                row.push(String::new());
            }
        }
        result.push(row);
    }

    result
}

/// Convert SLANet HTML tokens and cell bboxes into a 2D table cell grid using
/// IoU-based matching (following PaddleOCR's `matcher.py` reference implementation).
///
/// Each OCR text box `(text, [x1, y1, x2, y2])` is matched to the SLANet cell
/// with the highest IoU (tiebroken by L1 distance). OCR boxes whose bottom edge
/// is above the top of all cell bboxes are filtered out (table title removal).
pub fn html_to_table_cells_iou(
    html_tokens: &[String],
    cell_bboxes: &[[f32; 8]],
    ocr_boxes: &[(String, [f32; 4])], // (text, [x_min, y_min, x_max, y_max])
) -> Vec<Vec<String>> {
    // Parse HTML tokens to determine table grid structure
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut current_row: Vec<usize> = Vec::new();
    let mut cell_idx = 0;

    for token in html_tokens {
        match token.as_str() {
            "<tr>" => {
                current_row = Vec::new();
            }
            "</tr>" => {
                rows.push(current_row.clone());
            }
            "<td></td>" | "<td>" => {
                if cell_idx < cell_bboxes.len() {
                    current_row.push(cell_idx);
                    cell_idx += 1;
                }
            }
            t if t.starts_with("<td") && !t.starts_with("<td>") => {
                if cell_idx < cell_bboxes.len() {
                    current_row.push(cell_idx);
                    cell_idx += 1;
                }
            }
            _ => {}
        }
    }

    // Convert 8-point cell bboxes to axis-aligned [x1, y1, x2, y2]
    let cell_rects: Vec<[f32; 4]> = cell_bboxes
        .iter()
        .map(|bbox| {
            let x1 = bbox[0].min(bbox[2]).min(bbox[4]).min(bbox[6]);
            let y1 = bbox[1].min(bbox[3]).min(bbox[5]).min(bbox[7]);
            let x2 = bbox[0].max(bbox[2]).max(bbox[4]).max(bbox[6]);
            let y2 = bbox[1].max(bbox[3]).max(bbox[5]).max(bbox[7]);
            [x1, y1, x2, y2]
        })
        .collect();

    // Filter OCR boxes: remove those whose bottom edge is above the topmost
    // cell bbox (table titles/headers above the table structure).
    let table_top_y = cell_rects
        .iter()
        .map(|r| r[1])
        .fold(f32::MAX, f32::min);

    let filtered_ocr: Vec<&(String, [f32; 4])> = ocr_boxes
        .iter()
        .filter(|(_, bbox)| bbox[3] >= table_top_y) // bottom edge >= table top
        .collect();

    // Match each OCR box to the best SLANet cell by IoU (tiebroken by L1 distance).
    // Following PaddleOCR's matcher.py: sort by (1-IoU, L1_distance).
    let mut cell_texts: Vec<Vec<String>> = vec![Vec::new(); cell_bboxes.len()];

    for (text, ocr_bbox) in &filtered_ocr {
        let mut best_idx = None;
        let mut best_iou_inv = f32::MAX;
        let mut best_l1 = f32::MAX;

        for (i, cell_rect) in cell_rects.iter().enumerate() {
            let iou = compute_iou(ocr_bbox, cell_rect);
            let l1 = compute_l1(ocr_bbox, cell_rect);
            let iou_inv = 1.0 - iou;

            if iou_inv < best_iou_inv || (iou_inv == best_iou_inv && l1 < best_l1) {
                best_iou_inv = iou_inv;
                best_l1 = l1;
                best_idx = Some(i);
            }
        }

        if let Some(idx) = best_idx {
            cell_texts[idx].push(text.clone());
        }
    }

    // Build the 2D grid
    let mut result: Vec<Vec<String>> = Vec::new();
    for row_indices in &rows {
        let mut row: Vec<String> = Vec::new();
        for &ci in row_indices {
            if ci < cell_texts.len() {
                row.push(cell_texts[ci].join(" "));
            } else {
                row.push(String::new());
            }
        }
        result.push(row);
    }

    result
}

/// Compute IoU (Intersection over Union) between two axis-aligned rectangles.
fn compute_iou(a: &[f32; 4], b: &[f32; 4]) -> f32 {
    let inter_x1 = a[0].max(b[0]);
    let inter_y1 = a[1].max(b[1]);
    let inter_x2 = a[2].min(b[2]);
    let inter_y2 = a[3].min(b[3]);

    if inter_x1 >= inter_x2 || inter_y1 >= inter_y2 {
        return 0.0;
    }

    let inter_area = (inter_x2 - inter_x1) * (inter_y2 - inter_y1);
    let area_a = (a[2] - a[0]) * (a[3] - a[1]);
    let area_b = (b[2] - b[0]) * (b[3] - b[1]);
    let union_area = area_a + area_b - inter_area;

    if union_area <= 0.0 {
        0.0
    } else {
        inter_area / union_area
    }
}

/// Compute L1 distance between two rectangles (PaddleOCR's `distance` function).
/// Uses sum of corner distances plus minimum of top-left/bottom-right distances.
fn compute_l1(a: &[f32; 4], b: &[f32; 4]) -> f32 {
    let dis = (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
        + (a[2] - b[2]).abs() + (a[3] - b[3]).abs();
    let dis_tl = (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
    let dis_br = (a[2] - b[2]).abs() + (a[3] - b[3]).abs();
    dis + dis_tl.min(dis_br)
}

/// Convert a 2D cell grid to markdown pipe table format.
pub fn cells_to_markdown(cells: &[Vec<String>]) -> String {
    if cells.is_empty() {
        return String::new();
    }

    let mut md = String::new();

    // Find max column count
    let max_cols = cells.iter().map(|r| r.len()).max().unwrap_or(0);
    if max_cols == 0 {
        return String::new();
    }

    for (i, row) in cells.iter().enumerate() {
        md.push('|');
        for col_idx in 0..max_cols {
            let cell = row.get(col_idx).map(|s| s.as_str()).unwrap_or("");
            md.push(' ');
            md.push_str(cell);
            md.push_str(" |");
        }
        md.push('\n');

        // Add separator after first row (header)
        if i == 0 {
            md.push('|');
            for _ in 0..max_cols {
                md.push_str(" --- |");
            }
            md.push('\n');
        }
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cells_to_markdown_basic() {
        let cells = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        let md = cells_to_markdown(&cells);
        assert!(md.contains("| Name | Age |"));
        assert!(md.contains("| --- | --- |"));
        assert!(md.contains("| Alice | 30 |"));
        assert!(md.contains("| Bob | 25 |"));
    }

    #[test]
    fn test_cells_to_markdown_empty() {
        assert_eq!(cells_to_markdown(&[]), "");
        assert_eq!(cells_to_markdown(&[vec![]]), "");
    }

    #[test]
    fn test_html_to_table_cells_basic() {
        let tokens: Vec<String> = vec![
            "<html>", "<body>", "<table>",
            "<tr>", "<td></td>", "<td></td>", "</tr>",
            "<tr>", "<td></td>", "<td></td>", "</tr>",
            "</table>", "</body>", "</html>",
        ].into_iter().map(String::from).collect();

        let bboxes: Vec<[f32; 8]> = vec![
            [0.0, 0.0, 50.0, 0.0, 50.0, 20.0, 0.0, 20.0],   // cell 0,0
            [50.0, 0.0, 100.0, 0.0, 100.0, 20.0, 50.0, 20.0], // cell 0,1
            [0.0, 20.0, 50.0, 20.0, 50.0, 40.0, 0.0, 40.0],   // cell 1,0
            [50.0, 20.0, 100.0, 20.0, 100.0, 40.0, 50.0, 40.0], // cell 1,1
        ];

        let text_blocks = vec![
            ("Hello".to_string(), 25.0_f32, 10.0_f32),
            ("World".to_string(), 75.0, 10.0),
            ("Foo".to_string(), 25.0, 30.0),
            ("Bar".to_string(), 75.0, 30.0),
        ];

        let cells = html_to_table_cells(&tokens, &bboxes, &text_blocks);
        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0], vec!["Hello", "World"]);
        assert_eq!(cells[1], vec!["Foo", "Bar"]);
    }

    #[test]
    fn test_decode_bbox() {
        let sla = SlaNet::new();
        // With ratio=1.0 (image already 488x488), w_ratio and h_ratio should be 1.0
        let bbox = sla.decode_bbox(
            &[0.1, 0.2, 0.9, 0.2, 0.9, 0.8, 0.1, 0.8],
            488, 488, 1.0,
        );
        // x coords * 488 * 1.0, y coords * 488 * 1.0
        assert!((bbox[0] - 48.8).abs() < 0.1);
        assert!((bbox[1] - 97.6).abs() < 0.1);
    }

    #[test]
    fn test_preprocess_shape() {
        let img = image::RgbImage::new(200, 100);
        let (tensor, ratio) = SlaNet::preprocess(&img);
        assert_eq!(tensor.shape(), &[1, 3, 488, 488]);
        assert!((ratio - 488.0 / 200.0).abs() < 0.01);
    }
}
