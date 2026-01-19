use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

// Plugin impl stub - not currently functional after module refactoring

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
#[allow(dead_code)]
pub struct JsValidator {
    pub name: String,
    pub validate_fn: ThreadsafeFunction<String>,
    pub priority: i32,
}

#[napi]
pub fn register_validator(_validator: Object) -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "register_validator not yet implemented for refactored module",
    ))
}

#[napi]
pub fn unregister_validator(_name: String) -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "unregister_validator not yet implemented",
    ))
}

#[napi]
pub fn clear_validators() -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "clear_validators not yet implemented",
    ))
}

#[napi]
pub fn list_validators() -> Result<Vec<String>> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "list_validators not yet implemented",
    ))
}
