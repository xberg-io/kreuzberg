use async_trait::async_trait;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use std::sync::Arc;

use kreuzberg::plugins::Plugin;
use kreuzberg::plugins::PostProcessor as RustPostProcessor;
use kreuzberg::plugins::ProcessingStage;
use kreuzberg::plugins::registry::get_post_processor_registry;

use crate::result::JsExtractionResult;

/// Wrapper that makes a JavaScript PostProcessor usable from Rust.
///
/// The process_fn is an async JavaScript function that:
/// - Takes: String (JSON-serialized ExtractionResult)
/// - Returns: Promise<String> (JSON-serialized ExtractionResult)
///
/// Type parameters:
/// - Input: String
/// - Return: Promise<String>
/// - CallJsBackArgs: Vec<String> (because build_callback returns vec![value])
/// - ErrorStatus: napi::Status
/// - CalleeHandled: false (default with build_callback)
struct JsPostProcessor {
    name: String,
    process_fn: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
    stage: ProcessingStage,
}

unsafe impl Send for JsPostProcessor {}
unsafe impl Sync for JsPostProcessor {}

impl Plugin for JsPostProcessor {
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
impl RustPostProcessor for JsPostProcessor {
    async fn process(
        &self,
        result: &mut kreuzberg::ExtractionResult,
        _config: &kreuzberg::ExtractionConfig,
    ) -> std::result::Result<(), kreuzberg::KreuzbergError> {
        let js_result =
            JsExtractionResult::try_from(result.clone()).map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("Failed to convert result for JavaScript PostProcessor: {}", e),
                plugin_name: self.name.clone(),
            })?;

        let json_string = serde_json::to_string(&js_result).map_err(|e| kreuzberg::KreuzbergError::Plugin {
            message: format!("Failed to serialize result to JSON for JavaScript PostProcessor: {}", e),
            plugin_name: self.name.clone(),
        })?;

        let output_json = self
            .process_fn
            .call_async(json_string)
            .await
            .map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("JavaScript PostProcessor '{}' call failed: {}", self.name, e),
                plugin_name: self.name.clone(),
            })?
            .await
            .map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("JavaScript PostProcessor '{}' promise failed: {}", self.name, e),
                plugin_name: self.name.clone(),
            })?;

        let updated: JsExtractionResult =
            serde_json::from_str(&output_json).map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!(
                    "Failed to deserialize JSON result from JavaScript PostProcessor '{}': {}",
                    self.name, e
                ),
                plugin_name: self.name.clone(),
            })?;

        let rust_result =
            kreuzberg::ExtractionResult::try_from(updated).map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("Failed to convert result from JavaScript PostProcessor: {}", e),
                plugin_name: self.name.clone(),
            })?;

        *result = rust_result;

        Ok(())
    }

    fn processing_stage(&self) -> ProcessingStage {
        self.stage
    }
}

/// Register a custom postprocessor
///
/// Registers a JavaScript PostProcessor that will be called after extraction.
///
/// # Arguments
///
/// * `processor` - JavaScript object with the following interface:
///   - `name(): string` - Unique processor name
///   - `process(...args): string` - Process function that receives JSON string as args\[0\]
///   - `processingStage(): "early" | "middle" | "late"` - Optional processing stage
///
/// # Implementation Notes
///
/// Due to NAPI ThreadsafeFunction limitations, the process function receives the extraction
/// result as a JSON string in args\[0\] and must return a JSON string. Use the TypeScript
/// wrapper functions for a cleaner API.
///
/// # Example
///
/// ```typescript
/// import { registerPostProcessor } from '@kreuzberg/node';
///
/// registerPostProcessor({
///   name: () => "word-counter",
///   processingStage: () => "middle",
///   process: (...args) => {
///     const result = JSON.parse(args[0]);
///     const wordCount = result.content.split(/\s+/).length;
///     result.metadata.word_count = wordCount;
///     return JSON.stringify(result);
///   }
/// });
/// ```
#[napi]
pub fn register_post_processor(_env: Env, processor: Object) -> Result<()> {
    use super::validate_plugin_object;

    validate_plugin_object(&processor, "PostProcessor", &["name", "process"])?;

    let name: String = processor.get_named_property::<String>("name").or_else(|_| {
        let name_fn: Function<(), String> = processor.get_named_property("name")?;
        name_fn.call(())
    })?;

    if name.is_empty() {
        return Err(napi::Error::new(
            napi::Status::InvalidArg,
            "Processor name cannot be empty".to_string(),
        ));
    }

    let stage = if let Ok(stage_str) = processor.get_named_property::<String>("processingStage") {
        match stage_str.to_lowercase().as_str() {
            "early" => ProcessingStage::Early,
            "middle" => ProcessingStage::Middle,
            "late" => ProcessingStage::Late,
            _ => ProcessingStage::Middle,
        }
    } else if let Ok(stage_fn) = processor.get_named_property::<Function<(), String>>("processingStage") {
        let stage_str: String = stage_fn.call(())?;
        match stage_str.to_lowercase().as_str() {
            "early" => ProcessingStage::Early,
            "middle" => ProcessingStage::Middle,
            "late" => ProcessingStage::Late,
            _ => ProcessingStage::Middle,
        }
    } else {
        ProcessingStage::Middle
    };

    let process_fn: Function<String, Promise<String>> = processor.get_named_property("process")?;

    let tsfn = process_fn
        .build_threadsafe_function()
        .build_callback(|ctx| Ok(vec![ctx.value]))?;

    let js_processor = JsPostProcessor {
        name: name.clone(),
        process_fn: Arc::new(tsfn),
        stage,
    };

    let arc_processor: Arc<dyn RustPostProcessor> = Arc::new(js_processor);
    let registry = get_post_processor_registry();
    let mut registry = registry.write();

    registry.register(arc_processor, 50).map_err(|e| {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to register PostProcessor '{}': {}", name, e),
        )
    })?;

    Ok(())
}

/// Unregister a postprocessor by name
#[napi]
pub fn unregister_post_processor(name: String) -> Result<()> {
    let registry = get_post_processor_registry();
    let mut registry = registry.write();

    registry.remove(&name).map_err(|e| {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("Failed to unregister PostProcessor '{}': {}", name, e),
        )
    })?;
    Ok(())
}

/// Clear all registered postprocessors
#[napi]
pub fn clear_post_processors() -> Result<()> {
    let registry = get_post_processor_registry();
    let mut registry = registry.write();

    *registry = Default::default();
    Ok(())
}

/// List all registered post-processors
#[napi]
pub fn list_post_processors() -> Result<Vec<String>> {
    let registry = get_post_processor_registry();
    let registry = registry.read();

    Ok(registry.list())
}
