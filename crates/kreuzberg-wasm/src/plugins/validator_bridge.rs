//! Validator wrapper implementation for WASM bindings
//!
//! This module provides the WASM bridge for custom validator plugins that
//! can validate extraction results after processing.

#[allow(unused_imports)]
use super::{JsPluginValue, MakeSend, acquire_read_lock, acquire_write_lock};
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use js_sys::{Promise, Reflect};
use kreuzberg::plugins::{Plugin, Validator};
#[allow(unused_imports)]
use kreuzberg::{ExtractionConfig, ExtractionResult, KreuzbergError};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use wasm_bindgen_futures::JsFuture;

/// Wrapper that makes a JavaScript Validator object usable from Rust.
///
/// # Thread Safety
///
/// This wrapper contains a JsValue which is NOT Send/Sync. Plugin callbacks
/// MUST be invoked only on the main JavaScript thread. The type system
/// enforces this by preventing the wrapper from being moved across threads.
struct JsValidatorWrapper {
    name: String,
    #[allow(dead_code)]
    js_obj: JsPluginValue,
    #[allow(dead_code)]
    priority: i32,
}

impl JsValidatorWrapper {
    /// Create a new wrapper from a JS object
    ///
    /// # Safety
    ///
    /// This wrapper must only be accessed from the main JavaScript thread.
    /// Do not pass this to Web Workers or rayon tasks.
    fn new(js_obj: JsValue, name: String, priority: i32) -> Self {
        Self {
            js_obj: JsPluginValue(js_obj),
            name,
            priority,
        }
    }
}

impl Plugin for JsValidatorWrapper {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> String {
        "1.0.0".to_string()
    }

    fn initialize(&self) -> kreuzberg::Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> kreuzberg::Result<()> {
        Ok(())
    }
}

#[cfg(not(test))]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Validator for JsValidatorWrapper {
    async fn validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> kreuzberg::Result<()> {
        let json_input = serde_json::to_string(result).map_err(|e| KreuzbergError::Plugin {
            message: format!("Failed to serialize extraction result: {}", e),
            plugin_name: self.name.clone(),
        })?;

        let promise = {
            let validate_fn = Reflect::get(&self.js_obj.0, &JsValue::from_str("validate"))
                .map_err(|_| KreuzbergError::Plugin {
                    message: format!("Validator '{}' missing 'validate' method", self.name),
                    plugin_name: self.name.clone(),
                })?
                .dyn_into::<js_sys::Function>()
                .map_err(|_| KreuzbergError::Plugin {
                    message: format!("Validator '{}' validate is not a function", self.name),
                    plugin_name: self.name.clone(),
                })?;

            let promise_val = validate_fn
                .call1(&self.js_obj.0, &JsValue::from_str(&json_input))
                .map_err(|e| KreuzbergError::Plugin {
                    message: format!("Validator '{}' validate call failed: {:?}", self.name, e),
                    plugin_name: self.name.clone(),
                })?;

            Promise::resolve(&promise_val)
        };

        let result_val = MakeSend(JsFuture::from(promise)).await.map_err(|e| {
            let err_msg = format!("{:?}", e);
            if err_msg.contains("ValidationError") || err_msg.contains("validation") {
                KreuzbergError::Validation {
                    message: err_msg,
                    source: None,
                }
            } else {
                KreuzbergError::Plugin {
                    message: format!("Validator '{}' promise failed: {}", self.name, err_msg),
                    plugin_name: self.name.clone(),
                }
            }
        })?;

        if let Some(error_msg) = result_val.as_string()
            && !error_msg.is_empty()
        {
            return Err(KreuzbergError::Validation {
                message: error_msg,
                source: None,
            });
        }

        Ok(())
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}

#[cfg(test)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Validator for JsValidatorWrapper {
    async fn validate(&self, _result: &ExtractionResult, _config: &ExtractionConfig) -> kreuzberg::Result<()> {
        Ok(())
    }
}

