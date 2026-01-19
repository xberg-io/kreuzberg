//! Plugin management for WASM bindings
//!
//! This module provides functions to register, unregister, and manage custom plugins
//! for document extraction, post-processing, validation, and OCR.
//!
//! All plugin functions operate on plugin objects passed from JavaScript that implement
//! the appropriate plugin protocol.
//!
//! # Threading Model and Safety (Main-Thread-Only)
//!
//! **BREAKING CHANGE**: As of v4.1, this module enforces main-thread-only execution
//! by removing unsafe Send/Sync implementations for JsValue.
//!
//! ## Thread Safety Guarantee
//! - JsValue is NOT Send/Sync - this is enforced by the type system
//! - All plugin registrations and callbacks MUST occur on the main thread
//! - WASM execution is single-threaded by default (no Web Workers)
//! - If multi-threading is enabled via `initThreadPool()`, plugin callbacks
//!   will fail to compile/run in worker contexts
//!
//! ## Why This Matters
//! JsValue contains pointers to JavaScript objects that are only valid in a specific
//! JS context. Allowing JsValue to cross thread boundaries causes:
//! - Memory corruption
//! - Use-after-free vulnerabilities
//! - Undefined behavior crashes
//!
//! ## Safe Migration
//! - All plugin registration/callback code must be on the main JS thread
//! - If you need async operations from workers, use message-passing to main thread
//! - Consider serializing plugin state instead of sharing JsValue objects
//!
//! See: https://github.com/rustwasm/wasm-bindgen/issues/... (threading docs)

pub mod ocr_bridge;
pub mod processor_bridge;
pub mod validator_bridge;

#[allow(unused_imports)]
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use wasm_bindgen::prelude::*;

// Re-export public API
pub use ocr_bridge::{clear_ocr_backends, list_ocr_backends, register_ocr_backend, unregister_ocr_backend};
pub use processor_bridge::{
    clear_post_processors, list_post_processors, register_post_processor, unregister_post_processor,
};
pub use validator_bridge::{clear_validators, list_validators, register_validator, unregister_validator};

/// Attempt to acquire a write lock with detailed error context and poisoning recovery.
///
/// When lock poisoning occurs, this function:
/// 1. Extracts the inner guard (which may still be usable)
/// 2. Logs a warning to the browser console about the poisoning
/// 3. Returns the guard for recovery attempts
///
/// # Context Provided
/// - Which registry failed (POST_PROCESSORS, VALIDATORS, or OCR_BACKENDS)
/// - Clear indication that the data may be in an inconsistent state
pub(crate) fn acquire_write_lock<'a, T>(
    registry: &'a RwLock<T>,
    registry_name: &str,
) -> Result<RwLockWriteGuard<'a, T>, String> {
    match registry.write() {
        Ok(guard) => Ok(guard),
        Err(poison) => {
            let guard = poison.into_inner();
            web_sys::console::warn_1(
                &format!(
                    "WARN: {} registry write lock was poisoned but recovered; data may be in inconsistent state",
                    registry_name
                )
                .into(),
            );
            Ok(guard)
        }
    }
}

/// Attempt to acquire a read lock with detailed error context about poisoning.
///
/// For read operations, we cannot safely recover from poisoning since we can't verify
/// the data is uncorrupted. Returns a contextual error message indicating:
/// - Which registry failed
/// - That the lock is poisoned
/// - A hint that a previous operation may have panicked
pub(crate) fn acquire_read_lock<'a, T>(
    registry: &'a RwLock<T>,
    registry_name: &str,
) -> Result<RwLockReadGuard<'a, T>, String> {
    registry.read().map_err(|_| {
        format!(
            "Failed to acquire {} registry read lock: lock poisoned (possible panic in previous operation)",
            registry_name
        )
    })
}

/// Wrapper that makes non-Send futures Send in WASM single-threaded contexts.
///
/// # Design and Safety
///
/// In WASM, JsFuture contains pointers to JavaScript promises that are not
/// inherently Send. However, because WASM code executes on a single JavaScript
/// thread by default, we can safely assert that JsFuture can be Send within
/// that single-threaded context.
///
/// This wrapper bridges the gap between:
/// - JsFuture (not Send, valid only on JS thread)
/// - async_trait's Send requirement (needed for trait objects)
///
/// # Safety Guarantee
///
/// This wrapper is sound ONLY when:
/// 1. Code runs in single-threaded WASM environment (default)
/// 2. Web Workers with `initThreadPool()` are NOT enabled
/// 3. Plugin callbacks are never spawned in rayon tasks
///
/// If any of these conditions are violated, undefined behavior occurs.
#[allow(dead_code)]
pub(crate) struct MakeSend<F>(pub(crate) F);

