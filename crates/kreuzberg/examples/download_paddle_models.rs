//! Downloads PaddleOCR ONNX models for offline use.
//!
//! This example demonstrates how to use the PaddleOCR model manager to download
//! and cache ONNX models locally. This is useful for offline applications or
//! pre-warming the model cache before starting document extraction.
//!
//! # Usage
//!
//! ```sh
//! # Download models to default cache directory
//! cargo run --example download_paddle_models --features paddle-ocr
//!
//! # Download models to custom cache directory
//! cargo run --example download_paddle_models --features paddle-ocr -- \
//!   --cache-dir /path/to/models
//!
//! # Display cache statistics
//! cargo run --example download_paddle_models --features paddle-ocr -- \
//!   --show-stats
//!
//! # Clear cache before downloading
//! cargo run --example download_paddle_models --features paddle-ocr -- \
//!   --clear-cache
//! ```
//!
//! # Language Support
//!
//! This implementation supports 11 script families (all PP-OCRv5) covering 80+ languages:
//! - **English**: English-optimized recognition models
//! - **Chinese**: Simplified and Traditional Chinese, Japanese
//! - **Latin**: European languages using Latin script
//! - **Korean**: Hangul script
//! - **Eslav**: Cyrillic-based languages (Russian, Ukrainian, etc.)
//! - **Thai**: Thai script
//! - **Greek**: Greek script
//! - **Arabic**: Arabic, Persian, Urdu
//! - **Devanagari**: Hindi, Marathi, Sanskrit, Nepali
//! - **Tamil**: Tamil script
//! - **Telugu**: Telugu script
//!
//! Models are downloaded on-demand per script family. The English recognition model
//! and dictionary are downloaded by default. Other language families are automatically
//! downloaded when needed during document processing.
//!
//! # Examples
//!
//! ## Download and Display Models
//!
//! ```no_run
//! use kreuzberg::ModelManager;
//! use std::path::PathBuf;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create model manager with default cache directory
//!     let cache_dir = PathBuf::from("/tmp/paddle_models");
//!     let manager = ModelManager::new(cache_dir);
//!
//!     // Ensure models exist (download if needed)
//!     let models = manager.ensure_models_exist()?;
//!     println!("Detection model:       {:?}", models.det_model);
//!     println!("Classification model: {:?}", models.cls_model);
//!     println!("Recognition model:    {:?}", models.rec_model);
//!     println!("Dictionary file:      {:?}", models.dict_file);
//!
//!     // Show cache statistics
//!     let stats = manager.cache_stats()?;
//!     println!("\nCache Statistics:");
//!     println!("  Total size: {:.2} MB", stats.total_size_bytes as f64 / 1_000_000.0);
//!     println!("  Model count: {}", stats.model_count);
//!     println!("  Cache directory: {:?}", stats.cache_dir);
//!
//!     Ok(())
//! }
//! ```

use std::env;
#[cfg(feature = "paddle-ocr")]
use std::fs;
use std::path::PathBuf;

