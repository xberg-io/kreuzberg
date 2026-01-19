//! Validation and normalization for server configuration.
//!
//! This module provides functionality to validate and normalize server configuration
//! values, including legacy field migration.

/// Normalize legacy field values for backward compatibility.
///
/// If `max_upload_mb` is set, it will be converted to bytes and used to
/// override `max_multipart_field_bytes`. This allows old configurations
/// using the legacy field to continue working.
pub fn normalize_legacy_fields(max_upload_mb: Option<usize>, max_multipart_field_bytes: &mut usize) {
    if let Some(max_upload_mb_value) = max_upload_mb {
        // Convert MB to bytes
        let max_bytes = max_upload_mb_value.saturating_mul(1_048_576);
        *max_multipart_field_bytes = max_bytes;
    }
}
