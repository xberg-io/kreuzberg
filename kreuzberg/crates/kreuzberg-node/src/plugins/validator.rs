use async_trait::async_trait;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use std::sync::Arc;

use kreuzberg::plugins::Plugin;
use kreuzberg::plugins::Validator as RustValidator;
use kreuzberg::plugins::registry::get_validator_registry;

use crate::result::JsExtractionResult;

/// Wrapper that makes a JavaScript Validator usable from Rust.
///
/// The validate_fn is an async JavaScript function that:
/// - Takes: String (JSON-serialized ExtractionResult)
/// - Returns: Promise<String> (empty string on success, rejects on validation failure)
///
/// Type parameters:
/// - Input: String
/// - Return: Promise<String>
/// - CallJsBackArgs: Vec<String> (because build_callback returns vec![value])
/// - ErrorStatus: napi::Status
/// - CalleeHandled: false (default with build_callback)
struct JsValidator {
    name: String,
    validate_fn: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
    priority: i32,
}

unsafe impl Send for JsValidator {}
unsafe impl Sync for JsValidator {}

impl Plugin for JsValidator {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> String {
        "1.0.0".to_string()
    }

    fn initialize(&self) -> std::result::Result<(), kreuzberg::KreuzbergError> {
        Ok(())
    }

    fn shutdown(&self) -> std::result::Result<(), kreuzberg::KreuzbergError> {
        Ok(())
    }
}

#[async_trait]
impl RustValidator for JsValidator {
    async fn validate(
        &self,
        result: &kreuzberg::ExtractionResult,
        _config: &kreuzberg::ExtractionConfig,
    ) -> std::result::Result<(), kreuzberg::KreuzbergError> {
        let js_result =
            JsExtractionResult::try_from(result.clone()).map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("Failed to convert result for JavaScript Validator: {}", e),
                plugin_name: self.name.clone(),
            })?;

        let json_string = serde_json::to_string(&js_result).map_err(|e| kreuzberg::KreuzbergError::Plugin {
            message: format!("Failed to serialize result to JSON for JavaScript Validator: {}", e),
            plugin_name: self.name.clone(),
        })?;

        self.validate_fn
            .call_async(json_string)
            .await
            .map_err(|e| {
                let err_msg = e.to_string();
                if err_msg.contains("ValidationError") || err_msg.contains("validation") {
                    kreuzberg::KreuzbergError::Validation {
                        message: err_msg,
                        source: None,
                    }
                } else {
                    kreuzberg::KreuzbergError::Plugin {
                        message: format!("JavaScript Validator '{}' call failed: {}", self.name, err_msg),
                        plugin_name: self.name.clone(),
                    }
                }
            })?
            .await
            .map_err(|e| {
                let err_msg = e.to_string();
                if err_msg.contains("ValidationError") || err_msg.contains("validation") {
                    kreuzberg::KreuzbergError::Validation {
                        message: err_msg,
                        source: None,
                    }
                } else {
                    kreuzberg::KreuzbergError::Plugin {
                        message: format!("JavaScript Validator '{}' promise failed: {}", self.name, err_msg),
                        plugin_name: self.name.clone(),
                    }
                }
            })?;

        Ok(())
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}

/// Register a custom validator
///
/// Registers a JavaScript Validator that will be called after extraction.
///
/// # Arguments
///
/// * `validator` - JavaScript object with the following interface:
///   - `name(): string` - Unique validator name
///   - `validate(...args): Promise<string>` - Validate function that receives JSON string as args\[0\]
///   - `priority(): number` - Optional priority (defaults to 50, higher runs first)
///
/// # Implementation Notes
///
/// Due to NAPI ThreadsafeFunction limitations, the validate function receives the extraction
/// result as a JSON string in args\[0\]. On success, return an empty string. On validation
/// failure, throw an error (the Promise should reject). Use the TypeScript wrapper functions
/// for a cleaner API.
///
/// # Example
///
/// ```typescript
/// import { registerValidator } from '@kreuzberg/node';
///
/// registerValidator({
///   name: () => "min-length",
///   priority: () => 100,
///   validate: async (...args) => {
///     const result = JSON.parse(args[0]);
///     if (result.content.length < 100) {
///       throw new Error("ValidationError: Content too short");
///     }
///     return ""; // Success - return empty string
///   }
/// });
/// ```
#[napi]
pub fn register_validator(_env: Env, validator: Object) -> Result<()> {
    use super::validate_plugin_object;

    validate_plugin_object(&validator, "Validator", &["name", "validate"])?;

    let name: String = validator.get_named_property::<String>("name").or_else(|_| {
        let name_fn: Function<(), String> = validator.get_named_property("name")?;
        name_fn.call(())
    })?;

    if name.is_empty() {
        return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "Validator name cannot be empty".to_string(),
        ));
    }

    let priority = if let Ok(priority_val) = validator.get_named_property::<i32>("priority") {
        priority_val
    } else if let Ok(priority_fn) = validator.get_named_property::<Function<(), i32>>("priority") {
        priority_fn.call(())?
    } else {
        50
    };

    let validate_fn: Function<String, Promise<String>> = validator.get_named_property("validate")?;

    let tsfn = validate_fn
        .build_threadsafe_function()
        .build_callback(|ctx| Ok(vec![ctx.value]))?;

    let js_validator = JsValidator {
        name: name.clone(),
        validate_fn: Arc::new(tsfn),
        priority,
    };

    let arc_validator: Arc<dyn RustValidator> = Arc::new(js_validator);
    let registry = get_validator_registry();
    let mut registry = registry.write();

    registry.register(arc_validator).map_err(|e| {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to register Validator '{}': {}", name, e),
        )
    })?;

    Ok(())
}

/// Unregister a validator by name
#[napi]
pub fn unregister_validator(name: String) -> Result<()> {
    let registry = get_validator_registry();
    let mut registry = registry.write();

    registry.remove(&name).map_err(|e| {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to unregister Validator '{}': {}", name, e),
        )
    })?;
    Ok(())
}

/// Clear all registered validators
#[napi]
pub fn clear_validators() -> Result<()> {
    let registry = get_validator_registry();
    let mut registry = registry.write();

    *registry = Default::default();
    Ok(())
}

/// List all registered validators
#[napi]
pub fn list_validators() -> Result<Vec<String>> {
    let registry = get_validator_registry();
    let registry = registry.read();

    Ok(registry.list())
}
