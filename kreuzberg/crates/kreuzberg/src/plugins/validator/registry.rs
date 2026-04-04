//! Validator registry management.
//!
//! This module provides functions for managing the global validator registry.

use super::r#trait::Validator;
use std::sync::Arc;

/// Register a validator with the global registry.
///
/// The validator will be registered with its default priority and will be called
/// during extraction validation. The validator's `name()` method is used as the
/// registration name.
///
/// # Arguments
///
/// * `validator` - The validator implementation wrapped in Arc
///
/// # Returns
///
/// - `Ok(())` if registration succeeded
/// - `Err(...)` if validation failed or initialization failed
///
/// # Errors
///
/// - `KreuzbergError::Validation` - Invalid validator name (empty or contains whitespace)
/// - Any error from the validator's `initialize()` method
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::{Plugin, Validator, register_validator};
/// use kreuzberg::{Result, ExtractionResult, ExtractionConfig, KreuzbergError};
/// use async_trait::async_trait;
/// use std::sync::Arc;
///
/// struct MinLengthValidator { min_length: usize }
///
/// impl Plugin for MinLengthValidator {
///     fn name(&self) -> &str { "min-length" }
///     fn version(&self) -> String { "1.0.0".to_string() }
///     fn initialize(&self) -> Result<()> { Ok(()) }
///     fn shutdown(&self) -> Result<()> { Ok(()) }
/// }
///
/// #[async_trait]
/// impl Validator for MinLengthValidator {
///     async fn validate(&self, result: &ExtractionResult, _: &ExtractionConfig) -> Result<()> {
///         if result.content.len() < self.min_length {
///             return Err(KreuzbergError::validation(
///                 format!("Content too short: {} < {}", result.content.len(), self.min_length)
///             ));
///         }
///         Ok(())
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let validator = Arc::new(MinLengthValidator { min_length: 10 });
/// register_validator(validator)?;
/// # Ok::<(), KreuzbergError>(())
/// # });
/// ```
pub fn register_validator(validator: Arc<dyn Validator>) -> crate::Result<()> {
    use crate::plugins::registry::get_validator_registry;

    let registry = get_validator_registry();
    let mut registry = registry.write();

    registry.register(validator)
}

/// Unregister a validator by name.
///
/// Removes the validator from the global registry and calls its `shutdown()` method.
///
/// # Arguments
///
/// * `name` - Name of the validator to unregister
///
/// # Returns
///
/// - `Ok(())` if the validator was unregistered or didn't exist
/// - `Err(...)` if the shutdown method failed
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::unregister_validator;
///
/// # tokio_test::block_on(async {
/// unregister_validator("min-length")?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn unregister_validator(name: &str) -> crate::Result<()> {
    use crate::plugins::registry::get_validator_registry;

    let registry = get_validator_registry();
    let mut registry = registry.write();

    registry.remove(name)
}

/// List all registered validators.
///
/// Returns the names of all validators currently registered in the global registry.
///
/// # Returns
///
/// A vector of validator names.
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::list_validators;
///
/// # tokio_test::block_on(async {
/// let validators = list_validators()?;
/// for name in validators {
///     println!("Registered validator: {}", name);
/// }
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn list_validators() -> crate::Result<Vec<String>> {
    use crate::plugins::registry::get_validator_registry;

    let registry = get_validator_registry();
    let registry = registry.read();

    Ok(registry.list())
}

