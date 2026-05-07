//! Core PDF extraction functionality.
//!
//! Handles document loading, text extraction, metadata parsing, and table detection.

use crate::Result;
use crate::core::config::{ExtractionConfig, OutputFormat};
use crate::types::{PageBoundary, PageContent, PdfAnnotation};

#[cfg(feature = "pdf")]
use crate::types::Table;

#[cfg(feature = "pdf")]
pub(crate) type PdfExtractionPhaseResult = (
    crate::pdf::metadata::PdfExtractionMetadata,
    String,
    Vec<Table>,
    Option<Vec<PageContent>>,
    Option<Vec<PageBoundary>>,
    Option<crate::types::internal::InternalDocument>, // pre-rendered structured doc (when output_format == Markdown/Djot/Html)
    bool,                                             // has_font_encoding_issues (unicode map errors detected)
    Option<Vec<PdfAnnotation>>,                       // extracted annotations (when extract_annotations is enabled)
);

/// Extract text, metadata, tables, and annotations from a PDF document using the pdf_oxide backend.
///
/// Opens the document via `OxideDocument`, then delegates to each oxide extraction module.
/// The return type is `PdfExtractionPhaseResult` so callers can switch transparently between
/// backends.
///
/// # Notes
///
/// - Layout detection is not yet supported on the oxide path.
/// - When output format is Markdown/Djot/HTML, the oxide hierarchy module extracts font
///   metrics and feeds them into the backend-agnostic structure pipeline for heading detection.
/// - Font encoding issue detection is not available; the flag is always `false`.
#[cfg(feature = "pdf")]
pub(crate) fn extract_all_from_oxide_document(
    content: &[u8],
    config: &ExtractionConfig,
    layout_hints: Option<&[Vec<crate::pdf::structure::types::LayoutHint>]>,
    #[cfg(feature = "layout-detection")] layout_images: Option<&[image::DynamicImage]>,
    #[cfg(not(feature = "layout-detection"))] _layout_images: Option<()>,
    #[cfg(feature = "layout-detection")] layout_results: Option<&[crate::pdf::structure::types::PageLayoutResult]>,
    #[cfg(not(feature = "layout-detection"))] _layout_results: Option<()>,
) -> Result<PdfExtractionPhaseResult> {
    let _span = tracing::debug_span!("extract_pdf_oxide").entered();

    let mut doc = crate::pdf::oxide::OxideDocument::open_bytes(content)?;

    // --- Text + metadata (single pass) ---
    let (native_text, boundaries, page_contents, pdf_metadata) =
        crate::pdf::oxide::text::extract_text_and_metadata(&mut doc, Some(config)).map_err(|e| {
            crate::error::KreuzbergError::Parsing {
                message: format!("pdf_oxide text extraction failed: {e}"),
                source: None,
            }
        })?;

    // --- Tables (native pdf_oxide detection) ---
    // Use unwrap_or_default so table detection failures don't block extraction.
    let tables = crate::pdf::oxide::table::extract_tables_native(&mut doc).unwrap_or_default();

    // --- Annotations ---
    let annotations = if config.pdf_options.as_ref().is_some_and(|opts| opts.extract_annotations) {
        let extracted = crate::pdf::oxide::annotations::extract_annotations(&mut doc);
        if extracted.is_empty() { None } else { Some(extracted) }
    } else {
        None
    };

    // --- Image positions for assembly pipeline ---
    let image_positions = crate::pdf::oxide::images::extract_image_positions(&mut doc).map_err(|e| {
        crate::error::KreuzbergError::Parsing {
            message: format!("pdf_oxide image position extraction failed: {e}"),
            source: None,
        }
    })?;

    // Pre-render structured document for output formats that benefit from headings.
    let needs_structured = matches!(
        config.output_format,
        OutputFormat::Markdown | OutputFormat::Djot | OutputFormat::Html
    );

    let allow_single_column = config
        .pdf_options
        .as_ref()
        .is_some_and(|o| o.allow_single_column_tables);

    let pre_rendered_doc =
        if needs_structured && !config.force_ocr {
            let k = config
                .pdf_options
                .as_ref()
                .and_then(|opts| opts.hierarchy.as_ref())
                .map(|h| h.k_clusters)
                .unwrap_or(4);

            let (strip_repeating_text, include_headers, include_footers) = config
                .content_filter
                .as_ref()
                .map(|cf| (cf.strip_repeating_text, cf.include_headers, cf.include_footers))
                .unwrap_or((true, false, false));

            // Extract font-metric segments from oxide for heading detection.
            // When the PDF has a reliable structure tree, segments carry pre-assigned
            // heading roles (assigned_role) and the pipeline can skip font-size clustering.
            let (segments, used_structure_tree) = crate::pdf::oxide::hierarchy::extract_all_segments(&mut doc)
                .map_err(|e| crate::error::KreuzbergError::Parsing {
                    message: format!("pdf_oxide hierarchy extraction failed: {e}"),
                    source: None,
                })?;

            let total_segs: usize = segments.iter().map(|s| s.len()).sum();
            tracing::debug!(
                total_segs,
                k,
                used_structure_tree,
                "oxide structure: extracted segments for heading detection"
            );

            // Same gate as the oxide path: only inject placeholders when image extraction
            // is explicitly enabled. Prevents base64 data from leaking into results when
            // the caller sets extract_images=false (fixes #796).
            let images_extraction_enabled = config.images.as_ref().map(|c| c.extract_images).unwrap_or(false)
                || config.pdf_options.as_ref().map(|p| p.extract_images).unwrap_or(false);
            let inject_placeholders =
                images_extraction_enabled && config.images.as_ref().map(|c| c.inject_placeholders).unwrap_or(true);

            match crate::pdf::structure::extract_document_structure_from_segments(
                segments,
                crate::pdf::structure::SegmentStructureConfig {
                    k_clusters: k,
                    tables: &tables,
                    strip_repeating_text,
                    include_headers,
                    include_footers,
                    used_structure_tree,
                    image_positions: &image_positions,
                    inject_placeholders,
                    layout_hints,
                    allow_single_column,
                    cancel_token: config.cancel_token.as_ref(),
                    #[cfg(feature = "layout-detection")]
                    layout_images,
                    #[cfg(feature = "layout-detection")]
                    layout_results,
                    #[cfg(feature = "layout-detection")]
                    table_model: config.layout.as_ref().map(|l| l.table_model).unwrap_or_default(),
                    #[cfg(feature = "layout-detection")]
                    acceleration: config.acceleration.as_ref(),
                },
            ) {
                Ok(structured_doc) if !structured_doc.elements.is_empty() => {
                    tracing::debug!(
                        elements = structured_doc.elements.len(),
                        has_headings = structured_doc
                            .elements
                            .iter()
                            .any(|e| matches!(e.kind, crate::types::internal::ElementKind::Heading { .. })),
                        "oxide structure: render succeeded"
                    );
                    Some(structured_doc)
                }
                Ok(_) => {
                    tracing::warn!("oxide structure: rendering produced empty output, falling back to plain text");
                    None
                }
                Err(e) => {
                    tracing::warn!("oxide structure: rendering failed: {:?}, falling back to plain text", e);
                    None
                }
            }
        } else {
            None
        };

    let has_font_encoding_issues = false;

    Ok((
        pdf_metadata,
        native_text,
        tables,
        page_contents,
        boundaries,
        pre_rendered_doc,
        has_font_encoding_issues,
        annotations,
    ))
}