/// Register a custom validator.
///
/// # Arguments
///
/// * `validator` - JavaScript object implementing the ValidatorProtocol interface:
///   - `name(): string` - Unique validator name
///   - `validate(jsonString: string): Promise<string>` - Validation function returning empty string on success, error message on failure
///   - `priority(): number` - Optional priority (defaults to 50, higher runs first)
///
/// # Returns
///
/// Ok if registration succeeds, Err with description if it fails.
///
/// # Example
///
/// ```javascript
/// registerValidator({
///   name: () => "min-content-length",
///   priority: () => 100,
///   validate: async (jsonString) => {
///     const result = JSON.parse(jsonString);
///     if (result.content.length < 100) {
///       return "Content too short"; // Validation failure
///     }
///     return ""; // Success
///   }
/// });
/// ```
#[wasm_bindgen]
pub fn register_validator(validator: JsValue) -> Result<(), JsValue> {
    let name_fn =
        Reflect::get(&validator, &JsValue::from_str("name")).map_err(|e| format!("Missing 'name' method: {:?}", e))?;

    let validate_fn = Reflect::get(&validator, &JsValue::from_str("validate"))
        .map_err(|e| format!("Missing 'validate' method: {:?}", e))?;

    if !name_fn.is_function() || !validate_fn.is_function() {
        return Err(JsValue::from_str("name and validate must be functions"));
    }

    let name_fn = name_fn
        .dyn_into::<js_sys::Function>()
        .map_err(|_| "Failed to convert name to function")?;
    let name = name_fn
        .call0(&validator)
        .map_err(|e| format!("Failed to call name(): {:?}", e))?
        .as_string()
        .ok_or("name() must return a string")?;

    if name.is_empty() {
        return Err(JsValue::from_str("Validator name cannot be empty"));
    }

    let priority = if let Ok(priority_fn) = Reflect::get(&validator, &JsValue::from_str("priority")) {
        if priority_fn.is_function() {
            let priority_fn = priority_fn
                .dyn_into::<js_sys::Function>()
                .map_err(|_| "Failed to convert priority to function")?;
            priority_fn
                .call0(&validator)
                .map_err(|e| format!("Failed to call priority(): {:?}", e))?
                .as_f64()
                .map(|n| n as i32)
                .unwrap_or(50)
        } else {
            50
        }
    } else {
        50
    };

    let wrapper = JsValidatorWrapper::new(validator, name.clone(), priority);
    let registry = kreuzberg::plugins::registry::get_validator_registry();
    let mut registry = acquire_write_lock(&registry, "VALIDATORS").map_err(|e| JsValue::from_str(&e))?;

    registry
        .register(Arc::new(wrapper))
        .map_err(|e| JsValue::from_str(&format!("Registration failed: {}", e)))
}

/// Unregister a validator by name.
///
/// # Arguments
///
/// * `name` - Name of the validator to unregister
///
/// # Returns
///
/// Ok if unregistration succeeds, Err if the validator is not found or other error occurs.
///
/// # Example
///
/// ```javascript
/// unregisterValidator("min-content-length");
/// ```
#[wasm_bindgen]
pub fn unregister_validator(name: String) -> Result<(), JsValue> {
    let registry = kreuzberg::plugins::registry::get_validator_registry();
    let mut registry = acquire_write_lock(&registry, "VALIDATORS").map_err(|e| JsValue::from_str(&e))?;

    registry
        .remove(&name)
        .map_err(|e| JsValue::from_str(&format!("Unregistration failed: {}", e)))
}

/// Clear all registered validators.
///
/// # Returns
///
/// Ok if clearing succeeds, Err if an error occurs.
///
/// # Example
///
/// ```javascript
/// clearValidators();
/// ```
#[wasm_bindgen]
pub fn clear_validators() -> Result<(), JsValue> {
    let registry = kreuzberg::plugins::registry::get_validator_registry();
    let mut registry = acquire_write_lock(&registry, "VALIDATORS").map_err(|e| JsValue::from_str(&e))?;

    let names = registry.list();
    for name in names {
        registry
            .remove(&name)
            .map_err(|e| JsValue::from_str(&format!("Failed to remove validator: {}", e)))?;
    }

    Ok(())
}

