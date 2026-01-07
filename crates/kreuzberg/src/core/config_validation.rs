//! Configuration validation module.
//!
//! Provides centralized validation for configuration values across all bindings.
//! This eliminates duplication of validation logic in Python, TypeScript, Java, Go, and other language bindings.
//!
//! All validation functions return `Result<()>` and produce detailed error messages
//! suitable for user-facing error handling.
//!
//! # Examples
//!
//! ```rust
//! use kreuzberg::core::config_validation::{
//!     validate_binarization_method,
//!     validate_token_reduction_level,
//!     validate_language_code,
//! };
//!
//! // Valid values
//! assert!(validate_binarization_method("otsu").is_ok());
//! assert!(validate_token_reduction_level("moderate").is_ok());
//! assert!(validate_language_code("en").is_ok());
//!
//! // Invalid values
//! assert!(validate_binarization_method("invalid").is_err());
//! assert!(validate_token_reduction_level("extreme").is_err());
//! ```

use crate::{KreuzbergError, Result};

/// Valid binarization methods for image preprocessing.
const VALID_BINARIZATION_METHODS: &[&str] = &["otsu", "adaptive", "sauvola"];

/// Valid token reduction levels.
const VALID_TOKEN_REDUCTION_LEVELS: &[&str] = &["off", "light", "moderate", "aggressive", "maximum"];

/// Valid OCR backends.
const VALID_OCR_BACKENDS: &[&str] = &["tesseract", "easyocr", "paddleocr"];

/// Common ISO 639-1 language codes (extended list).
/// Covers most major languages and variants used in document processing.
const VALID_LANGUAGE_CODES: &[&str] = &[
    "en", "de", "fr", "es", "it", "pt", "nl", "pl", "ru", "zh", "ja", "ko", "bg", "cs", "da", "el", "et", "fi", "hu",
    "lt", "lv", "ro", "sk", "sl", "sv", "uk", "ar", "hi", "th", "tr", "vi", "eng", "deu", "fra", "spa", "ita", "por",
    "nld", "pol", "rus", "zho", "jpn", "kor", "ces", "dan", "ell", "est", "fin", "hun", "lit", "lav", "ron", "slk",
    "slv", "swe", "tur",
];

/// Valid tesseract PSM (Page Segmentation Mode) values.
const VALID_TESSERACT_PSM: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

/// Valid tesseract OEM (OCR Engine Mode) values.
const VALID_TESSERACT_OEM: &[i32] = &[0, 1, 2, 3];

/// Valid output formats for tesseract.
const VALID_OUTPUT_FORMATS: &[&str] = &["text", "markdown"];

/// Validate a binarization method string.
///
/// # Arguments
///
/// * `method` - The binarization method to validate (e.g., "otsu", "adaptive", "sauvola")
///
/// # Returns
///
/// `Ok(())` if the method is valid, or a `ValidationError` with details about valid options.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_binarization_method;
///
/// assert!(validate_binarization_method("otsu").is_ok());
/// assert!(validate_binarization_method("adaptive").is_ok());
/// assert!(validate_binarization_method("invalid").is_err());
/// ```
pub fn validate_binarization_method(method: &str) -> Result<()> {
    let method = method.to_lowercase();
    if VALID_BINARIZATION_METHODS.contains(&method.as_str()) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid binarization method '{}'. Valid options are: {}",
                method,
                VALID_BINARIZATION_METHODS.join(", ")
            ),
            source: None,
        })
    }
}

/// Validate a token reduction level string.
///
/// # Arguments
///
/// * `level` - The token reduction level to validate (e.g., "off", "light", "moderate")
///
/// # Returns
///
/// `Ok(())` if the level is valid, or a `ValidationError` with details about valid options.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_token_reduction_level;
///
/// assert!(validate_token_reduction_level("off").is_ok());
/// assert!(validate_token_reduction_level("moderate").is_ok());
/// assert!(validate_token_reduction_level("extreme").is_err());
/// ```
pub fn validate_token_reduction_level(level: &str) -> Result<()> {
    let level = level.to_lowercase();
    if VALID_TOKEN_REDUCTION_LEVELS.contains(&level.as_str()) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid token reduction level '{}'. Valid options are: {}",
                level,
                VALID_TOKEN_REDUCTION_LEVELS.join(", ")
            ),
            source: None,
        })
    }
}

