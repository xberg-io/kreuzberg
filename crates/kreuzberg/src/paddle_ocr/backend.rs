//! PaddleOCR backend implementation.
//!
//! This module implements the `OcrBackend` trait for PaddleOCR using ONNX Runtime.
//! PaddleOCR provides excellent recognition quality, especially for CJK languages.
//!
//! The backend maintains a pool of OCR engines keyed by script family.
//! Each family gets its own lazily-initialized engine with the appropriate
//! recognition model and character dictionary.

use ahash::AHashMap;
use async_trait::async_trait;
use std::borrow::Cow;
use std::collections::HashMap;
use std::panic::catch_unwind;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::Result;
use crate::core::config::OcrConfig;
use crate::ocr::conversion::{elements_to_hocr_words, text_block_to_element};
use crate::ocr::table::{reconstruct_table, table_to_markdown};
use crate::plugins::{OcrBackend, OcrBackendType, Plugin};
use crate::types::{ExtractionResult, FormatMetadata, Metadata, OcrElement, OcrMetadata, Table};

use super::config::PaddleOcrConfig;
use super::model_manager::{ModelManager, SharedModelPaths};
use super::{is_language_supported, language_to_script_family, map_language_code};

use kreuzberg_paddle_ocr::OcrLite;

/// Per-table result: SLANet structure + OCR text blocks from the cropped region.
/// Both SLANet cell bboxes and OCR text bboxes are in crop-local coordinates,
/// following PaddleOCR's reference pipeline where OCR runs on the crop.
struct TableOcrResult {
    structure: kreuzberg_paddle_ocr::TableStructureResult,
    crop_text_blocks: Vec<kreuzberg_paddle_ocr::TextBlock>,
}

/// PaddleOCR backend using ONNX Runtime.
///
/// Maintains a pool of OCR engines keyed by script family. Each family has its own
/// recognition model and character dictionary, while detection and classification
/// models are shared across all families.
///
/// # Thread Safety
///
/// The backend is `Send + Sync` and can be used across threads safely via `Arc`.
/// Each engine in the pool has its own mutex, so concurrent OCR on different
/// script families does not block.
pub struct PaddleOcrBackend {
    config: Arc<PaddleOcrConfig>,
    model_manager: ModelManager,
    shared_paths: Mutex<Option<SharedModelPaths>>,
    /// Per-script-family OCR engines, lazily initialized.
    engine_pool: Mutex<HashMap<String, Arc<Mutex<OcrLite>>>>,
}

impl PaddleOcrBackend {
    /// Create a new PaddleOCR backend with default configuration.
    pub fn new() -> Result<Self> {
        Self::with_config(PaddleOcrConfig::default())
    }

    /// Create a new PaddleOCR backend with custom configuration.
    pub fn with_config(config: PaddleOcrConfig) -> Result<Self> {
        let cache_dir = config.resolve_cache_dir();
        Ok(Self {
            config: Arc::new(config),
            model_manager: ModelManager::new(cache_dir),
            shared_paths: Mutex::new(None),
            engine_pool: Mutex::new(HashMap::new()),
        })
    }

    /// Get or initialize shared model paths (det + cls).
    fn get_or_init_shared_paths(&self) -> Result<SharedModelPaths> {
        let mut paths = self.shared_paths.lock().map_err(|e| crate::KreuzbergError::Plugin {
            message: format!("Failed to acquire shared paths lock: {e}"),
            plugin_name: "paddle-ocr".to_string(),
        })?;

        if let Some(ref p) = *paths {
            return Ok(p.clone());
        }

        let shared = self.model_manager.ensure_shared_models()?;
        *paths = Some(shared.clone());
        Ok(shared)
    }

