//! Shared image format detection for Office document extractors.

use std::borrow::Cow;

/// Detect image format from raw bytes using magic byte signatures.
///
/// Returns a format string like "jpeg", "png", etc. Used by both DOCX and PPTX extractors.
pub fn detect_image_format(data: &[u8]) -> Cow<'static, str> {
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Cow::Borrowed("jpeg")
    } else if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        Cow::Borrowed("png")
    } else if data.starts_with(b"GIF") {
        Cow::Borrowed("gif")
    } else if data.starts_with(b"BM") {
        Cow::Borrowed("bmp")
    } else if data.starts_with(b"<svg") || data.starts_with(b"<?xml") {
        Cow::Borrowed("svg")
    } else if data.starts_with(b"II\x2A\x00") || data.starts_with(b"MM\x00\x2A") {
        Cow::Borrowed("tiff")
    } else if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        Cow::Borrowed("webp")
    } else if data.starts_with(&[0xD7, 0xCD, 0xC6, 0x9A]) {
        Cow::Borrowed("wmf")
    } else if data.len() >= 44 && data[0..4] == [0x01, 0x00, 0x00, 0x00] && &data[40..44] == b" EMF" {
        Cow::Borrowed("emf")
    } else {
        Cow::Borrowed("unknown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jpeg() {
        assert_eq!(detect_image_format(&[0xFF, 0xD8, 0xFF, 0xE0]), "jpeg");
    }

    #[test]
    fn test_detect_png() {
        assert_eq!(detect_image_format(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A]), "png");
    }

    #[test]
    fn test_detect_gif() {
        assert_eq!(detect_image_format(b"GIF89a"), "gif");
    }

    #[test]
    fn test_detect_bmp() {
        assert_eq!(detect_image_format(b"BM\x00\x00"), "bmp");
    }

    #[test]
    fn test_detect_tiff_le() {
        assert_eq!(detect_image_format(b"II\x2A\x00"), "tiff");
    }

    #[test]
    fn test_detect_tiff_be() {
        assert_eq!(detect_image_format(b"MM\x00\x2A"), "tiff");
    }

    #[test]
    fn test_detect_webp() {
        assert_eq!(detect_image_format(b"RIFF\x00\x00\x00\x00WEBP"), "webp");
    }

    #[test]
    fn test_detect_wmf() {
        assert_eq!(detect_image_format(&[0xD7, 0xCD, 0xC6, 0x9A, 0x00]), "wmf");
    }

    #[test]
    fn test_detect_emf() {
        let mut data = vec![0x01, 0x00, 0x00, 0x00];
        data.extend(vec![0u8; 36]); // padding to reach offset 40
        data.extend(b" EMF"); // EMF signature at bytes 40-43
        assert_eq!(detect_image_format(&data), "emf");
    }

    #[test]
    fn test_detect_svg() {
        assert_eq!(detect_image_format(b"<svg xmlns="), "svg");
    }

    #[test]
    fn test_detect_unknown() {
        assert_eq!(detect_image_format(b"random data"), "unknown");
    }

    #[test]
    fn test_detect_empty() {
        assert_eq!(detect_image_format(b""), "unknown");
    }
}
