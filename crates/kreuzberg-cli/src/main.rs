//! Kreuzberg CLI - Command-line interface for document intelligence.
//!
//! This binary provides a command-line interface to the Kreuzberg document intelligence
//! library, supporting document extraction, MIME type detection, caching, and batch operations.
//!
//! # Architecture
//!
//! The CLI is built using `clap` for argument parsing and provides five main commands:
//! - `extract`: Extract text/data from a single document
//! - `batch`: Process multiple documents in parallel
//! - `detect`: Identify MIME type of a file
//! - `cache`: Manage cache (clear, stats)
//! - `serve`: Start API server (requires `api` feature)
//! - `version`: Show version information
//!
//! # Configuration
//!
//! The CLI supports configuration files in TOML, YAML, or JSON formats:
//! - Explicit: `--config path/to/config.toml`
//! - Auto-discovery: Searches for `kreuzberg.{toml,yaml,json}` in current and parent directories
//! - Command-line flags override config file settings
//!
//! # Exit Codes
//!
//! - 0: Success
//! - Non-zero: Error (see stderr for details)
//!
//! # Examples
//!
//! ```bash
//! # Extract text from a PDF
//! kreuzberg extract document.pdf
//!
//! # Extract with OCR enabled
//! kreuzberg extract scanned.pdf --ocr true
//!
//! # Batch processing
//! kreuzberg batch *.pdf --format json
//!
//! # Detect MIME type
//! kreuzberg detect unknown-file.bin
//! ```

#![deny(unsafe_code)]

mod commands;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
#[cfg(feature = "mcp")]
use commands::mcp_command;
#[cfg(feature = "api")]
use commands::serve_command;
use commands::{apply_extraction_overrides, batch_command, clear_command, extract_command, load_config, stats_command};
use kreuzberg::{OutputFormat as ContentOutputFormat, detect_mime_type};
use serde_json::json;
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

/// Kreuzberg document intelligence CLI
#[derive(Parser)]
#[command(name = "kreuzberg")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract text from a document
    Extract {
        /// Path to the document
        path: PathBuf,

        /// Path to config file (TOML, YAML, or JSON). If not specified, searches for kreuzberg.toml/yaml/json in current and parent directories.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// MIME type hint (auto-detected if not provided)
        #[arg(short, long)]
        mime_type: Option<String>,

        /// Output format (text or json)
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,

        /// Enable OCR (overrides config file)
        #[arg(long)]
        ocr: Option<bool>,

        /// Force OCR even if text extraction succeeds (overrides config file)
        #[arg(long)]
        force_ocr: Option<bool>,

        /// Disable caching (overrides config file)
        #[arg(long)]
        no_cache: Option<bool>,

        /// Enable chunking (overrides config file)
        #[arg(long)]
        chunk: Option<bool>,

        /// Chunk size in characters (overrides config file)
        #[arg(long)]
        chunk_size: Option<usize>,

        /// Chunk overlap in characters (overrides config file)
        #[arg(long)]
        chunk_overlap: Option<usize>,

        /// Enable quality processing (overrides config file)
        #[arg(long)]
        quality: Option<bool>,

        /// Enable language detection (overrides config file)
        #[arg(long)]
        detect_language: Option<bool>,

        /// Content output format (plain, markdown, djot, html)
        ///
        /// Controls the format of the extracted content.
        /// Note: This is different from --format which controls CLI output (text/json).
        #[arg(long, value_enum)]
        content_format: Option<ContentOutputFormatArg>,
    },

    /// Batch extract from multiple documents
    Batch {
        /// Paths to documents
        paths: Vec<PathBuf>,

        /// Path to config file (TOML, YAML, or JSON). If not specified, searches for kreuzberg.toml/yaml/json in current and parent directories.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Output format (text or json)
        #[arg(short, long, default_value = "json")]
        format: OutputFormat,

        /// Enable OCR (overrides config file)
        #[arg(long)]
        ocr: Option<bool>,

        /// Force OCR even if text extraction succeeds (overrides config file)
        #[arg(long)]
        force_ocr: Option<bool>,

        /// Disable caching (overrides config file)
        #[arg(long)]
        no_cache: Option<bool>,

        /// Enable quality processing (overrides config file)
        #[arg(long)]
        quality: Option<bool>,

        /// Content output format (plain, markdown, djot, html)
        ///
        /// Controls the format of the extracted content.
        /// Note: This is different from --format which controls CLI output (text/json).
        #[arg(long, value_enum)]
        content_format: Option<ContentOutputFormatArg>,
    },

    /// Detect MIME type of a file
    Detect {
        /// Path to the file
        path: PathBuf,

        /// Output format (text or json)
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
    },

    /// Show version information
    Version {
        /// Output format (text or json)
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
    },

    /// Cache management operations
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },

    /// Start the API server
    ///
    /// Configuration is loaded with the following precedence (highest to lowest):
    /// 1. CLI arguments (--host, --port)
    /// 2. Environment variables (KREUZBERG_HOST, KREUZBERG_PORT)
    /// 3. Config file (TOML, YAML, or JSON)
    /// 4. Built-in defaults (127.0.0.1:8000)
    ///
    /// The config file can contain both extraction and server settings under [server] section.
    #[cfg(feature = "api")]
    Serve {
        /// Host to bind to (e.g., "127.0.0.1" or "0.0.0.0"). CLI arg overrides config file and env vars.
        #[arg(short = 'H', long)]
        host: Option<String>,

        /// Port to bind to. CLI arg overrides config file and env vars.
        #[arg(short, long)]
        port: Option<u16>,

        /// Path to config file (TOML, YAML, or JSON). If not specified, searches for kreuzberg.toml/yaml/json in current and parent directories.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Start the MCP (Model Context Protocol) server
    #[cfg(feature = "mcp")]
    Mcp {
        /// Path to config file (TOML, YAML, or JSON). If not specified, searches for kreuzberg.toml/yaml/json in current and parent directories.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Transport mode: stdio (default) or http
        #[arg(long, default_value = "stdio")]
        transport: String,

        /// HTTP host (only for --transport http)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// HTTP port (only for --transport http)
        #[arg(long, default_value = "8001")]
        port: u16,
    },
}

