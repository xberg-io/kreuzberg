//! Deferred result type for async PHP extraction.
//!
//! Provides a pollable result object that PHP code can use to check
//! if an async operation has completed, and retrieve the result when ready.

use ext_php_rs::prelude::*;
use parking_lot::{Condvar, Mutex};
use std::sync::Arc;

use crate::types::ExtractionResult;

/// Internal state for a deferred extraction result.
///
/// Wraps either a single result or a batch of results (for batch operations).
/// Results are stored behind `Arc` so that:
/// - The large `ExtractionResult` struct is heap-allocated, keeping enum variant sizes equal.
/// - Multiple retrieve calls (e.g. repeated `getResult()` or `wait()`) share the same
///   allocation; `Arc::unwrap_or_clone` moves the value on the last retrieval to avoid copying.
pub(crate) enum DeferredInner {
    Single(Option<Result<Arc<kreuzberg::ExtractionResult>, String>>),
    Batch(Option<Result<Vec<Arc<kreuzberg::ExtractionResult>>, String>>),
}

/// Shared state pairing the result slot with a condition variable.
///
/// The `Condvar` is notified (via `notify_all`) by the background worker after
/// it writes the result into `inner`, allowing waiting threads to wake up
/// immediately instead of busy-polling.
pub(crate) struct DeferredShared {
    pub(crate) inner: Mutex<DeferredInner>,
    pub(crate) ready: Condvar,
}

impl DeferredShared {
    pub(crate) fn new_single() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(DeferredInner::Single(None)),
            ready: Condvar::new(),
        })
    }

    pub(crate) fn new_batch() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(DeferredInner::Batch(None)),
            ready: Condvar::new(),
        })
    }
}

/// A deferred result from an async extraction operation.
///
/// This object is returned immediately from async extraction functions.
/// The actual extraction runs on a background Tokio worker thread.
///
/// # PHP Usage
///
/// ```php
/// $deferred = kreuzberg_extract_file_async('document.pdf');
///
/// // Non-blocking check
/// if ($deferred->isReady()) {
///     $result = $deferred->getResult();
/// }
///
/// // Blocking wait
/// $result = $deferred->getResult();
///
/// // Wait with timeout (milliseconds)
/// $result = $deferred->wait(5000);
/// ```
#[php_class]
#[php(name = "Kreuzberg\\Types\\DeferredResult")]
pub struct DeferredResult {
    shared: Arc<DeferredShared>,
    extract_tables: bool,
}

#[php_impl]
impl DeferredResult {
    /// Check if the result is ready (non-blocking).
    ///
    /// # Returns
    ///
    /// `true` if the result is available, `false` if still processing.
    #[php(name = "isReady")]
    pub fn is_ready(&self) -> bool {
        let guard = self.shared.inner.lock();
        match &*guard {
            DeferredInner::Single(slot) => slot.is_some(),
            DeferredInner::Batch(slot) => slot.is_some(),
        }
    }

    /// Try to get the result without blocking.
    ///
    /// # Returns
    ///
    /// The `ExtractionResult` if ready, or `null` if still processing.
    ///
    /// # Throws
    ///
    /// Exception if the extraction failed.
    #[php(name = "tryGetResult")]
    pub fn try_get_result(&self) -> PhpResult<Option<ExtractionResult>> {
        let guard = self.shared.inner.lock();
        match &*guard {
            DeferredInner::Single(Some(result)) => match result {
                Ok(r) => Ok(Some(ExtractionResult::from_rust_with_config(
                    // Clone only the Arc pointer here; Arc::unwrap_or_clone moves the value
                    // out if this is the last reference, otherwise clones the inner data.
                    Arc::unwrap_or_clone(Arc::clone(r)),
                    self.extract_tables,
                )?)),
                Err(e) => Err(PhpException::default(e.clone())),
            },
            DeferredInner::Single(None) => Ok(None),
            DeferredInner::Batch(_) => Err(PhpException::default(
                "Use tryGetResults() for batch operations".to_string(),
            )),
        }
    }

    /// Get the result, blocking until it's ready.
    ///
    /// # Returns
    ///
    /// The `ExtractionResult` from the async extraction.
    ///
    /// # Throws
    ///
    /// Exception if the extraction failed or timed out.
    #[php(name = "getResult")]
    pub fn get_result(&self) -> PhpResult<ExtractionResult> {
        let mut guard = self.shared.inner.lock();
        loop {
            match &*guard {
                DeferredInner::Single(Some(result)) => {
                    return match result {
                        Ok(r) => ExtractionResult::from_rust_with_config(
                            Arc::unwrap_or_clone(Arc::clone(r)),
                            self.extract_tables,
                        ),
                        Err(e) => Err(PhpException::default(e.clone())),
                    };
                }
                DeferredInner::Batch(_) => {
                    return Err(PhpException::default(
                        "Use getResults() for batch operations".to_string(),
                    ));
                }
                _ => {}
            }
            self.shared.ready.wait(&mut guard);
        }
    }

