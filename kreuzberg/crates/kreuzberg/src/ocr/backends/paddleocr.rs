//! PaddleOCR language support.
//!
//! PaddleOCR supports 14 optimized languages for production deployments.

/// Get list of languages supported by PaddleOCR.
///
/// # Returns
///
/// A vector of 14 language codes supported by PaddleOCR.
pub(in crate::ocr) fn languages() -> Vec<String> {
    vec![
        "ch".to_string(),
        "en".to_string(),
        "french".to_string(),
        "german".to_string(),
        "korean".to_string(),
        "japan".to_string(),
        "chinese_cht".to_string(),
        "ta".to_string(),
        "te".to_string(),
        "ka".to_string(),
        "latin".to_string(),
        "arabic".to_string(),
        "cyrillic".to_string(),
        "devanagari".to_string(),
    ]
}
