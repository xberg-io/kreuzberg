//! Subprocess-based adapter for language bindings
//!
//! This adapter provides a base for running extraction via subprocess.
//! It's used by Python, Node.js, and Ruby adapters to execute extraction
//! in separate processes while monitoring resource usage.

use crate::adapter::FrameworkAdapter;
use crate::monitoring::ResourceMonitor;
use crate::types::{BenchmarkResult, FrameworkCapabilities, OcrStatus, PerformanceMetrics};
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::process::Command;

/// Base adapter for subprocess-based extraction
///
/// This adapter spawns a subprocess to perform extraction and monitors
/// its resource usage. Subclasses implement the specific command construction
/// for each language binding.
pub struct SubprocessAdapter {
    name: String,
    command: PathBuf,
    args: Vec<String>,
    env: Vec<(String, String)>,
    supports_batch: bool,
    working_dir: Option<PathBuf>,
    supported_formats: Vec<String>,
}

impl SubprocessAdapter {
    /// Determine if a framework supports OCR based on its name
    ///
    /// Known frameworks with OCR support:
    /// - kreuzberg-* (all Kreuzberg bindings support OCR)
    /// - pymupdf (supports OCR via tesseract)
    ///
    /// Frameworks without OCR support:
    /// - pdfplumber
    /// - pypdf
    /// - Other basic PDF parsers
    fn framework_supports_ocr(framework_name: &str) -> bool {
        let name_lower = framework_name.to_lowercase();

        // Kreuzberg bindings all support OCR
        if name_lower.starts_with("kreuzberg-") || name_lower == "kreuzberg" {
            return true;
        }

        // PyMuPDF supports OCR via tesseract
        if name_lower.contains("pymupdf") {
            return true;
        }

        // Docling supports OCR via EasyOCR/Tesseract
        if name_lower.contains("docling") {
            return true;
        }

        // Unstructured supports OCR via Tesseract
        if name_lower.contains("unstructured") {
            return true;
        }

        // Tika supports OCR via Tika OCR parser
        if name_lower.contains("tika") {
            return true;
        }

        // MinerU supports OCR via PaddleOCR
        if name_lower.contains("mineru") {
            return true;
        }

        // Most other frameworks don't support OCR
        false
    }