/// Validate an OCR backend string.
///
/// # Arguments
///
/// * `backend` - The OCR backend to validate (e.g., "tesseract", "easyocr", "paddleocr")
///
/// # Returns
///
/// `Ok(())` if the backend is valid, or a `ValidationError` with details about valid options.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_ocr_backend;
///
/// assert!(validate_ocr_backend("tesseract").is_ok());
/// assert!(validate_ocr_backend("easyocr").is_ok());
/// assert!(validate_ocr_backend("invalid").is_err());
/// ```
pub fn validate_ocr_backend(backend: &str) -> Result<()> {
    let backend = backend.to_lowercase();
    if VALID_OCR_BACKENDS.contains(&backend.as_str()) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid OCR backend '{}'. Valid options are: {}",
                backend,
                VALID_OCR_BACKENDS.join(", ")
            ),
            source: None,
        })
    }
}

/// Validate a language code (ISO 639-1 or 639-3 format).
///
/// Accepts both 2-letter ISO 639-1 codes (e.g., "en", "de") and
/// 3-letter ISO 639-3 codes (e.g., "eng", "deu") for broader compatibility.
///
/// # Arguments
///
/// * `code` - The language code to validate
///
/// # Returns
///
/// `Ok(())` if the code is valid, or a `ValidationError` indicating an invalid language code.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_language_code;
///
/// assert!(validate_language_code("en").is_ok());
/// assert!(validate_language_code("eng").is_ok());
/// assert!(validate_language_code("de").is_ok());
/// assert!(validate_language_code("deu").is_ok());
/// assert!(validate_language_code("invalid").is_err());
/// ```
pub fn validate_language_code(code: &str) -> Result<()> {
    let code_lower = code.to_lowercase();

    if VALID_LANGUAGE_CODES.contains(&code_lower.as_str()) {
        return Ok(());
    }

    Err(KreuzbergError::Validation {
        message: format!(
            "Invalid language code '{}'. Use ISO 639-1 (2-letter, e.g., 'en', 'de') \
             or ISO 639-3 (3-letter, e.g., 'eng', 'deu') codes. \
             Common codes: en, de, fr, es, it, pt, nl, pl, ru, zh, ja, ko, ar, hi, th.",
            code
        ),
        source: None,
    })
}

/// Validate a tesseract Page Segmentation Mode (PSM).
///
/// # Arguments
///
/// * `psm` - The PSM value to validate (0-13)
///
/// # Returns
///
/// `Ok(())` if the PSM is valid, or a `ValidationError` with details about valid ranges.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_tesseract_psm;
///
/// assert!(validate_tesseract_psm(3).is_ok());  // Fully automatic
/// assert!(validate_tesseract_psm(6).is_ok());  // Single block of text
/// assert!(validate_tesseract_psm(14).is_err()); // Out of range
/// ```
pub fn validate_tesseract_psm(psm: i32) -> Result<()> {
    if VALID_TESSERACT_PSM.contains(&psm) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid tesseract PSM value '{}'. Valid range is 0-13. \
                 Common values: 3 (auto), 6 (single block), 11 (sparse text).",
                psm
            ),
            source: None,
        })
    }
}