    /// Get or create an OCR engine for the given script family.
    ///
    /// Returns an `Arc<Mutex<OcrLite>>` for the requested family. If the engine
    /// doesn't exist yet, it will be created with the family's recognition model
    /// and character dictionary.
    fn get_or_init_engine_for_family(&self, family: &str) -> Result<Arc<Mutex<OcrLite>>> {
        // Fast path: check if engine already exists
        {
            let pool = self.engine_pool.lock().map_err(|e| crate::KreuzbergError::Plugin {
                message: format!("Failed to acquire engine pool lock: {e}"),
                plugin_name: "paddle-ocr".to_string(),
            })?;
            if let Some(engine) = pool.get(family) {
                return Ok(Arc::clone(engine));
            }
        }

        // Slow path: create new engine
        let shared = self.get_or_init_shared_paths()?;
        let rec_paths = self.model_manager.ensure_rec_model(family)?;

        crate::ort_discovery::ensure_ort_available();

        tracing::info!(family, "Initializing PaddleOCR engine");

        let mut ocr_lite = OcrLite::new();

        let det_model_path = Self::find_onnx_model(&shared.det_model)?;
        let cls_model_path = Self::find_onnx_model(&shared.cls_model)?;
        let rec_model_path = Self::find_onnx_model(&rec_paths.rec_model)?;

        let num_threads = num_cpus::get().min(4);

        let dict_path = rec_paths.dict_file.to_str().ok_or_else(|| crate::KreuzbergError::Ocr {
            message: "Invalid dictionary file path".to_string(),
            source: None,
        })?;

        ocr_lite
            .init_models_with_dict(
                det_model_path.to_str().ok_or_else(|| crate::KreuzbergError::Ocr {
                    message: "Invalid detection model path".to_string(),
                    source: None,
                })?,
                cls_model_path.to_str().ok_or_else(|| crate::KreuzbergError::Ocr {
                    message: "Invalid classification model path".to_string(),
                    source: None,
                })?,
                rec_model_path.to_str().ok_or_else(|| crate::KreuzbergError::Ocr {
                    message: "Invalid recognition model path".to_string(),
                    source: None,
                })?,
                dict_path,
                num_threads,
            )
            .map_err(|e| crate::KreuzbergError::Ocr {
                message: format!("Failed to initialize PaddleOCR models for {family}: {e}"),
                source: None,
            })?;

        // Load SLANet table model if available in cache.
        // We always load it during engine init (it's small and fast) so that
        // per-request enable_table_detection can use it without re-initializing.
        if self.model_manager.is_table_model_cached() || self.config.enable_table_detection {
            match self.model_manager.ensure_table_model() {
                Ok(table_paths) => {
                    let table_model_path = Self::find_onnx_model(&table_paths.table_model)?;
                    let table_path_str = table_model_path.to_str().ok_or_else(|| crate::KreuzbergError::Ocr {
                        message: "Invalid table model path".to_string(),
                        source: None,
                    })?;
                    if let Err(e) = ocr_lite.init_table_model(table_path_str, num_threads) {
                        tracing::warn!(family, error = %e, "Failed to load SLANet table model, falling back to heuristic");
                    } else {
                        tracing::info!(family, "SLANet table model loaded");
                    }
                }
                Err(e) => {
                    tracing::warn!(family, error = %e, "Failed to download SLANet table model, falling back to heuristic");
                }
            }
        }

        // Load PicoDet layout detection model for table region detection.
        // When available, layout detection identifies table regions first,
        // then SLANet processes only the cropped table regions (not full page).
        if self.model_manager.is_layout_model_cached() || self.config.enable_table_detection {
            match self.model_manager.ensure_layout_model() {
                Ok(layout_paths) => {
                    let layout_model_path = Self::find_onnx_model(&layout_paths.layout_model)?;
                    let layout_path_str = layout_model_path.to_str().ok_or_else(|| crate::KreuzbergError::Ocr {
                        message: "Invalid layout model path".to_string(),
                        source: None,
                    })?;
                    if let Err(e) = ocr_lite.init_layout_model(layout_path_str, num_threads) {
                        tracing::warn!(family, error = %e, "Failed to load layout detection model");
                    } else {
                        tracing::info!(family, "PicoDet layout detection model loaded");
                    }
                }
                Err(e) => {
                    tracing::warn!(family, error = %e, "Failed to download layout detection model");
                }
            }
        }

        tracing::info!(family, "PaddleOCR engine initialized successfully");

        let engine = Arc::new(Mutex::new(ocr_lite));

        // Insert into pool (with double-check for concurrent initialization)
        let mut pool = self.engine_pool.lock().map_err(|e| crate::KreuzbergError::Plugin {
            message: format!("Failed to acquire engine pool lock: {e}"),
            plugin_name: "paddle-ocr".to_string(),
        })?;

        // Re-check if another thread already inserted an engine while we were creating ours
        if let Some(existing_engine) = pool.get(family) {
            // Another thread beat us; use their engine instead
            return Ok(Arc::clone(existing_engine));
        }

        // We're first; insert our engine
        pool.insert(family.to_string(), Arc::clone(&engine));

        Ok(engine)
    }