    /// Create a new subprocess adapter
    ///
    /// # Arguments
    /// * `name` - Framework name (e.g., "kreuzberg-python")
    /// * `command` - Path to executable (e.g., "python3", "node")
    /// * `args` - Base arguments (e.g., ["-m", "kreuzberg"])
    /// * `env` - Environment variables
    /// * `supported_formats` - List of file extensions this framework can process (e.g., ["pdf", "docx"])
    pub fn new(
        name: impl Into<String>,
        command: impl Into<PathBuf>,
        args: Vec<String>,
        env: Vec<(String, String)>,
        supported_formats: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            args,
            env,
            supports_batch: false,
            working_dir: None,
            supported_formats,
        }
    }

    /// Create a new subprocess adapter with batch support
    ///
    /// This adapter will call `extract_batch()` with all files at once,
    /// allowing the subprocess to use its native batch API for parallel processing.
    ///
    /// # Arguments
    /// * `name` - Framework name (e.g., "kreuzberg-python-batch")
    /// * `command` - Path to executable (e.g., "python3", "node")
    /// * `args` - Base arguments (e.g., ["-m", "kreuzberg"])
    /// * `env` - Environment variables
    /// * `supported_formats` - List of file extensions this framework can process
    pub fn with_batch_support(
        name: impl Into<String>,
        command: impl Into<PathBuf>,
        args: Vec<String>,
        env: Vec<(String, String)>,
        supported_formats: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            args,
            env,
            supports_batch: true,
            working_dir: None,
            supported_formats,
        }
    }

    /// Set the working directory for subprocess execution
    ///
    /// # Arguments
    /// * `dir` - Directory path to change to before running the command
    pub fn set_working_dir(&mut self, dir: PathBuf) {
        self.working_dir = Some(dir);
    }

    /// Execute the extraction subprocess
    async fn execute_subprocess(&self, file_path: &Path, timeout: Duration) -> Result<(String, String, Duration)> {
        let start = Instant::now();

        let absolute_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            std::env::current_dir().map_err(Error::Io)?.join(file_path)
        };

        let mut cmd = Command::new(&self.command);
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(&self.args);
        cmd.arg(&*absolute_path.to_string_lossy());

        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn().map_err(|e| {
            Error::Benchmark(format!(
                "Failed to spawn subprocess '{}' with args {:?}: {}",
                self.command.display(),
                self.args,
                e
            ))
        })?;

        let output = match tokio::time::timeout(timeout, child.wait_with_output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Err(Error::Benchmark(format!("Failed to wait for subprocess: {}", e)));
            }
            Err(_) => {
                return Err(Error::Timeout(format!("Subprocess exceeded {:?}", timeout)));
            }
        };

        let duration = start.elapsed();

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            let mut error_msg = format!("Subprocess failed with exit code {:?}", output.status.code());
            if !stderr.is_empty() {
                error_msg.push_str(&format!("\nstderr: {}", stderr));
            }
            if !stdout.is_empty() && stdout.len() < 500 {
                error_msg.push_str(&format!("\nstdout: {}", stdout));
            }
            return Err(Error::Benchmark(error_msg));
        }

        Ok((stdout, stderr, duration))
    }

    /// Execute batch extraction subprocess with multiple files
    async fn execute_subprocess_batch(
        &self,
        file_paths: &[&Path],
        timeout: Duration,
    ) -> Result<(String, String, Duration)> {
        let start = Instant::now();

        let mut cmd = Command::new(&self.command);
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd.args(&self.args);

        let cwd = std::env::current_dir().map_err(Error::Io)?;
        for path in file_paths {
            let absolute_path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                cwd.join(path)
            };
            cmd.arg(&*absolute_path.to_string_lossy());
        }

        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd
            .spawn()
            .map_err(|e| Error::Benchmark(format!("Failed to spawn batch subprocess: {}", e)))?;

        let output = match tokio::time::timeout(timeout, child.wait_with_output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Err(Error::Benchmark(format!("Failed to wait for batch subprocess: {}", e)));
            }
            Err(_) => {
                return Err(Error::Timeout(format!("Batch subprocess exceeded {:?}", timeout)));
            }
        };

        let duration = start.elapsed();

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(Error::Benchmark(format!(
                "Batch subprocess failed with exit code {:?}\nstderr: {}",
                output.status.code(),
                stderr
            )));
        }

        Ok((stdout, stderr, duration))
    }

    /// Parse extraction result from subprocess output
    ///
    /// Expected subprocess output format:
    /// ```json
    /// {
    ///   "content": "extracted text...",          // REQUIRED
    ///   "_ocr_used": true|false,                 // optional
    ///   "_extraction_time_ms": 123.45            // optional
    /// }
    /// ```
    fn parse_output(&self, stdout: &str) -> Result<serde_json::Value> {
        let parsed: serde_json::Value = serde_json::from_str(stdout)
            .map_err(|e| Error::Benchmark(format!("Failed to parse subprocess output as JSON: {}", e)))?;

        // Validate that content field exists and is a string
        if !parsed.is_object() {
            return Err(Error::Benchmark(
                "Subprocess output must be a JSON object with 'content' field".to_string(),
            ));
        }

        if !parsed.get("content").is_some_and(|v| v.is_string()) {
            return Err(Error::Benchmark(
                "Subprocess output missing required 'content' field (must be a string)".to_string(),
            ));
        }

        Ok(parsed)
    }
}

