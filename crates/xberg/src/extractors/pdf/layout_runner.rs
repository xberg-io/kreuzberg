//! Layout detection runner for PDF pages.
//!
//! Renders pages of a PDF document in chunks, runs the layout engine on each
//! chunk in sequence, and converts pixel-space detections to PDF
//! coordinate–space [`PageLayoutResult`] values.
//!
//! Chunked rendering+detection keeps peak memory proportional to
//! `LAYOUT_BATCH_CHUNK_SIZE` images plus the accumulated output images,
//! rather than requiring the whole document's rasterised frames and the full
//! ONNX batch tensor to be live simultaneously.
//!
//! The resulting `(Vec<RgbImage>, Vec<PageLayoutResult>)` pair is consumed
//! by [`super::extraction::extract_all_from_oxide_document`] via
//! `layout_images` / `layout_results`, which feeds the segment structure
//! pipeline with layout hints for heading / table / list / figure
//! classification (the "layout-for-markdown" path).

/// Maximum number of pages sent to the layout model in a single ONNX call.
///
/// Each chunk requires:  chunk_size × 3 × 640 × 640 × 4 bytes ≈ 4.9 MB × chunk_size
/// for the batch tensor.  8 pages ≈ 39 MB vs 214 pages ≈ 1.05 GB without chunking.
#[cfg(all(feature = "pdf", feature = "layout-detection"))]
const LAYOUT_BATCH_CHUNK_SIZE: usize = 8;

#[cfg(all(feature = "pdf", feature = "layout-detection"))]
use crate::{
    Result, XbergError,
    core::config::{ExtractionConfig, layout::LayoutDetectionConfig},
    extractors::pdf::layout_hints::pixel_detection_to_layout_hints_pdf_space,
    pdf::structure::types::{LayoutHint, PageLayoutResult},
};

/// Render every page of `content` to RGB (in chunks) and run layout detection.
///
/// Returns `(images, results, hints_per_page)` where:
/// - `images[i]` is the rendered RGB image for page `i` (or a 1×1 placeholder
///   if the page failed to render).
/// - `results[i]` holds per-region detection metadata in PDF coordinate space.
/// - `hints_per_page[i]` holds the layout hints derived from detections on
///   page `i` (empty for pages that failed to render or produced no detections).
///
/// # Memory behaviour
///
/// Pages are rendered and detected in chunks of [`LAYOUT_BATCH_CHUNK_SIZE`]
/// so the peak ONNX batch tensor size is bounded.  The returned `images` vec
/// accumulates all page images for downstream table recognition.
///
/// # Errors
///
/// Returns an error if the PDF cannot be opened, the layout engine cannot be
/// initialised, or detection fails on any chunk.  Individual page render
/// failures are logged and produce empty layout for that page without aborting
/// the whole document.
#[cfg(all(feature = "pdf", feature = "layout-detection"))]
type LayoutForMarkdownOutput = (Vec<image::RgbImage>, Vec<PageLayoutResult>, Vec<Vec<LayoutHint>>);

