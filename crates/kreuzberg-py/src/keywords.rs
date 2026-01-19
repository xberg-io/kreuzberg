//! Keyword extraction configuration
//!
//! Provides Python bindings for keyword extraction configuration including
//! algorithm selection, parameters, and result handling.

use pyo3::prelude::*;

/// Keyword extraction algorithm.
///
/// Example:
///     >>> from kreuzberg import KeywordAlgorithm
///     >>> algo = KeywordAlgorithm.YAKE
#[pyclass(name = "KeywordAlgorithm", module = "kreuzberg")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeywordAlgorithm {
    /// YAKE (Yet Another Keyword Extractor) - statistical approach
    Yake,

    /// RAKE (Rapid Automatic Keyword Extraction) - co-occurrence based
    Rake,
}

impl From<KeywordAlgorithm> for kreuzberg::keywords::KeywordAlgorithm {
    fn from(algo: KeywordAlgorithm) -> Self {
        match algo {
            KeywordAlgorithm::Yake => kreuzberg::keywords::KeywordAlgorithm::Yake,
            KeywordAlgorithm::Rake => kreuzberg::keywords::KeywordAlgorithm::Rake,
        }
    }
}

impl From<kreuzberg::keywords::KeywordAlgorithm> for KeywordAlgorithm {
    fn from(algo: kreuzberg::keywords::KeywordAlgorithm) -> Self {
        match algo {
            kreuzberg::keywords::KeywordAlgorithm::Yake => KeywordAlgorithm::Yake,
            kreuzberg::keywords::KeywordAlgorithm::Rake => KeywordAlgorithm::Rake,
        }
    }
}

/// YAKE-specific parameters.
///
/// Example:
///     >>> from kreuzberg import YakeParams
///     >>> params = YakeParams(window_size=3, deduplicate=True, dedup_threshold=0.8)
#[pyclass(name = "YakeParams", module = "kreuzberg")]
#[derive(Clone)]
pub struct YakeParams {
    pub inner: kreuzberg::keywords::YakeParams,
}

#[pymethods]
impl YakeParams {
    #[new]
    #[pyo3(signature = (window_size=None))]
    fn new(window_size: Option<usize>) -> Self {
        Self {
            inner: kreuzberg::keywords::YakeParams {
                window_size: window_size.unwrap_or(2),
            },
        }
    }

    #[getter]
    fn window_size(&self) -> usize {
        self.inner.window_size
    }

    #[setter]
    fn set_window_size(&mut self, value: usize) {
        self.inner.window_size = value;
    }

    fn __repr__(&self) -> String {
        format!("YakeParams(window_size={})", self.inner.window_size)
    }
}

impl From<YakeParams> for kreuzberg::keywords::YakeParams {
    fn from(params: YakeParams) -> Self {
        params.inner
    }
}

impl From<kreuzberg::keywords::YakeParams> for YakeParams {
    fn from(params: kreuzberg::keywords::YakeParams) -> Self {
        Self { inner: params }
    }
}

/// RAKE-specific parameters.
///
/// Example:
///     >>> from kreuzberg import RakeParams
///     >>> params = RakeParams(min_word_length=2, max_words_per_phrase=4)
#[pyclass(name = "RakeParams", module = "kreuzberg")]
#[derive(Clone)]
pub struct RakeParams {
    pub inner: kreuzberg::keywords::RakeParams,
}

#[pymethods]
impl RakeParams {
    #[new]
    #[pyo3(signature = (min_word_length=None, max_words_per_phrase=None))]
    fn new(min_word_length: Option<usize>, max_words_per_phrase: Option<usize>) -> Self {
        Self {
            inner: kreuzberg::keywords::RakeParams {
                min_word_length: min_word_length.unwrap_or(1),
                max_words_per_phrase: max_words_per_phrase.unwrap_or(3),
            },
        }
    }

    #[getter]
    fn min_word_length(&self) -> usize {
        self.inner.min_word_length
    }

    #[setter]
    fn set_min_word_length(&mut self, value: usize) {
        self.inner.min_word_length = value;
    }

    #[getter]
    fn max_words_per_phrase(&self) -> usize {
        self.inner.max_words_per_phrase
    }

    #[setter]
    fn set_max_words_per_phrase(&mut self, value: usize) {
        self.inner.max_words_per_phrase = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "RakeParams(min_word_length={}, max_words_per_phrase={})",
            self.inner.min_word_length, self.inner.max_words_per_phrase
        )
    }
}

