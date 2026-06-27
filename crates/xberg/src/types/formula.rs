//! Mathematical formula extracted from a document.

use serde::{Deserialize, Serialize};

use super::extraction::BoundingBox;

/// A mathematical formula detected and recognized in a document.
///
/// Populated by the layout-guided formula pipeline: regions classified as
/// `LayoutClass::Formula` are routed to the formula OCR task, which returns the
/// LaTeX source for the region. The field is always present on
/// [`ExtractedDocument`](super::extraction::ExtractedDocument) but only populated
/// when the `layout-detection` feature is active and the document contains
/// formula regions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Formula {
    /// LaTeX source of the recognized formula, without surrounding `$$` delimiters.
    ///
    /// This field contains the raw LaTeX code as produced by the OCR backend.
    /// To render the formula in Markdown or other formats, wrap with `$$..$$` delimiters as needed.
    pub latex: String,

    /// Bounding box of the formula region on its page, in rendered-image pixel coordinates.
    ///
    /// The coordinates are in the space of the OCR-rendered page image at the OCR DPI
    /// (typically 300 DPI). These coordinates are NOT comparable to bounding boxes from
    /// native PDF text extraction, which use PDF point coordinates.
    pub bbox: BoundingBox,

    /// 1-indexed page number the formula appears on in the document.
    ///
    /// This is set by the extraction pipeline based on which page the formula was found on.
    pub page: u32,
}