/// Validate a tesseract OCR Engine Mode (OEM).
///
/// # Arguments
///
/// * `oem` - The OEM value to validate (0-3)
///
/// # Returns
///
/// `Ok(())` if the OEM is valid, or a `ValidationError` with details about valid options.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_tesseract_oem;
///
/// assert!(validate_tesseract_oem(1).is_ok());  // Neural nets (LSTM)
/// assert!(validate_tesseract_oem(2).is_ok());  // Legacy + LSTM
/// assert!(validate_tesseract_oem(4).is_err()); // Out of range
/// ```
pub fn validate_tesseract_oem(oem: i32) -> Result<()> {
    if VALID_TESSERACT_OEM.contains(&oem) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid tesseract OEM value '{}'. Valid range is 0-3. \
                 0=Legacy, 1=LSTM, 2=Legacy+LSTM, 3=Default",
                oem
            ),
            source: None,
        })
    }
}

/// Validate a tesseract output format.
///
/// # Arguments
///
/// * `format` - The output format to validate (e.g., "text", "markdown")
///
/// # Returns
///
/// `Ok(())` if the format is valid, or a `ValidationError` with details about valid options.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_output_format;
///
/// assert!(validate_output_format("text").is_ok());
/// assert!(validate_output_format("markdown").is_ok());
/// assert!(validate_output_format("json").is_err());
/// ```
pub fn validate_output_format(format: &str) -> Result<()> {
    let format = format.to_lowercase();
    if VALID_OUTPUT_FORMATS.contains(&format.as_str()) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid output format '{}'. Valid options are: {}",
                format,
                VALID_OUTPUT_FORMATS.join(", ")
            ),
            source: None,
        })
    }
}

/// Validate a confidence threshold value.
///
/// Confidence thresholds should be between 0.0 and 1.0 inclusive.
///
/// # Arguments
///
/// * `confidence` - The confidence threshold to validate
///
/// # Returns
///
/// `Ok(())` if the confidence is valid, or a `ValidationError` with details about valid ranges.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_confidence;
///
/// assert!(validate_confidence(0.5).is_ok());
/// assert!(validate_confidence(0.0).is_ok());
/// assert!(validate_confidence(1.0).is_ok());
/// assert!(validate_confidence(1.5).is_err());
/// assert!(validate_confidence(-0.1).is_err());
/// ```
pub fn validate_confidence(confidence: f64) -> Result<()> {
    if (0.0..=1.0).contains(&confidence) {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid confidence threshold '{}'. Must be between 0.0 and 1.0.",
                confidence
            ),
            source: None,
        })
    }
}

/// Validate a DPI (dots per inch) value.
///
/// DPI should be a positive integer, typically 72-600.
///
/// # Arguments
///
/// * `dpi` - The DPI value to validate
///
/// # Returns
///
/// `Ok(())` if the DPI is valid, or a `ValidationError` with details about valid ranges.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_dpi;
///
/// assert!(validate_dpi(96).is_ok());
/// assert!(validate_dpi(300).is_ok());
/// assert!(validate_dpi(0).is_err());
/// assert!(validate_dpi(-1).is_err());
/// ```
pub fn validate_dpi(dpi: i32) -> Result<()> {
    if dpi > 0 && dpi <= 2400 {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!(
                "Invalid DPI value '{}'. Must be a positive integer, typically 72-600.",
                dpi
            ),
            source: None,
        })
    }
}

/// Validate chunk size parameters.
///
/// Checks that max_chars > 0 and max_overlap < max_chars.
///
/// # Arguments
///
/// * `max_chars` - The maximum characters per chunk
/// * `max_overlap` - The maximum overlap between chunks
///
/// # Returns
///
/// `Ok(())` if the parameters are valid, or a `ValidationError` with details about constraints.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_chunking_params;
///
/// assert!(validate_chunking_params(1000, 200).is_ok());
/// assert!(validate_chunking_params(500, 50).is_ok());
/// assert!(validate_chunking_params(0, 100).is_err()); // max_chars must be > 0
/// assert!(validate_chunking_params(100, 150).is_err()); // overlap >= max_chars
/// ```
pub fn validate_chunking_params(max_chars: usize, max_overlap: usize) -> Result<()> {
    if max_chars == 0 {
        return Err(KreuzbergError::Validation {
            message: "max_chars must be greater than 0".to_string(),
            source: None,
        });
    }

    if max_overlap >= max_chars {
        return Err(KreuzbergError::Validation {
            message: format!(
                "max_overlap ({}) must be less than max_chars ({})",
                max_overlap, max_chars
            ),
            source: None,
        });
    }

    Ok(())
}