#[cfg(all(feature = "pdf", feature = "layout-detection"))]
pub(super) fn run_layout_for_pdf_pages(
    content: &[u8],
    layout_config: &LayoutDetectionConfig,
) -> Result<LayoutForMarkdownOutput> {
    // --- 1. Open document ---
    let doc = pdf_oxide::PdfDocument::from_bytes(content.to_vec()).map_err(|e| XbergError::Parsing {
        message: format!("layout runner: failed to open PDF: {e}"),
        source: None,
    })?;

    let page_count = doc.page_count().map_err(|e| XbergError::Parsing {
        message: format!("layout runner: failed to get page count: {e}"),
        source: None,
    })?;

    if page_count == 0 {
        return Ok((Vec::new(), Vec::new(), Vec::new()));
    }

    // --- 2. Initialise engine ---
    let mut engine = crate::layout::take_or_create_engine(layout_config)
        .map_err(|e| XbergError::Other(format!("layout runner: engine init failed: {e}")))?;

    // --- 3. Chunked render + detect ---
    let mut all_images: Vec<image::RgbImage> = Vec::with_capacity(page_count);
    let mut all_layout_results: Vec<PageLayoutResult> = Vec::with_capacity(page_count);
    let mut all_hints: Vec<Vec<LayoutHint>> = Vec::with_capacity(page_count);

    let total_chunks = (page_count + LAYOUT_BATCH_CHUNK_SIZE - 1) / LAYOUT_BATCH_CHUNK_SIZE;

    for (chunk_idx, chunk_start) in (0..page_count).step_by(LAYOUT_BATCH_CHUNK_SIZE).enumerate() {
        let chunk_end = (chunk_start + LAYOUT_BATCH_CHUNK_SIZE).min(page_count);
        let chunk_size = chunk_end - chunk_start;

        // Phase A: render pages in this chunk.
        // chunk_page_meta[k] = (page_width_pts, page_height_pts)
        // chunk_images[k]    = Some(RgbImage) if the page rendered, else None
        let mut chunk_page_meta: Vec<(f32, f32)> = Vec::with_capacity(chunk_size);
        let mut chunk_images: Vec<Option<image::RgbImage>> = Vec::with_capacity(chunk_size);

        for page_idx in chunk_start..chunk_end {
            let (pw, ph) = doc
                .get_page_media_box(page_idx)
                .map(|(llx, lly, urx, ury)| ((urx - llx).abs(), (ury - lly).abs()))
                .unwrap_or((612.0, 792.0));
            chunk_page_meta.push((pw, ph));

            let rgb_opt = match crate::pdf::render::render_page_with_safeguards(&doc, page_idx, 150) {
                Err(e) => {
                    tracing::warn!(
                        page = page_idx + 1,
                        page_width_pts = pw,
                        page_height_pts = ph,
                        error = %e,
                        "layout runner: skipping page with render failure, returning empty detections"
                    );
                    None
                }
                Ok(rendered) => match image::load_from_memory(&rendered.data) {
                    Err(e) => {
                        tracing::warn!(
                            page = page_idx + 1,
                            page_width_pts = pw,
                            page_height_pts = ph,
                            error = %e,
                            "layout runner: skipping page (PNG decode failed), returning empty detections"
                        );
                        None
                    }
                    Ok(img) => Some(img.into_rgb8()),
                },
            };
            chunk_images.push(rgb_opt);
        }

        // Phase B: run detection on the successfully rendered pages.
        // rendered_positions[k] = index into chunk_images that is Some.
        let rendered_positions: Vec<usize> = chunk_images
            .iter()
            .enumerate()
            .filter_map(|(k, opt)| opt.as_ref().map(|_| k))
            .collect();

        let detection_results = if rendered_positions.is_empty() {
            tracing::debug!(
                chunk_idx,
                total_chunks,
                "layout runner: all pages in chunk failed to render, skipping detection"
            );
            Vec::new()
        } else {
            let rgb_refs: Vec<&image::RgbImage> = rendered_positions
                .iter()
                .map(|&k| chunk_images[k].as_ref().expect("filtered to Some above"))
                .collect();

            tracing::debug!(
                chunk_idx,
                total_chunks,
                chunk_start,
                chunk_end,
                rendered = rgb_refs.len(),
                "layout runner: detecting chunk"
            );

            match engine.detect_batch(&rgb_refs) {
                Ok(r) => r,
                Err(e) => {
                    crate::layout::return_engine(engine);
                    return Err(XbergError::Other(format!("layout runner: batch detection failed: {e}")));
                }
            }
        };

        // Map detection results back to their position within this chunk.
        let mut detected_by_pos: Vec<Option<_>> = (0..chunk_size).map(|_| None).collect();
        for (&pos, result) in rendered_positions.iter().zip(detection_results) {
            detected_by_pos[pos] = Some(result);
        }

        // Phase C: assemble outputs in page order, one entry per page.
        for k in 0..chunk_size {
            let (pw, ph) = chunk_page_meta[k];
            // Take the image out of the Option (None → 1×1 placeholder for downstream callers).
            let img = chunk_images[k].take().unwrap_or_else(|| image::RgbImage::new(1, 1));

            if let Some((detection, _timings)) = detected_by_pos[k].take() {
                let image_width_px = img.width();
                let image_height_px = img.height();

                let hints =
                    pixel_detection_to_layout_hints_pdf_space(&detection, image_width_px, image_height_px, pw, ph);

                tracing::debug!(
                    detections = detection.detections.len(),
                    hints = hints.len(),
                    page_width_pts = pw,
                    page_height_pts = ph,
                    image_width_px,
                    image_height_px,
                    "layout runner: page detections"
                );

                all_hints.push(hints);
            } else {
                all_hints.push(Vec::new());
            }

            all_layout_results.push(PageLayoutResult {
                page_width_pts: pw,
                page_height_pts: ph,
            });
            all_images.push(img);
        }
    }

    crate::layout::return_engine(engine);

    Ok((all_images, all_layout_results, all_hints))
}

/// Convenience wrapper that reads `use_layout_for_markdown` and other gate
/// conditions from `config` and, when they are all satisfied, runs
/// [`run_layout_for_pdf_pages`].
///
/// Returns `(None, None, None)` when the feature is not requested, or on soft
/// failure (logged as a warning so the markdown path can continue without
/// layout hints).
#[cfg(all(feature = "pdf", feature = "layout-detection"))]
type LayoutForMarkdownOptional = (
    Option<Vec<image::RgbImage>>,
    Option<Vec<PageLayoutResult>>,
    Option<Vec<Vec<LayoutHint>>>,
);

#[cfg(all(feature = "pdf", feature = "layout-detection"))]
pub(super) fn maybe_run_layout_for_markdown(content: &[u8], config: &ExtractionConfig) -> LayoutForMarkdownOptional {
    if !config.use_layout_for_markdown {
        return (None, None, None);
    }
    let Some(ref layout_config) = config.layout else {
        return (None, None, None);
    };
    if config.force_ocr {
        // force_ocr runs every page through OCR, which has its own layout detection path.
        // Running layout here too would be wasteful and produce conflicting hints.
        return (None, None, None);
    }
    match run_layout_for_pdf_pages(content, layout_config) {
        Ok((images, results, hints)) => {
            let total_hints: usize = hints.iter().map(|h| h.len()).sum();
            tracing::info!(
                pages = images.len(),
                total_hints,
                "layout-for-markdown: detection succeeded"
            );
            (Some(images), Some(results), Some(hints))
        }
        Err(e) => {
            tracing::warn!(
                error = %e,
                "layout-for-markdown: detection failed, continuing without layout hints"
            );
            (None, None, None)
        }
    }
}
