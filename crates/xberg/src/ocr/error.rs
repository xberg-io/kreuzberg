use std::fmt;

/// OCR-specific errors (pure Rust, no PyO3).
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone)]
pub enum OcrError {
    /// Tesseract failed to initialize with the given configuration.
    TesseractInitializationFailed(String),
    /// Tesseract version is below the minimum supported version.
    UnsupportedVersion(String),
    /// Configuration parameter is invalid or out of range.
    InvalidConfiguration(String),
    /// Language code is not recognized by Tesseract.
    InvalidLanguageCode(String),
    /// Image preprocessing or decoding failed.
    ImageProcessingFailed(String),
    /// The OCR recognition step itself failed.
    ProcessingFailed(String),
    /// Reading or writing the OCR result cache failed.
    CacheError(String),
    /// An I/O error occurred while reading an image or writing output.
    IOError(String),
}

impl fmt::Display for OcrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TesseractInitializationFailed(msg) => {
                write!(f, "Tesseract initialization failed: {}", msg)
            }
            Self::UnsupportedVersion(msg) => {
                write!(f, "Unsupported Tesseract version: {}", msg)
            }
            Self::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            Self::InvalidLanguageCode(msg) => write!(f, "Invalid language code: {}", msg),
            Self::ImageProcessingFailed(msg) => write!(f, "Image processing failed: {}", msg),
            Self::ProcessingFailed(msg) => write!(f, "OCR processing failed: {}", msg),
            Self::CacheError(msg) => write!(f, "Cache error: {}", msg),
            Self::IOError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for OcrError {}

// NOTE: No From<std::io::Error> impl - IO errors must bubble up unchanged per error handling policy
