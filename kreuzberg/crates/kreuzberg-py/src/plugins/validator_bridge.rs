//! Python Validator wrapper bridge with GIL management.
//!
//! Provides a Rust wrapper that makes Python Validators usable from Rust by implementing
//! the `Validator` trait and managing the FFI boundary with proper GIL handling.

use async_trait::async_trait;
use pyo3::prelude::*;
use std::sync::Arc;

use kreuzberg::core::config::ExtractionConfig;
use kreuzberg::plugins::registry::get_validator_registry;
use kreuzberg::plugins::{Plugin, Validator};
use kreuzberg::types::ExtractionResult;
use kreuzberg::{KreuzbergError, Result};

use crate::types::ExtractionResult as PyExtractionResult;

use super::common::validate_plugin_object;

/// Wrapper that makes a Python Validator usable from Rust.
///
/// This struct implements the Rust `Validator` trait by forwarding calls
/// to a Python object via PyO3, bridging the FFI boundary with proper
/// GIL management and type conversions.
pub struct PythonValidator {
    /// Python object implementing the Validator protocol
    python_obj: Py<PyAny>,
    /// Cached validator name (to avoid repeated GIL acquisition)
    name: String,
    /// Cached priority
    priority: i32,
}

impl PythonValidator {
    /// Create a new Python Validator wrapper.
    ///
    /// # Arguments
    ///
    /// * `py` - Python GIL token
    /// * `python_obj` - Python object implementing the validator protocol
    ///
    /// # Returns
    ///
    /// A new `PythonValidator` or an error if the Python object is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Python object doesn't have required methods
    /// - Method calls fail during initialization
    pub fn new(py: Python<'_>, python_obj: Py<PyAny>) -> PyResult<Self> {
        let obj = python_obj.bind(py);

        validate_plugin_object(obj, "Validator", &["name", "validate"])?;

        let name: String = obj.call_method0("name")?.extract()?;
        if name.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Validator name cannot be empty",
            ));
        }

        let priority = if obj.hasattr("priority")? {
            obj.call_method0("priority")?.extract()?
        } else {
            50
        };

        Ok(Self {
            python_obj,
            name,
            priority,
        })
    }
}

impl Plugin for PythonValidator {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> String {
        Python::attach(|py| {
            self.python_obj
                .bind(py)
                .getattr("version")
                .and_then(|v| v.call0())
                .and_then(|v| v.extract::<String>())
                .unwrap_or_else(|_| "1.0.0".to_string())
        })
    }

    fn initialize(&self) -> Result<()> {
        Python::attach(|py| {
            let obj = self.python_obj.bind(py);
            if obj.hasattr("initialize")? {
                obj.call_method0("initialize")?;
            }
            Ok(())
        })
        .map_err(|e: PyErr| KreuzbergError::Plugin {
            message: format!("Failed to initialize Python Validator '{}': {}", self.name, e),
            plugin_name: self.name.clone(),
        })
    }

    fn shutdown(&self) -> Result<()> {
        Python::attach(|py| {
            let obj = self.python_obj.bind(py);
            if obj.hasattr("shutdown")? {
                obj.call_method0("shutdown")?;
            }
            Ok(())
        })
        .map_err(|e: PyErr| KreuzbergError::Plugin {
            message: format!("Failed to shutdown Python Validator '{}': {}", self.name, e),
            plugin_name: self.name.clone(),
        })
    }
}

#[async_trait]
impl Validator for PythonValidator {
    async fn validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
        let validator_name = self.name.clone();

        tokio::task::block_in_place(|| {
            Python::attach(|py| {
                let obj = self.python_obj.bind(py);

                // Convert Rust ExtractionResult to Python ExtractionResult class instance
                let py_extraction_result =
                    PyExtractionResult::from_rust(result.clone(), py, None, None).map_err(|e| {
                        KreuzbergError::Plugin {
                            message: format!("Failed to convert ExtractionResult to Python: {}", e),
                            plugin_name: validator_name.clone(),
                        }
                    })?;

                let py_result_obj = Py::new(py, py_extraction_result).map_err(|e| KreuzbergError::Plugin {
                    message: format!("Failed to create Python ExtractionResult: {}", e),
                    plugin_name: validator_name.clone(),
                })?;

                obj.call_method1("validate", (py_result_obj,)).map_err(|e| {
                    let is_validation_error = e.is_instance_of::<pyo3::exceptions::PyValueError>(py)
                        || e.get_type(py)
                            .name()
                            .ok()
                            .and_then(|n| n.to_str().ok().map(|s| s.to_string()))
                            .map(|s| s.contains("ValidationError"))
                            .unwrap_or(false);

                    if is_validation_error {
                        KreuzbergError::Validation {
                            message: e.to_string(),
                            source: None,
                        }
                    } else {
                        KreuzbergError::Plugin {
                            message: format!("Python Validator '{}' failed during validate: {}", validator_name, e),
                            plugin_name: validator_name.clone(),
                        }
                    }
                })?;

                Ok::<(), KreuzbergError>(())
            })
        })?;

