use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

use kreuzberg::plugins::ProcessingStage;

// Plugin impl stub - not currently functional after module refactoring
// impl Plugin for JsPostProcessor {
//     fn name(&self) -> &str {
//         &self.name
//     }
//
//     fn version(&self) -> String {
//         "1.0.0".to_string()
//     }
//
//     fn initialize(&self) -> std::result::Result<(), kreuzberg::KreuzbergError> {
//         Ok(())
//     }
//
//     fn shutdown(&self) -> std::result::Result<(), kreuzberg::KreuzbergError> {
//         Ok(())
//     }
// }
//
// #[async_trait]
// impl RustPostProcessor for JsPostProcessor {
//     async fn process(
//         &self,
//         result: &mut kreuzberg::ExtractionResult,
//         _config: &kreuzberg::ExtractionConfig,
//     ) -> std::result::Result<(), kreuzberg::KreuzbergError> {
//         Ok(())
//     }
//
//     fn processing_stage(&self) -> ProcessingStage {
//         self.stage
//     }
// }

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
#[allow(dead_code)]
pub struct JsPostProcessor {
    pub name: String,
    pub process_fn: ThreadsafeFunction<String>,
    pub stage: ProcessingStage,
}

#[napi]
pub fn register_post_processor(_processor: Object) -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "register_post_processor not yet implemented for refactored module",
    ))
}

#[napi]
pub fn unregister_post_processor(_name: String) -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "unregister_post_processor not yet implemented",
    ))
}

#[napi]
pub fn clear_post_processors() -> Result<()> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "clear_post_processors not yet implemented",
    ))
}

#[napi]
pub fn list_post_processors() -> Result<Vec<String>> {
    Err(napi::Error::new(
        napi::Status::GenericFailure,
        "list_post_processors not yet implemented",
    ))
}
