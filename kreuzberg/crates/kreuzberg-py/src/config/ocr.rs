//! OCR configuration types for Python bindings

use pyo3::prelude::*;

/// Tesseract OCR configuration.
///
/// Provides fine-grained control over Tesseract OCR behavior including
/// page segmentation mode, table detection, and various Tesseract-specific options.
///
/// Example:
///     >>> from kreuzberg import TesseractConfig
///     >>> config = TesseractConfig(
///     ...     language="eng",
///     ...     psm=6,
///     ...     enable_table_detection=True,
///     ...     tessedit_char_whitelist="0123456789"
///     ... )
#[pyclass(name = "TesseractConfig", module = "kreuzberg")]
#[derive(Clone)]
pub struct TesseractConfig {
    pub inner: kreuzberg::types::TesseractConfig,
}

#[pymethods]
impl TesseractConfig {
    #[new]
    #[pyo3(signature = (
        language=None,
        psm=None,
        output_format=None,
        oem=None,
        min_confidence=None,
        preprocessing=None,
        enable_table_detection=None,
        table_min_confidence=None,
        table_column_threshold=None,
        table_row_threshold_ratio=None,
        use_cache=None,
        classify_use_pre_adapted_templates=None,
        language_model_ngram_on=None,
        tessedit_dont_blkrej_good_wds=None,
        tessedit_dont_rowrej_good_wds=None,
        tessedit_enable_dict_correction=None,
        tessedit_char_whitelist=None,
        tessedit_char_blacklist=None,
        tessedit_use_primary_params_model=None,
        textord_space_size_is_variable=None,
        thresholding_method=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        language: Option<String>,
        psm: Option<i32>,
        output_format: Option<String>,
        oem: Option<i32>,
        min_confidence: Option<f64>,
        preprocessing: Option<ImagePreprocessingConfig>,
        enable_table_detection: Option<bool>,
        table_min_confidence: Option<f64>,
        table_column_threshold: Option<i32>,
        table_row_threshold_ratio: Option<f64>,
        use_cache: Option<bool>,
        classify_use_pre_adapted_templates: Option<bool>,
        language_model_ngram_on: Option<bool>,
        tessedit_dont_blkrej_good_wds: Option<bool>,
        tessedit_dont_rowrej_good_wds: Option<bool>,
        tessedit_enable_dict_correction: Option<bool>,
        tessedit_char_whitelist: Option<String>,
        tessedit_char_blacklist: Option<String>,
        tessedit_use_primary_params_model: Option<bool>,
        textord_space_size_is_variable: Option<bool>,
        thresholding_method: Option<bool>,
    ) -> Self {
        Self {
            inner: kreuzberg::types::TesseractConfig {
                language: language.unwrap_or_else(|| "eng".to_string()),
                psm: psm.unwrap_or(3),
                output_format: output_format.unwrap_or_else(|| "markdown".to_string()),
                oem: oem.unwrap_or(3),
                min_confidence: min_confidence.unwrap_or(0.0),
                preprocessing: preprocessing.map(Into::into),
                enable_table_detection: enable_table_detection.unwrap_or(true),
                table_min_confidence: table_min_confidence.unwrap_or(0.0),
                table_column_threshold: table_column_threshold.unwrap_or(50),
                table_row_threshold_ratio: table_row_threshold_ratio.unwrap_or(0.5),
                use_cache: use_cache.unwrap_or(true),
                classify_use_pre_adapted_templates: classify_use_pre_adapted_templates.unwrap_or(true),
                language_model_ngram_on: language_model_ngram_on.unwrap_or(false),
                tessedit_dont_blkrej_good_wds: tessedit_dont_blkrej_good_wds.unwrap_or(true),
                tessedit_dont_rowrej_good_wds: tessedit_dont_rowrej_good_wds.unwrap_or(true),
                tessedit_enable_dict_correction: tessedit_enable_dict_correction.unwrap_or(true),
                tessedit_char_whitelist: tessedit_char_whitelist.unwrap_or_default(),
                tessedit_char_blacklist: tessedit_char_blacklist.unwrap_or_default(),
                tessedit_use_primary_params_model: tessedit_use_primary_params_model.unwrap_or(true),
                textord_space_size_is_variable: textord_space_size_is_variable.unwrap_or(true),
                thresholding_method: thresholding_method.unwrap_or(false),
            },
        }
    }

    #[getter]
    fn language(&self) -> String {
        self.inner.language.clone()
    }

    #[setter]
    fn set_language(&mut self, value: String) {
        self.inner.language = value;
    }

    #[getter]
    fn psm(&self) -> i32 {
        self.inner.psm
    }

    #[setter]
    fn set_psm(&mut self, value: i32) {
        self.inner.psm = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "TesseractConfig(language='{}', psm={}, output_format='{}', enable_table_detection={})",
            self.inner.language, self.inner.psm, self.inner.output_format, self.inner.enable_table_detection
        )
    }
}

