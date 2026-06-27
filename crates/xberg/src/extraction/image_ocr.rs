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

use crate::types::{ExtractedDocument, ExtractedImage};

/// Process extracted images with OCR if configured.
///
/// For each image, spawns an async OCR task using the backend from the registry
/// and stores the result in `image.ocr_result`. If OCR is not configured or
/// fails for an individual image, that image's `ocr_result` remains `None`.
///
/// This function is the single shared implementation used by all
/// document extractors (DOCX, PPTX, Jupyter, Markdown, etc.).
///
/// # Recursion Safety
///
/// The produced `ExtractedDocument` for each image explicitly sets
/// `images: None`, preventing further image extraction cycles when
/// OCR results are consumed by archive or recursive extraction paths.
///
/// # Concurrency
///
/// Concurrency is bounded by the configured thread budget
/// using a semaphore to prevent resource exhaustion.
#[cfg(all(feature = "ocr", feature = "tokio-runtime"))]
pub(crate) async fn process_images_with_ocr(
    mut images: Vec<ExtractedImage>,
    config: &crate::core::config::ExtractionConfig,
    warnings: &mut Vec<crate::types::ProcessingWarning>,
) -> crate::Result<Vec<ExtractedImage>> {
    if images.is_empty() || config.ocr.is_none() {
        return Ok(images);
    }

    let ocr_config = config.ocr.as_ref().unwrap();
    let output_format = config.output_format.clone();
    let acceleration = ocr_config.acceleration.clone();

    use std::sync::Arc;
    use tokio::sync::Semaphore;
    use tokio::task::JoinSet;

    // Bound concurrency to prevent resource exhaustion with many images.
    let max_tasks = crate::core::config::concurrency::resolve_thread_budget(config.concurrency.as_ref());
    let semaphore = Arc::new(Semaphore::new(max_tasks));

    // Each spawned task returns `(image_index, ocr_result)`.
    type OcrTaskResult = (usize, crate::Result<ExtractedDocument>);
    let mut join_set: JoinSet<OcrTaskResult> = JoinSet::new();

    for (idx, image) in images.iter().enumerate() {
        let image_data = image.data.clone();
        let permit = Arc::clone(&semaphore);
        let mut ocr_config_clone = ocr_config.clone();
        ocr_config_clone.output_format = Some(output_format.clone());
        ocr_config_clone.acceleration = acceleration.clone();

        join_set.spawn(async move {
            // Acquire a semaphore permit before starting OCR work.
            // The permit is held for the duration of the OCR task,
            // ensuring at most max_tasks run simultaneously.
            let _permit = match permit.acquire().await {
                Ok(p) => p,
                Err(_) => {
                    return (
                        idx,
                        Err(crate::XbergError::Ocr {
                            message: "OCR concurrency semaphore closed unexpectedly".to_string(),
                            source: None,
                        }),
                    );
                }
            };

            let backend = {
                let registry = crate::plugins::registry::get_ocr_backend_registry();
                let registry = registry.read();
                match registry.get(&ocr_config_clone.backend) {
                    Ok(b) => b.clone(),
                    Err(e) => {
                        return (
                            idx,
                            Err(crate::XbergError::Ocr {
                                message: format!("OCR backend '{}' not found: {}", ocr_config_clone.backend, e),
                                source: None,
                            }),
                        );
                    }
                }
            };

            let ocr_result = backend.process_image(&image_data, &ocr_config_clone).await;
            (idx, ocr_result)
        });
    }

    while let Some(join_result) = join_set.join_next().await {
        // JoinSet join error means the async wrapper itself panicked, which is
        // not expected; propagate as a hard error.
        let (idx, ocr_result) = join_result.map_err(|e| crate::XbergError::Ocr {
            message: format!("OCR task panicked: {}", e),
            source: None,
        })?;

        match ocr_result {
            Ok(extraction_result) => {
                // Recursion prevention: the child ExtractedDocument explicitly
                // disables image extraction (`images: None`) and omits all
                // expensive post-processing fields (chunking, language detection,
                // keywords, etc.) to prevent further extraction cycles and
                // minimize overhead.
                images[idx].ocr_result = Some(Box::new(ExtractedDocument {
                    content: extraction_result.content,
                    mime_type: extraction_result.mime_type,
                    ocr_elements: extraction_result.ocr_elements,
                    ..Default::default()
                }));
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
