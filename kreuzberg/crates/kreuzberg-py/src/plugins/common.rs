//! Shared utilities for Python plugin bridges with GIL management.

use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList};

/// Validate that a Python plugin object has all required methods.
///
/// # Arguments
///
/// * `obj` - Python plugin object to validate
/// * `plugin_type` - Human-readable plugin type name (e.g., "OCR backend", "PostProcessor")
/// * `required_methods` - Slice of required method names
///
/// # Returns
///
/// `Ok(())` if all methods exist, otherwise `PyErr` describing missing methods.
///
/// # Errors
///
/// Returns `PyAttributeError` if any required methods are missing.
pub fn validate_plugin_object(obj: &Bound<'_, PyAny>, plugin_type: &str, required_methods: &[&str]) -> PyResult<()> {
    let mut missing_methods = Vec::new();

    for method_name in required_methods {
        if !obj.hasattr(*method_name)? {
            missing_methods.push(*method_name);
        }
    }

    if !missing_methods.is_empty() {
        return Err(pyo3::exceptions::PyAttributeError::new_err(format!(
            "{} is missing required methods: {}. Please ensure your plugin implements all required methods.",
            plugin_type,
            missing_methods.join(", ")
        )));
    }

    Ok(())
}

/// Convert serde_json::Value to Python object
pub fn json_value_to_py<'py>(py: Python<'py>, value: &serde_json::Value) -> PyResult<Bound<'py, PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None().into_bound(py)),
        serde_json::Value::Bool(b) => {
            let py_bool = PyBool::new(py, *b);
            Ok(py_bool.as_any().clone())
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any())
            } else {
                Ok(py.None().into_bound(py))
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_py(py, item)?)?;
            }
            Ok(list.into_any())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

/// Convert Python value to serde_json::Value.
pub fn python_to_json(obj: &Bound<'_, PyAny>) -> kreuzberg::Result<serde_json::Value> {
    use kreuzberg::KreuzbergError;

    if obj.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(serde_json::Value::Number(i.into()))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(serde_json::to_value(f).unwrap_or(serde_json::Value::Null))
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(list) = obj.cast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_json(&item)?);
        }
        Ok(serde_json::Value::Array(vec))
    } else if let Ok(dict) = obj.cast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str: String = key.extract().map_err(|_| KreuzbergError::Validation {
                message: "Dict keys must be strings for JSON conversion".to_string(),
                source: None,
            })?;
            map.insert(key_str, python_to_json(&value)?);
        }
        Ok(serde_json::Value::Object(map))
    } else {
        Ok(serde_json::Value::String(
            obj.str()
                .map_err(|_| KreuzbergError::Validation {
                    message: "Failed to convert Python value to JSON".to_string(),
                    source: None,
                })?
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    fn prepare_python() {
        static INIT: Once = Once::new();
        INIT.call_once(Python::initialize);
    }

    fn with_py<F, R>(f: F) -> R
    where
        F: FnOnce(Python<'_>) -> R,
    {
        prepare_python();
        Python::attach(f)
    }

    #[pyclass]
    struct TestPlugin;

    #[pymethods]
    impl TestPlugin {
        fn name(&self) -> &'static str {
            "demo"
        }

        fn process(&self) -> bool {
            true
        }
    }

    #[pyclass]
    struct IncompletePlugin;

    #[pymethods]
    impl IncompletePlugin {
        fn name(&self) -> &'static str {
            "demo"
        }
    }

    #[test]
    fn test_validate_plugin_object_success() {
        with_py(|py| {
            let plugin = Py::new(py, TestPlugin).expect("should allocate plugin");
            let instance = plugin.into_bound(py).into_any();
            validate_plugin_object(&instance, "postprocessor", &["name", "process"])
                .expect("plugin object with methods should validate");
        });
    }

    #[test]
    fn test_validate_plugin_object_reports_missing_methods() {
        with_py(|py| {
            let plugin = Py::new(py, IncompletePlugin).expect("should allocate plugin");
            let instance = plugin.into_bound(py).into_any();
            let err = validate_plugin_object(&instance, "validator", &["name", "process"]).unwrap_err();
            assert!(err.is_instance_of::<pyo3::exceptions::PyAttributeError>(py));
        });
    }

    #[test]
    fn test_json_value_to_py_converts_nested_structures() {
        with_py(|py| {
            let json_value = serde_json::json!({
                "name": "example",
                "enabled": true,
                "weights": [1, 2, 3],
                "settings": {
                    "threshold": 0.85,
                    "modes": ["fast", "safe"]
                }
            });

            let py_obj = json_value_to_py(py, &json_value).expect("conversion should succeed");
            let dict = py_obj.cast::<PyDict>().expect("expected dictionary");
            let name_item = dict.get_item("name").expect("lookup should succeed").expect("name key");
            let name: String = name_item.extract().expect("string value");
            assert_eq!(name, "example");

            let enabled_item = dict
                .get_item("enabled")
                .expect("lookup should succeed")
                .expect("enabled key");
            let enabled: bool = enabled_item.extract().expect("bool value");
            assert!(enabled);

            let weights_item = dict
                .get_item("weights")
                .expect("lookup should succeed")
                .expect("weights key");
            let weights = weights_item.cast::<PyList>().expect("weights list");
            assert_eq!(weights.len(), 3);

            let settings_item = dict
                .get_item("settings")
                .expect("lookup should succeed")
                .expect("settings key");
            let settings = settings_item.cast::<PyDict>().expect("settings dict");
            let threshold_item = settings
                .get_item("threshold")
                .expect("lookup should succeed")
                .expect("threshold key");
            let threshold: f64 = threshold_item.extract().expect("float value");
            assert_eq!(threshold, 0.85);
        });
    }
}
