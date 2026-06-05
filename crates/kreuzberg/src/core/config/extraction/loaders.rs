//! Configuration file loading.
//!
//! This module provides methods for loading extraction configuration from
//! TOML, YAML, and JSON files.

use crate::{KreuzbergError, Result};
use std::path::Path;

use super::core::ExtractionConfig;

impl ExtractionConfig {
    /// Load configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns `KreuzbergError::Validation` if file doesn't exist or is invalid TOML.
    pub fn from_toml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        toml::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid TOML in {}: {}", path.display(), e)))
    }

    /// Load configuration from a YAML file.
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        serde_yaml_ng::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid YAML in {}: {}", path.display(), e)))
    }

    /// Load configuration from a JSON file.
    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| KreuzbergError::validation(format!("Failed to read config file {}: {}", path.display(), e)))?;
        serde_json::from_str(&content)
            .map_err(|e| KreuzbergError::validation(format!("Invalid JSON in {}: {}", path.display(), e)))
    }

    /// Load configuration from a file, auto-detecting format by extension.
    ///
    /// Supported formats: `.toml`, `.yaml`, `.yml`, `.json`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let extension = path.extension().and_then(|ext| ext.to_str()).ok_or_else(|| {
            KreuzbergError::validation(format!(
                "Cannot determine file format: no extension found in {}",
                path.display()
            ))
        })?;

        match extension.to_lowercase().as_str() {
            "toml" => Self::from_toml_file(path),
            "yaml" | "yml" => Self::from_yaml_file(path),
            "json" => Self::from_json_file(path),
            other => Err(KreuzbergError::validation(format!(
                "Unsupported config file format: .{}. Supported formats: .toml, .yaml, .json",
                other
            ))),
        }
    }

    /// Discover configuration file in parent directories.
    ///
    /// Searches for `kreuzberg.toml` in current directory and parent directories.
    pub fn discover() -> Result<Option<Self>> {
        let mut current = std::env::current_dir().map_err(crate::KreuzbergError::from)?;

        loop {
            let kreuzberg_toml = current.join("kreuzberg.toml");
            if kreuzberg_toml.exists() {
                return Ok(Some(Self::from_toml_file(kreuzberg_toml)?));
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        Ok(None)
    }
}