    /// Find the ONNX model file within a model directory.
    fn find_onnx_model(model_dir: &std::path::Path) -> Result<std::path::PathBuf> {
        if !model_dir.exists() {
            return Err(crate::KreuzbergError::Ocr {
                message: format!("Model directory does not exist: {:?}", model_dir),
                source: None,
            });
        }

        let standard_path = model_dir.join("model.onnx");
        if standard_path.exists() {
            return Ok(standard_path);
        }

        let entries = std::fs::read_dir(model_dir).map_err(|e| crate::KreuzbergError::Ocr {
            message: format!("Failed to read model directory {:?}: {}", model_dir, e),
            source: None,
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| crate::KreuzbergError::Ocr {
                message: format!("Failed to read directory entry: {}", e),
                source: None,
            })?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "onnx") {
                return Ok(path);
            }
        }

        Err(crate::KreuzbergError::Ocr {
            message: format!("No ONNX model file found in directory: {:?}", model_dir),
            source: None,
        })
    }

    /// Perform OCR on image bytes using the appropriate script family engine.
    async fn do_ocr(
        &self,
        image_bytes: &[u8],
        language: &str,
        effective_config: Arc<PaddleOcrConfig>,
    ) -> Result<(String, Vec<OcrElement>, Vec<TableOcrResult>)> {
        let family = language_to_script_family(language);
        let engine = self.get_or_init_engine_for_family(family)?;

        let image_bytes_owned = image_bytes.to_vec();
        let config = effective_config;

        let (text_blocks, table_results) = tokio::task::spawn_blocking(move || {
            catch_unwind(std::panic::AssertUnwindSafe(|| {
                Self::perform_ocr(&image_bytes_owned, &engine, &config)
            }))
            .map_err(|_| crate::KreuzbergError::Plugin {
                message: "PaddleOCR inference panicked (ONNX Runtime error)".to_string(),
                plugin_name: "paddle-ocr".to_string(),
            })?
        })
        .await
        .map_err(|e| crate::KreuzbergError::Plugin {
            message: format!("PaddleOCR task panicked: {}", e),
            plugin_name: "paddle-ocr".to_string(),
        })??;

        let ocr_elements: Result<Vec<OcrElement>> = text_blocks
            .iter()
            .map(|block| text_block_to_element(block, 1))
            .collect();

        let ocr_elements = ocr_elements?;

        let text = text_blocks
            .iter()
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        Ok((text, ocr_elements, table_results))
    }

    /// Perform actual OCR inference (runs in blocking context).
    /// Returns full-page text blocks and per-table results with crop-local OCR.
    fn perform_ocr(
        image_bytes: &[u8],
        ocr_engine: &Arc<Mutex<OcrLite>>,
        config: &PaddleOcrConfig,
    ) -> Result<(Vec<kreuzberg_paddle_ocr::TextBlock>, Vec<TableOcrResult>)> {
        let img = image::load_from_memory(image_bytes)
            .map_err(|e| crate::KreuzbergError::Ocr {
                message: format!("Failed to decode image: {}", e),
                source: None,
            })?
            .to_rgb8();

        let mut engine_guard = ocr_engine.lock().map_err(|e| crate::KreuzbergError::Plugin {
            message: format!("Failed to acquire OCR engine lock: {}", e),
            plugin_name: "paddle-ocr".to_string(),
        })?;

        let padding = config.padding;
        let max_side_len = config.det_limit_side_len;
        let box_score_thresh = config.det_db_thresh;
        let box_thresh = config.det_db_box_thresh;
        let un_clip_ratio = config.det_db_unclip_ratio;
        let do_angle = config.use_angle_cls;
        let most_angle = false;

        // Full-page OCR for text extraction
        let result = engine_guard
            .detect(
                &img,
                padding,
                max_side_len,
                box_score_thresh,
                box_thresh,
                un_clip_ratio,
                do_angle,
                most_angle,
            )
            .map_err(|e| crate::KreuzbergError::Ocr {
                message: format!("PaddleOCR detection failed: {}", e),
                source: None,
            })?;

        tracing::debug!(
            text_block_count = result.text_blocks.len(),
            "PaddleOCR detection completed"
        );

        // Table detection: layout detect -> crop -> OCR on crop + SLANet on crop
        // Following PaddleOCR reference: both OCR and SLANet run on the same
        // cropped region so coordinates are naturally in the same space.
        let mut table_results: Vec<TableOcrResult> = Vec::new();

        if config.enable_table_detection && engine_guard.has_table_model() {
            if engine_guard.has_layout_model() {
                match engine_guard.detect_layout(&img) {
                    Ok(Some(layout_result)) => {
                        let table_dets: Vec<_> = layout_result
                            .detections
                            .iter()
                            .filter(|d| d.label == "table")
                            .collect();

                        tracing::debug!(table_count = table_dets.len(), "Layout detection found table regions");

                        for det in &table_dets {
                            let [x1, y1, x2, y2] = det.bbox;
                            let cx1 = (x1 as u32).min(img.width());
                            let cy1 = (y1 as u32).min(img.height());
                            let cw = ((x2 - x1) as u32).min(img.width().saturating_sub(cx1));
                            let ch = ((y2 - y1) as u32).min(img.height().saturating_sub(cy1));

                            if cw < 10 || ch < 10 {
                                continue;
                            }

                            let cropped = image::imageops::crop_imm(&img, cx1, cy1, cw, ch).to_image();

                            // When the crop is very wide (aspect > 2:1), pad to
                            // square so SLANet sees content at usable resolution.
                            // Cell bboxes (decode_bbox: x*w, y*h) will be in padded
                            // image space but content is at top-left, so Y coords
                            // in the content region (0..ch) match the crop space.
                            let slanet_input = if cw > ch * 2 {
                                let side = cw;
                                let mut padded = image::RgbImage::from_pixel(
                                    side, side, image::Rgb([255, 255, 255]),
                                );
                                image::imageops::overlay(&mut padded, &cropped, 0, 0);
                                padded
                            } else {
                                cropped.clone()
                            };

                            let table_result = match engine_guard.detect_table_structure(&slanet_input) {
                                Ok(Some(r)) => r,
                                Ok(None) => {
                                    tracing::debug!("SLANet found no table structure in cropped region");
                                    continue;
                                }
                                Err(e) => {
                                    tracing::warn!(error = %e, "SLANet failed on cropped table region");
                                    continue;
                                }
                            };

                            // Run OCR on the same crop (matching PaddleOCR reference)
                            let crop_ocr = engine_guard
                                .detect(
                                    &cropped,
                                    padding,
                                    max_side_len,
                                    box_score_thresh,
                                    box_thresh,
                                    un_clip_ratio,
                                    do_angle,
                                    most_angle,
                                )
                                .map_err(|e| crate::KreuzbergError::Ocr {
                                    message: format!("PaddleOCR crop OCR failed: {}", e),
                                    source: None,
                                })?;

                            tracing::debug!(
                                crop_text_blocks = crop_ocr.text_blocks.len(),
                                slanet_cells = table_result.cell_bboxes.len(),
                                "Table crop OCR + SLANet completed"
                            );

                            table_results.push(TableOcrResult {
                                structure: table_result,
                                crop_text_blocks: crop_ocr.text_blocks,
                            });
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        tracing::warn!(error = %e, "Layout detection failed, falling back to full-page SLANet");
                        if let Ok(Some(table_result)) = engine_guard.detect_table_structure(&img) {
                            table_results.push(TableOcrResult {
                                structure: table_result,
                                crop_text_blocks: result.text_blocks.clone(),
                            });
                        }
                    }
                }
            } else {
                // No layout model: run SLANet on full page (legacy behavior)
                match engine_guard.detect_table_structure(&img) {
                    Ok(Some(table_result)) => {
                        table_results.push(TableOcrResult {
                            structure: table_result,
                            crop_text_blocks: result.text_blocks.clone(),
                        });
                    }
                    Ok(None) => {}
                    Err(e) => {
                        tracing::warn!(error = %e, "SLANet table detection failed");
                    }
                }
            }
        }

        Ok((result.text_blocks, table_results))
    }
}