/// Clear all validators from the global registry.
///
/// Removes all validators and calls their `shutdown()` methods.
///
/// # Returns
///
/// - `Ok(())` if all validators were cleared successfully
/// - `Err(...)` if any shutdown method failed
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::clear_validators;
///
/// # tokio_test::block_on(async {
/// clear_validators()?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn clear_validators() -> crate::Result<()> {
    use crate::plugins::registry::get_validator_registry;

    let registry = get_validator_registry();
    let mut registry = registry.write();

    registry.shutdown_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KreuzbergError;
    use crate::Result;
    use crate::core::config::ExtractionConfig;
    use crate::plugins::Plugin;
    use crate::types::ExtractionResult;
    use async_trait::async_trait;

    struct MockValidator {
        should_fail: bool,
    }

    impl Plugin for MockValidator {
        fn name(&self) -> &str {
            "mock-validator"
        }

        fn version(&self) -> String {
            "1.0.0".to_string()
        }

        fn initialize(&self) -> Result<()> {
            Ok(())
        }

        fn shutdown(&self) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl Validator for MockValidator {
        async fn validate(&self, _result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
            if self.should_fail {
                Err(KreuzbergError::validation("Validation failed".to_string()))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_register_validator() {
        use std::sync::Arc;

        let validator = Arc::new(MockValidator { should_fail: false });
        let result = super::register_validator(validator);
        assert!(result.is_ok());

        let _ = super::unregister_validator("mock-validator");
    }

    #[test]
    #[serial_test::serial]
    fn test_unregister_validator() {
        use std::sync::Arc;

        let validator = Arc::new(MockValidator { should_fail: false });
        super::register_validator(validator).unwrap();

        let result = super::unregister_validator("mock-validator");
        assert!(result.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_unregister_nonexistent_validator() {
        let result = super::unregister_validator("nonexistent-validator-xyz");
        assert!(result.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_list_validators() {
        use std::sync::Arc;

        super::clear_validators().unwrap();

        let validator1 = Arc::new(MockValidator { should_fail: false });
        let validator2 = Arc::new(MockValidator { should_fail: false });

        let list_before = super::list_validators().unwrap();
        assert_eq!(list_before.len(), 0);

        super::register_validator(validator1).unwrap();
        super::register_validator(validator2).unwrap();

        let list = super::list_validators().unwrap();
        assert_eq!(list.len(), 1);
        assert!(list.contains(&"mock-validator".to_string()));

        super::unregister_validator("mock-validator").unwrap();
    }

    #[test]
    #[serial_test::serial]
    fn test_clear_validators() {
        use std::sync::Arc;

        super::clear_validators().unwrap();

        let validator1 = Arc::new(MockValidator { should_fail: false });
        let validator2 = Arc::new(MockValidator { should_fail: false });

        super::register_validator(validator1).unwrap();
        super::register_validator(validator2).unwrap();

        let list_before = super::list_validators().unwrap();
        assert!(!list_before.is_empty());

        let result = super::clear_validators();
        assert!(result.is_ok());

        let list = super::list_validators().unwrap();
        assert_eq!(list.len(), 0);
    }

    #[test]
    #[serial_test::serial]
    fn test_register_validator_with_invalid_name() {
        use std::sync::Arc;

        struct InvalidNameValidator;
        impl Plugin for InvalidNameValidator {
            fn name(&self) -> &str {
                "invalid name with spaces"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl Validator for InvalidNameValidator {
            async fn validate(&self, _: &ExtractionResult, _: &ExtractionConfig) -> Result<()> {
                Ok(())
            }
        }

        let validator = Arc::new(InvalidNameValidator);
        let result = super::register_validator(validator);
        assert!(matches!(result, Err(KreuzbergError::Validation { .. })));
    }

    #[test]
    #[serial_test::serial]
    fn test_register_validator_with_empty_name() {
        use std::sync::Arc;

        struct EmptyNameValidator;
        impl Plugin for EmptyNameValidator {
            fn name(&self) -> &str {
                ""
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl Validator for EmptyNameValidator {
            async fn validate(&self, _: &ExtractionResult, _: &ExtractionConfig) -> Result<()> {
                Ok(())
            }
        }

        let validator = Arc::new(EmptyNameValidator);
        let result = super::register_validator(validator);
        assert!(matches!(result, Err(KreuzbergError::Validation { .. })));
    }
}
