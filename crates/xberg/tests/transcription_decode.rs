//! Integration tests for the symphonia-based audio decoder.
//!
//! Run with:
//!
//! ```text
//! cargo test -p xberg --features transcription --test transcription_decode
//! ```

#![cfg(feature = "transcription")]

mod helpers;

use xberg::transcription::decode::decode_audio_to_pcm;

#[test]
fn decode_16khz_mono_wav_returns_exact_sample_count() {
    let path = helpers::get_test_file_path("audio/silence-1s.wav");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/silence-1s.wav");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    assert_eq!(pcm.sample_rate_hz, 16_000);
    assert_eq!(pcm.channels, 1);
    assert_eq!(pcm.samples.len(), 16_000);
    assert_eq!(pcm.duration_ms, 1_000);
}

#[test]
fn decode_stereo_44k1_downmixes_and_resamples() {
    let path = helpers::get_test_file_path("audio/stereo-44k1-1s.wav");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/stereo-44k1-1s.wav");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    assert_eq!(pcm.sample_rate_hz, 16_000);
    assert_eq!(pcm.channels, 1);
    // Linear resample of 44100 → 16000: expect ~16000 samples (±2% tolerance).
    assert!(
        (15_900..=16_100).contains(&pcm.samples.len()),
        "expected ~16k samples after resample, got {}",
        pcm.samples.len()
    );
}

#[test]
fn decode_mp3_yields_16khz_mono_pcm() {
    let path = helpers::get_test_file_path("audio/hello-world.mp3");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/hello-world.mp3");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode failed");
    assert_eq!(pcm.sample_rate_hz, 16_000);
    assert_eq!(pcm.channels, 1);
    assert!(!pcm.samples.is_empty());
    // The fixture is recorded speech ("hello world…") — non-zero amplitude expected.
    assert!(
        pcm.samples.iter().any(|s| s.abs() > 0.01),
        "audio fixture should have non-trivial signal"
    );
}

#[test]
fn decode_long_wav_returns_full_duration() {
    let path = helpers::get_test_file_path("audio/long-35s.wav");
    let bytes = std::fs::read(&path).expect("fixture missing: audio/long-35s.wav");
    let pcm = decode_audio_to_pcm(&bytes, None).expect("decode itself should succeed");
    // 35 s of silence at 16 kHz → 560 000 samples; allow ±1 s tolerance.
    assert!(
        (34_500..=35_500).contains(&pcm.duration_ms),
        "expected ~35 000 ms, got {} ms",
        pcm.duration_ms
    );
    assert!(
        (34_500 * 16..=35_500 * 16).contains(&pcm.samples.len()),
        "expected ~560 000 samples, got {}",
        pcm.samples.len()
    );
}

#[test]
fn decode_rejects_oversize_bytes_before_decode() {
    let err = decode_audio_to_pcm(&[0u8; 20], Some(10));
    assert!(err.is_err(), "expected error for oversize input");
    let msg = err.unwrap_err().to_string();
    assert!(
        msg.contains("exceed") || msg.contains("limit"),
        "unexpected error message: {msg}"
    );
}
