//! Integration tests for the transcription extractor end-to-end path.
//!
//! Each test exercises `extract_bytes_document` / `extract_bytes_document_blocking` with a real
//! audio fixture and asserts that the returned `ExtractedDocument.content`
//! contains the expected spoken words.  The Whisper Tiny model is downloaded
//! from HuggingFace Hub on first run (~150 MB) and cached under the default
//! xberg cache directory for all subsequent runs.
//!
//! Run with:
//!
//! ```text
//! cargo test -p xberg --features transcription --test transcription_extractor
//! ```

#![cfg(feature = "transcription")]

mod helpers;
use helpers::{extract_bytes_document, extract_bytes_document_blocking};

use xberg::core::config::ExtractionConfig;
use xberg::core::config::transcription::{TranscriptionConfig, WhisperModel};

fn config_with_transcription() -> ExtractionConfig {
    ExtractionConfig {
        transcription: Some(TranscriptionConfig {
            enabled: true,
            model: WhisperModel::Tiny,
            language: None,
            timestamps: false,
            max_duration_ms: None,
            max_bytes: None,
            timeout_ms: None,
            model_cache_dir: None,
            allow_network: true,
            verify_hash: false,
        }),
        ..Default::default()
    }
}

#[tokio::test]
async fn async_extract_audio_wav_returns_transcript() {
    let bytes = std::fs::read(helpers::get_test_file_path("audio/hello-world.wav")).expect("fixture");
    let config = config_with_transcription();
    let result = extract_bytes_document(&bytes, "audio/wav", &config)
        .await
        .expect("extract");
    assert!(
        result.content.to_lowercase().contains("hello"),
        "expected 'hello' in transcript, got: {:?}",
        result.content
    );
}

#[test]
fn sync_extract_audio_wav_returns_transcript() {
    let bytes = std::fs::read(helpers::get_test_file_path("audio/hello-world.wav")).expect("fixture");
    let config = config_with_transcription();
    let result = extract_bytes_document_blocking(&bytes, "audio/wav", &config).expect("extract");
    assert!(
        result.content.to_lowercase().contains("hello"),
        "expected 'hello' in transcript, got: {:?}",
        result.content
    );
}

#[tokio::test]
async fn async_extract_audio_mp3_returns_transcript() {
    let bytes = std::fs::read(helpers::get_test_file_path("audio/hello-world.mp3")).expect("fixture");
    let config = config_with_transcription();
    let result = extract_bytes_document(&bytes, "audio/mpeg", &config)
        .await
        .expect("extract");
    assert!(
        result.content.to_lowercase().contains("hello"),
        "expected 'hello' in transcript from mp3, got: {:?}",
        result.content
    );
}

#[tokio::test]
async fn async_extract_no_transcription_config_returns_error() {
    let bytes = std::fs::read(helpers::get_test_file_path("audio/hello-world.wav")).expect("fixture");
    let config = ExtractionConfig::default(); // no transcription block
    let result = extract_bytes_document(&bytes, "audio/wav", &config).await;
    assert!(result.is_err(), "expected error with no transcription config");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("config") || msg.contains("disabled") || msg.contains("enabled"),
        "unexpected error: {msg}"
    );
}
