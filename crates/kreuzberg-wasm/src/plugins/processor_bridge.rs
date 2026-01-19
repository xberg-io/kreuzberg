//! PostProcessor wrapper implementation for WASM bindings
//!
//! This module provides the WASM bridge for custom post-processor plugins that
//! can process extraction results after initial content extraction.

#[allow(unused_imports)]
use super::{JsPluginValue, MakeSend, acquire_read_lock, acquire_write_lock};
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use js_sys::{Promise, Reflect};
use kreuzberg::plugins::{Plugin, PostProcessor, ProcessingStage};
#[allow(unused_imports)]
use kreuzberg::{ExtractionConfig, ExtractionResult, KreuzbergError};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use wasm_bindgen_futures::JsFuture;

/// Wrapper that makes a JavaScript PostProcessor object usable from Rust.
///
/// # Thread Safety
///
/// This wrapper contains a JsValue which is NOT Send/Sync. Plugin callbacks
/// MUST be invoked only on the main JavaScript thread. The type system
/// enforces this by preventing the wrapper from being moved across threads.
struct JsPostProcessorWrapper {
    name: String,
    #[allow(dead_code)]
    js_obj: JsPluginValue,
    stage: ProcessingStage,
}

impl JsPostProcessorWrapper {
    /// Create a new wrapper from a JS object
    ///
    /// # Safety
    ///
    /// This wrapper must only be accessed from the main JavaScript thread.
    /// Do not pass this to Web Workers or rayon tasks.
    fn new(js_obj: JsValue, name: String, stage: ProcessingStage) -> Self {
        Self {
            js_obj: JsPluginValue(js_obj),
            name,
            stage,
        }
    }
}

impl Plugin for JsPostProcessorWrapper {
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
impl PostProcessor for JsPostProcessorWrapper {
    async fn process(&self, result: &mut ExtractionResult, _config: &ExtractionConfig) -> kreuzberg::Result<()> {
        let json_input = serde_json::to_string(&*result).map_err(|e| KreuzbergError::Plugin {
            message: format!("Failed to serialize extraction result: {}", e),
            plugin_name: self.name.clone(),
        })?;

        let json_input_copy = json_input.clone();
        let name_copy = self.name.clone();

        let promise = {
            let process_fn = Reflect::get(&self.js_obj.0, &JsValue::from_str("process"))
                .map_err(|_| KreuzbergError::Plugin {
                    message: format!("PostProcessor '{}' missing 'process' method", self.name),
                    plugin_name: self.name.clone(),
                })?
                .dyn_into::<js_sys::Function>()
                .map_err(|_| KreuzbergError::Plugin {
                    message: format!("PostProcessor '{}' process is not a function", self.name),
                    plugin_name: self.name.clone(),
                })?;

            let promise_val = process_fn
                .call1(&self.js_obj.0, &JsValue::from_str(&json_input_copy))
                .map_err(|e| KreuzbergError::Plugin {
                    message: format!("PostProcessor '{}' process call failed: {:?}", name_copy, e),
                    plugin_name: name_copy.clone(),
                })?;

            Promise::resolve(&promise_val)
        };

        let result_val = MakeSend(JsFuture::from(promise))
            .await
            .map_err(|e| KreuzbergError::Plugin {
                message: format!("PostProcessor '{}' promise failed: {:?}", self.name, e),
                plugin_name: self.name.clone(),
            })?;

        let json_output = result_val.as_string().ok_or_else(|| KreuzbergError::Plugin {
            message: format!("PostProcessor '{}' returned non-string result", self.name),
            plugin_name: self.name.clone(),
        })?;

        let updated: ExtractionResult = serde_json::from_str(&json_output).map_err(|e| KreuzbergError::Plugin {
            message: format!("Failed to deserialize PostProcessor result: {}", e),
            plugin_name: self.name.clone(),
        })?;

        *result = updated;
        Ok(())
    }

    fn processing_stage(&self) -> ProcessingStage {
        self.stage
    }
}

#[cfg(test)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PostProcessor for JsPostProcessorWrapper {
    async fn process(&self, _result: &mut ExtractionResult, _config: &ExtractionConfig) -> kreuzberg::Result<()> {
        let _ = &self.js_obj.0;
        Ok(())
    }

    fn processing_stage(&self) -> ProcessingStage {
        self.stage
    }
}