/// Validate a port number for server configuration.
///
/// Port must be in the range 1-65535. While ports 1-1023 are privileged and may require
/// special permissions on some systems, they are still valid port numbers.
///
/// # Arguments
///
/// * `port` - The port number to validate
///
/// # Returns
///
/// `Ok(())` if the port is valid, or a `ValidationError` with details about valid ranges.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_port;
///
/// assert!(validate_port(8000).is_ok());
/// assert!(validate_port(80).is_ok());
/// assert!(validate_port(1).is_ok());
/// assert!(validate_port(65535).is_ok());
/// assert!(validate_port(0).is_err());
/// assert!(validate_port(65536).is_err());
/// ```
pub fn validate_port(port: u16) -> Result<()> {
    if port > 0 {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!("Port must be 1-65535, got {}", port),
            source: None,
        })
    }
}

/// Validate a host/IP address string for server configuration.
///
/// Accepts valid IPv4 addresses (e.g., "127.0.0.1", "0.0.0.0"), valid IPv6 addresses
/// (e.g., "::1", "::"), and hostnames (e.g., "localhost", "example.com").
///
/// # Arguments
///
/// * `host` - The host/IP address string to validate
///
/// # Returns
///
/// `Ok(())` if the host is valid, or a `ValidationError` with details about valid formats.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_host;
///
/// assert!(validate_host("127.0.0.1").is_ok());
/// assert!(validate_host("0.0.0.0").is_ok());
/// assert!(validate_host("::1").is_ok());
/// assert!(validate_host("::").is_ok());
/// assert!(validate_host("localhost").is_ok());
/// assert!(validate_host("example.com").is_ok());
/// assert!(validate_host("").is_err());
/// ```
pub fn validate_host(host: &str) -> Result<()> {
    let host = host.trim();

    if host.is_empty() {
        return Err(KreuzbergError::Validation {
            message: "Invalid host '': must be a valid IP address or hostname".to_string(),
            source: None,
        });
    }

    // Check if it's a valid IPv4 address
    if host.parse::<std::net::Ipv4Addr>().is_ok() {
        return Ok(());
    }

    // Check if it's a valid IPv6 address
    if host.parse::<std::net::Ipv6Addr>().is_ok() {
        return Ok(());
    }

    // Check if it's a valid hostname (basic validation)
    // Hostnames must contain only alphanumeric characters, dots, and hyphens
    // Must not look like an invalid IPv4 address (all numeric with dots)
    let looks_like_ipv4 = host
        .split('.')
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_numeric()));
    if !looks_like_ipv4
        && host.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
        && !host.starts_with('-')
        && !host.ends_with('-')
    {
        return Ok(());
    }

    Err(KreuzbergError::Validation {
        message: format!("Invalid host '{}': must be a valid IP address or hostname", host),
        source: None,
    })
}

/// Validate a CORS (Cross-Origin Resource Sharing) origin URL.
///
/// Accepts valid HTTP/HTTPS URLs (e.g., "https://example.com") or the wildcard "*"
/// to allow all origins. URLs must start with "http://" or "https://", or be exactly "*".
///
/// # Arguments
///
/// * `origin` - The CORS origin URL to validate
///
/// # Returns
///
/// `Ok(())` if the origin is valid, or a `ValidationError` with details about valid formats.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_cors_origin;
///
/// assert!(validate_cors_origin("https://example.com").is_ok());
/// assert!(validate_cors_origin("http://localhost:3000").is_ok());
/// assert!(validate_cors_origin("*").is_ok());
/// assert!(validate_cors_origin("not-a-url").is_err());
/// assert!(validate_cors_origin("ftp://example.com").is_err());
/// ```
pub fn validate_cors_origin(origin: &str) -> Result<()> {
    let origin = origin.trim();

    if origin == "*" {
        return Ok(());
    }

    if origin.starts_with("http://") || origin.starts_with("https://") {
        // Basic validation: ensure there's something after the protocol
        if origin.len() > 8 && (origin.starts_with("http://") && origin.len() > 7 || origin.starts_with("https://")) {
            return Ok(());
        }
    }

    Err(KreuzbergError::Validation {
        message: format!(
            "Invalid CORS origin '{}': must be a valid HTTP/HTTPS URL or '*'",
            origin
        ),
        source: None,
    })
}