impl Plugin for PaddleOcrBackend {
    fn name(&self) -> &str {
        "paddle-ocr"
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl OcrBackend for PaddleOcrBackend {
    async fn process_image(&self, image_bytes: &[u8], config: &OcrConfig) -> Result<ExtractionResult> {
        if image_bytes.is_empty() {
            return Err(crate::KreuzbergError::Validation {
                message: "Empty image data provided to PaddleOCR".to_string(),
                source: None,
            });
        }

        let effective_config: Arc<PaddleOcrConfig> = if let Some(ref paddle_json) = config.paddle_ocr_config {
            let overridden: PaddleOcrConfig =
                serde_json::from_value(paddle_json.clone()).map_err(|e| crate::KreuzbergError::Validation {
                    message: format!("Failed to deserialize paddle_ocr_config: {}", e),
                    source: None,
                })?;
            Arc::new(overridden)
        } else {
            Arc::clone(&self.config)
        };

        // Map language code to PaddleOCR language, then use it for engine selection
        let paddle_lang = map_language_code(&config.language).unwrap_or("en");

        let (text, ocr_elements, slanet_results) = self
            .do_ocr(image_bytes, paddle_lang, Arc::clone(&effective_config))
            .await?;

        // Table detection: use SLANet results (possibly multiple from layout detection)
        let mut tables: Vec<Table> = vec![];
        let mut table_count = 0;
        let mut table_rows: Option<usize> = None;
        let mut table_cols: Option<usize> = None;

        if effective_config.enable_table_detection {
            if !slanet_results.is_empty() {
                for table_ocr in &slanet_results {
                    // Both SLANet cell bboxes and OCR text bboxes are in crop-local
                    // coordinates (OCR ran on the crop, matching PaddleOCR reference).
                    // Convert OCR text blocks to axis-aligned bboxes for IoU matching.
                    let ocr_boxes: Vec<(String, [f32; 4])> = table_ocr
                        .crop_text_blocks
                        .iter()
                        .filter_map(|tb| {
                            if tb.box_points.len() < 4 || tb.text.is_empty() {
                                return None;
                            }
                            let x_min = tb.box_points.iter().map(|p| p.x).min().unwrap_or(0) as f32;
                            let y_min = tb.box_points.iter().map(|p| p.y).min().unwrap_or(0) as f32;
                            let x_max = tb.box_points.iter().map(|p| p.x).max().unwrap_or(0) as f32;
                            let y_max = tb.box_points.iter().map(|p| p.y).max().unwrap_or(0) as f32;
                            Some((tb.text.clone(), [x_min, y_min, x_max, y_max]))
                        })
                        .collect();

                    let mut cells = kreuzberg_paddle_ocr::sla_net::html_to_table_cells_iou(
                        &table_ocr.structure.html_tokens,
                        &table_ocr.structure.cell_bboxes,
                        &ocr_boxes,
                    );

                    // Strip trailing all-empty rows
                    while cells.last().map_or(false, |row| row.iter().all(|c| c.is_empty())) {
                        cells.pop();
                    }

                    if !cells.is_empty() {
                        if table_rows.is_none() {
                            table_rows = Some(cells.len());
                            table_cols = cells.first().map(|row| row.len());
                        }

                        let table_markdown = kreuzberg_paddle_ocr::sla_net::cells_to_markdown(&cells);

                        tables.push(Table {
                            cells,
                            markdown: table_markdown,
                            page_number: 1,
                            bounding_box: None,
                        });
                    }
                }

                table_count = tables.len();

                tracing::debug!(
                    table_count,
                    "SLANet table detection completed"
                );
            } else if !ocr_elements.is_empty() {
                // Fallback: heuristic table detection
                let words = elements_to_hocr_words(&ocr_elements, 0.3);

                if !words.is_empty() {
                    let cells = reconstruct_table(&words, 20, 0.5);

                    if !cells.is_empty() {
                        table_count = 1;
                        table_rows = Some(cells.len());
                        table_cols = cells.first().map(|row| row.len());

                        let table_markdown = table_to_markdown(&cells);

                        tables.push(Table {
                            cells,
                            markdown: table_markdown,
                            page_number: 1,
                            bounding_box: None,
                        });
                    }
                }
            }
        }

        let mut additional = AHashMap::new();
        additional.insert(Cow::Borrowed("backend"), serde_json::json!("paddle-ocr"));

        let metadata = Metadata {
            format: Some(FormatMetadata::Ocr(OcrMetadata {
                language: config.language.clone(),
                psm: 3,
                output_format: "text".to_string(),
                table_count,
                table_rows,
                table_cols,
            })),
            additional,
            ..Default::default()
        };

        let include_elements = config.element_config.as_ref().is_some_and(|ec| ec.include_elements);

        let ocr_elements_opt = if include_elements && !ocr_elements.is_empty() {
            Some(ocr_elements)
        } else {
            None
        };

        Ok(ExtractionResult {
            content: text,
            mime_type: Cow::Borrowed("text/plain"),
            metadata,
            tables,
            detected_languages: Some(vec![config.language.clone()]),
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
            ocr_elements: ocr_elements_opt,
            document: None,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            extracted_keywords: None,
            quality_score: None,
            processing_warnings: Vec::new(),
            annotations: None,
        })
    }

    async fn process_file(&self, path: &Path, config: &OcrConfig) -> Result<ExtractionResult> {
        let bytes = tokio::fs::read(path).await?;
        self.process_image(&bytes, config).await
    }

    fn supports_language(&self, lang: &str) -> bool {
        is_language_supported(lang) || map_language_code(lang).is_some()
    }

    fn backend_type(&self) -> OcrBackendType {
        OcrBackendType::PaddleOCR
    }

    fn supported_languages(&self) -> Vec<String> {
        super::SUPPORTED_LANGUAGES.iter().map(|s| s.to_string()).collect()
    }

    fn supports_table_detection(&self) -> bool {
        self.config.enable_table_detection
    }
}

impl Default for PaddleOcrBackend {
    fn default() -> Self {
        Self::with_config(PaddleOcrConfig::default())
            .unwrap_or_else(|e| panic!("Failed to create default PaddleOcrBackend: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paddle_ocr_backend_creation() {
        let result = PaddleOcrBackend::new();
        assert!(result.is_ok(), "Failed to create PaddleOCR backend");
    }

    #[test]
    fn test_paddle_ocr_backend_with_config() {
        let config = PaddleOcrConfig::default();
        let result = PaddleOcrBackend::with_config(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_paddle_ocr_language_support_direct() {
        let backend = PaddleOcrBackend::new().unwrap();

        assert!(backend.supports_language("ch"));
        assert!(backend.supports_language("en"));
        assert!(backend.supports_language("japan"));
        assert!(backend.supports_language("korean"));
        assert!(backend.supports_language("french"));
        assert!(backend.supports_language("thai"));
        assert!(backend.supports_language("greek"));
    }

    #[test]
    fn test_paddle_ocr_language_support_mapped() {
        let backend = PaddleOcrBackend::new().unwrap();

        assert!(backend.supports_language("chi_sim"));
        assert!(backend.supports_language("eng"));
        assert!(backend.supports_language("jpn"));
        assert!(backend.supports_language("kor"));
        assert!(backend.supports_language("fra"));
        assert!(backend.supports_language("zho"));
        assert!(backend.supports_language("tha"));
        assert!(backend.supports_language("ell"));
        assert!(backend.supports_language("rus"));
    }

    #[test]
    fn test_paddle_ocr_language_unsupported() {
        let backend = PaddleOcrBackend::new().unwrap();

        assert!(!backend.supports_language("xyz"));
        assert!(!backend.supports_language("invalid"));
    }

    #[test]
    fn test_paddle_ocr_plugin_interface() {
        let backend = PaddleOcrBackend::new().unwrap();

        assert_eq!(backend.name(), "paddle-ocr");
        assert!(!backend.version().is_empty());
        assert!(backend.initialize().is_ok());
        assert!(backend.shutdown().is_ok());
    }

    #[test]
    fn test_paddle_ocr_backend_type() {
        let backend = PaddleOcrBackend::new().unwrap();
        assert_eq!(backend.backend_type(), OcrBackendType::PaddleOCR);
    }

    #[test]
    fn test_paddle_ocr_supported_languages() {
        let backend = PaddleOcrBackend::new().unwrap();
        let languages = backend.supported_languages();

        assert!(!languages.is_empty());
        assert!(languages.contains(&"ch".to_string()));
        assert!(languages.contains(&"en".to_string()));
        assert!(languages.contains(&"thai".to_string()));
        assert!(languages.contains(&"greek".to_string()));
    }

    #[test]
    fn test_paddle_ocr_table_detection_disabled_by_default() {
        let backend = PaddleOcrBackend::new().unwrap();
        assert!(!backend.supports_table_detection());
    }

    #[test]
    fn test_paddle_ocr_table_detection_enabled() {
        let config = PaddleOcrConfig::default().with_table_detection(true);
        let backend = PaddleOcrBackend::with_config(config).unwrap();
        assert!(backend.supports_table_detection());
    }

    #[test]
    fn test_paddle_ocr_default() {
        let backend = PaddleOcrBackend::default();
        assert_eq!(backend.name(), "paddle-ocr");
    }

    #[tokio::test]
    async fn test_paddle_ocr_process_empty_image() {
        let backend = PaddleOcrBackend::new().unwrap();
        let config = OcrConfig {
            backend: "paddle-ocr".to_string(),
            language: "ch".to_string(),
            ..Default::default()
        };

        let result = backend.process_image(&[], &config).await;
        assert!(result.is_err(), "Should error on empty image");
    }
}