/// Convert layout detection results to per-page layout hints for the markdown pipeline.
///
/// Maps `LayoutClass` (from `crate::layout`) to `LayoutHintClass` (feature-gate-free
/// types in the markdown module) and flattens per-page regions into hint vectors.
#[cfg(all(feature = "pdf", feature = "layout-detection"))]
pub(crate) fn convert_results_to_hints(
    results: &[crate::pdf::structure::types::PageLayoutResult],
) -> Vec<Vec<crate::pdf::structure::types::LayoutHint>> {
    use crate::layout::LayoutClass;
    use crate::pdf::structure::types::{LayoutHint, LayoutHintClass};

    results
        .iter()
        .enumerate()
        .map(|(page_idx, page)| {
            let hints: Vec<LayoutHint> = page
                .regions
                .iter()
                .map(|region| {
                    let class = match region.class_name {
                        LayoutClass::Title => LayoutHintClass::Title,
                        LayoutClass::SectionHeader => LayoutHintClass::SectionHeader,
                        LayoutClass::Code => LayoutHintClass::Code,
                        LayoutClass::Formula => LayoutHintClass::Formula,
                        LayoutClass::ListItem => LayoutHintClass::ListItem,
                        LayoutClass::Caption => LayoutHintClass::Caption,
                        LayoutClass::Footnote => LayoutHintClass::Footnote,
                        LayoutClass::PageHeader => LayoutHintClass::PageHeader,
                        LayoutClass::PageFooter => LayoutHintClass::PageFooter,
                        LayoutClass::Table => LayoutHintClass::Table,
                        LayoutClass::Picture => LayoutHintClass::Picture,
                        LayoutClass::Text => LayoutHintClass::Text,
                        _ => LayoutHintClass::Other,
                    };
                    LayoutHint {
                        class_name: class,
                        confidence: region.confidence,
                        left: region.bbox.left,
                        bottom: region.bbox.bottom,
                        right: region.bbox.right,
                        top: region.bbox.top,
                    }
                })
                .collect();
            tracing::trace!(
                page = page_idx,
                table_hints = hints
                    .iter()
                    .filter(|h| matches!(h.class_name, LayoutHintClass::Table))
                    .count(),
                "Layout hints for page"
            );
            hints
        })
        .collect()
}

