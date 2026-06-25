//! Integration tests for the Whisper ONNX inference engine.
//!
//! All tests require the `transcription` feature.  The first test that runs
//! will download the Tiny model (~150 MB) from HuggingFace Hub if not already
//! cached; subsequent runs use the local cache.
//!
//! Run all tests in this file with:
//!
//! ```text
//! cargo test -p xberg --features transcription --test whisper_inference
//! ```

#![cfg(feature = "transcription")]

mod helpers;

use xberg::core::config::transcription::WhisperModel;
use xberg::transcription::decode::decode_audio_to_pcm;
use xberg::transcription::engine::WhisperEngine;
use xberg::transcription::model::ensure_whisper_model;

/// Download (or locate the cached) Tiny model and build a `WhisperEngine`.
///
/// Called from multiple tests; model download is gated behind `allow_network = true`
/// so the first cold run fetches from HF Hub.
fn load_tiny() -> WhisperEngine {
    let paths = ensure_whisper_model(WhisperModel::Tiny, None, true, false).expect("Tiny model download/cache failed");
    WhisperEngine::load(&paths).expect("WhisperEngine::load failed")
}

#[test]
fn engine_load_resolves_special_tokens() {
    // If this test passes, the tokenizer found all required special tokens.
    let _engine = load_tiny();
}

#[test]
fn transcribes_hello_world_wav() {
    let path = helpers::get_test_file_path("audio/hello-world.wav");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/hello-world.wav");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    let engine = load_tiny();
    let text = engine.transcribe(&pcm, None, false).expect("transcribe failed");
    assert!(
        text.to_lowercase().contains("hello"),
        "expected 'hello' in transcript, got: {text:?}"
    );
}

#[test]
fn transcribes_hello_world_with_explicit_english() {
    let path = helpers::get_test_file_path("audio/hello-world.wav");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/hello-world.wav");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    let engine = load_tiny();
    let text = engine.transcribe(&pcm, Some("en"), false).expect("transcribe failed");
    assert!(
        text.to_lowercase().contains("hello"),
        "expected 'hello' in transcript with explicit lang, got: {text:?}"
    );
}

#[test]
fn transcribes_mp3_variant() {
    let path = helpers::get_test_file_path("audio/hello-world.mp3");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/hello-world.mp3");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    let engine = load_tiny();
    let text = engine.transcribe(&pcm, None, false).expect("transcribe failed");
    assert!(
        text.to_lowercase().contains("hello"),
        "expected 'hello' in transcript from mp3, got: {text:?}"
    );
}

#[test]
fn chunks_audio_longer_than_30s() {
    let path = helpers::get_test_file_path("audio/long-35s.wav");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/long-35s.wav");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    // Sanity: the fixture is silence, so we just verify no panic and a valid string.
    assert!(pcm.samples.len() > 480_000, "fixture must be > 30 s");
    let engine = load_tiny();
    let text = engine.transcribe(&pcm, None, false).expect("transcribe must not error");
    // Silence typically produces an empty or near-empty transcript.
    let _ = text; // result consumed without assertion on content
}

#[test]
fn empty_pcm_returns_empty_string() {
    use xberg::transcription::decode::PcmAudio;
    let engine = load_tiny();
    let pcm = PcmAudio {
        samples: Vec::new(),
        sample_rate_hz: 16_000,
        channels: 1,
        duration_ms: 0,
    };
    let text = engine.transcribe(&pcm, None, false).expect("empty PCM must not error");
    assert!(
        text.is_empty(),
        "empty PCM should produce empty transcript, got: {text:?}"
    );
}
