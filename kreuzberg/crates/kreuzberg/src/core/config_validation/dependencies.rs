//! Cross-section dependency validation.
//!
//! This module contains validation functions that check dependencies and relationships
//! between different configuration sections. These validators ensure that related
//! configuration values are consistent and compatible with each other.

use crate::{KreuzbergError, Result};

/// Validate a port number for server configuration.
///
/// Port must be in the range 1-65535. While ports 1-1023 are privileged and may require
/// special permissions on some systems, they are still valid port numbers.
///
/// # Arguments
///
/// * `port` - The port number to validate
///
/// # Returns
///
/// `Ok(())` if the port is valid, or a `ValidationError` with details about valid ranges.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_port;
///
/// assert!(validate_port(8000).is_ok());
/// assert!(validate_port(80).is_ok());
/// assert!(validate_port(1).is_ok());
/// assert!(validate_port(65535).is_ok());
/// assert!(validate_port(0).is_err());
/// ```
pub fn validate_port(port: u16) -> Result<()> {
    if port > 0 {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!("Port must be 1-65535, got {}", port),
            source: None,
        })
    }
}

/// Validate a host/IP address string for server configuration.
///
/// Accepts valid IPv4 addresses (e.g., "127.0.0.1", "0.0.0.0"), valid IPv6 addresses
/// (e.g., "::1", "::"), and hostnames (e.g., "localhost", "example.com").
///
/// # Arguments
///
/// * `host` - The host/IP address string to validate
///
/// # Returns
///
/// `Ok(())` if the host is valid, or a `ValidationError` with details about valid formats.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_host;
///
/// assert!(validate_host("127.0.0.1").is_ok());
/// assert!(validate_host("0.0.0.0").is_ok());
/// assert!(validate_host("::1").is_ok());
/// assert!(validate_host("::").is_ok());
/// assert!(validate_host("localhost").is_ok());
/// assert!(validate_host("example.com").is_ok());
/// assert!(validate_host("").is_err());
/// ```
pub fn validate_host(host: &str) -> Result<()> {
    let host = host.trim();

    if host.is_empty() {
        return Err(KreuzbergError::Validation {
            message: "Invalid host '': must be a valid IP address or hostname".to_string(),
            source: None,
        });
    }

    // Check if it's a valid IPv4 address
    if host.parse::<std::net::Ipv4Addr>().is_ok() {
        return Ok(());
    }

    // Check if it's a valid IPv6 address
    if host.parse::<std::net::Ipv6Addr>().is_ok() {
        return Ok(());
    }

    // Check if it's a valid hostname (basic validation)
    // Hostnames must contain only alphanumeric characters, dots, and hyphens
    // Must not look like an invalid IPv4 address (all numeric with dots)
    let looks_like_ipv4 = host
        .split('.')
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_numeric()));
    if !looks_like_ipv4
        && host.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
        && !host.starts_with('-')
        && !host.ends_with('-')
    {
        return Ok(());
    }

    Err(KreuzbergError::Validation {
        message: format!("Invalid host '{}': must be a valid IP address or hostname", host),
        source: None,
    })
}

/// Validate a CORS (Cross-Origin Resource Sharing) origin URL.
///
/// Accepts valid HTTP/HTTPS URLs (e.g., "https://example.com") or the wildcard "*"
/// to allow all origins. URLs must start with "http://" or "https://", or be exactly "*".
///
/// # Arguments
///
/// * `origin` - The CORS origin URL to validate
///
/// # Returns
///
/// `Ok(())` if the origin is valid, or a `ValidationError` with details about valid formats.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_cors_origin;
///
/// assert!(validate_cors_origin("https://example.com").is_ok());
/// assert!(validate_cors_origin("http://localhost:3000").is_ok());
/// assert!(validate_cors_origin("*").is_ok());
/// assert!(validate_cors_origin("not-a-url").is_err());
/// assert!(validate_cors_origin("ftp://example.com").is_err());
/// ```
pub fn validate_cors_origin(origin: &str) -> Result<()> {
    let origin = origin.trim();

    if origin == "*" {
        return Ok(());
    }

    if origin.starts_with("http://") || origin.starts_with("https://") {
        // Basic validation: ensure there's something after the protocol
        if origin.len() > 8 && (origin.starts_with("http://") && origin.len() > 7 || origin.starts_with("https://")) {
            return Ok(());
        }
    }

    Err(KreuzbergError::Validation {
        message: format!(
            "Invalid CORS origin '{}': must be a valid HTTP/HTTPS URL or '*'",
            origin
        ),
        source: None,
    })
}

/// Validate an upload size limit for server configuration.
///
/// Upload size must be greater than 0 (measured in bytes).
///
/// # Arguments
///
/// * `size` - The maximum upload size in bytes to validate
///
/// # Returns
///
/// `Ok(())` if the size is valid, or a `ValidationError` with details about constraints.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::core::config_validation::validate_upload_size;
///
/// assert!(validate_upload_size(1024).is_ok());
/// assert!(validate_upload_size(1_000_000).is_ok());
/// assert!(validate_upload_size(0).is_err());
/// ```
pub fn validate_upload_size(size: usize) -> Result<()> {
    if size > 0 {
        Ok(())
    } else {
        Err(KreuzbergError::Validation {
            message: format!("Upload size must be greater than 0, got {}", size),
            source: None,
        })
    }
}
