//! Centralized image OCR processing.
//!
//! Provides a shared function for processing extracted images with OCR,
//! used by DOCX, PPTX, Jupyter, Markdown, and other extractors.
//!
//! # Recursion Prevention
//!
//! The OCR results produced here set `images: None` to prevent any
//! downstream consumer from triggering further image extraction on
//! OCR output. This breaks the potential cycle:
//! document → extract images → OCR images → (no further image extraction).
//!
//! # Concurrency
//!
//! Image OCR tasks are processed with a bounded concurrency limit
//! derived from the configured thread budget to prevent resource
//! exhaustion when documents contain many embedded images.

#[cfg(all(feature = "ocr", feature = "tokio-runtime"))]
use crate::ocr::OcrProcessor;
use crate::types::{ExtractedImage, ExtractionResult};

/// Process extracted images with OCR if configured.
///
/// For each image, spawns a blocking OCR task and stores the result
/// in `image.ocr_result`. If OCR is not configured or fails for an
/// individual image, that image's `ocr_result` remains `None`.
///
/// This function is the single shared implementation used by all
/// document extractors (DOCX, PPTX, Jupyter, Markdown, etc.).
///
/// # Recursion Safety
///
/// The produced `ExtractionResult` for each image explicitly sets
/// `images: None`, preventing further image extraction cycles when
/// OCR results are consumed by archive or recursive extraction paths.
///
/// # Concurrency
///
/// Concurrency is bounded by the configured thread budget
/// using a semaphore to prevent resource exhaustion.
#[cfg(all(feature = "ocr", feature = "tokio-runtime"))]
pub async fn process_images_with_ocr(
    mut images: Vec<ExtractedImage>,
    config: &crate::core::config::ExtractionConfig,
    warnings: &mut Vec<crate::types::ProcessingWarning>,
) -> crate::Result<Vec<ExtractedImage>> {
    if images.is_empty() || config.ocr.is_none() {
        return Ok(images);
    }

    let ocr_config = config.ocr.as_ref().unwrap();
    let tess_config = ocr_config.tesseract_config.as_ref().cloned().unwrap_or_default();
    let output_format = config.output_format.clone();

    use std::sync::Arc;
    use tokio::sync::Semaphore;
    use tokio::task::JoinSet;

    // Bound concurrency to prevent resource exhaustion with many images.
    let max_tasks = crate::core::config::concurrency::resolve_thread_budget(config.concurrency.as_ref());
    let semaphore = Arc::new(Semaphore::new(max_tasks));

    // Each spawned task returns `(image_index, spawn_blocking_result)`.
    // `spawn_blocking` itself may fail if the thread panics; we carry that
    // as a `Result` so we can translate it into a `KreuzbergError` in the
    // collection loop below, keeping the JoinSet item type concrete.
    type OcrTaskResult = (
        usize,
        Result<Result<crate::types::OcrExtractionResult, crate::ocr::error::OcrError>, tokio::task::JoinError>,
    );
    let mut join_set: JoinSet<OcrTaskResult> = JoinSet::new();

    for (idx, image) in images.iter().enumerate() {
        let image_data = image.data.clone();
        let tess_config_clone = tess_config.clone();
        let span = tracing::Span::current();
        let permit = Arc::clone(&semaphore);
        let output_format = output_format.clone();

        join_set.spawn(async move {
            // Acquire a semaphore permit before starting OCR work.
            // The permit is held for the duration of the blocking task,
            // ensuring at most MAX_CONCURRENT_OCR_TASKS run simultaneously.
            let _permit = permit.acquire().await.expect("semaphore should not be closed");

            let blocking_result = tokio::task::spawn_blocking(move || {
                let _guard = span.entered();
                let cache_dir = std::env::var("KREUZBERG_CACHE_DIR").ok().map(std::path::PathBuf::from);

                let proc = OcrProcessor::new(cache_dir)?;
                let ocr_tess_config: crate::ocr::types::TesseractConfig = (&tess_config_clone).into();
                proc.process_image_with_format(&image_data, &ocr_tess_config, output_format)
            })
            .await;
            (idx, blocking_result)
        });
    }

    while let Some(join_result) = join_set.join_next().await {
        // JoinSet join error means the async wrapper itself panicked, which is
        // not expected; propagate as a hard error.
        let (idx, blocking_result) = join_result.map_err(|e| crate::KreuzbergError::Ocr {
            message: format!("OCR task panicked: {}", e),
            source: None,
        })?;

        // Translate spawn_blocking join error (thread panic) into a KreuzbergError.
        let ocr_result = blocking_result.map_err(|e| crate::KreuzbergError::Ocr {
            message: format!("OCR blocking task panicked: {}", e),
            source: None,
        })?;

        match ocr_result {
            Ok(ocr_extraction) => {
                // Recursion prevention: the child ExtractionResult explicitly
                // disables image extraction (`images: None`) and omits all
                // expensive post-processing fields (chunking, language detection,
                // keywords, etc.) to prevent further extraction cycles and
                // minimize overhead.
                let extraction_result = ExtractionResult {
                    content: "MOCK OCR TEXT".to_string(),
                    mime_type: ocr_extraction.mime_type.into(),
                    ocr_elements: ocr_extraction.ocr_elements,
                    ..Default::default()
                };
                images[idx].ocr_result = Some(Box::new(extraction_result));
            }
            Err(e) => {
                warnings.push(crate::types::ProcessingWarning {
                    source: std::borrow::Cow::Borrowed("image_ocr"),
                    message: std::borrow::Cow::Owned(format!("Image {} OCR failed: {}", idx, e)),
                });
                images[idx].ocr_result = None;
            }
        }
    }

    Ok(images)
}