        Ok(())
    }

    fn should_validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> bool {
        let validator_name = self.name.clone();
        Python::attach(|py| {
            let obj = self.python_obj.bind(py);

            // If hasattr fails due to GIL error, log and default to true ~keep
            let has_should_validate = obj
                .hasattr("should_validate")
                .map_err(|e| {
                    tracing::debug!(
                        "WARNING: Validator '{}': Failed to check for should_validate method due to GIL error ({}), defaulting to true",
                        validator_name, e
                    );
                    e
                })
                .unwrap_or(false);

            if has_should_validate {
                let py_extraction_result =
                    PyExtractionResult::from_rust(result.clone(), py, None, None).ok()?;
                let py_result_obj = Py::new(py, py_extraction_result).ok()?;
                obj.call_method1("should_validate", (py_result_obj,))
                    .and_then(|v| v.extract::<bool>())
                    .ok()
            } else {
                Some(true)
            }
        })
        .unwrap_or(true)
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}

/// Register a Python Validator with the Rust core.
///
/// This function validates the Python validator object, wraps it in a Rust
/// `Validator` implementation, and registers it with the global Validator
/// registry. Once registered, the validator will be called automatically after
/// extraction to validate results.
///
/// # Arguments
///
/// * `validator` - Python object implementing the Validator protocol
///
/// # Required Methods on Python Validator
///
/// The Python validator must implement:
/// - `name() -> str` - Return validator name
/// - `validate(result: ExtractionResult) -> None` - Validate the extraction result (raise error to fail)
///
/// # Optional Methods
///
/// - `should_validate(result: ExtractionResult) -> bool` - Check if validator should run (defaults to True)
/// - `priority() -> int` - Return priority (defaults to 50, higher runs first)
/// - `initialize()` - Called when validator is registered
/// - `shutdown()` - Called when validator is unregistered
/// - `version() -> str` - Validator version (defaults to "1.0.0")
///
/// # Example
///
/// ```python
/// from kreuzberg import register_validator, ExtractionResult
/// from kreuzberg.exceptions import ValidationError
///
/// class MinLengthValidator:
///     def name(self) -> str:
///         return "min_length_validator"
///
///     def priority(self) -> int:
///         return 100  # Run early
///
///     def validate(self, result: ExtractionResult) -> None:
///         if len(result.content) < 100:
///             raise ValidationError(
///                 f"Content too short: {len(result.content)} < 100 characters"
///             )
///
/// register_validator(MinLengthValidator())
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Validator is missing required methods
/// - Validator name is empty or duplicate
/// - Registration fails
#[pyfunction]
pub fn register_validator(py: Python<'_>, validator: Py<PyAny>) -> PyResult<()> {
    let rust_validator = PythonValidator::new(py, validator)?;
    let validator_name = rust_validator.name().to_string();

    let arc_validator: Arc<dyn Validator> = Arc::new(rust_validator);

    py.detach(|| {
        let registry = get_validator_registry();
        let mut registry = registry.write();

        registry.register(arc_validator).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to register Validator '{}': {}",
                validator_name, e
            ))
        })
    })?;

    Ok(())
}

/// Unregister a Validator by name.
///
/// Removes a previously registered validator from the global registry and
/// calls its `shutdown()` method to release resources.
///
/// # Arguments
///
/// * `name` - Validator name to unregister
///
/// # Example
///
/// ```python
/// from kreuzberg import register_validator, unregister_validator
///
/// class MyValidator:
///     def name(self) -> str:
///         return "my_validator"
///
///     def validate(self, result: ExtractionResult) -> None:
///         pass
///
/// register_validator(MyValidator())
/// # ... use validator ...
/// unregister_validator("my_validator")
/// ```
#[pyfunction]
pub fn unregister_validator(py: Python<'_>, name: &str) -> PyResult<()> {
    py.detach(|| {
        let registry = get_validator_registry();
        let mut registry = registry.write();

        registry.remove(name).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to unregister Validator '{}': {}", name, e))
        })
    })?;

    Ok(())
}

/// Clear all registered Validators.
///
/// Removes all validators from the global registry and calls their `shutdown()`
/// methods. Useful for test cleanup or resetting state.
///
/// # Example
///
/// ```python
/// from kreuzberg import clear_validators
///
/// # In pytest fixture or test cleanup
/// clear_validators()
/// ```
#[pyfunction]
pub fn clear_validators(py: Python<'_>) -> PyResult<()> {
    py.detach(|| {
        let registry = get_validator_registry();
        let mut registry = registry.write();

        registry.shutdown_all().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to clear Validator registry: {}", e))
        })
    })?;

    Ok(())
}

/// List all registered validator names.
///
/// Returns a list of all validator names currently registered in the global registry.
///
/// # Returns
///
/// List of validator names.
///
/// # Example
///
/// ```python
/// from kreuzberg import list_validators, register_validator, clear_validators
///
/// class MyValidator:
///     def name(self) -> str:
///         return "my_validator"
///
///     def validate(self, result: ExtractionResult) -> None:
///         pass
///
/// # Register validator
/// register_validator(MyValidator())
///
/// # List validators
/// validators = list_validators()
/// assert "my_validator" in validators
///
/// # Cleanup
/// clear_validators()
/// ```
#[pyfunction]
pub fn list_validators() -> PyResult<Vec<String>> {
    kreuzberg::plugins::list_validators().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}
