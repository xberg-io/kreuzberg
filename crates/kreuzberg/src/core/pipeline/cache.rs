//! Processor caching to reduce lock contention.
//!
//! This module manages the caching of post-processors by processing stage,
//! eliminating repeated registry lock acquisitions.

use crate::Result;
use crate::plugins::{PostProcessor, ProcessingStage};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::sync::RwLock as StdRwLock;

/// Cached post-processors for each stage to reduce lock contention.
///
/// This cache is populated once during the first pipeline run and reused
/// for all subsequent extractions, eliminating 3 of 4 registry lock acquisitions
/// per extraction.
pub(super) struct ProcessorCache {
    pub(super) early: Arc<Vec<Arc<dyn PostProcessor>>>,
    pub(super) middle: Arc<Vec<Arc<dyn PostProcessor>>>,
    pub(super) late: Arc<Vec<Arc<dyn PostProcessor>>>,
}

impl ProcessorCache {
    /// Create a new processor cache by fetching from the registry.
    pub(super) fn new() -> Result<Self> {
        let processor_registry = crate::plugins::registry::get_post_processor_registry();
        let registry = processor_registry
            .read()
            .map_err(|e| crate::KreuzbergError::Other(format!("Post-processor registry lock poisoned: {}", e)))?;

        Ok(Self {
            early: Arc::new(registry.get_for_stage(ProcessingStage::Early)),
            middle: Arc::new(registry.get_for_stage(ProcessingStage::Middle)),
            late: Arc::new(registry.get_for_stage(ProcessingStage::Late)),
        })
    }

    /// Get processors for a specific stage from cache.
    #[allow(dead_code)]
    pub(super) fn get_for_stage(&self, stage: ProcessingStage) -> Arc<Vec<Arc<dyn PostProcessor>>> {
        match stage {
            ProcessingStage::Early => Arc::clone(&self.early),
            ProcessingStage::Middle => Arc::clone(&self.middle),
            ProcessingStage::Late => Arc::clone(&self.late),
        }
    }
}

/// Lazy processor cache - initialized on first use, then cached.
pub(super) static PROCESSOR_CACHE: Lazy<StdRwLock<Option<ProcessorCache>>> = Lazy::new(|| StdRwLock::new(None));

/// Clear the processor cache (primarily for testing when registry changes).
#[allow(dead_code)]
pub fn clear_processor_cache() -> Result<()> {
    let mut cache = PROCESSOR_CACHE
        .write()
        .map_err(|e| crate::KreuzbergError::Other(format!("Processor cache lock poisoned: {}", e)))?;
    *cache = None;
    Ok(())
}
