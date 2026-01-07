//! Server configuration for the Kreuzberg API.
//!
//! This module provides the `ServerConfig` struct for managing API server settings
//! including host, port, CORS, and upload size limits. Configuration can be loaded
//! from TOML, YAML, or JSON files and can be overridden by environment variables.
//!
//! # Features
//!
//! - **Multi-format support**: Load configuration from TOML, YAML, or JSON files
//! - **Environment overrides**: All settings can be overridden via environment variables
//! - **Backward compatibility**: Supports legacy `max_upload_mb` field for smooth migrations
//! - **Sensible defaults**: All fields have reasonable defaults matching current behavior
//! - **Flexible CORS**: Support for all origins (default) or specific origin lists
//!
//! # Example
//!
//! ```rust,no_run
//! use kreuzberg::core::ServerConfig;
//!
//! # fn example() -> kreuzberg::Result<()> {
//! // Create with defaults
//! let mut config = ServerConfig::default();
//!
//! // Or load from file
//! let mut config = ServerConfig::from_file("kreuzberg.toml")?;
//!
//! // Apply environment variable overrides
//! config.apply_env_overrides()?;
//!
//! # Ok(())
//! # }
//! ```

use crate::{KreuzbergError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Default host address for API server
const DEFAULT_HOST: &str = "127.0.0.1";

/// Default port for API server
const DEFAULT_PORT: u16 = 8000;

/// Default maximum request body size: 100 MB
const DEFAULT_MAX_REQUEST_BODY_BYTES: usize = 104_857_600;

/// Default maximum multipart field size: 100 MB
const DEFAULT_MAX_MULTIPART_FIELD_BYTES: usize = 104_857_600;

/// API server configuration.
///
/// This struct holds all configuration options for the Kreuzberg API server,
/// including host/port settings, CORS configuration, and upload limits.
///
/// # Defaults
///
/// - `host`: "127.0.0.1" (localhost only)
/// - `port`: 8000
/// - `cors_origins`: empty vector (allows all origins)
/// - `max_request_body_bytes`: 104_857_600 (100 MB)
/// - `max_multipart_field_bytes`: 104_857_600 (100 MB)
/// - `max_upload_mb`: None (legacy field, not used if other fields set)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    /// Server host address (e.g., "127.0.0.1", "0.0.0.0")
    #[serde(default = "default_host")]
    pub host: String,

    /// Server port number
    #[serde(default = "default_port")]
    pub port: u16,

    /// CORS allowed origins. Empty vector means allow all origins.
    ///
    /// If this is an empty vector, the server will accept requests from any origin.
    /// If populated with specific origins (e.g., ["https://example.com"]), only
    /// those origins will be allowed.
    #[serde(default)]
    pub cors_origins: Vec<String>,

    /// Maximum size of request body in bytes (default: 100 MB)
    #[serde(default = "default_max_request_body_bytes")]
    pub max_request_body_bytes: usize,

    /// Maximum size of multipart fields in bytes (default: 100 MB)
    #[serde(default = "default_max_multipart_field_bytes")]
    pub max_multipart_field_bytes: usize,

    /// Legacy upload size limit in MB (for backward compatibility).
    ///
    /// This field is deprecated and only used for backward compatibility.
    /// If set, it will override `max_multipart_field_bytes` during normalization.
    /// New configurations should use `max_multipart_field_bytes` directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_mb: Option<usize>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            cors_origins: Vec::new(),
            max_request_body_bytes: default_max_request_body_bytes(),
            max_multipart_field_bytes: default_max_multipart_field_bytes(),
            max_upload_mb: None,
        }
    }
}

// Default value functions for serde
fn default_host() -> String {
    DEFAULT_HOST.to_string()
}

fn default_port() -> u16 {
    DEFAULT_PORT
}

fn default_max_request_body_bytes() -> usize {
    DEFAULT_MAX_REQUEST_BODY_BYTES
}

fn default_max_multipart_field_bytes() -> usize {
    DEFAULT_MAX_MULTIPART_FIELD_BYTES
}

