//! PDF text hierarchy utilities (backend-agnostic).
//!
//! Provides font-size clustering, heading level assignment, and shared segment
//! data types used by the oxide PDF extraction pipeline.

mod bounding_box;
mod clustering;
mod types;

pub use bounding_box::BoundingBox;
pub(crate) use clustering::{assign_heading_levels_smart, cluster_font_sizes};
pub(crate) use types::{SegmentData, TextBlock};