/// List all registered validator names.
///
/// # Returns
///
/// Array of validator names, or Err if an error occurs.
///
/// # Example
///
/// ```javascript
/// const validators = listValidators();
/// console.log(validators); // ["min-content-length", ...]
/// ```
#[wasm_bindgen]
pub fn list_validators() -> Result<js_sys::Array, JsValue> {
    let registry = kreuzberg::plugins::registry::get_validator_registry();
    let registry = acquire_read_lock(&registry, "VALIDATORS").map_err(|e| JsValue::from_str(&e))?;

    let names = registry.list();
    let arr = js_sys::Array::new();
    for name in names {
        arr.push(&JsValue::from_str(&name));
    }

    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_mock_validator(name: &str) -> Result<JsValue, String> {
        let obj = js_sys::Object::new();

        Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &js_sys::Function::new_with_args("", &format!("return '{}'", name)),
        )
        .map_err(|_| "Failed to set name method".to_string())?;

        Reflect::set(
            &obj,
            &JsValue::from_str("validate"),
            &js_sys::Function::new_with_args("json", "return Promise.resolve('')"),
        )
        .map_err(|_| "Failed to set validate method".to_string())?;

        Reflect::set(
            &obj,
            &JsValue::from_str("priority"),
            &js_sys::Function::new_with_args("", "return 50"),
        )
        .map_err(|_| "Failed to set priority method".to_string())?;

        Ok(JsValue::from(obj))
    }

    #[wasm_bindgen_test]
    fn test_register_validator_valid_validator_succeeds() {
        clear_validators().ok();
        let validator = create_mock_validator("test-validator").expect("Failed to create mock validator");

        let result = register_validator(validator);

        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_register_validator_missing_name_fails() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("validate"),
            &js_sys::Function::new_with_args("json", "return Promise.resolve('')"),
        )
        .ok();

        let result = register_validator(JsValue::from(obj));

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_register_validator_missing_validate_fails() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &js_sys::Function::new_with_args("", "return 'test'"),
        )
        .ok();

        let result = register_validator(JsValue::from(obj));

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_register_validator_empty_name_fails() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &js_sys::Function::new_with_args("", "return ''"),
        )
        .ok();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("validate"),
            &js_sys::Function::new_with_args("json", "return Promise.resolve('')"),
        )
        .ok();

        let result = register_validator(JsValue::from(obj));

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_unregister_validator_registered_validator_succeeds() {
        clear_validators().ok();
        let validator = create_mock_validator("test-validator").expect("Failed to create mock validator");
        register_validator(validator).ok();

        let result = unregister_validator("test-validator".to_string());

        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_unregister_validator_unregistered_validator_fails() {
        clear_validators().ok();

        let result = unregister_validator("nonexistent".to_string());

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_clear_validators_removes_all() {
        clear_validators().ok();
        let validator1 = create_mock_validator("validator1").expect("Failed to create mock validator 1");
        let validator2 = create_mock_validator("validator2").expect("Failed to create mock validator 2");
        register_validator(validator1).ok();
        register_validator(validator2).ok();

        let result = clear_validators();

        assert!(result.is_ok());
        let list = list_validators().unwrap_or_else(|_| js_sys::Array::new());
        assert_eq!(list.length(), 0);
    }

    #[wasm_bindgen_test]
    fn test_list_validators_returns_array() {
        clear_validators().ok();

        let result = list_validators();

        assert!(result.is_ok());
        let arr = result.unwrap();
        assert!(arr.is_array());
    }

    #[wasm_bindgen_test]
    fn test_list_validators_after_register_contains_name() {
        clear_validators().ok();
        let validator = create_mock_validator("test-validator").expect("Failed to create mock validator");
        register_validator(validator).ok();

        let result = list_validators();

        assert!(result.is_ok());
        let arr = result.unwrap();
        assert!(arr.length() > 0);
    }

    #[wasm_bindgen_test]
    fn test_js_validator_wrapper_implements_plugin() {
        let validator = create_mock_validator("test").expect("Failed to create mock validator");
        let wrapper = JsValidatorWrapper::new(validator, "test".to_string(), 50);

        assert_eq!(wrapper.name(), "test");
        assert_eq!(wrapper.version(), "1.0.0");
        assert_eq!(wrapper.priority(), 50);
        assert!(wrapper.initialize().is_ok());
        assert!(wrapper.shutdown().is_ok());
    }

    #[wasm_bindgen_test]
    fn test_register_multiple_validators() {
        clear_validators().ok();
        let v1 = create_mock_validator("val1").expect("Failed to create mock validator 1");
        let v2 = create_mock_validator("val2").expect("Failed to create mock validator 2");

        assert!(register_validator(v1).is_ok());
        assert!(register_validator(v2).is_ok());

        let list = list_validators().unwrap();
        assert!(list.length() >= 2);
    }
}
