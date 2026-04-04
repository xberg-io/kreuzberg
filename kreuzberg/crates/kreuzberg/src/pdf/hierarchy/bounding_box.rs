//! Bounding box geometry for PDF text positioning.
//!
//! This module provides the BoundingBox type and geometric operations used
//! for spatial analysis of text elements in PDF documents.

// Constants for weighted distance calculation
const WEIGHTED_DISTANCE_X_WEIGHT: f32 = 5.0;
const WEIGHTED_DISTANCE_Y_WEIGHT: f32 = 1.0;

/// A bounding box for text or elements.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Create a new bounding box with zero-size validation.
    ///
    /// # Arguments
    ///
    /// * `left` - Left x-coordinate
    /// * `top` - Top y-coordinate
    /// * `right` - Right x-coordinate
    /// * `bottom` - Bottom y-coordinate
    ///
    /// # Returns
    ///
    /// `Ok(BoundingBox)` if the box has non-zero area, or
    /// `Err` if the box has zero width or height
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Width (`right - left`) is less than 1e-10 (near-zero)
    /// - Height (`bottom - top`) is less than 1e-10 (near-zero)
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> std::result::Result<BoundingBox, String> {
        let width = (right - left).abs();
        let height = (bottom - top).abs();

        if width < 1e-10 || height < 1e-10 {
            return Err(format!(
                "BoundingBox has zero or near-zero area: width={}, height={}",
                width, height
            ));
        }

        Ok(BoundingBox {
            left,
            top,
            right,
            bottom,
        })
    }

    /// Create a new bounding box without validation (unchecked).
    ///
    /// This is useful when you know the coordinates are valid or want to
    /// defer validation. Use with caution - invalid boxes may cause issues
    /// in calculations like area, width, and height.
    ///
    /// # Arguments
    ///
    /// * `left` - Left x-coordinate
    /// * `top` - Top y-coordinate
    /// * `right` - Right x-coordinate
    /// * `bottom` - Bottom y-coordinate
    ///
    /// # Returns
    ///
    /// A BoundingBox without any validation
    pub fn new_unchecked(left: f32, top: f32, right: f32, bottom: f32) -> BoundingBox {
        BoundingBox {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Get the width of the bounding box.
    ///
    /// # Returns
    ///
    /// The width (right - left). No absolute value is taken as
    /// the BoundingBox::new() constructor ensures correct ordering.
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Get the height of the bounding box.
    ///
    /// # Returns
    ///
    /// The height (bottom - top). No absolute value is taken as
    /// the BoundingBox::new() constructor ensures correct ordering.
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    /// Calculate the Intersection over Union (IOU) between this bounding box and another.
    ///
    /// IOU = intersection_area / union_area
    ///
    /// # Arguments
    ///
    /// * `other` - The other bounding box to compare with
    ///
    /// # Returns
    ///
    /// The IOU value between 0.0 and 1.0
    pub fn iou(&self, other: &BoundingBox) -> f32 {
        let intersection_area = self.calculate_intersection_area(other);
        let self_area = self.calculate_area();
        let other_area = other.calculate_area();
        let union_area = self_area + other_area - intersection_area;

        if union_area <= 0.0 {
            0.0
        } else {
            intersection_area / union_area
        }
    }

    /// Calculate the weighted distance between the centers of two bounding boxes.
    ///
    /// The distance is weighted with X-axis having weight 5.0 and Y-axis having weight 1.0.
    /// This reflects the greater importance of horizontal distance in text layout.
    ///
    /// # Arguments
    ///
    /// * `other` - The other bounding box to compare with
    ///
    /// # Returns
    ///
    /// The weighted distance value
    pub fn weighted_distance(&self, other: &BoundingBox) -> f32 {
        let (self_center_x, self_center_y) = self.center();
        let (other_center_x, other_center_y) = other.center();

        let dx = (self_center_x - other_center_x).abs();
        let dy = (self_center_y - other_center_y).abs();

        dx * WEIGHTED_DISTANCE_X_WEIGHT + dy * WEIGHTED_DISTANCE_Y_WEIGHT
    }

    /// Calculate the intersection ratio relative to this bounding box's area.
    ///
    /// intersection_ratio = intersection_area / self_area
    ///
    /// # Arguments
    ///
    /// * `other` - The other bounding box to compare with
    ///
    /// # Returns
    ///
    /// The intersection ratio between 0.0 and 1.0
    pub fn intersection_ratio(&self, other: &BoundingBox) -> f32 {
        let intersection_area = self.calculate_intersection_area(other);
        let self_area = self.calculate_area();

        if self_area <= 0.0 {
            0.0
        } else {
            intersection_area / self_area
        }
    }

    /// Check if this bounding box contains another bounding box.
    pub fn contains(&self, other: &BoundingBox) -> bool {
        other.left >= self.left && other.right <= self.right && other.top >= self.top && other.bottom <= self.bottom
    }

    /// Calculate the center coordinates of this bounding box.
    pub fn center(&self) -> (f32, f32) {
        ((self.left + self.right) / 2.0, (self.top + self.bottom) / 2.0)
    }

    /// Merge this bounding box with another, creating a box that contains both.
    pub fn merge(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }

    /// Calculate a relaxed IOU with an expansion factor.
    pub fn relaxed_iou(&self, other: &BoundingBox, relaxation: f32) -> f32 {
        let self_width = self.right - self.left;
        let self_height = self.bottom - self.top;
        let self_expansion = relaxation * self_width.min(self_height).max(0.0);

        let other_width = other.right - other.left;
        let other_height = other.bottom - other.top;
        let other_expansion = relaxation * other_width.min(other_height).max(0.0);

        let expanded_self = BoundingBox {
            left: (self.left - self_expansion).max(0.0),
            top: (self.top - self_expansion).max(0.0),
            right: self.right + self_expansion,
            bottom: self.bottom + self_expansion,
        };

        let expanded_other = BoundingBox {
            left: (other.left - other_expansion).max(0.0),
            top: (other.top - other_expansion).max(0.0),
            right: other.right + other_expansion,
            bottom: other.bottom + other_expansion,
        };

        expanded_self.iou(&expanded_other)
    }

    /// Calculate the area of this bounding box.
    fn calculate_area(&self) -> f32 {
        let width = (self.right - self.left).max(0.0);
        let height = (self.bottom - self.top).max(0.0);
        width * height
    }

    /// Calculate the intersection area between this bounding box and another.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_new_valid() {
        let bbox = BoundingBox::new(10.0, 20.0, 30.0, 40.0);
        assert!(bbox.is_ok());
        let bbox = bbox.unwrap();
        assert_eq!(bbox.width(), 20.0);
        assert_eq!(bbox.height(), 20.0);
    }

    #[test]
    fn test_bounding_box_new_zero_width() {
        let bbox = BoundingBox::new(10.0, 20.0, 10.0, 40.0);
        assert!(bbox.is_err());
        let error_msg = bbox.unwrap_err();
        assert!(error_msg.contains("zero or near-zero area"));
    }

    #[test]
    fn test_bounding_box_new_zero_height() {
        let bbox = BoundingBox::new(10.0, 20.0, 30.0, 20.0);
        assert!(bbox.is_err());
        let error_msg = bbox.unwrap_err();
        assert!(error_msg.contains("zero or near-zero area"));
    }

    #[test]
    fn test_bounding_box_new_unchecked() {
        let bbox = BoundingBox::new_unchecked(10.0, 20.0, 30.0, 40.0);
        assert_eq!(bbox.width(), 20.0);
        assert_eq!(bbox.height(), 20.0);
    }

    #[test]
    fn test_bounding_box_width_and_height() {
        let bbox = BoundingBox {
            left: 5.0,
            top: 10.0,
            right: 25.0,
            bottom: 50.0,
        };
        assert_eq!(bbox.width(), 20.0);
        assert_eq!(bbox.height(), 40.0);
    }
}