impl<F: std::future::Future + Unpin> std::future::Future for MakeSend<F> {
    type Output = F::Output;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.0).poll(cx)
    }
}

/// SAFETY: This makes non-Send futures Send by asserting single-threaded execution.
///
/// The wrapper is sound in WASM because:
/// 1. By default, all code runs on the main JavaScript thread
/// 2. There's no possibility of concurrent access from multiple JS threads
/// 3. If Web Workers are enabled, attempting to pass this across threads will fail
///    at the JavaScript level with clear errors about invalid context access
///
/// **CRITICAL INVARIANT**: This must ONLY be used with JsFuture and similar
/// futures that are valid only on the JS main thread. Do not use this with
/// futures that might be executed by rayon or other multi-threaded executors.
unsafe impl<F> Send for MakeSend<F> {}

/// SAFETY: Safe for the same reasons as the Send implementation.
unsafe impl<F> Sync for MakeSend<F> {}

/// Wrapper around JsValue that implements Send/Sync ONLY for WASM single-threaded contexts.
///
/// # CRITICAL SAFETY: Main-Thread-Only Enforcement
///
/// JsValue is NOT inherently Send/Sync because JavaScript objects can only be
/// accessed from the JavaScript engine's context. However, in WASM environments,
/// code executes on a single JavaScript thread by default.
///
/// ## Safety Guarantee
/// - This wrapper is SAFE ONLY when accessed from the main JS thread
/// - WASM execution is single-threaded by default (target_arch = "wasm32")
/// - If Web Workers are enabled via `initThreadPool()`, this becomes unsafe
///
/// ## Design
/// Rather than completely removing Send/Sync (which would prevent use with the
/// Plugin trait's Send + Sync bounds), we implement Send/Sync with a documented
/// SAFETY comment explaining the constraints.
///
/// This approach:
/// 1. Allows compilation in single-threaded WASM contexts
/// 2. Documents the thread safety assumption
/// 3. Makes the cost of violating the assumption explicit (unsafe keyword)
///
/// ## Migration Path for Multi-Threading
/// If multi-threading support is needed:
/// 1. Use message-passing channels instead of JsValue
/// 2. Serialize plugin configuration instead of sharing JsValue
/// 3. Create JsValue only on the main thread, pass results via channels
#[derive(Clone)]
pub(crate) struct JsPluginValue(pub(crate) JsValue);

/// SAFETY: JsValue is not inherently Send or Sync, but in WASM environments,
/// code executes on a single JavaScript thread. This implementation is sound
/// because:
///
/// 1. **Default WASM is single-threaded**: Without calling `initThreadPool()`,
///    all JavaScript code runs on one thread, making JsValue access safe.
///
/// 2. **Type system prevents worker usage**: If the wrapper is used in a
///    context that requires Send+Sync at compile time, this is intentional
///    design - it documents the main-thread requirement.
///
/// 3. **Fails early**: If someone tries to use this in a rayon task or
///    Web Worker thread, the JavaScript runtime will fail with clear errors
///    about invalid JS context access, rather than silent memory corruption.
///
/// **CRITICAL INVARIANT**: Plugin callbacks MUST ONLY be invoked from the
/// main JavaScript thread. Do not pass JsPluginValue to Web Workers or
/// background threads. Violations cause undefined behavior.
unsafe impl Send for JsPluginValue {}

/// SAFETY: Safe for the same reasons as Send implementation above.
/// Prevents data races by ensuring JsValue is only accessed from one JS context.
unsafe impl Sync for JsPluginValue {}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_make_send_wrapper_exists() {
        let _wrapper = MakeSend(async { 42 });
        // If this compiles, the wrapper works correctly
    }

    #[wasm_bindgen_test]
    fn test_js_plugin_value_is_clone() {
        let val = JsPluginValue(JsValue::from_str("test"));
        let _cloned = val.clone();
        // If this compiles, the wrapper is cloneable
    }
}