#[cfg(feature = "paddle-ocr")]
use kreuzberg::ModelManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging for tracing output
    #[cfg(feature = "paddle-ocr")]
    {
        // Optional: Initialize tracing for debug output
        // tracing_subscriber::fmt().init();
    }

    #[cfg(feature = "paddle-ocr")]
    let args: Vec<String> = env::args().collect();
    #[cfg(not(feature = "paddle-ocr"))]
    let _args: Vec<String> = env::args().collect();

    // Parse command-line arguments
    #[cfg(feature = "paddle-ocr")]
    let mut cache_dir = get_default_cache_dir();
    #[cfg(feature = "paddle-ocr")]
    let mut show_stats = false;
    #[cfg(feature = "paddle-ocr")]
    let mut clear_cache_flag = false;

    #[cfg(feature = "paddle-ocr")]
    {
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--cache-dir" => {
                    if i + 1 < args.len() {
                        cache_dir = PathBuf::from(&args[i + 1]);
                        i += 2;
                    } else {
                        eprintln!("Error: --cache-dir requires an argument");
                        print_usage(&args[0]);
                        std::process::exit(1);
                    }
                }
                "--show-stats" => {
                    show_stats = true;
                    i += 1;
                }
                "--clear-cache" => {
                    clear_cache_flag = true;
                    i += 1;
                }
                "--help" | "-h" => {
                    print_usage(&args[0]);
                    return Ok(());
                }
                arg => {
                    eprintln!("Error: Unknown argument: {}", arg);
                    print_usage(&args[0]);
                    std::process::exit(1);
                }
            }
        }
    }

    #[cfg(not(feature = "paddle-ocr"))]
    {
        eprintln!("Error: This example requires the 'paddle-ocr' feature to be enabled");
        eprintln!("Run: cargo run --example download_paddle_models --features paddle-ocr");
        std::process::exit(1);
    }

    #[cfg(feature = "paddle-ocr")]
    {
        println!("PaddleOCR Model Download Tool");
        println!("==============================\n");

        // Create model manager
        let manager = ModelManager::new(cache_dir.clone());

        println!("Cache directory: {}\n", cache_dir.display());

        // Clear cache if requested
        if clear_cache_flag {
            println!("Clearing cache directory...");
            manager.clear_cache()?;
            println!("Cache cleared.\n");
        }

        // Check if models are already cached
        if manager.are_models_cached() {
            println!("All models are already cached!");
        } else {
            println!("Downloading PaddleOCR models...\n");
            println!("This may take a few minutes depending on your internet connection.");
            println!("Models being downloaded:");
            println!("  - Detection model (PP-OCRv5 server det)");
            println!("  - Classification model (PPOCRv2 cls)");
            println!("  - Recognition model (PP-OCRv5 rec, English)");
            println!("  - Dictionary file (for text recognition)\n");
            println!("Additional language family models are downloaded on-demand.");
            println!("Supported families: English, Chinese, Latin, Korean, Eslav, Thai, Greek,");
            println!("Arabic, Devanagari, Tamil, Telugu.\n");

            // Download and ensure models exist
            // SHA256 checksums are now embedded and verified automatically
            match manager.ensure_models_exist() {
                Ok(paths) => {
                    println!("\nModels downloaded successfully!\n");
                    println!("Model locations:");
                    println!("  Detection:       {}", paths.det_model.display());
                    println!("  Classification:  {}", paths.cls_model.display());
                    println!("  Recognition:     {}", paths.rec_model.display());
                    println!("  Dictionary:      {}", paths.dict_file.display());
                }
                Err(e) => {
                    eprintln!("Error downloading models: {}", e);
                    return Err(Box::new(e));
                }
            }
        }

        // Show statistics if requested
        if show_stats {
            println!("\nCache Statistics:");
            println!("================");

            match manager.cache_stats() {
                Ok(stats) => {
                    let size_mb = stats.total_size_bytes as f64 / 1_000_000.0;
                    let size_gb = stats.total_size_bytes as f64 / 1_000_000_000.0;

                    println!("  Total size: {:.2} MB ({:.3} GB)", size_mb, size_gb);
                    println!("  Model count: {}", stats.model_count);
                    println!("  Cache directory: {}", stats.cache_dir.display());

                    // List individual model files
                    println!("\nDetailed file listing:");
                    list_cache_contents(&stats.cache_dir)?;
                }
                Err(e) => {
                    eprintln!("Error retrieving cache stats: {}", e);
                }
            }
        }

        println!("\nReady to use PaddleOCR for document extraction!");
        Ok(())
    }
}