/// Validate an upload size limit for server configuration.
///
/// Upload size must be greater than 0 (measured in bytes).
///
/// # Arguments
///
/// * `size` - The maximum upload size in bytes to validate
///
/// # Returns
///
/// `Ok(())` if the size is valid, or a `ValidationError` with details about constraints.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_upload_size;
///
/// assert!(validate_upload_size(1024).is_ok());
/// assert!(validate_upload_size(1_000_000).is_ok());
/// assert!(validate_upload_size(0).is_err());
/// ```
pub fn validate_upload_size(size: usize) -> Result<()> {
    if size > 0 {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!("Upload size must be greater than 0, got {}", size),
            source: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_binarization_method_valid() {
        assert!(validate_binarization_method("otsu").is_ok());
        assert!(validate_binarization_method("adaptive").is_ok());
        assert!(validate_binarization_method("sauvola").is_ok());
    }

    #[test]
    fn test_validate_binarization_method_case_insensitive() {
        assert!(validate_binarization_method("OTSU").is_ok());
        assert!(validate_binarization_method("Adaptive").is_ok());
        assert!(validate_binarization_method("SAUVOLA").is_ok());
    }

    #[test]
    fn test_validate_binarization_method_invalid() {
        let result = validate_binarization_method("invalid");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid binarization method"));
        assert!(msg.contains("otsu"));
    }

    #[test]
    fn test_validate_token_reduction_level_valid() {
        assert!(validate_token_reduction_level("off").is_ok());
        assert!(validate_token_reduction_level("light").is_ok());
        assert!(validate_token_reduction_level("moderate").is_ok());
        assert!(validate_token_reduction_level("aggressive").is_ok());
        assert!(validate_token_reduction_level("maximum").is_ok());
    }

    #[test]
    fn test_validate_token_reduction_level_case_insensitive() {
        assert!(validate_token_reduction_level("OFF").is_ok());
        assert!(validate_token_reduction_level("Moderate").is_ok());
        assert!(validate_token_reduction_level("MAXIMUM").is_ok());
    }

    #[test]
    fn test_validate_token_reduction_level_invalid() {
        let result = validate_token_reduction_level("extreme");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid token reduction level"));
    }

    #[test]
    fn test_validate_ocr_backend_valid() {
        assert!(validate_ocr_backend("tesseract").is_ok());
        assert!(validate_ocr_backend("easyocr").is_ok());
        assert!(validate_ocr_backend("paddleocr").is_ok());
    }

    #[test]
    fn test_validate_ocr_backend_case_insensitive() {
        assert!(validate_ocr_backend("TESSERACT").is_ok());
        assert!(validate_ocr_backend("EasyOCR").is_ok());
        assert!(validate_ocr_backend("PADDLEOCR").is_ok());
    }

    #[test]
    fn test_validate_ocr_backend_invalid() {
        let result = validate_ocr_backend("invalid_backend");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid OCR backend"));
    }

    #[test]
    fn test_validate_language_code_valid_iso639_1() {
        assert!(validate_language_code("en").is_ok());
        assert!(validate_language_code("de").is_ok());
        assert!(validate_language_code("fr").is_ok());
        assert!(validate_language_code("es").is_ok());
        assert!(validate_language_code("zh").is_ok());
        assert!(validate_language_code("ja").is_ok());
        assert!(validate_language_code("ko").is_ok());
    }

    #[test]
    fn test_validate_language_code_valid_iso639_3() {
        assert!(validate_language_code("eng").is_ok());
        assert!(validate_language_code("deu").is_ok());
        assert!(validate_language_code("fra").is_ok());
        assert!(validate_language_code("spa").is_ok());
        assert!(validate_language_code("zho").is_ok());
        assert!(validate_language_code("jpn").is_ok());
        assert!(validate_language_code("kor").is_ok());
    }

    #[test]
    fn test_validate_language_code_case_insensitive() {
        assert!(validate_language_code("EN").is_ok());
        assert!(validate_language_code("ENG").is_ok());
        assert!(validate_language_code("De").is_ok());
        assert!(validate_language_code("DEU").is_ok());
    }

    #[test]
    fn test_validate_language_code_invalid() {
        let result = validate_language_code("invalid");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid language code"));
        assert!(msg.contains("ISO 639"));
    }

    #[test]
    fn test_validate_tesseract_psm_valid() {
        for psm in 0..=13 {
            assert!(validate_tesseract_psm(psm).is_ok(), "PSM {} should be valid", psm);
        }
    }

    #[test]
    fn test_validate_tesseract_psm_invalid() {
        assert!(validate_tesseract_psm(-1).is_err());
        assert!(validate_tesseract_psm(14).is_err());
        assert!(validate_tesseract_psm(100).is_err());
    }

    #[test]
    fn test_validate_tesseract_oem_valid() {
        for oem in 0..=3 {
            assert!(validate_tesseract_oem(oem).is_ok(), "OEM {} should be valid", oem);
        }
    }

    #[test]
    fn test_validate_tesseract_oem_invalid() {
        assert!(validate_tesseract_oem(-1).is_err());
        assert!(validate_tesseract_oem(4).is_err());
        assert!(validate_tesseract_oem(10).is_err());
    }

    #[test]
    fn test_validate_output_format_valid() {
        assert!(validate_output_format("text").is_ok());
        assert!(validate_output_format("markdown").is_ok());
    }

    #[test]
    fn test_validate_output_format_case_insensitive() {
        assert!(validate_output_format("TEXT").is_ok());
        assert!(validate_output_format("Markdown").is_ok());
    }

    #[test]
    fn test_validate_output_format_invalid() {
        let result = validate_output_format("json");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid output format"));
    }

    #[test]
    fn test_validate_confidence_valid() {
        assert!(validate_confidence(0.0).is_ok());
        assert!(validate_confidence(0.5).is_ok());
        assert!(validate_confidence(1.0).is_ok());
        assert!(validate_confidence(0.75).is_ok());
    }

    #[test]
    fn test_validate_confidence_invalid() {
        assert!(validate_confidence(-0.1).is_err());
        assert!(validate_confidence(1.1).is_err());
        assert!(validate_confidence(2.0).is_err());
    }

    #[test]
    fn test_validate_dpi_valid() {
        assert!(validate_dpi(72).is_ok());
        assert!(validate_dpi(96).is_ok());
        assert!(validate_dpi(300).is_ok());
        assert!(validate_dpi(600).is_ok());
        assert!(validate_dpi(1).is_ok());
    }

    #[test]
    fn test_validate_dpi_invalid() {
        assert!(validate_dpi(0).is_err());
        assert!(validate_dpi(-1).is_err());
        assert!(validate_dpi(2401).is_err());
    }

    #[test]
    fn test_validate_chunking_params_valid() {
        assert!(validate_chunking_params(1000, 200).is_ok());
        assert!(validate_chunking_params(500, 50).is_ok());
        assert!(validate_chunking_params(1, 0).is_ok());
    }

    #[test]
    fn test_validate_chunking_params_zero_chars() {
        let result = validate_chunking_params(0, 100);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max_chars"));
    }

    #[test]
    fn test_validate_chunking_params_overlap_too_large() {
        let result = validate_chunking_params(100, 100);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("overlap"));

        let result = validate_chunking_params(100, 150);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_messages_are_helpful() {
        let err = validate_binarization_method("bad").unwrap_err().to_string();
        assert!(err.contains("otsu"));
        assert!(err.contains("adaptive"));
        assert!(err.contains("sauvola"));

        let err = validate_token_reduction_level("bad").unwrap_err().to_string();
        assert!(err.contains("off"));
        assert!(err.contains("moderate"));

        let err = validate_language_code("bad").unwrap_err().to_string();
        assert!(err.contains("ISO 639"));
        assert!(err.contains("en"));
    }

    #[test]
    fn test_validate_port_valid() {
        assert!(validate_port(1).is_ok());
        assert!(validate_port(80).is_ok());
        assert!(validate_port(443).is_ok());
        assert!(validate_port(8000).is_ok());
        assert!(validate_port(65535).is_ok());
    }

    #[test]
    fn test_validate_port_invalid() {
        let result = validate_port(0);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Port must be 1-65535"));
        assert!(msg.contains("0"));
    }

    #[test]
    fn test_validate_host_ipv4() {
        assert!(validate_host("127.0.0.1").is_ok());
        assert!(validate_host("0.0.0.0").is_ok());
        assert!(validate_host("192.168.1.1").is_ok());
        assert!(validate_host("10.0.0.1").is_ok());
        assert!(validate_host("255.255.255.255").is_ok());
    }

    #[test]
    fn test_validate_host_ipv6() {
        assert!(validate_host("::1").is_ok());
        assert!(validate_host("::").is_ok());
        assert!(validate_host("2001:db8::1").is_ok());
        assert!(validate_host("fe80::1").is_ok());
    }

    #[test]
    fn test_validate_host_hostname() {
        assert!(validate_host("localhost").is_ok());
        assert!(validate_host("example.com").is_ok());
        assert!(validate_host("sub.example.com").is_ok());
        assert!(validate_host("api-server").is_ok());
        assert!(validate_host("app123").is_ok());
    }

    #[test]
    fn test_validate_host_invalid() {
        let result = validate_host("");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid host"));

        let result = validate_host("not a valid host");
        assert!(result.is_err());

        let result = validate_host("256.256.256.256");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cors_origin_https() {
        assert!(validate_cors_origin("https://example.com").is_ok());
        assert!(validate_cors_origin("https://localhost:3000").is_ok());
        assert!(validate_cors_origin("https://sub.example.com").is_ok());
        assert!(validate_cors_origin("https://192.168.1.1").is_ok());
        assert!(validate_cors_origin("https://example.com/path").is_ok());
    }

    #[test]
    fn test_validate_cors_origin_http() {
        assert!(validate_cors_origin("http://example.com").is_ok());
        assert!(validate_cors_origin("http://localhost:3000").is_ok());
        assert!(validate_cors_origin("http://127.0.0.1:8000").is_ok());
    }

    #[test]
    fn test_validate_cors_origin_wildcard() {
        assert!(validate_cors_origin("*").is_ok());
    }

    #[test]
    fn test_validate_cors_origin_invalid() {
        let result = validate_cors_origin("not-a-url");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Invalid CORS origin"));

        let result = validate_cors_origin("ftp://example.com");
        assert!(result.is_err());

        let result = validate_cors_origin("example.com");
        assert!(result.is_err());

        let result = validate_cors_origin("http://");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_upload_size_valid() {
        assert!(validate_upload_size(1).is_ok());
        assert!(validate_upload_size(1024).is_ok());
        assert!(validate_upload_size(1_000_000).is_ok());
        assert!(validate_upload_size(1_000_000_000).is_ok());
        assert!(validate_upload_size(usize::MAX).is_ok());
    }

    #[test]
    fn test_validate_upload_size_invalid() {
        let result = validate_upload_size(0);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Upload size must be greater than 0"));
        assert!(msg.contains("0"));
    }
}