/// Image preprocessing configuration for OCR.
#[pyclass(name = "ImagePreprocessingConfig", module = "kreuzberg")]
#[derive(Clone)]
pub struct ImagePreprocessingConfig {
    pub inner: kreuzberg::types::ImagePreprocessingConfig,
}

#[pymethods]
impl ImagePreprocessingConfig {
    #[new]
    #[pyo3(signature = (
        target_dpi=None,
        auto_rotate=None,
        deskew=None,
        denoise=None,
        contrast_enhance=None,
        binarization_method=None,
        invert_colors=None
    ))]
    fn new(
        target_dpi: Option<i32>,
        auto_rotate: Option<bool>,
        deskew: Option<bool>,
        denoise: Option<bool>,
        contrast_enhance: Option<bool>,
        binarization_method: Option<String>,
        invert_colors: Option<bool>,
    ) -> Self {
        Self {
            inner: kreuzberg::types::ImagePreprocessingConfig {
                target_dpi: target_dpi.unwrap_or(300),
                auto_rotate: auto_rotate.unwrap_or(true),
                deskew: deskew.unwrap_or(true),
                denoise: denoise.unwrap_or(false),
                contrast_enhance: contrast_enhance.unwrap_or(false),
                binarization_method: binarization_method.unwrap_or_else(|| "otsu".to_string()),
                invert_colors: invert_colors.unwrap_or(false),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "ImagePreprocessingConfig(target_dpi={}, auto_rotate={}, deskew={}, denoise={})",
            self.inner.target_dpi, self.inner.auto_rotate, self.inner.deskew, self.inner.denoise
        )
    }
}

/// OCR configuration.
#[pyclass(name = "OcrConfig", module = "kreuzberg")]
#[derive(Clone)]
pub struct OcrConfig {
    pub inner: kreuzberg::OcrConfig,
}

#[pymethods]
impl OcrConfig {
    #[new]
    #[pyo3(signature = (backend=None, language=None, tesseract_config=None))]
    fn new(backend: Option<String>, language: Option<String>, tesseract_config: Option<TesseractConfig>) -> Self {
        Self {
            inner: kreuzberg::OcrConfig {
                backend: backend.unwrap_or_else(|| "tesseract".to_string()),
                language: language.unwrap_or_else(|| "eng".to_string()),
                tesseract_config: tesseract_config.map(Into::into),
                output_format: None,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "OcrConfig(backend='{}', language='{}', tesseract_config={})",
            self.inner.backend,
            self.inner.language,
            if self.inner.tesseract_config.is_some() {
                "Some(...)"
            } else {
                "None"
            }
        )
    }
}

// Conversion implementations
impl From<ImagePreprocessingConfig> for kreuzberg::types::ImagePreprocessingConfig {
    fn from(val: ImagePreprocessingConfig) -> Self {
        val.inner
    }
}

impl From<kreuzberg::types::ImagePreprocessingConfig> for ImagePreprocessingConfig {
    fn from(inner: kreuzberg::types::ImagePreprocessingConfig) -> Self {
        Self { inner }
    }
}

impl From<TesseractConfig> for kreuzberg::types::TesseractConfig {
    fn from(val: TesseractConfig) -> Self {
        val.inner
    }
}

impl From<kreuzberg::types::TesseractConfig> for TesseractConfig {
    fn from(inner: kreuzberg::types::TesseractConfig) -> Self {
        Self { inner }
    }
}

impl From<OcrConfig> for kreuzberg::OcrConfig {
    fn from(val: OcrConfig) -> Self {
        val.inner
    }
}

impl From<kreuzberg::OcrConfig> for OcrConfig {
    fn from(inner: kreuzberg::OcrConfig) -> Self {
        Self { inner }
    }
}
