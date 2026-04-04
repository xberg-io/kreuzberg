//! Worker thread pool for concurrent document extraction operations.
//!
//! This module provides a simplified thread pool that uses tokio's spawn_blocking
//! for CPU-bound document extraction tasks.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Worker thread pool for concurrent extraction operations
#[derive(Clone)]
pub struct WorkerPool {
    /// Maximum number of concurrent operations
    size: usize,
    /// Number of currently active workers
    active_workers: Arc<AtomicUsize>,
}

impl WorkerPool {
    /// Create a new worker pool with the specified size
    pub fn new(size: usize) -> napi::Result<Self> {
        if size == 0 {
            return Err(napi::Error::new(
                napi::Status::InvalidArg,
                "Worker pool size must be greater than 0",
            ));
        }

        Ok(Self {
            size,
            active_workers: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// Get pool size (maximum concurrent operations)
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get number of active workers
    pub fn active_workers(&self) -> usize {
        self.active_workers.load(Ordering::Relaxed)
    }

    /// Check if we can accept more work
    pub fn can_accept_work(&self) -> bool {
        self.active_workers.load(Ordering::Relaxed) < self.size
    }

    /// Increment active worker count
    pub fn increment_active(&self) {
        self.active_workers.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active worker count
    pub fn decrement_active(&self) {
        self.active_workers.fetch_sub(1, Ordering::Relaxed);
    }

    /// Wait for all active workers to complete
    pub async fn wait_for_completion(&self) {
        while self.active_workers.load(Ordering::Relaxed) > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }
}