#[derive(Subcommand)]
enum CacheCommands {
    /// Show cache statistics
    Stats {
        /// Cache directory (default: .kreuzberg in current directory)
        #[arg(short, long)]
        cache_dir: Option<PathBuf>,

        /// Output format (text or json)
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
    },

    /// Clear the cache
    Clear {
        /// Cache directory (default: .kreuzberg in current directory)
        #[arg(short, long)]
        cache_dir: Option<PathBuf>,

        /// Output format (text or json)
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Invalid format: {}. Use 'text' or 'json'", s)),
        }
    }
}

/// Content output format for extraction results.
///
/// Controls the format of the extracted content (not the CLI output format).
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
enum ContentOutputFormatArg {
    /// Plain text (default)
    Plain,
    /// Markdown format
    Markdown,
    /// Djot markup format
    Djot,
    /// HTML format
    Html,
}

impl From<ContentOutputFormatArg> for ContentOutputFormat {
    fn from(arg: ContentOutputFormatArg) -> Self {
        match arg {
            ContentOutputFormatArg::Plain => ContentOutputFormat::Plain,
            ContentOutputFormatArg::Markdown => ContentOutputFormat::Markdown,
            ContentOutputFormatArg::Djot => ContentOutputFormat::Djot,
            ContentOutputFormatArg::Html => ContentOutputFormat::Html,
        }
    }
}

/// Validates that a file exists and is accessible.
///
/// Checks that the path exists in the filesystem and points to a regular file
/// (not a directory or special file). Provides user-friendly error messages if validation fails.
///
/// # Errors
///
/// Returns an error if:
/// - The path does not exist in the filesystem
/// - The path exists but is not a regular file (e.g., is a directory)
fn validate_file_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        anyhow::bail!(
            "File not found: '{}'. Please check that the file exists and is accessible.",
            path.display()
        );
    }
    if !path.is_file() {
        anyhow::bail!(
            "Path is not a file: '{}'. Please provide a path to a regular file.",
            path.display()
        );
    }
    Ok(())
}

/// Validates chunking parameters for correctness.
///
/// Ensures that chunking configuration makes sense: size must be positive and reasonable,
/// and overlap must be smaller than chunk size. This prevents common configuration errors
/// that would lead to cryptic failures from the underlying library.
///
/// # Errors
///
/// Returns an error if:
/// - `chunk_size` is 0 (must be at least 1 character)
/// - `chunk_size` exceeds 1,000,000 characters (to prevent excessive memory usage)
/// - `chunk_overlap` is greater than or equal to `chunk_size` (overlap must be smaller)
fn validate_chunk_params(chunk_size: Option<usize>, chunk_overlap: Option<usize>) -> Result<()> {
    if let Some(size) = chunk_size {
        if size == 0 {
            anyhow::bail!("Invalid chunk size: {}. Chunk size must be greater than 0.", size);
        }
        if size > 1_000_000 {
            anyhow::bail!(
                "Invalid chunk size: {}. Chunk size must be less than 1,000,000 characters to avoid excessive memory usage.",
                size
            );
        }
    }

    if let Some(overlap) = chunk_overlap
        && let Some(size) = chunk_size
        && overlap >= size
    {
        anyhow::bail!(
            "Invalid chunk overlap: {}. Overlap ({}) must be less than chunk size ({}).",
            overlap,
            overlap,
            size
        );
    }

    Ok(())
}