#[async_trait]
impl FrameworkAdapter for SubprocessAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    fn supports_format(&self, file_type: &str) -> bool {
        let file_type_lower = file_type.to_lowercase();
        self.supported_formats
            .iter()
            .any(|fmt| fmt.to_lowercase() == file_type_lower)
    }

    async fn extract(&self, file_path: &Path, timeout: Duration) -> Result<BenchmarkResult> {
        let file_size = std::fs::metadata(file_path).map_err(Error::Io)?.len();

        let start_time = std::time::Instant::now();
        let monitor = ResourceMonitor::new();
        let sampling_ms = crate::monitoring::adaptive_sampling_interval_ms(file_size);
        monitor.start(Duration::from_millis(sampling_ms)).await;

        let (stdout, _stderr, duration) = match self.execute_subprocess(file_path, timeout).await {
            Ok(result) => result,
            Err(e) => {
                let samples = monitor.stop().await;
                let snapshots = monitor.get_snapshots().await;
                let resource_stats = ResourceMonitor::calculate_stats(&samples, &snapshots);
                let actual_duration = start_time.elapsed();

                let throughput = if actual_duration.as_secs_f64() > 0.0 {
                    file_size as f64 / actual_duration.as_secs_f64()
                } else {
                    0.0
                };

                let framework_capabilities = FrameworkCapabilities {
                    ocr_support: Self::framework_supports_ocr(&self.name),
                    batch_support: self.supports_batch,
                    ..Default::default()
                };

                return Ok(BenchmarkResult {
                    framework: self.name.clone(),
                    file_path: file_path.to_path_buf(),
                    file_size,
                    success: false,
                    error_message: Some(e.to_string()),
                    duration: actual_duration,
                    extraction_duration: None,
                    subprocess_overhead: None,
                    metrics: PerformanceMetrics {
                        peak_memory_bytes: resource_stats.peak_memory_bytes,
                        avg_cpu_percent: resource_stats.avg_cpu_percent,
                        throughput_bytes_per_sec: throughput,
                        p50_memory_bytes: resource_stats.p50_memory_bytes,
                        p95_memory_bytes: resource_stats.p95_memory_bytes,
                        p99_memory_bytes: resource_stats.p99_memory_bytes,
                    },
                    quality: None,
                    iterations: vec![],
                    statistics: None,
                    cold_start_duration: None,
                    file_extension: file_path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("unknown")
                        .to_lowercase(),
                    framework_capabilities,
                    pdf_metadata: None,
                    ocr_status: OcrStatus::Unknown,
                    extracted_text: None,
                });
            }
        };

        let samples = monitor.stop().await;
        let snapshots = monitor.get_snapshots().await;
        let resource_stats = ResourceMonitor::calculate_stats(&samples, &snapshots);

        let parsed = match self.parse_output(&stdout) {
            Ok(value) => value,
            Err(e) => {
                let throughput = if duration.as_secs_f64() > 0.0 {
                    file_size as f64 / duration.as_secs_f64()
                } else {
                    0.0
                };

                let framework_capabilities = FrameworkCapabilities {
                    ocr_support: Self::framework_supports_ocr(&self.name),
                    batch_support: self.supports_batch,
                    ..Default::default()
                };

                return Ok(BenchmarkResult {
                    framework: self.name.clone(),
                    file_path: file_path.to_path_buf(),
                    file_size,
                    success: false,
                    error_message: Some(e.to_string()),
                    duration,
                    extraction_duration: None,
                    subprocess_overhead: None,
                    metrics: PerformanceMetrics {
                        peak_memory_bytes: resource_stats.peak_memory_bytes,
                        avg_cpu_percent: resource_stats.avg_cpu_percent,
                        throughput_bytes_per_sec: throughput,
                        p50_memory_bytes: resource_stats.p50_memory_bytes,
                        p95_memory_bytes: resource_stats.p95_memory_bytes,
                        p99_memory_bytes: resource_stats.p99_memory_bytes,
                    },
                    quality: None,
                    iterations: vec![],
                    statistics: None,
                    cold_start_duration: None,
                    file_extension: file_path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("unknown")
                        .to_lowercase(),
                    framework_capabilities,
                    pdf_metadata: None,
                    ocr_status: OcrStatus::Unknown,
                    extracted_text: None,
                });
            }
        };

        let extraction_duration = parsed
            .get("_extraction_time_ms")
            .and_then(|v| v.as_f64())
            .map(|ms| Duration::from_secs_f64(ms / 1000.0));

        // Capture extracted text for quality assessment
        let extracted_text = parsed.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());

        let subprocess_overhead = extraction_duration.map(|ext| duration.saturating_sub(ext));

        let throughput = if duration.as_secs_f64() > 0.0 {
            file_size as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        let metrics = PerformanceMetrics {
            peak_memory_bytes: resource_stats.peak_memory_bytes,
            avg_cpu_percent: resource_stats.avg_cpu_percent,
            throughput_bytes_per_sec: throughput,
            p50_memory_bytes: resource_stats.p50_memory_bytes,
            p95_memory_bytes: resource_stats.p95_memory_bytes,
            p99_memory_bytes: resource_stats.p99_memory_bytes,
        };

        // Check if subprocess reported OCR usage
        let ocr_status = parsed
            .get("_ocr_used")
            .and_then(|v| v.as_bool())
            .map(|used| if used { OcrStatus::Used } else { OcrStatus::NotUsed })
            .unwrap_or(OcrStatus::Unknown);

        // Build framework capabilities
        let framework_capabilities = FrameworkCapabilities {
            ocr_support: Self::framework_supports_ocr(&self.name),
            batch_support: self.supports_batch,
            ..Default::default()
        };

        // Build PDF metadata if this is a PDF file
        let pdf_metadata = if file_path.extension().and_then(|e| e.to_str()) == Some("pdf") {
            Some(crate::types::PdfMetadata {
                has_text_layer: false, // Unknown from subprocess
                detection_method: "unknown".to_string(),
                page_count: None,
                ocr_enabled: ocr_status == OcrStatus::Used,
                text_quality_score: None,
            })
        } else {
            None
        };

        Ok(BenchmarkResult {
            framework: self.name.clone(),
            file_path: file_path.to_path_buf(),
            file_size,
            success: true,
            error_message: None,
            duration,
            extraction_duration,
            subprocess_overhead,
            metrics,
            quality: None,
            iterations: vec![],
            statistics: None,
            cold_start_duration: None,
            file_extension: file_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_lowercase(),
            framework_capabilities,
            pdf_metadata,
            ocr_status,
            extracted_text,
        })
    }

    fn version(&self) -> String {
        "unknown".to_string()
    }

    fn supports_batch(&self) -> bool {
        self.supports_batch
    }

    async fn extract_batch(&self, file_paths: &[&Path], timeout: Duration) -> Result<Vec<BenchmarkResult>> {
        // Early return if file_paths is empty
        if file_paths.is_empty() {
            return Ok(Vec::new());
        }

        if !self.supports_batch {
            let mut results = Vec::new();
            for path in file_paths {
                results.push(self.extract(path, timeout).await?);
            }
            return Ok(results);
        }

        let total_file_size: u64 = file_paths
            .iter()
            .filter_map(|p| std::fs::metadata(p).ok().map(|m| m.len()))
            .sum();

        let start_time = std::time::Instant::now();
        let monitor = ResourceMonitor::new();
        let sampling_ms = crate::monitoring::adaptive_sampling_interval_ms(total_file_size);
        monitor.start(Duration::from_millis(sampling_ms)).await;

        let (stdout, _stderr, duration) = match self.execute_subprocess_batch(file_paths, timeout).await {
            Ok(result) => result,
            Err(e) => {
                let samples = monitor.stop().await;
                let snapshots = monitor.get_snapshots().await;
                let resource_stats = ResourceMonitor::calculate_stats(&samples, &snapshots);
                let actual_duration = start_time.elapsed();

                // Create one failure result per file instead of a single aggregated failure
                // Use the actual elapsed time divided by number of files
                let num_files = file_paths.len() as f64;
                // Amortized per-file duration: total batch wall time divided by file count.
                // For concurrent batch processing, this represents average cost, not individual file duration.
                let avg_duration_per_file = Duration::from_secs_f64(actual_duration.as_secs_f64() / num_files.max(1.0));

                let framework_capabilities = FrameworkCapabilities {
                    ocr_support: Self::framework_supports_ocr(&self.name),
                    batch_support: self.supports_batch,
                    ..Default::default()
                };

                let failure_results: Vec<BenchmarkResult> = file_paths
                    .iter()
                    .map(|file_path| {
                        let file_size = std::fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);
                        let file_extension = file_path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or("")
                            .to_string();

                        let throughput = if avg_duration_per_file.as_secs_f64() > 0.0 {
                            file_size as f64 / avg_duration_per_file.as_secs_f64()
                        } else {
                            0.0
                        };

                        BenchmarkResult {
                            framework: self.name.clone(),
                            file_path: file_path.to_path_buf(),
                            file_size,
                            success: false,
                            error_message: Some(e.to_string()),
                            duration: avg_duration_per_file,
                            extraction_duration: None,
                            subprocess_overhead: None,
                            metrics: PerformanceMetrics {
                                peak_memory_bytes: resource_stats.peak_memory_bytes,
                                avg_cpu_percent: resource_stats.avg_cpu_percent,
                                throughput_bytes_per_sec: throughput,
                                p50_memory_bytes: resource_stats.p50_memory_bytes,
                                p95_memory_bytes: resource_stats.p95_memory_bytes,
                                p99_memory_bytes: resource_stats.p99_memory_bytes,
                            },
                            quality: None,
                            iterations: vec![],
                            statistics: None,
                            cold_start_duration: None,
                            file_extension,
                            framework_capabilities: framework_capabilities.clone(),
                            pdf_metadata: None,
                            ocr_status: OcrStatus::Unknown,
                            extracted_text: None,
                        }
                    })
                    .collect();

                return Ok(failure_results);
            }
        };

        let samples = monitor.stop().await;
        let snapshots = monitor.get_snapshots().await;
        let resource_stats = ResourceMonitor::calculate_stats(&samples, &snapshots);

        // Parse batch output to extract per-file OCR status and extraction times
        // Try to parse as JSON array; fall back to defaults if parsing fails
        let parsed_batch: Option<Vec<serde_json::Value>> = serde_json::from_str(&stdout).ok();

        let batch_ocr_statuses: Vec<OcrStatus> = parsed_batch
            .as_ref()
            .map(|results| {
                results
                    .iter()
                    .map(|item| {
                        item.get("_ocr_used")
                            .and_then(|v| v.as_bool())
                            .map(|used| if used { OcrStatus::Used } else { OcrStatus::NotUsed })
                            .unwrap_or(OcrStatus::Unknown)
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec![OcrStatus::Unknown; file_paths.len()]);

        // Extract per-file extraction times from batch JSON results
        let batch_extraction_times: Vec<Option<Duration>> = parsed_batch
            .as_ref()
            .map(|results| {
                results
                    .iter()
                    .map(|item| {
                        item.get("_extraction_time_ms")
                            .and_then(|v| v.as_f64())
                            .map(|ms| Duration::from_secs_f64(ms / 1000.0))
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec![None; file_paths.len()]);

        // Create one result per file instead of a single aggregated result
        // Since batch processing doesn't give us per-file timing, we use average duration
        let num_files = file_paths.len() as f64;
        let avg_duration_per_file = Duration::from_secs_f64(duration.as_secs_f64() / num_files.max(1.0));

        let framework_capabilities = FrameworkCapabilities {
            ocr_support: Self::framework_supports_ocr(&self.name),
            batch_support: self.supports_batch,
            ..Default::default()
        };

        let results: Vec<BenchmarkResult> = file_paths
            .iter()
            .enumerate()
            .map(|(idx, file_path)| {
                let file_size = std::fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);

                let file_throughput = if avg_duration_per_file > Duration::from_secs(0) {
                    file_size as f64 / avg_duration_per_file.as_secs_f64()
                } else {
                    0.0
                };

                let file_extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();

                // Use per-file OCR status if available, otherwise Unknown
                let ocr_status = batch_ocr_statuses.get(idx).copied().unwrap_or(OcrStatus::Unknown);

                // Use per-file extraction time if available from batch JSON
                let extraction_duration = batch_extraction_times.get(idx).copied().flatten();
                let subprocess_overhead = extraction_duration.map(|ext| avg_duration_per_file.saturating_sub(ext));

                BenchmarkResult {
                    framework: self.name.clone(),
                    file_path: file_path.to_path_buf(),
                    file_size,
                    success: true,
                    error_message: None,
                    duration: avg_duration_per_file,
                    extraction_duration,
                    subprocess_overhead,
                    metrics: PerformanceMetrics {
                        peak_memory_bytes: resource_stats.peak_memory_bytes,
                        avg_cpu_percent: resource_stats.avg_cpu_percent,
                        throughput_bytes_per_sec: file_throughput,
                        p50_memory_bytes: resource_stats.p50_memory_bytes,
                        p95_memory_bytes: resource_stats.p95_memory_bytes,
                        p99_memory_bytes: resource_stats.p99_memory_bytes,
                    },
                    quality: None,
                    iterations: vec![],
                    statistics: None,
                    cold_start_duration: None,
                    file_extension,
                    framework_capabilities: framework_capabilities.clone(),
                    pdf_metadata: None,
                    ocr_status,
                    extracted_text: None,
                }
            })
            .collect();

        Ok(results)
    }

    async fn setup(&self) -> Result<()> {
        which::which(&self.command)
            .map_err(|e| Error::Benchmark(format!("Command '{}' not found: {}", self.command.display(), e)))?;

        Ok(())
    }

    async fn teardown(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            peak_memory_bytes: 0,
            avg_cpu_percent: 0.0,
            throughput_bytes_per_sec: 0.0,
            p50_memory_bytes: 0,
            p95_memory_bytes: 0,
            p99_memory_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subprocess_adapter_creation() {
        let adapter = SubprocessAdapter::new(
            "test-adapter",
            "echo",
            vec!["test".to_string()],
            vec![],
            vec!["pdf".to_string(), "docx".to_string()],
        );
        assert_eq!(adapter.name(), "test-adapter");
    }

    #[test]
    fn test_supports_format() {
        let adapter = SubprocessAdapter::new(
            "test",
            "echo",
            vec![],
            vec![],
            vec!["pdf".to_string(), "docx".to_string()],
        );
        assert!(adapter.supports_format("pdf"));
        assert!(adapter.supports_format("docx"));
        assert!(!adapter.supports_format("unknown"));
    }
}
