//! Bounding box geometry for PDF text positioning.
//!
//! This module provides the BoundingBox type and geometric operations used
//! for spatial analysis of text elements in PDF documents.

use serde::{Deserialize, Serialize};

/// A bounding box for text or elements.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Left x-coordinate
    pub left: f32,
    /// Top y-coordinate
    pub top: f32,
    /// Right x-coordinate
    pub right: f32,
    /// Bottom y-coordinate
    pub bottom: f32,
}

impl BoundingBox {
    /// Create a new bounding box without validation (unchecked).
    #[cfg(test)]
    pub(crate) fn new_unchecked(left: f32, top: f32, right: f32, bottom: f32) -> BoundingBox {
        BoundingBox {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Get the width of the bounding box.
    #[cfg(test)]
    pub(crate) fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Get the height of the bounding box.
    #[cfg(test)]
    pub(crate) fn height(&self) -> f32 {
        self.bottom - self.top
    }

    /// Calculate the intersection ratio relative to this bounding box's area.
    pub(crate) fn intersection_ratio(&self, other: &BoundingBox) -> f32 {
        let intersection_area = self.calculate_intersection_area(other);
        let self_area = self.calculate_area();

        if self_area <= 0.0 {
            0.0
        } else {
            intersection_area / self_area
        }
    }

    /// Calculate the center coordinates of this bounding box.
    pub(crate) fn center(&self) -> (f32, f32) {
        ((self.left + self.right) / 2.0, (self.top + self.bottom) / 2.0)
    }

    fn calculate_area(&self) -> f32 {
        let width = (self.right - self.left).max(0.0);
        let height = (self.bottom - self.top).max(0.0);
        width * height
    }

    fn calculate_intersection_area(&self, other: &BoundingBox) -> f32 {
        let left = self.left.max(other.left);
        let top = self.top.max(other.top);
        let right = self.right.min(other.right);
        let bottom = self.bottom.min(other.bottom);

        let width = (right - left).max(0.0);
        let height = (bottom - top).max(0.0);
        width * height
    }
}
