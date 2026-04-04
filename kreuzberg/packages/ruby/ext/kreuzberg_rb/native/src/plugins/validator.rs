//! Validator plugin registration and management

use crate::{error_handling::{kreuzberg_error, runtime_error}, gc_guarded_value::GcGuardedValue};
use magnus::{Error, Value, scan_args::scan_args, Ruby};
use magnus::value::ReprValue;
use std::sync::Arc;

/// Register a validator plugin
pub fn register_validator(args: &[Value]) -> Result<(), Error> {
    let _ruby = Ruby::get().expect("Ruby not initialized");
    let args = scan_args::<(String, Value), (Option<i32>,), (), (), (), ()>(args)?;
    let (name, validator) = args.required;
    let (priority,) = args.optional;
    let priority = priority.unwrap_or(50);

    if !validator.respond_to("call", true)? {
        return Err(runtime_error("Validator must be a Proc or respond to 'call'"));
    }

    use async_trait::async_trait;
    use kreuzberg::plugins::{Plugin, Validator};

    struct RubyValidator {
        name: String,
        validator: GcGuardedValue,
        priority: i32,
    }

    unsafe impl Send for RubyValidator {}
    unsafe impl Sync for RubyValidator {}

    impl Plugin for RubyValidator {
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

    #[async_trait]
    impl Validator for RubyValidator {
        async fn validate(
            &self,
            result: &kreuzberg::ExtractionResult,
            _config: &kreuzberg::ExtractionConfig,
        ) -> kreuzberg::Result<()> {
            let validator_name = self.name.clone();
            let validator = self.validator.value();
            let result_clone = result.clone();

            tokio::task::block_in_place(|| {
                let ruby = Ruby::get().expect("Ruby not initialized");
                let result_hash =
                    crate::result::extraction_result_to_ruby(&ruby, result_clone).map_err(|e| kreuzberg::KreuzbergError::Plugin {
                        message: format!("Failed to convert result to Ruby: {}", e),
                        plugin_name: validator_name.clone(),
                    })?;

                validator
                    .funcall::<_, _, magnus::Value>("call", (result_hash,))
                    .map_err(|e| kreuzberg::KreuzbergError::Validation {
                        message: format!("Validation failed: {}", e),
                        source: None,
                    })?;

                Ok(())
            })
        }

        fn priority(&self) -> i32 {
            self.priority
        }
    }

    let validator_impl = Arc::new(RubyValidator {
        name: name.clone(),
        validator: GcGuardedValue::new(validator),
        priority,
    });

    let registry = kreuzberg::get_validator_registry();
    registry
        .write()
        .register(validator_impl)
        .map_err(kreuzberg_error)?;

    Ok(())
}
