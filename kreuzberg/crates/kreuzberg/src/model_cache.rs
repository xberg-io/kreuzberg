//! Generic global model cache for ONNX-based models.
//!
//! Provides a take/return pattern that avoids holding a mutex during
//! inference while still reusing expensive-to-create model sessions
//! across extractions.
//!
//! # Usage
//!
//! ```ignore
//! static MY_CACHE: ModelCache<MyModel> = ModelCache::new();
//!
//! // Take from cache or create new
//! let model = MY_CACHE.take_or_create(|| MyModel::new())?;
//!
//! // Use the model...
//! model.infer(...);
//!
//! // Return to cache for reuse
//! MY_CACHE.put(model);
//! ```

use std::sync::Mutex;

/// A global cache for a single model instance.
///
/// Uses a take/return pattern: the caller takes ownership of the model,
/// uses it without holding any lock, then returns it when done. This
/// avoids contention during inference while still amortizing session
/// creation cost across documents.
pub struct ModelCache<T: Send> {
    slot: Mutex<Option<T>>,
}

impl<T: Send> Default for ModelCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send> ModelCache<T> {
    /// Create an empty cache.
    pub const fn new() -> Self {
        Self { slot: Mutex::new(None) }
    }

    /// Take the cached model, or create a new one using `create_fn`.
    ///
    /// The caller owns the returned model and should call [`put`] when
    /// done to return it for reuse.
    pub fn take_or_create<E>(&self, create_fn: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
        if let Ok(mut guard) = self.slot.lock()
            && let Some(model) = guard.take()
        {
            tracing::debug!("Reusing cached model");
            return Ok(model);
        }

        tracing::debug!("Creating new model (cache miss)");
        create_fn()
    }

    /// Return a model to the cache for reuse.
    ///
    /// If the cache already holds a model (e.g. from a concurrent caller),
    /// the returned model is silently dropped.
    pub fn put(&self, model: T) {
        if let Ok(mut guard) = self.slot.lock() {
            *guard = Some(model);
        }
    }

    /// Take the cached model if one exists, without creating a new one.
    pub fn take(&self) -> Option<T> {
        self.slot.lock().ok().and_then(|mut guard| guard.take())
    }
}