/// Register a custom post-processor.
///
/// # Arguments
///
/// * `processor` - JavaScript object implementing the PostProcessorProtocol interface:
///   - `name(): string` - Unique processor name
///   - `process(jsonString: string): Promise<string>` - Process function that takes JSON input
///   - `processingStage(): "early" | "middle" | "late"` - Optional processing stage (defaults to "middle")
///
/// # Returns
///
/// Ok if registration succeeds, Err with description if it fails.
///
/// # Example
///
/// ```javascript
/// registerPostProcessor({
///   name: () => "my-post-processor",
///   processingStage: () => "middle",
///   process: async (jsonString) => {
///     const result = JSON.parse(jsonString);
///     // Process the extraction result
///     result.metadata.processed_by = "my-post-processor";
///     return JSON.stringify(result);
///   }
/// });
/// ```
#[wasm_bindgen]
pub fn register_post_processor(processor: JsValue) -> Result<(), JsValue> {
    let name_fn =
        Reflect::get(&processor, &JsValue::from_str("name")).map_err(|e| format!("Missing 'name' method: {:?}", e))?;

    let process_fn = Reflect::get(&processor, &JsValue::from_str("process"))
        .map_err(|e| format!("Missing 'process' method: {:?}", e))?;

    if !name_fn.is_function() || !process_fn.is_function() {
        return Err(JsValue::from_str("name and process must be functions"));
    }

    let name_fn = name_fn
        .dyn_into::<js_sys::Function>()
        .map_err(|_| "Failed to convert name to function")?;
    let name = name_fn
        .call0(&processor)
        .map_err(|e| format!("Failed to call name(): {:?}", e))?
        .as_string()
        .ok_or("name() must return a string")?;

    if name.is_empty() {
        return Err(JsValue::from_str("Processor name cannot be empty"));
    }

    let stage = if let Ok(stage_fn) = Reflect::get(&processor, &JsValue::from_str("processingStage")) {
        if stage_fn.is_function() {
            let stage_fn = stage_fn
                .dyn_into::<js_sys::Function>()
                .map_err(|_| "Failed to convert processingStage to function")?;
            let stage_str = stage_fn
                .call0(&processor)
                .map_err(|e| format!("Failed to call processingStage(): {:?}", e))?
                .as_string()
                .unwrap_or_else(|| "middle".to_string());

            match stage_str.to_lowercase().as_str() {
                "early" => ProcessingStage::Early,
                "late" => ProcessingStage::Late,
                _ => ProcessingStage::Middle,
            }
        } else {
            ProcessingStage::Middle
        }
    } else {
        ProcessingStage::Middle
    };

    let wrapper = JsPostProcessorWrapper::new(processor, name.clone(), stage);
    let registry = kreuzberg::plugins::registry::get_post_processor_registry();
    let mut registry = acquire_write_lock(&registry, "POST_PROCESSORS").map_err(|e| JsValue::from_str(&e))?;

    registry
        .register(Arc::new(wrapper), 0)
        .map_err(|e| JsValue::from_str(&format!("Registration failed: {}", e)))
}

/// Unregister a post-processor by name.
///
/// # Arguments
///
/// * `name` - Name of the post-processor to unregister
///
/// # Returns
///
/// Ok if unregistration succeeds, Err if the processor is not found or other error occurs.
///
/// # Example
///
/// ```javascript
/// unregisterPostProcessor("my-post-processor");
/// ```
#[wasm_bindgen]
pub fn unregister_post_processor(name: String) -> Result<(), JsValue> {
    let registry = kreuzberg::plugins::registry::get_post_processor_registry();
    let mut registry = acquire_write_lock(&registry, "POST_PROCESSORS").map_err(|e| JsValue::from_str(&e))?;

    registry
        .remove(&name)
        .map_err(|e| JsValue::from_str(&format!("Unregistration failed: {}", e)))
}

/// Clear all registered post-processors.
///
/// # Returns
///
/// Ok if clearing succeeds, Err if an error occurs.
///
/// # Example
///
/// ```javascript
/// clearPostProcessors();
/// ```
#[wasm_bindgen]
pub fn clear_post_processors() -> Result<(), JsValue> {
    let registry = kreuzberg::plugins::registry::get_post_processor_registry();
    let mut registry = acquire_write_lock(&registry, "POST_PROCESSORS").map_err(|e| JsValue::from_str(&e))?;

    let names = registry.list();
    for name in names {
        registry
            .remove(&name)
            .map_err(|e| JsValue::from_str(&format!("Failed to remove post-processor: {}", e)))?;
    }

    Ok(())
}

