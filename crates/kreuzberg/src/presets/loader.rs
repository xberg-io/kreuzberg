//! Parse and validate a preset JSON file against the meta-schema.

use jsonschema::Validator;
use sha2::{Digest, Sha256};

use crate::presets::types::Preset;

/// Errors produced while loading or validating a preset file.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// The file is not valid JSON.
    #[error("preset {path}: failed to parse JSON: {source}")]
    Parse {
        /// File path that failed to parse.
        path: String,
        /// Underlying serde error.
        #[source]
        #[cfg_attr(alef, alef(skip))]
        source: serde_json::Error,
    },
    /// The file parses as JSON but does not validate against the meta-schema.
    #[error("preset {path}: failed meta-schema validation: {errors}")]
    SchemaValidation {
        /// File path that failed validation.
        path: String,
        /// Concatenated validation errors.
        errors: String,
    },
    /// The file validates but cannot be deserialized into [`Preset`].
    #[error("preset {path}: failed to deserialize after validation: {source}")]
    Deserialize {
        /// File path that failed to deserialize.
        path: String,
        /// Underlying serde error.
        #[source]
        #[cfg_attr(alef, alef(skip))]
        source: serde_json::Error,
    },
    /// The preset's declared `id` does not match its file-system location.
    #[error("preset {path}: id `{declared}` must match file path stem `{expected}`")]
    IdMismatch {
        /// File path of the offending preset.
        path: String,
        /// Identifier the preset declared in its body.
        declared: String,
        /// Identifier derived from the file path stem.
        expected: String,
    },
    /// The meta-schema itself failed to compile.
    #[error("meta-schema is invalid: {0}")]
    BadMetaSchema(String),
    /// A filesystem I/O error occurred while reading a preset directory.
    #[error("I/O error reading preset directory: {0}")]
    Io(
        #[from]
        #[cfg_attr(alef, alef(skip))]
        std::io::Error,
    ),
}

/// Compiled meta-schema validator over `preset.schema.json`.
pub struct MetaSchema {
    validator: Validator,
}

impl MetaSchema {
    /// Compile the given JSON text as a Draft 2020-12 meta-schema.
    pub fn compile(meta_schema_json: &str) -> Result<Self, LoadError> {
        let schema: serde_json::Value =
            serde_json::from_str(meta_schema_json).map_err(|e| LoadError::BadMetaSchema(format!("parse: {e}")))?;
        let validator =
            jsonschema::draft202012::new(&schema).map_err(|e| LoadError::BadMetaSchema(format!("compile: {e}")))?;
        Ok(Self { validator })
    }

    /// Validate `raw` against the meta-schema and deserialize into a [`Preset`],
    /// stamping the fingerprint over the canonical file bytes.
    pub fn parse_preset(&self, path: &str, raw: &[u8]) -> Result<Preset, LoadError> {
        let value: serde_json::Value = serde_json::from_slice(raw).map_err(|source| LoadError::Parse {
            path: path.to_string(),
            source,
        })?;
        let issues: Vec<String> = self
            .validator
            .iter_errors(&value)
            .map(|e| format!("- {} at {}", e, e.instance_path()))
            .collect();
        if !issues.is_empty() {
            return Err(LoadError::SchemaValidation {
                path: path.to_string(),
                errors: issues.join("\n"),
            });
        }
        let mut preset: Preset = serde_json::from_value(value).map_err(|source| LoadError::Deserialize {
            path: path.to_string(),
            source,
        })?;
        preset.fingerprint = fingerprint(raw);
        Ok(preset)
    }
}

/// Stable sha256 fingerprint of `raw`, formatted as `sha256:<hex>`.
pub fn fingerprint(raw: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(7 + digest.len() * 2);
    hex.push_str("sha256:");
    for byte in digest.iter() {
        use std::fmt::Write;
        let _ = write!(&mut hex, "{byte:02x}");
    }
    hex
}
