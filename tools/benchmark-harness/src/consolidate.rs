//! Loading benchmark results from disk for consolidation
//!
//! This module provides `load_run_results` which recursively loads benchmark
//! result JSON files from a directory tree, tagging them with batch mode info
//! inferred from directory names.

use crate::types::BenchmarkResult;
use crate::{Error, Result};
use std::fs;
use std::path::Path;

/// Load benchmark results from results.json files in a directory
///
/// Recursively walks the given directory, loading any `results.json` files found.
/// For directories whose name ends with `-batch`, the framework name in each result
/// is suffixed with `-batch` so that the aggregation layer can distinguish single
/// vs batch results.
pub fn load_run_results(dir: &Path) -> Result<Vec<BenchmarkResult>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir).map_err(Error::Io)? {
        let entry = entry.map_err(Error::Io)?;
        let path = entry.path();

        if path.is_file() && path.file_name().is_some_and(|n| n == "results.json") {
            eprintln!("Loading results from {}", path.display());
            let json_content = fs::read_to_string(&path).map_err(Error::Io)?;
            let mut run_results: Vec<BenchmarkResult> = serde_json::from_str(&json_content)
                .map_err(|e| Error::Benchmark(format!("Failed to parse {}: {}", path.display(), e)))?;

            // Infer benchmark mode from the parent directory name.
            // The runner outputs to `benchmark-results/{FRAMEWORK}-{MODE}/results.json`
            // where MODE is "batch" or "single-file". The framework field inside
            // results.json does NOT include the mode, so we tag it here to allow
            // the aggregation to distinguish single vs batch results.
            let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let is_batch = dir_name.ends_with("-batch");

            if is_batch {
                for result in &mut run_results {
                    if !result.framework.ends_with("-batch") {
                        result.framework = format!("{}-batch", result.framework);
                    }
                }
            }

            // Validate loaded results
            for result in &run_results {
                crate::output::validate_result(result)
                    .map_err(|e| Error::Benchmark(format!("Invalid result in {}: {}", path.display(), e)))?;
            }

            results.extend(run_results);
        } else if path.is_dir() {
            match load_run_results(&path) {
                Ok(mut run_results) => results.append(&mut run_results),
                Err(e) => eprintln!("Warning: Failed to load results from {}: {}", path.display(), e),
            }
        }
    }
    Ok(results)
}
