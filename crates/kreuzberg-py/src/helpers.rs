//! Configuration utilities and helpers
//!
//! Provides utility functions for loading, discovering, and manipulating configurations,
//! including JSON serialization and field access helpers.

use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::config::ExtractionConfig;

/// Serialize an ExtractionConfig to JSON string.
///
/// Converts the configuration to its JSON representation.
///
/// Args:
///     config (ExtractionConfig): Configuration to serialize
///
/// Returns:
///     str: JSON string representation of the config
///
/// Example:
///     >>> from kreuzberg import ExtractionConfig, config_to_json
///     >>> config = ExtractionConfig(use_cache=True)
///     >>> json_str = config_to_json(config)
///     >>> print(json_str)
#[pyfunction]
#[pyo3(signature = (config))]
pub fn config_to_json(config: ExtractionConfig) -> PyResult<String> {
    serde_json::to_string(&config.inner)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to serialize config to JSON: {}", e)))
}

/// Get a specific field from config.
///
/// Retrieves a nested field from the configuration by path. Supports dot notation
/// for nested fields (e.g., "ocr.backend").
///
/// Args:
///     config (ExtractionConfig): Configuration to query
///     field_name (str): Field path (e.g., "use_cache", "ocr.backend")
///
/// Returns:
///     Any | None: Field value parsed from JSON, or None if field not found
///
/// Example:
///     >>> from kreuzberg import ExtractionConfig, config_get_field
///     >>> config = ExtractionConfig(use_cache=True)
///     >>> use_cache = config_get_field(config, "use_cache")
///     >>> print(use_cache)  # True
#[pyfunction]
#[pyo3(signature = (config, field_name))]
pub fn config_get_field(config: ExtractionConfig, field_name: &str) -> PyResult<Option<Py<PyAny>>> {
    let json_value = serde_json::to_value(&config.inner)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to serialize config: {}", e)))?;

    let mut current = &json_value;
    for part in field_name.split('.') {
        if let Some(obj) = current.as_object() {
            match obj.get(part) {
                Some(val) => current = val,
                None => return Ok(None),
            }
        } else {
            return Ok(None);
        }
    }

    Python::attach(|py| {
        crate::plugins::json_value_to_py(py, current)
            .map(|v| v.unbind())
            .map(Some)
    })
}

/// Merge two configurations.
///
/// Performs a merge of two ExtractionConfig structures, where override takes
/// precedence. The base config is modified in-place.
///
/// Args:
///     base (ExtractionConfig): Base configuration (will be modified)
///     override (ExtractionConfig): Override configuration (applied on top of base)
///
/// Example:
///     >>> from kreuzberg import ExtractionConfig, config_merge
///     >>> base = ExtractionConfig(use_cache=True, force_ocr=False)
///     >>> override = ExtractionConfig(force_ocr=True)
///     >>> config_merge(base, override)
///     >>> print(base.force_ocr)  # True
///     >>> print(base.use_cache)  # True (unchanged)
#[pyfunction]
#[pyo3(signature = (base, override_config))]
pub fn config_merge(py: Python<'_>, base: Py<ExtractionConfig>, override_config: &ExtractionConfig) -> PyResult<()> {
    let override_default = kreuzberg::ExtractionConfig::default();

    let mut base_mut = base.borrow_mut(py);

    if override_config.inner.use_cache != override_default.use_cache {
        base_mut.inner.use_cache = override_config.inner.use_cache;
    }
    if override_config.inner.enable_quality_processing != override_default.enable_quality_processing {
        base_mut.inner.enable_quality_processing = override_config.inner.enable_quality_processing;
    }
    if override_config.inner.force_ocr != override_default.force_ocr {
        base_mut.inner.force_ocr = override_config.inner.force_ocr;
    }
    if override_config.inner.ocr.is_some() {
        base_mut.inner.ocr = override_config.inner.ocr.clone();
    }
    if override_config.inner.chunking.is_some() {
        base_mut.inner.chunking = override_config.inner.chunking.clone();
    }
    if override_config.inner.images.is_some() {
        base_mut.inner.images = override_config.inner.images.clone();
    }
    if override_config.inner.pdf_options.is_some() {
        base_mut.inner.pdf_options = override_config.inner.pdf_options.clone();
    }
    if override_config.inner.token_reduction.is_some() {
        base_mut.inner.token_reduction = override_config.inner.token_reduction.clone();
    }
    if override_config.inner.language_detection.is_some() {
        base_mut.inner.language_detection = override_config.inner.language_detection.clone();
    }
    if override_config.inner.keywords.is_some() {
        base_mut.inner.keywords = override_config.inner.keywords.clone();
    }
    if override_config.inner.postprocessor.is_some() {
        base_mut.inner.postprocessor = override_config.inner.postprocessor.clone();
    }
    if override_config.inner.html_options.is_some() {
        base_mut.inner.html_options = override_config.inner.html_options.clone();
    }
    if override_config.inner.max_concurrent_extractions.is_some() {
        base_mut.inner.max_concurrent_extractions = override_config.inner.max_concurrent_extractions;
    }
    if override_config.inner.pages.is_some() {
        base_mut.inner.pages = override_config.inner.pages.clone();
    }

    Ok(())
}

/// Discover extraction configuration from the environment.
///
/// Attempts to locate a Kreuzberg configuration file using the following strategy:
/// 1. If KREUZBERG_CONFIG_PATH environment variable is set, load from that path
/// 2. Otherwise, search for kreuzberg.toml, kreuzberg.yaml, or kreuzberg.json
///    in the current directory and parent directories (walking up the tree)
/// 3. Return None if no configuration file is found
///
/// Returns:
///     ExtractionConfig | None: Configuration if found and valid, None otherwise
///
/// # Errors
/// Raises RuntimeError if the discovered config file is invalid or cannot be parsed.
#[pyfunction]
pub fn _discover_extraction_config_impl(py: Python<'_>) -> PyResult<Option<Py<ExtractionConfig>>> {
    match kreuzberg::ExtractionConfig::discover() {
        Ok(Some(inner)) => {
            let config = ExtractionConfig {
                inner,
                html_options_dict: None,
            };
            Ok(Some(Py::new(py, config)?))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
    }
}

/// Load extraction configuration from a specific file.
///
/// Loads an ExtractionConfig from the specified file path. The file format
/// is determined by the file extension (.toml, .yaml, or .json).
///
/// Args:
///     path (str): Path to the configuration file (absolute or relative)
///
/// Returns:
///     ExtractionConfig: Configuration parsed from the file
///
/// # Errors
/// Raises:
///     - FileNotFoundError: If the configuration file does not exist
///     - RuntimeError: If the file cannot be read or parsed
///     - ValueError: If the file format is invalid or unsupported
#[pyfunction]
pub fn _load_extraction_config_from_file_impl(py: Python<'_>, path: &str) -> PyResult<Py<ExtractionConfig>> {
    match kreuzberg::ExtractionConfig::from_file(path) {
        Ok(inner) => {
            let config = ExtractionConfig {
                inner,
                html_options_dict: None,
            };
            Py::new(py, config).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("No such file") || error_msg.contains("not found") {
                Err(pyo3::exceptions::PyFileNotFoundError::new_err(error_msg))
            } else if error_msg.contains("Invalid") || error_msg.contains("malformed") {
                Err(pyo3::exceptions::PyValueError::new_err(error_msg))
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err(error_msg))
            }
        }
    }
}
