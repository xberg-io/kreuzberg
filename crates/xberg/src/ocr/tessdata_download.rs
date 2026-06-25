//! Tessdata language pack download utilities for runtime language pack materialization.
//!
//! Provides functions to download `.traineddata` files from the official Tesseract
//! tessdata_fast repository on GitHub, with fallback URLs for redundancy.

use crate::ocr::error::OcrError;
use std::path::Path;

/// Download a language pack from the tessdata_fast repository.
///
/// Attempts to download `{lang}.traineddata` from GitHub with fallback URLs
/// for redundancy. Skips download if the file already exists. Uses `ureq`
/// with rustls + platform-verifier for HTTPS (OS trust store; respects SSL_CERT_FILE).
///
/// # Arguments
///
/// * `lang` - Language code (e.g., "eng", "fra", "deu")
/// * `output_dir` - Directory to write the traineddata file
///
/// # Returns
///
/// `Ok(())` if download succeeds or file already exists, `Err(OcrError)` on failure.
///
/// # Panics
///
/// Does not panic. All errors are returned as `OcrError`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn download_language_pack(lang: &str, output_dir: &Path) -> Result<(), OcrError> {
    let traineddata_path = output_dir.join(format!("{}.traineddata", lang));

    // Skip if already exists
    if traineddata_path.exists() {
        tracing::debug!(
            "Language pack '{}' already exists at {}",
            lang,
            traineddata_path.display()
        );
        return Ok(());
    }

    let urls = [
        format!(
            "https://github.com/tesseract-ocr/tessdata_fast/raw/main/{}.traineddata",
            lang
        ),
        format!(
            "https://raw.githubusercontent.com/tesseract-ocr/tessdata_fast/main/{}.traineddata",
            lang
        ),
    ];

    for url in &urls {
        match download_file(url, &traineddata_path) {
            Ok(_) => {
                tracing::info!(
                    "Successfully downloaded language pack '{}' to {}",
                    lang,
                    traineddata_path.display()
                );
                return Ok(());
            }
            Err(e) => {
                tracing::warn!("Failed to download from {}: {}", url, e);
                // Try next URL
                continue;
            }
        }
    }

    Err(OcrError::TesseractInitializationFailed(format!(
        "Failed to download language pack '{}' from tessdata_fast repository. \
         Tried URLs: {}. Check your network connection and verify the language code is valid.",
        lang,
        urls.join(", ")
    )))
}

/// Maximum traineddata download size (64 MB). The largest `tessdata_fast`
/// language packs are well under this; it guards against pathological responses.
#[cfg(not(target_arch = "wasm32"))]
const MAX_TRAINEDDATA_BYTES: u64 = 64 * 1024 * 1024;

/// Download a file from a URL and write it atomically to the specified path.
///
/// Writes to a sibling temp file first, then renames into place so an
/// interrupted download never leaves a partial `.traineddata` behind.
#[cfg(not(target_arch = "wasm32"))]
fn download_file(url: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get(url).call()?;

    if response.status() != 200 {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let bytes = response
        .into_body()
        .with_config()
        .limit(MAX_TRAINEDDATA_BYTES)
        .read_to_vec()?;

    let tmp_path = output_path.with_extension("traineddata.partial");
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, output_path)?;

    Ok(())
}
