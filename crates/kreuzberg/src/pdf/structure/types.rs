//! Core types for the PDF-to-Markdown pipeline.

use crate::pdf::hierarchy::SegmentData;

/// A line of text composed of segments sharing a common baseline.
#[derive(Debug, Clone)]
pub(crate) struct PdfLine {
    pub segments: Vec<SegmentData>,
    pub baseline_y: f32,
    #[allow(dead_code)]
    pub dominant_font_size: f32,
    #[allow(dead_code)]
    pub is_bold: bool,
    pub is_monospace: bool,
}

/// A paragraph composed of lines, with optional heading classification.
#[derive(Debug, Clone)]
pub(crate) struct PdfParagraph {
    /// Full text content from `page.text().all()` (heuristic path).
    /// When populated, assembly uses this directly instead of joining segment texts.
    /// Empty for the structure tree path (which uses lines/segments).
    pub text: String,
    pub lines: Vec<PdfLine>,
    pub dominant_font_size: f32,
    pub heading_level: Option<u8>,
    pub is_bold: bool,
    pub is_list_item: bool,
    pub is_code_block: bool,
    pub is_formula: bool,
    pub is_page_furniture: bool,
    pub layout_class: Option<LayoutHintClass>,
    /// Index of the parent element this caption is associated with (tables/pictures).
    pub caption_for: Option<usize>,
    /// Block-level bounding box from structure tree extraction.
    /// Used for spatial matching when per-segment positions aren't available.
    /// Format: (left, bottom, right, top) in PDF coordinate space.
    pub block_bbox: Option<(f32, f32, f32, f32)>,
}

impl PdfParagraph {
    /// Check if this paragraph is monospace (full-text path uses is_code_block flag,
    /// structure tree path checks line-level flags).
    pub(crate) fn is_monospace_hint(&self) -> bool {
        self.is_code_block
    }
}

/// Simplified layout class for the markdown pipeline.
///
/// Decoupled from `crate::layout::LayoutClass` so the markdown module
/// compiles without the `layout-detection` feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Variants constructed via layout-detection feature
pub(crate) enum LayoutHintClass {
    Title,
    SectionHeader,
    Code,
    Formula,
    ListItem,
    Caption,
    Footnote,
    PageHeader,
    PageFooter,
    Table,
    Picture,
    Text,
    Other,
}

/// A layout hint for paragraph classification.
///
/// Contains a simplified layout class with confidence and bounding box
/// in PDF coordinate space (points, y=0 at bottom of page).
#[derive(Debug, Clone)]
pub(crate) struct LayoutHint {
    pub class_name: LayoutHintClass,
    pub confidence: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

/// Bounding box in PDF coordinate space (points, y=0 at bottom of page).
#[cfg(feature = "layout-detection")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PdfLayoutBBox {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

#[cfg(feature = "layout-detection")]
impl PdfLayoutBBox {
    pub(crate) fn width(&self) -> f32 {
        (self.right - self.left).max(0.0)
    }

    pub(crate) fn height(&self) -> f32 {
        (self.top - self.bottom).max(0.0)
    }
}

/// A detected layout region mapped to PDF coordinate space.
#[cfg(feature = "layout-detection")]
#[derive(Debug, Clone)]
pub struct PageLayoutRegion {
    pub class_name: crate::layout::LayoutClass,
    pub confidence: f32,
    pub bbox: PdfLayoutBBox,
}

/// Layout detection results for a single page.
///
/// Carries the per-page bounding boxes from layout detection (in PDF coordinate space)
/// and the pixel dimensions of the rendered image used for detection.
/// Used by table recognition models to map pixel predictions back to PDF coordinates.
#[cfg(feature = "layout-detection")]
#[derive(Debug, Clone)]
pub struct PageLayoutResult {
    pub page_index: usize,
    pub regions: Vec<PageLayoutRegion>,
    pub page_width_pts: f32,
    pub page_height_pts: f32,
    /// Width of the rendered image used for layout detection (pixels).
    pub render_width_px: u32,
    /// Height of the rendered image used for layout detection (pixels).
    pub render_height_px: u32,
}