/// Check whether words on a page exhibit column alignment consistent with a table.
///
/// Groups word left-edges into buckets and checks that at least 3 buckets each contain
/// multiple words. Two-column text layouts naturally produce 2 alignment clusters, so
/// we require ≥3 to avoid false positives from academic papers and similar documents.
#[cfg(feature = "pdf")]
fn has_column_alignment(words: &[crate::pdf::table_reconstruct::HocrWord]) -> bool {
    if words.len() < 6 {
        return false;
    }

    // Bucket word left positions using a tolerance of 15px
    const BUCKET_TOLERANCE: u32 = 15;
    let mut buckets: Vec<(u32, usize)> = Vec::new(); // (representative_x, count)

    for w in words {
        let x = w.left;
        if let Some(bucket) = buckets.iter_mut().find(|(bx, _)| x.abs_diff(*bx) <= BUCKET_TOLERANCE) {
            bucket.1 += 1;
        } else {
            buckets.push((x, 1));
        }
    }

    // Require ≥3 distinct columns with ≥3 words each.
    // Two-column text layouts have exactly 2 alignment clusters, so requiring 3
    // eliminates false positives from multi-column prose while still detecting
    // real tables (which typically have 3+ columns).
    let significant_columns = buckets.iter().filter(|(_, count)| *count >= 3).count();
    significant_columns >= 3
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_bounding_box_coordinate_conversion() {
        // Test the bounding box computation logic independently
        // Simulate words at known positions and verify the resulting bbox
        let page_height = 800.0_f64;

        // Simulated word positions in image coordinates (y=0 at top)
        // Word 1: left=50, top=100, width=200, height=20
        // Word 2: left=50, top=130, width=250, height=20
        let img_left = 50.0_f64;
        let img_top = 100.0_f64;
        let img_right = 300.0_f64; // max(50+200, 50+250)
        let img_bottom = 150.0_f64; // max(100+20, 130+20)

        let bbox = crate::types::BoundingBox {
            x0: img_left,
            y0: page_height - img_bottom, // 800 - 150 = 650
            x1: img_right,
            y1: page_height - img_top, // 800 - 100 = 700
        };

        assert_eq!(bbox.x0, 50.0);
        assert_eq!(bbox.y0, 650.0); // bottom in PDF coords
        assert_eq!(bbox.x1, 300.0);
        assert_eq!(bbox.y1, 700.0); // top in PDF coords
        // y1 > y0 confirms the table is above the bottom
        assert!(bbox.y1 > bbox.y0);
    }

    #[test]
    fn test_bounding_box_coordinate_conversion_different_scales() {
        // Test with different page height and word positions
        let page_height = 1000.0_f64;

        // Words spanning from top=50 to bottom=400
        let img_left = 100.0_f64;
        let img_top = 50.0_f64;
        let img_right = 600.0_f64;
        let img_bottom = 400.0_f64;

        let bbox = crate::types::BoundingBox {
            x0: img_left,
            y0: page_height - img_bottom, // 1000 - 400 = 600
            x1: img_right,
            y1: page_height - img_top, // 1000 - 50 = 950
        };

        assert_eq!(bbox.x0, 100.0);
        assert_eq!(bbox.y0, 600.0);
        assert_eq!(bbox.x1, 600.0);
        assert_eq!(bbox.y1, 950.0);
        // Height of table: 950 - 600 = 350 pixels
        assert_eq!(bbox.y1 - bbox.y0, 350.0);
    }

    #[test]
    fn test_bounding_box_coordinate_conversion_preserves_width() {
        // Width should be preserved during coordinate transformation
        let page_height = 595.0_f64; // Standard letter page height

        let img_left = 72.0_f64;
        let img_right = 522.0_f64; // width = 450
        let img_top = 36.0_f64;
        let img_bottom = 300.0_f64; // height = 264

        let bbox = crate::types::BoundingBox {
            x0: img_left,
            y0: page_height - img_bottom,
            x1: img_right,
            y1: page_height - img_top,
        };

        let expected_width = img_right - img_left;
        let actual_width = bbox.x1 - bbox.x0;
        assert_eq!(actual_width, expected_width);
        assert_eq!(actual_width, 450.0);
    }

    #[test]
    fn test_bounding_box_serialization_round_trip() {
        let original = crate::types::BoundingBox {
            x0: 10.5,
            y0: 20.25,
            x1: 100.75,
            y1: 200.5,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: crate::types::BoundingBox = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
        assert_eq!(deserialized.x0, 10.5);
        assert_eq!(deserialized.y0, 20.25);
        assert_eq!(deserialized.x1, 100.75);
        assert_eq!(deserialized.y1, 200.5);
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_has_column_alignment_table_layout() {
        use crate::pdf::table_reconstruct::HocrWord;

        // Simulate a 3-column table: words at x=50, x=200, x=400
        // Requires ≥3 columns with ≥3 words each to pass.
        let words = vec![
            // Row 1
            HocrWord {
                text: "Name".into(),
                left: 50,
                top: 100,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "Age".into(),
                left: 200,
                top: 100,
                width: 40,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "City".into(),
                left: 400,
                top: 100,
                width: 50,
                height: 12,
                confidence: 95.0,
            },
            // Row 2
            HocrWord {
                text: "Alice".into(),
                left: 50,
                top: 120,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "30".into(),
                left: 200,
                top: 120,
                width: 30,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "NYC".into(),
                left: 400,
                top: 120,
                width: 40,
                height: 12,
                confidence: 95.0,
            },
            // Row 3
            HocrWord {
                text: "Bob".into(),
                left: 50,
                top: 140,
                width: 50,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "25".into(),
                left: 200,
                top: 140,
                width: 30,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "LA".into(),
                left: 400,
                top: 140,
                width: 30,
                height: 12,
                confidence: 95.0,
            },
        ];
        assert!(super::has_column_alignment(&words));
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_has_column_alignment_rejects_two_column_layout() {
        use crate::pdf::table_reconstruct::HocrWord;

        // Two-column text layout (like academic papers) should NOT be detected as a table.
        let words = vec![
            HocrWord {
                text: "Left".into(),
                left: 50,
                top: 100,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "Right".into(),
                left: 300,
                top: 100,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "More".into(),
                left: 50,
                top: 120,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "Text".into(),
                left: 300,
                top: 120,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "Here".into(),
                left: 50,
                top: 140,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "Also".into(),
                left: 300,
                top: 140,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
        ];
        assert!(!super::has_column_alignment(&words));
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_has_column_alignment_body_text() {
        use crate::pdf::table_reconstruct::HocrWord;

        // Body text: words flow left-to-right on each line with distinct x positions.
        // Each word has a unique left-edge so no bucket accumulates >= 2 words,
        // meaning column alignment should NOT be detected.
        let words = vec![
            HocrWord {
                text: "This".into(),
                left: 50,
                top: 100,
                width: 40,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "is".into(),
                left: 100,
                top: 100,
                width: 20,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "some".into(),
                left: 130,
                top: 100,
                width: 45,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "body".into(),
                left: 185,
                top: 100,
                width: 45,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "text".into(),
                left: 240,
                top: 100,
                width: 40,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "here".into(),
                left: 290,
                top: 100,
                width: 40,
                height: 12,
                confidence: 95.0,
            },
        ];
        assert!(!super::has_column_alignment(&words));
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_has_column_alignment_too_few_words() {
        use crate::pdf::table_reconstruct::HocrWord;

        let words = vec![
            HocrWord {
                text: "Hello".into(),
                left: 50,
                top: 100,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
            HocrWord {
                text: "World".into(),
                left: 300,
                top: 100,
                width: 60,
                height: 12,
                confidence: 95.0,
            },
        ];
        assert!(!super::has_column_alignment(&words));
    }
}