/// Validates batch extraction paths for correctness.
///
/// Ensures that at least one file path is provided and that all paths point to valid,
/// accessible files. This prevents processing empty batches or failing mid-batch due
/// to invalid paths.
///
/// # Errors
///
/// Returns an error if:
/// - The paths array is empty (at least one file is required)
/// - Any path does not exist or is not a regular file
fn validate_batch_paths(paths: &[PathBuf]) -> Result<()> {
    if paths.is_empty() {
        anyhow::bail!("No files provided for batch extraction. Please provide at least one file path.");
    }

    for (i, path) in paths.iter().enumerate() {
        validate_file_exists(path).with_context(|| format!("Invalid file at position {}", i + 1))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with_writer(std::io::stderr)
        .try_init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Extract {
            path,
            config: config_path,
            mime_type,
            format,
            ocr,
            force_ocr,
            no_cache,
            chunk,
            chunk_size,
            chunk_overlap,
            quality,
            detect_language,
            content_format,
        } => {
            validate_file_exists(&path)?;
            validate_chunk_params(chunk_size, chunk_overlap)?;

            let mut config = load_config(config_path)?;
            apply_extraction_overrides(
                &mut config,
                ocr,
                force_ocr,
                no_cache,
                chunk,
                chunk_size,
                chunk_overlap,
                quality,
                detect_language,
                content_format,
            );

            extract_command(path, config, mime_type, format)?;
        }

        Commands::Batch {
            paths,
            config: config_path,
            format,
            ocr,
            force_ocr,
            no_cache,
            quality,
            content_format,
        } => {
            validate_batch_paths(&paths)?;

            let mut config = load_config(config_path)?;
            apply_extraction_overrides(
                &mut config,
                ocr,
                force_ocr,
                no_cache,
                None,
                None,
                None,
                quality,
                None,
                content_format,
            );

            batch_command(paths, config, format)?;
        }

        Commands::Detect { path, format } => {
            validate_file_exists(&path)?;

            let path_str = path.to_string_lossy().to_string();
            let mime_type = detect_mime_type(&path_str, true).with_context(|| {
                format!(
                    "Failed to detect MIME type for file '{}'. Ensure the file is readable.",
                    path.display()
                )
            })?;

            match format {
                OutputFormat::Text => {
                    println!("{}", mime_type);
                }
                OutputFormat::Json => {
                    let output = json!({
                        "path": path_str,
                        "mime_type": mime_type,
                    });
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&output)
                            .context("Failed to serialize MIME type detection result to JSON")?
                    );
                }
            }
        }

        Commands::Version { format } => {
            let version = env!("CARGO_PKG_VERSION");
            let name = env!("CARGO_PKG_NAME");

            match format {
                OutputFormat::Text => {
                    println!("{} {}", name, version);
                }
                OutputFormat::Json => {
                    let output = json!({
                        "name": name,
                        "version": version,
                    });
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&output)
                            .context("Failed to serialize version information to JSON")?
                    );
                }
            }
        }

        #[cfg(feature = "api")]
        Commands::Serve {
            host: cli_host,
            port: cli_port,
            config: config_path,
        } => {
            let extraction_config = load_config(config_path.clone())?;
            serve_command(cli_host, cli_port, extraction_config, config_path)?;
        }

        #[cfg(feature = "mcp")]
        Commands::Mcp {
            config: config_path,
            transport,
            #[cfg(feature = "mcp-http")]
            host,
            #[cfg(feature = "mcp-http")]
            port,
            #[cfg(not(feature = "mcp-http"))]
            host,
            #[cfg(not(feature = "mcp-http"))]
            port,
        } => {
            let config = load_config(config_path)?;
            mcp_command(config, transport, host, port)?;
        }

        Commands::Cache { command } => match command {
            CacheCommands::Stats { cache_dir, format } => {
                stats_command(cache_dir, format)?;
            }
            CacheCommands::Clear { cache_dir, format } => {
                clear_command(cache_dir, format)?;
            }
        },
    }

    Ok(())
}