impl ServerConfig {
    /// Create a new `ServerConfig` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the server listen address (host:port).
    ///
    /// # Example
    ///
    /// ```rust
    /// use kreuzberg::core::ServerConfig;
    ///
    /// let config = ServerConfig::default();
    /// assert_eq!(config.listen_addr(), "127.0.0.1:8000");
    /// ```
    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Check if CORS allows all origins.
    ///
    /// Returns `true` if the `cors_origins` vector is empty, meaning all origins
    /// are allowed. Returns `false` if specific origins are configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kreuzberg::core::ServerConfig;
    ///
    /// let mut config = ServerConfig::default();
    /// assert!(config.cors_allows_all());
    ///
    /// config.cors_origins.push("https://example.com".to_string());
    /// assert!(!config.cors_allows_all());
    /// ```
    pub fn cors_allows_all(&self) -> bool {
        self.cors_origins.is_empty()
    }

    /// Check if a given origin is allowed by CORS configuration.
    ///
    /// Returns `true` if:
    /// - CORS allows all origins (empty origins list), or
    /// - The given origin is in the allowed origins list
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin to check (e.g., "https://example.com")
    ///
    /// # Example
    ///
    /// ```rust
    /// use kreuzberg::core::ServerConfig;
    ///
    /// let mut config = ServerConfig::default();
    /// assert!(config.is_origin_allowed("https://example.com"));
    ///
    /// config.cors_origins.push("https://allowed.com".to_string());
    /// assert!(config.is_origin_allowed("https://allowed.com"));
    /// assert!(!config.is_origin_allowed("https://denied.com"));
    /// ```
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        self.cors_origins.is_empty() || self.cors_origins.contains(&origin.to_string())
    }

    /// Get maximum request body size in megabytes (rounded up).
    ///
    /// # Example
    ///
    /// ```rust
    /// use kreuzberg::core::ServerConfig;
    ///
    /// let mut config = ServerConfig::default();
    /// assert_eq!(config.max_request_body_mb(), 100);
    /// ```
    pub fn max_request_body_mb(&self) -> usize {
        self.max_request_body_bytes.div_ceil(1_048_576)
    }

    /// Get maximum multipart field size in megabytes (rounded up).
    ///
    /// # Example
    ///
    /// ```rust
    /// use kreuzberg::core::ServerConfig;
    ///
    /// let mut config = ServerConfig::default();
    /// assert_eq!(config.max_multipart_field_mb(), 100);
    /// ```
    pub fn max_multipart_field_mb(&self) -> usize {
        self.max_multipart_field_bytes.div_ceil(1_048_576)
    }

    /// Normalize legacy field values for backward compatibility.
    ///
    /// If `max_upload_mb` is set, it will be converted to bytes and used to
    /// override `max_multipart_field_bytes`. This allows old configurations
    /// using the legacy field to continue working.
    ///
    /// This method is automatically called by `apply_env_overrides()`.
    pub fn normalize_legacy_fields(&mut self) {
        if let Some(max_upload_mb) = self.max_upload_mb {
            // Convert MB to bytes
            let max_bytes = max_upload_mb.saturating_mul(1_048_576);
            self.max_multipart_field_bytes = max_bytes;
        }
    }

    /// Apply environment variable overrides to the configuration.
    ///
    /// Reads the following environment variables and overrides config values if set:
    ///
    /// - `KREUZBERG_HOST` - Server host address
    /// - `KREUZBERG_PORT` - Server port number (parsed as u16)
    /// - `KREUZBERG_CORS_ORIGINS` - Comma-separated list of allowed origins
    /// - `KREUZBERG_MAX_REQUEST_BODY_BYTES` - Max request body size in bytes
    /// - `KREUZBERG_MAX_MULTIPART_FIELD_BYTES` - Max multipart field size in bytes
    /// - `KREUZBERG_MAX_UPLOAD_SIZE_MB` - Max upload size in MB (legacy)
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if:
    /// - `KREUZBERG_PORT` cannot be parsed as u16
    /// - `KREUZBERG_MAX_REQUEST_BODY_BYTES` cannot be parsed as usize
    /// - `KREUZBERG_MAX_MULTIPART_FIELD_BYTES` cannot be parsed as usize
    /// - `KREUZBERG_MAX_UPLOAD_SIZE_MB` cannot be parsed as usize
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kreuzberg::core::ServerConfig;
    ///
    /// # fn example() -> kreuzberg::Result<()> {
    /// std::env::set_var("KREUZBERG_HOST", "0.0.0.0");
    /// std::env::set_var("KREUZBERG_PORT", "3000");
    ///
    /// let mut config = ServerConfig::default();
    /// config.apply_env_overrides()?;
    ///
    /// assert_eq!(config.host, "0.0.0.0");
    /// assert_eq!(config.port, 3000);
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply_env_overrides(&mut self) -> Result<()> {
        // Host override
        if let Ok(host) = std::env::var("KREUZBERG_HOST") {
            self.host = host;
        }

        // Port override
        if let Ok(port_str) = std::env::var("KREUZBERG_PORT") {
            self.port = port_str.parse::<u16>().map_err(|e| {
                KreuzbergError::validation(format!(
                    "KREUZBERG_PORT must be a valid u16 number, got '{}': {}",
                    port_str, e
                ))
            })?;
        }

        // CORS origins override (comma-separated)
        if let Ok(origins_str) = std::env::var("KREUZBERG_CORS_ORIGINS") {
            self.cors_origins = origins_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Max request body bytes override
        if let Ok(bytes_str) = std::env::var("KREUZBERG_MAX_REQUEST_BODY_BYTES") {
            self.max_request_body_bytes = bytes_str.parse::<usize>().map_err(|e| {
                KreuzbergError::validation(format!(
                    "KREUZBERG_MAX_REQUEST_BODY_BYTES must be a valid usize, got '{}': {}",
                    bytes_str, e
                ))
            })?;
        }

        // Max multipart field bytes override
        if let Ok(bytes_str) = std::env::var("KREUZBERG_MAX_MULTIPART_FIELD_BYTES") {
            self.max_multipart_field_bytes = bytes_str.parse::<usize>().map_err(|e| {
                KreuzbergError::validation(format!(
                    "KREUZBERG_MAX_MULTIPART_FIELD_BYTES must be a valid usize, got '{}': {}",
                    bytes_str, e
                ))
            })?;
        }

        // Legacy max upload size override (in MB)
        if let Ok(mb_str) = std::env::var("KREUZBERG_MAX_UPLOAD_SIZE_MB") {
            let mb = mb_str.parse::<usize>().map_err(|e| {
                KreuzbergError::validation(format!(
                    "KREUZBERG_MAX_UPLOAD_SIZE_MB must be a valid usize, got '{}': {}",
                    mb_str, e
                ))
            })?;
            self.max_upload_mb = Some(mb);
        }

        // Apply legacy field normalization
        self.normalize_legacy_fields();

        Ok(())
    }

    /// Load server configuration from a file.
    ///
    /// Automatically detects the file format based on extension:
    /// - `.toml` - TOML format
    /// - `.yaml` or `.yml` - YAML format
    /// - `.json` - JSON format
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if:
    /// - File doesn't exist or cannot be read
    /// - File extension is not recognized
    /// - File content is invalid for the detected format
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kreuzberg::core::ServerConfig;
    ///
    /// # fn example() -> kreuzberg::Result<()> {
    /// let config = ServerConfig::from_file("kreuzberg.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let extension = path.extension().and_then(|ext| ext.to_str()).ok_or_else(|| {
            KreuzbergError::validation(format!(
                "Cannot determine file format: no extension found in {}",
                path.display()
            ))
        })?;

        let mut config = match extension.to_lowercase().as_str() {
            "toml" => toml::from_str::<Self>(&content)
                .map_err(|e| KreuzbergError::validation(format!("Invalid TOML in {}: {}", path.display(), e)))?,
            "yaml" | "yml" => serde_yaml_ng::from_str::<Self>(&content)
                .map_err(|e| KreuzbergError::validation(format!("Invalid YAML in {}: {}", path.display(), e)))?,
            "json" => serde_json::from_str::<Self>(&content)
                .map_err(|e| KreuzbergError::validation(format!("Invalid JSON in {}: {}", path.display(), e)))?,
            _ => {
                return Err(KreuzbergError::validation(format!(
                    "Unsupported config file format: .{}. Supported formats: .toml, .yaml, .yml, .json",
                    extension
                )));
            }
        };

        // Normalize legacy fields
        config.normalize_legacy_fields();

        Ok(config)
    }

    /// Load server configuration from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML file
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if the file doesn't exist or is invalid TOML.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kreuzberg::core::ServerConfig;
    ///
    /// # fn example() -> kreuzberg::Result<()> {
    /// let config = ServerConfig::from_toml_file("kreuzberg.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_toml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let mut config: Self = toml::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid TOML in {}: {}", path.display(), e)))?;

        config.normalize_legacy_fields();

        Ok(config)
    }

    /// Load server configuration from a YAML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the YAML file
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if the file doesn't exist or is invalid YAML.
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let mut config: Self = serde_yaml_ng::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid YAML in {}: {}", path.display(), e)))?;

        config.normalize_legacy_fields();

        Ok(config)
    }

    /// Load server configuration from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON file
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if the file doesn't exist or is invalid JSON.
    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;

        let mut config: Self = serde_json::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid JSON in {}: {}", path.display(), e)))?;

        config.normalize_legacy_fields();

        Ok(config)
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8000);
        assert!(config.cors_origins.is_empty());
        assert_eq!(config.max_request_body_bytes, 104_857_600);
        assert_eq!(config.max_multipart_field_bytes, 104_857_600);
        assert!(config.max_upload_mb.is_none());
    }

    #[test]
    fn test_listen_addr() {
        let config = ServerConfig::default();
        assert_eq!(config.listen_addr(), "127.0.0.1:8000");
    }

    #[test]
    fn test_listen_addr_custom() {
        let mut config = ServerConfig::default();
        config.host = "0.0.0.0".to_string();
        config.port = 3000;
        assert_eq!(config.listen_addr(), "0.0.0.0:3000");
    }

    #[test]
    fn test_cors_allows_all() {
        let mut config = ServerConfig::default();
        assert!(config.cors_allows_all());

        config.cors_origins.push("https://example.com".to_string());
        assert!(!config.cors_allows_all());
    }

    #[test]
    fn test_is_origin_allowed_all() {
        let config = ServerConfig::default();
        assert!(config.is_origin_allowed("https://example.com"));
        assert!(config.is_origin_allowed("https://other.com"));
    }

    #[test]
    fn test_is_origin_allowed_specific() {
        let mut config = ServerConfig::default();
        config.cors_origins.push("https://allowed.com".to_string());

        assert!(config.is_origin_allowed("https://allowed.com"));
        assert!(!config.is_origin_allowed("https://denied.com"));
    }

    #[test]
    fn test_max_request_body_mb() {
        let config = ServerConfig::default();
        assert_eq!(config.max_request_body_mb(), 100);
    }

    #[test]
    fn test_max_multipart_field_mb() {
        let config = ServerConfig::default();
        assert_eq!(config.max_multipart_field_mb(), 100);
    }

    #[test]
    fn test_max_bytes_to_mb_rounding() {
        let mut config = ServerConfig::default();
        config.max_request_body_bytes = 1_048_576; // 1 MB
        assert_eq!(config.max_request_body_mb(), 1);

        config.max_request_body_bytes = 1_048_577; // 1 MB + 1 byte
        assert_eq!(config.max_request_body_mb(), 2); // Rounds up
    }

    #[test]
    fn test_normalize_legacy_max_upload_mb() {
        let mut config = ServerConfig::default();
        config.max_upload_mb = Some(50);

        config.normalize_legacy_fields();

        assert_eq!(config.max_multipart_field_bytes, 50 * 1_048_576);
    }

    #[test]
    fn test_normalize_legacy_max_upload_mb_zero() {
        let mut config = ServerConfig::default();
        config.max_upload_mb = Some(0);

        config.normalize_legacy_fields();

        assert_eq!(config.max_multipart_field_bytes, 0);
    }

    #[test]
    fn test_from_toml_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.toml");

        fs::write(
            &config_path,
            r#"
host = "0.0.0.0"
port = 3000
cors_origins = ["https://example.com", "https://other.com"]
max_request_body_bytes = 50000000
max_multipart_field_bytes = 75000000
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_toml_file(&config_path).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.cors_origins.len(), 2);
        assert_eq!(config.max_request_body_bytes, 50_000_000);
        assert_eq!(config.max_multipart_field_bytes, 75_000_000);
    }

    #[test]
    fn test_from_yaml_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.yaml");

        fs::write(
            &config_path,
            r#"
host: 0.0.0.0
port: 3000
cors_origins:
  - https://example.com
  - https://other.com
max_request_body_bytes: 50000000
max_multipart_field_bytes: 75000000
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_yaml_file(&config_path).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.cors_origins.len(), 2);
        assert_eq!(config.max_request_body_bytes, 50_000_000);
        assert_eq!(config.max_multipart_field_bytes, 75_000_000);
    }

    #[test]
    fn test_from_json_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.json");

        fs::write(
            &config_path,
            r#"{
  "host": "0.0.0.0",
  "port": 3000,
  "cors_origins": ["https://example.com", "https://other.com"],
  "max_request_body_bytes": 50000000,
  "max_multipart_field_bytes": 75000000
}
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_json_file(&config_path).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.cors_origins.len(), 2);
        assert_eq!(config.max_request_body_bytes, 50_000_000);
        assert_eq!(config.max_multipart_field_bytes, 75_000_000);
    }

    #[test]
    fn test_from_file_auto_detects_toml() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.toml");

        fs::write(
            &config_path,
            r#"
host = "0.0.0.0"
port = 3000
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_file(&config_path).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_from_file_auto_detects_yaml() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.yaml");

        fs::write(
            &config_path,
            r#"
host: 0.0.0.0
port: 3000
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_file(&config_path).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_from_file_auto_detects_json() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.json");

        fs::write(&config_path, r#"{"host": "0.0.0.0", "port": 3000}"#).unwrap();

        let config = ServerConfig::from_file(&config_path).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_from_file_unsupported_extension() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.txt");

        fs::write(&config_path, "host = 0.0.0.0").unwrap();

        let result = ServerConfig::from_file(&config_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unsupported config file format")
        );
    }

    #[test]
    fn test_from_file_no_extension() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server");

        fs::write(&config_path, "host = 0.0.0.0").unwrap();

        let result = ServerConfig::from_file(&config_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no extension found"));
    }

    #[test]
    fn test_legacy_max_upload_mb_in_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.toml");

        fs::write(
            &config_path,
            r#"
host = "127.0.0.1"
port = 8000
max_upload_mb = 50
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_toml_file(&config_path).unwrap();
        assert_eq!(config.max_upload_mb, Some(50));
        assert_eq!(config.max_multipart_field_bytes, 50 * 1_048_576);
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_host_override() {
        let original = std::env::var("KREUZBERG_HOST").ok();
        unsafe {
            std::env::set_var("KREUZBERG_HOST", "192.168.1.1");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.host, "192.168.1.1");

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_HOST", orig);
            } else {
                std::env::remove_var("KREUZBERG_HOST");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_port_override() {
        let original = std::env::var("KREUZBERG_PORT").ok();
        unsafe {
            std::env::set_var("KREUZBERG_PORT", "5000");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.port, 5000);

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_PORT", orig);
            } else {
                std::env::remove_var("KREUZBERG_PORT");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_port_invalid() {
        let original = std::env::var("KREUZBERG_PORT").ok();
        unsafe {
            std::env::set_var("KREUZBERG_PORT", "not_a_number");
        }

        let mut config = ServerConfig::default();
        let result = config.apply_env_overrides();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("KREUZBERG_PORT must be a valid u16")
        );

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_PORT", orig);
            } else {
                std::env::remove_var("KREUZBERG_PORT");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_cors_origins_override() {
        let original = std::env::var("KREUZBERG_CORS_ORIGINS").ok();
        unsafe {
            std::env::set_var("KREUZBERG_CORS_ORIGINS", "https://example.com, https://other.com");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.cors_origins.len(), 2);
        assert!(config.cors_origins.contains(&"https://example.com".to_string()));
        assert!(config.cors_origins.contains(&"https://other.com".to_string()));

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_CORS_ORIGINS", orig);
            } else {
                std::env::remove_var("KREUZBERG_CORS_ORIGINS");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_max_request_body_bytes_override() {
        let original = std::env::var("KREUZBERG_MAX_REQUEST_BODY_BYTES").ok();
        unsafe {
            std::env::set_var("KREUZBERG_MAX_REQUEST_BODY_BYTES", "52428800");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.max_request_body_bytes, 52_428_800);

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_MAX_REQUEST_BODY_BYTES", orig);
            } else {
                std::env::remove_var("KREUZBERG_MAX_REQUEST_BODY_BYTES");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_max_multipart_field_bytes_override() {
        let original = std::env::var("KREUZBERG_MAX_MULTIPART_FIELD_BYTES").ok();
        unsafe {
            std::env::set_var("KREUZBERG_MAX_MULTIPART_FIELD_BYTES", "78643200");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.max_multipart_field_bytes, 78_643_200);

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_MAX_MULTIPART_FIELD_BYTES", orig);
            } else {
                std::env::remove_var("KREUZBERG_MAX_MULTIPART_FIELD_BYTES");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_legacy_max_upload_size_mb_override() {
        let original = std::env::var("KREUZBERG_MAX_UPLOAD_SIZE_MB").ok();
        unsafe {
            std::env::set_var("KREUZBERG_MAX_UPLOAD_SIZE_MB", "75");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.max_upload_mb, Some(75));
        assert_eq!(config.max_multipart_field_bytes, 75 * 1_048_576);

        // Cleanup
        unsafe {
            if let Some(orig) = original {
                std::env::set_var("KREUZBERG_MAX_UPLOAD_SIZE_MB", orig);
            } else {
                std::env::remove_var("KREUZBERG_MAX_UPLOAD_SIZE_MB");
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn test_apply_env_multiple_overrides() {
        let host_orig = std::env::var("KREUZBERG_HOST").ok();
        let port_orig = std::env::var("KREUZBERG_PORT").ok();
        let cors_orig = std::env::var("KREUZBERG_CORS_ORIGINS").ok();

        unsafe {
            std::env::set_var("KREUZBERG_HOST", "0.0.0.0");
            std::env::set_var("KREUZBERG_PORT", "4000");
            std::env::set_var("KREUZBERG_CORS_ORIGINS", "https://api.example.com");
        }

        let mut config = ServerConfig::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 4000);
        assert_eq!(config.cors_origins.len(), 1);
        assert_eq!(config.cors_origins[0], "https://api.example.com");

        // Cleanup
        unsafe {
            if let Some(orig) = host_orig {
                std::env::set_var("KREUZBERG_HOST", orig);
            } else {
                std::env::remove_var("KREUZBERG_HOST");
            }
            if let Some(orig) = port_orig {
                std::env::set_var("KREUZBERG_PORT", orig);
            } else {
                std::env::remove_var("KREUZBERG_PORT");
            }
            if let Some(orig) = cors_orig {
                std::env::set_var("KREUZBERG_CORS_ORIGINS", orig);
            } else {
                std::env::remove_var("KREUZBERG_CORS_ORIGINS");
            }
        }
    }

    #[test]
    fn test_serde_default_serialization() {
        let config = ServerConfig::default();
        let json = serde_json::to_string(&config).unwrap();

        // Should serialize without the max_upload_mb field when None
        assert!(!json.contains("max_upload_mb"));
    }

    #[test]
    fn test_serde_with_max_upload_mb_serialization() {
        let mut config = ServerConfig::default();
        config.max_upload_mb = Some(50);
        let json = serde_json::to_string(&config).unwrap();

        // Should serialize with max_upload_mb when Some
        assert!(json.contains("max_upload_mb"));
    }

    #[test]
    fn test_cors_origins_empty_in_toml() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.toml");

        fs::write(
            &config_path,
            r#"
host = "127.0.0.1"
port = 8000
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_toml_file(&config_path).unwrap();
        assert!(config.cors_origins.is_empty());
        assert!(config.cors_allows_all());
    }

    #[test]
    fn test_full_configuration_toml() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("server.toml");

        fs::write(
            &config_path,
            r#"
host = "192.168.1.100"
port = 9000
cors_origins = ["https://app1.com", "https://app2.com", "https://app3.com"]
max_request_body_bytes = 200000000
max_multipart_field_bytes = 150000000
        "#,
        )
        .unwrap();

        let config = ServerConfig::from_toml_file(&config_path).unwrap();
        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 9000);
        assert_eq!(config.listen_addr(), "192.168.1.100:9000");
        assert_eq!(config.cors_origins.len(), 3);
        assert!(!config.cors_allows_all());
        assert!(config.is_origin_allowed("https://app1.com"));
        assert!(!config.is_origin_allowed("https://app4.com"));
        assert_eq!(config.max_request_body_bytes, 200_000_000);
        assert_eq!(config.max_multipart_field_bytes, 150_000_000);
        assert_eq!(config.max_request_body_mb(), 191);
        assert_eq!(config.max_multipart_field_mb(), 144);
    }
}
