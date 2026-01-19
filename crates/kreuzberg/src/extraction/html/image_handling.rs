//! Image handling and conversion functionality for HTML extraction.

use super::types::ExtractedInlineImage;
use html_to_markdown_rs::{InlineImage, InlineImageFormat};

/// Convert InlineImageFormat to a standardized string representation.
///
/// Handles special cases for custom formats, extracting meaningful file extensions
/// and normalizing MIME types.
pub fn inline_image_format_to_str(format: &InlineImageFormat) -> String {
    match format {
        InlineImageFormat::Png => "png".to_string(),
        InlineImageFormat::Jpeg => "jpeg".to_string(),
        InlineImageFormat::Gif => "gif".to_string(),
        InlineImageFormat::Bmp => "bmp".to_string(),
        InlineImageFormat::Webp => "webp".to_string(),
        InlineImageFormat::Svg => "svg".to_string(),
        InlineImageFormat::Other(custom) => {
            let trimmed = custom.trim();
            if trimmed.is_empty() {
                return "bin".to_string();
            }

            let lower = trimmed.to_ascii_lowercase();
            if lower.starts_with("svg") {
                return "svg".to_string();
            }

            let mut result = String::with_capacity(10);
            let mut candidate = lower.as_str();

            if let Some(idx) = candidate.find(['+', ';']) {
                candidate = &candidate[..idx];
            }

            if let Some(idx) = candidate.rfind('.') {
                candidate = &candidate[idx + 1..];
            }

            candidate = candidate.trim_start_matches("x-");

            if candidate.is_empty() {
                "bin".to_string()
            } else {
                result.push_str(candidate);
                result
            }
        }
    }
}

/// Convert a library InlineImage to an ExtractedInlineImage.
///
/// Maps the library's image representation to the extraction API's format,
/// converting the format enum to a string representation.
pub fn inline_image_to_extracted(image: InlineImage) -> ExtractedInlineImage {
    ExtractedInlineImage {
        data: image.data,
        format: inline_image_format_to_str(&image.format),
        filename: image.filename,
        description: image.description,
        dimensions: image.dimensions,
        attributes: image.attributes.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_image_format_conversion() {
        assert_eq!(inline_image_format_to_str(&InlineImageFormat::Png), "png");
        assert_eq!(inline_image_format_to_str(&InlineImageFormat::Jpeg), "jpeg");
        assert_eq!(inline_image_format_to_str(&InlineImageFormat::Svg), "svg");
    }

    #[test]
    fn test_inline_image_format_other_with_extension() {
        let format = InlineImageFormat::Other("image/x-custom.jpg".to_string());
        assert_eq!(inline_image_format_to_str(&format), "jpg");
    }

    #[test]
    fn test_inline_image_format_other_empty() {
        let format = InlineImageFormat::Other("".to_string());
        assert_eq!(inline_image_format_to_str(&format), "bin");
    }

    #[test]
    fn test_inline_image_format_other_x_prefix() {
        let format = InlineImageFormat::Other("x-custom".to_string());
        assert_eq!(inline_image_format_to_str(&format), "custom");
    }
}