#[cfg(feature = "paddle-ocr")]
#[allow(dead_code)]
fn get_default_cache_dir() -> PathBuf {
    // Try multiple default locations in order of preference:
    // 1. PADDLE_OCR_HOME environment variable
    // 2. XDG_CACHE_HOME/kreuzberg/paddle (Linux)
    // 3. ~/Library/Caches/kreuzberg/paddle (macOS)
    // 4. %APPDATA%/kreuzberg/paddle (Windows)
    // 5. ~/.kreuzberg/models (fallback)

    if let Ok(home) = env::var("PADDLE_OCR_HOME") {
        return PathBuf::from(home);
    }

    if cfg!(target_os = "linux")
        && let Ok(xdg_cache) = env::var("XDG_CACHE_HOME")
    {
        return PathBuf::from(xdg_cache).join("kreuzberg/paddle");
    }

    if let Ok(home) = env::var("HOME") {
        if cfg!(target_os = "macos") {
            return PathBuf::from(home).join("Library/Caches/kreuzberg/paddle");
        } else if cfg!(target_os = "linux") {
            return PathBuf::from(home).join(".cache/kreuzberg/paddle");
        }
    }

    if cfg!(target_os = "windows")
        && let Ok(appdata) = env::var("APPDATA")
    {
        return PathBuf::from(appdata).join("kreuzberg/paddle");
    }

    // Fallback to home directory cache
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".kreuzberg/models")
}

#[cfg(not(feature = "paddle-ocr"))]
#[allow(dead_code)]
fn get_default_cache_dir() -> PathBuf {
    PathBuf::from("/tmp/paddle_models")
}

#[allow(dead_code)]
fn print_usage(program_name: &str) {
    println!("PaddleOCR Model Download Tool");
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS]", program_name);
    println!();
    println!("OPTIONS:");
    println!("    --cache-dir <PATH>     Directory to cache models in");
    println!("    --show-stats           Display cache statistics after download");
    println!("    --clear-cache          Clear cache before downloading");
    println!("    --help, -h             Print this help message");
    println!();
    println!("NOTES:");
    println!("    Language-specific models are supported and downloaded on-demand.");
    println!("    Supported script families: English, Chinese, Latin, Korean, Eslav,");
    println!("    Thai, Greek, Arabic, Devanagari, Tamil, Telugu.");
    println!("    See example documentation for language support details.");
    println!();
    println!("EXAMPLES:");
    println!("    {} --cache-dir /tmp/models", program_name);
    println!("    {} --show-stats", program_name);
    println!("    {} --clear-cache --cache-dir ~/.paddle_models", program_name);
}

#[cfg(feature = "paddle-ocr")]
fn list_cache_contents(cache_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !cache_dir.exists() {
        println!("    (cache directory does not exist yet)");
        return Ok(());
    }

    for entry in fs::read_dir(cache_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();

        if path.is_dir() {
            println!("  [DIR] {}/", file_name.to_string_lossy());

            // List files in subdirectory (2 levels deep for most, 3 for rec/)
            for sub_entry in fs::read_dir(&path)? {
                let sub_entry = sub_entry?;
                let sub_path = sub_entry.path();
                let sub_name = sub_entry.file_name();

                if sub_path.is_file() {
                    let metadata = fs::metadata(&sub_path)?;
                    let size_kb = metadata.len() as f64 / 1000.0;
                    println!("      - {} ({:.1} KB)", sub_name.to_string_lossy(), size_kb);
                } else if sub_path.is_dir() {
                    // For rec/ directory, we have 3 levels: rec/{family}/{files}
                    println!("      [DIR] {}/", sub_name.to_string_lossy());

                    // List files in the third level
                    for third_entry in fs::read_dir(&sub_path)? {
                        let third_entry = third_entry?;
                        let third_path = third_entry.path();
                        let third_name = third_entry.file_name();

                        if third_path.is_file() {
                            let metadata = fs::metadata(&third_path)?;
                            let size_kb = metadata.len() as f64 / 1000.0;
                            println!("          - {} ({:.1} KB)", third_name.to_string_lossy(), size_kb);
                        } else if third_path.is_dir() {
                            println!("          [DIR] {}/", third_name.to_string_lossy());
                        }
                    }
                }
            }
        } else if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let size_kb = metadata.len() as f64 / 1000.0;
            println!("  {} ({:.1} KB)", file_name.to_string_lossy(), size_kb);
        }
    }

    Ok(())
}

#[cfg(not(feature = "paddle-ocr"))]
#[allow(dead_code)]
fn list_cache_contents(_cache_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
