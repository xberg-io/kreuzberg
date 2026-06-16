# Audio and Video Transcription

The `transcription` Cargo feature adds speech-to-text extraction for audio and
video MIME types via Whisper ONNX models.  Enable the feature and set a
`TranscriptionConfig` block in your `ExtractionConfig` to produce transcripts
from audio and video files.

## Supported MIME types

| MIME type    | Container            |
|--------------|----------------------|
| `audio/mpeg` | MP3                  |
| `audio/mp4`  | M4A / AAC in MP4     |
| `audio/wav`  | WAV / RIFF           |
| `audio/webm` | WebM audio           |
| `video/mp4`  | MP4 video (audio track only) |
| `video/webm` | WebM video (audio track only) |

## Model sizes

| Variant    | Download size | RAM at inference | Mel bins |
|------------|---------------|------------------|----------|
| `Tiny`     | ~75 MB        | ~273 MB          | 80       |
| `Base`     | ~145 MB       | ~390 MB          | 80       |
| `Small`    | ~466 MB       | ~967 MB          | 80       |
| `Medium`   | ~1.5 GB       | ~2.0 GB          | 80       |
| `LargeV3`  | ~3.1 GB       | ~3.9 GB          | 128      |

Models are downloaded from `onnx-community/whisper-{size}` on HuggingFace Hub
on first use and cached under `{KREUZBERG_CACHE_DIR}/whisper/{size}/`.

## Configuration knobs

| Field              | Type              | Default  | Description |
|--------------------|-------------------|----------|-------------|
| `enabled`          | `bool`            | `false`  | Must be `true` for the extractor to activate. |
| `model`            | `WhisperModel`    | `Tiny`   | Size variant to use. |
| `language`         | `Option<String>`  | `None`   | ISO-639-1 code (e.g. `"en"`, `"de"`). `None` defaults to English. |
| `timestamps`       | `bool`            | `false`  | Accepted for forward-compatibility; segment timestamps are not yet emitted. |
| `max_bytes`        | `Option<u64>`     | `None`   | Reject input larger than this many bytes before decoding. |
| `max_duration_ms`  | `Option<u64>`     | `None`   | Reject audio longer than this many milliseconds after decode. |
| `timeout_ms`       | `Option<u64>`     | `None`   | Wall-clock timeout for the full inference call (not yet enforced; reserved). |
| `model_cache_dir`  | `Option<PathBuf>` | `None`   | Override the default cache location. |
| `allow_network`    | `bool`            | `true`   | Set to `false` to disable automatic downloads; returns `ModelMissing` if the model is not already cached. |
| `verify_hash`      | `bool`            | `false`  | Hash verification is reserved for a future work item; currently a no-op with a warning. |

## First-run download

On the first call with `allow_network = true`, the extractor downloads the
required ONNX files and tokenizer from HuggingFace Hub.  The download is
serialised per process via a cross-process advisory file lock so concurrent
first-time callers do not race.  Subsequent calls use the local cache.

Set `allow_network = false` and pre-populate the cache directory if you need
air-gapped deployments.  When the model is absent and `allow_network = false`,
extraction returns a `KreuzbergError::Transcription` with the message
`"network access disabled and model not cached"`.

## Usage

Add the feature to `Cargo.toml`:

```toml
kreuzberg = { version = "5", features = ["transcription"] }
```

### Async

```rust
use kreuzberg::extract_bytes;
use kreuzberg::core::config::ExtractionConfig;
use kreuzberg::core::config::transcription::{TranscriptionConfig, WhisperModel};

let config = ExtractionConfig {
    transcription: Some(TranscriptionConfig {
        enabled: true,
        model: WhisperModel::Tiny,
        language: Some("en".to_string()),
        ..Default::default()
    }),
    ..Default::default()
};

let bytes = std::fs::read("recording.wav")?;
let result = extract_bytes(&bytes, "audio/wav", &config).await?;
println!("{}", result.content); // transcript
```

### Sync

```rust
use kreuzberg::extract_bytes_sync;
use kreuzberg::core::config::ExtractionConfig;
use kreuzberg::core::config::transcription::{TranscriptionConfig, WhisperModel};

let config = ExtractionConfig {
    transcription: Some(TranscriptionConfig {
        enabled: true,
        model: WhisperModel::Tiny,
        ..Default::default()
    }),
    ..Default::default()
};

let bytes = std::fs::read("recording.mp3")?;
let result = extract_bytes_sync(&bytes, "audio/mpeg", &config)?;
println!("{}", result.content);
```

## Notes

- Audio longer than 30 seconds is split into 30-second chunks; each chunk is
  transcribed independently and the results are joined with a space.
- The extractor always resamples to 16 kHz mono before inference; source sample
  rate and channel layout are handled automatically.
- Engine instances are cached per process keyed by model paths, so the ONNX
  sessions are loaded once and reused across calls.
- Concurrent inference calls are bounded by a semaphore sized to
  `resolve_thread_budget`, matching the same limit used by the embedding and
  reranking pipelines.