impl From<RakeParams> for kreuzberg::keywords::RakeParams {
    fn from(params: RakeParams) -> Self {
        params.inner
    }
}

impl From<kreuzberg::keywords::RakeParams> for RakeParams {
    fn from(params: kreuzberg::keywords::RakeParams) -> Self {
        Self { inner: params }
    }
}

/// Keyword extraction configuration.
///
/// Example:
///     >>> from kreuzberg import KeywordConfig, KeywordAlgorithm
///     >>> config = KeywordConfig(
///     ...     algorithm=KeywordAlgorithm.YAKE,
///     ...     max_keywords=15,
///     ...     min_score=0.1,
///     ...     language="en"
///     ... )
#[pyclass(name = "KeywordConfig", module = "kreuzberg")]
#[derive(Clone)]
pub struct KeywordConfig {
    pub inner: kreuzberg::keywords::KeywordConfig,
}

#[pymethods]
impl KeywordConfig {
    #[new]
    #[pyo3(signature = (
        algorithm=None,
        max_keywords=None,
        min_score=None,
        ngram_range=None,
        language=None,
        yake_params=None,
        rake_params=None
    ))]
    fn new(
        algorithm: Option<KeywordAlgorithm>,
        max_keywords: Option<usize>,
        min_score: Option<f32>,
        ngram_range: Option<(usize, usize)>,
        language: Option<String>,
        yake_params: Option<YakeParams>,
        rake_params: Option<RakeParams>,
    ) -> Self {
        Self {
            inner: kreuzberg::keywords::KeywordConfig {
                algorithm: algorithm.map(Into::into).unwrap_or_default(),
                max_keywords: max_keywords.unwrap_or(10),
                min_score: min_score.unwrap_or(0.0),
                ngram_range: ngram_range.unwrap_or((1, 3)),
                language: language.or_else(|| Some("en".to_string())),
                yake_params: yake_params.map(Into::into),
                rake_params: rake_params.map(Into::into),
            },
        }
    }

    #[getter]
    fn algorithm(&self) -> KeywordAlgorithm {
        self.inner.algorithm.into()
    }

    #[setter]
    fn set_algorithm(&mut self, value: KeywordAlgorithm) {
        self.inner.algorithm = value.into();
    }

    #[getter]
    fn max_keywords(&self) -> usize {
        self.inner.max_keywords
    }

    #[setter]
    fn set_max_keywords(&mut self, value: usize) {
        self.inner.max_keywords = value;
    }

    #[getter]
    fn min_score(&self) -> f32 {
        self.inner.min_score
    }

    #[setter]
    fn set_min_score(&mut self, value: f32) {
        self.inner.min_score = value;
    }

    #[getter]
    fn ngram_range(&self) -> (usize, usize) {
        self.inner.ngram_range
    }

    #[setter]
    fn set_ngram_range(&mut self, value: (usize, usize)) {
        self.inner.ngram_range = value;
    }

    #[getter]
    fn language(&self) -> Option<String> {
        self.inner.language.clone()
    }

    #[setter]
    fn set_language(&mut self, value: Option<String>) {
        self.inner.language = value;
    }

    #[getter]
    fn yake_params(&self) -> Option<YakeParams> {
        self.inner.yake_params.clone().map(Into::into)
    }

    #[setter]
    fn set_yake_params(&mut self, value: Option<YakeParams>) {
        self.inner.yake_params = value.map(Into::into);
    }

    #[getter]
    fn rake_params(&self) -> Option<RakeParams> {
        self.inner.rake_params.clone().map(Into::into)
    }

    #[setter]
    fn set_rake_params(&mut self, value: Option<RakeParams>) {
        self.inner.rake_params = value.map(Into::into);
    }

    fn __repr__(&self) -> String {
        format!(
            "KeywordConfig(algorithm={:?}, max_keywords={}, min_score={}, language={:?})",
            self.inner.algorithm, self.inner.max_keywords, self.inner.min_score, self.inner.language
        )
    }
}

impl From<KeywordConfig> for kreuzberg::keywords::KeywordConfig {
    fn from(config: KeywordConfig) -> Self {
        config.inner
    }
}

impl From<kreuzberg::keywords::KeywordConfig> for KeywordConfig {
    fn from(config: kreuzberg::keywords::KeywordConfig) -> Self {
        Self { inner: config }
    }
}