    /// Get the batch results, blocking until ready.
    ///
    /// # Returns
    ///
    /// Array of `ExtractionResult` from the async batch extraction.
    ///
    /// # Throws
    ///
    /// Exception if the extraction failed.
    #[php(name = "getResults")]
    pub fn get_results(&self) -> PhpResult<Vec<ExtractionResult>> {
        let mut guard = self.shared.inner.lock();
        loop {
            match &*guard {
                DeferredInner::Batch(Some(result)) => {
                    return match result {
                        Ok(results) => results
                            .iter()
                            .map(|r| {
                                ExtractionResult::from_rust_with_config(
                                    Arc::unwrap_or_clone(Arc::clone(r)),
                                    self.extract_tables,
                                )
                            })
                            .collect(),
                        Err(e) => Err(PhpException::default(e.clone())),
                    };
                }
                DeferredInner::Single(_) => {
                    return Err(PhpException::default(
                        "Use getResult() for single operations".to_string(),
                    ));
                }
                _ => {}
            }
            self.shared.ready.wait(&mut guard);
        }
    }

    /// Wait for the result with a timeout (in milliseconds).
    ///
    /// # Parameters
    ///
    /// - `timeout_ms` (int): Maximum time to wait in milliseconds
    ///
    /// # Returns
    ///
    /// The `ExtractionResult` if ready within timeout, or `null` if timed out.
    ///
    /// # Throws
    ///
    /// Exception if the extraction failed.
    pub fn wait(&self, timeout_ms: i64) -> PhpResult<Option<ExtractionResult>> {
        let mut remaining = std::time::Duration::from_millis(timeout_ms as u64);
        let mut guard = self.shared.inner.lock();
        loop {
            match &*guard {
                DeferredInner::Single(Some(result)) => {
                    return match result {
                        Ok(r) => Ok(Some(ExtractionResult::from_rust_with_config(
                            Arc::unwrap_or_clone(Arc::clone(r)),
                            self.extract_tables,
                        )?)),
                        Err(e) => Err(PhpException::default(e.clone())),
                    };
                }
                DeferredInner::Batch(_) => {
                    return Err(PhpException::default(
                        "Use waitBatch() for batch operations".to_string(),
                    ));
                }
                _ => {}
            }

            if remaining.is_zero() {
                return Ok(None);
            }

            let start = std::time::Instant::now();
            let wait_result = self.shared.ready.wait_for(&mut guard, remaining);
            let elapsed = start.elapsed();
            // Saturating subtraction avoids underflow if elapsed > remaining.
            remaining = remaining.saturating_sub(elapsed);
            if wait_result.timed_out() {
                return Ok(None);
            }
        }
    }

    /// Wait for batch results with a timeout (in milliseconds).
    ///
    /// # Parameters
    ///
    /// - `timeout_ms` (int): Maximum time to wait in milliseconds
    ///
    /// # Returns
    ///
    /// Array of `ExtractionResult` if ready within timeout, or `null` if timed out.
    ///
    /// # Throws
    ///
    /// Exception if the extraction failed.
    #[php(name = "waitBatch")]
    pub fn wait_batch(&self, timeout_ms: i64) -> PhpResult<Option<Vec<ExtractionResult>>> {
        let mut remaining = std::time::Duration::from_millis(timeout_ms as u64);
        let mut guard = self.shared.inner.lock();
        loop {
            match &*guard {
                DeferredInner::Batch(Some(result)) => {
                    return match result {
                        Ok(results) => Ok(Some(
                            results
                                .iter()
                                .map(|r| {
                                    ExtractionResult::from_rust_with_config(
                                        Arc::unwrap_or_clone(Arc::clone(r)),
                                        self.extract_tables,
                                    )
                                })
                                .collect::<PhpResult<Vec<_>>>()?,
                        )),
                        Err(e) => Err(PhpException::default(e.clone())),
                    };
                }
                DeferredInner::Single(_) => {
                    return Err(PhpException::default("Use wait() for single operations".to_string()));
                }
                _ => {}
            }

            if remaining.is_zero() {
                return Ok(None);
            }

            let start = std::time::Instant::now();
            let wait_result = self.shared.ready.wait_for(&mut guard, remaining);
            let elapsed = start.elapsed();
            remaining = remaining.saturating_sub(elapsed);
            if wait_result.timed_out() {
                return Ok(None);
            }
        }
    }
}

impl DeferredResult {
    /// Create a new single-result deferred with a shared slot.
    pub(crate) fn new_single(shared: Arc<DeferredShared>, extract_tables: bool) -> Self {
        Self { shared, extract_tables }
    }

    /// Create a new batch-result deferred with a shared slot.
    pub(crate) fn new_batch(shared: Arc<DeferredShared>, extract_tables: bool) -> Self {
        Self { shared, extract_tables }
    }
}