/// List all registered post-processor names.
///
/// # Returns
///
/// Array of post-processor names, or Err if an error occurs.
///
/// # Example
///
/// ```javascript
/// const processors = listPostProcessors();
/// console.log(processors); // ["my-post-processor", ...]
/// ```
#[wasm_bindgen]
pub fn list_post_processors() -> Result<js_sys::Array, JsValue> {
    let registry = kreuzberg::plugins::registry::get_post_processor_registry();
    let registry = acquire_read_lock(&registry, "POST_PROCESSORS").map_err(|e| JsValue::from_str(&e))?;

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

    fn create_mock_processor(name: &str) -> Result<JsValue, String> {
        let obj = js_sys::Object::new();

        Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &js_sys::Function::new_with_args("", &format!("return '{}'", name)),
        )
        .map_err(|_| "Failed to set name method".to_string())?;

        Reflect::set(
            &obj,
            &JsValue::from_str("process"),
            &js_sys::Function::new_with_args("json", "return Promise.resolve(json)"),
        )
        .map_err(|_| "Failed to set process method".to_string())?;

        Reflect::set(
            &obj,
            &JsValue::from_str("processingStage"),
            &js_sys::Function::new_with_args("", "return 'middle'"),
        )
        .map_err(|_| "Failed to set processingStage method".to_string())?;

        Ok(JsValue::from(obj))
    }

    #[wasm_bindgen_test]
    fn test_register_post_processor_valid_processor_succeeds() {
        clear_post_processors().ok();
        let processor = create_mock_processor("test-processor").expect("Failed to create mock processor");

        let result = register_post_processor(processor);

        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_register_post_processor_missing_name_fails() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("process"),
            &js_sys::Function::new_with_args("json", "return Promise.resolve(json)"),
        )
        .ok();

        let result = register_post_processor(JsValue::from(obj));

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_register_post_processor_missing_process_fails() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &js_sys::Function::new_with_args("", "return 'test'"),
        )
        .ok();

        let result = register_post_processor(JsValue::from(obj));

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_register_post_processor_empty_name_fails() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("name"),
            &js_sys::Function::new_with_args("", "return ''"),
        )
        .ok();
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("process"),
            &js_sys::Function::new_with_args("json", "return Promise.resolve(json)"),
        )
        .ok();

        let result = register_post_processor(JsValue::from(obj));

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_unregister_post_processor_registered_processor_succeeds() {
        clear_post_processors().ok();
        let processor = create_mock_processor("test-processor").expect("Failed to create mock processor");
        register_post_processor(processor).ok();

        let result = unregister_post_processor("test-processor".to_string());

        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_unregister_post_processor_unregistered_processor_fails() {
        clear_post_processors().ok();

        let result = unregister_post_processor("nonexistent".to_string());

        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_clear_post_processors_removes_all() {
        clear_post_processors().ok();
        let processor1 = create_mock_processor("processor1").expect("Failed to create mock processor 1");
        let processor2 = create_mock_processor("processor2").expect("Failed to create mock processor 2");
        register_post_processor(processor1).ok();
        register_post_processor(processor2).ok();

        let result = clear_post_processors();

        assert!(result.is_ok());
        let list = list_post_processors().unwrap_or_else(|_| js_sys::Array::new());
        assert_eq!(list.length(), 0);
    }

    #[wasm_bindgen_test]
    fn test_list_post_processors_returns_array() {
        clear_post_processors().ok();

        let result = list_post_processors();

        assert!(result.is_ok());
        let arr = result.unwrap();
        assert!(arr.is_array());
    }

    #[wasm_bindgen_test]
    fn test_list_post_processors_after_register_contains_name() {
        clear_post_processors().ok();
        let processor = create_mock_processor("test-processor").expect("Failed to create mock processor");
        register_post_processor(processor).ok();

        let result = list_post_processors();

        assert!(result.is_ok());
        let arr = result.unwrap();
        assert!(arr.length() > 0);
    }

    #[wasm_bindgen_test]
    fn test_js_post_processor_wrapper_implements_plugin() {
        let processor = create_mock_processor("test").expect("Failed to create mock processor");
        let wrapper = JsPostProcessorWrapper::new(processor, "test".to_string(), ProcessingStage::Middle);

        assert_eq!(wrapper.name(), "test");
        assert_eq!(wrapper.version(), "1.0.0");
        assert!(wrapper.initialize().is_ok());
        assert!(wrapper.shutdown().is_ok());
    }

    #[wasm_bindgen_test]
    fn test_processor_processing_stage_early() {
        let processor = create_mock_processor("test").expect("Failed to create mock processor");
        let wrapper = JsPostProcessorWrapper::new(processor, "test".to_string(), ProcessingStage::Early);

        assert_eq!(wrapper.processing_stage(), ProcessingStage::Early);
    }

    #[wasm_bindgen_test]
    fn test_processor_processing_stage_late() {
        let processor = create_mock_processor("test").expect("Failed to create mock processor");
        let wrapper = JsPostProcessorWrapper::new(processor, "test".to_string(), ProcessingStage::Late);

        assert_eq!(wrapper.processing_stage(), ProcessingStage::Late);
    }

    #[wasm_bindgen_test]
    fn test_register_multiple_post_processors() {
        clear_post_processors().ok();
        let p1 = create_mock_processor("proc1").expect("Failed to create mock processor 1");
        let p2 = create_mock_processor("proc2").expect("Failed to create mock processor 2");
        let p3 = create_mock_processor("proc3").expect("Failed to create mock processor 3");

        assert!(register_post_processor(p1).is_ok());
        assert!(register_post_processor(p2).is_ok());
        assert!(register_post_processor(p3).is_ok());

        let list = list_post_processors().unwrap();
        assert!(list.length() >= 3);
    }
}
