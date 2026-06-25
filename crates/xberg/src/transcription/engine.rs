//! Whisper ONNX inference engine.
//!
//! Loads three ONNX sessions (encoder, decoder, decoder_with_past) and runs
//! autoregressive greedy decoding to produce a transcript string from 16 kHz
//! mono f32 PCM audio.
//!
//! # Thread safety
//!
//! `WhisperEngine` is `Send + Sync` — the `ort::Session::run()` API takes
//! `&mut self` as an API-level constraint but its implementation delegates to
//! `run_inner(&self)`, which is thread-safe per the ONNX Runtime documentation.
//! We use the same `&self`-cast pattern established in `reranking/engine.rs`.
//!
//! # Architecture
//!
//! For each 30-second audio chunk the engine:
//! 1. Computes a log-mel spectrogram (shape `[1, n_mels, 3000]`) using `mel_spec`.
//! 2. Runs the encoder to obtain cross-attention key-value states.
//! 3. Seeds the decoder with a four-token prompt
//!    `[<|startoftranscript|>, <|{lang}|>, <|transcribe|>, <|notimestamps|>]`.
//! 4. Greedily generates tokens by running `decoder` (step 0) and then
//!    `decoder_with_past` (steps 1…N), accumulating KV-cache tensors.
//! 5. Stops on `<|endoftext|>` or a configurable max-token limit (448).
//! 6. Decodes the token IDs back to UTF-8 text via the HuggingFace tokenizer.

use std::collections::HashMap;

use mel_spec::mel::{BatchLogMelConfig, BatchLogMelSpectrogram};
use ndarray::{Array2, Array3};
use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel;
use ort::value::Value;
use thiserror::Error;
use tokenizers::Tokenizer;

use crate::transcription::decode::PcmAudio;
use crate::transcription::model::WhisperModelPaths;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Whisper operates on 30-second windows at 16 kHz.
const WHISPER_SAMPLE_RATE: usize = 16_000;
/// 30-second window in samples.
const WHISPER_CHUNK_SAMPLES: usize = WHISPER_SAMPLE_RATE * 30;
/// Number of STFT frames in a 30-second window (480000 / 160).
const WHISPER_N_FRAMES: usize = 3_000;
/// Whisper STFT n_fft.
const WHISPER_N_FFT: usize = 400;
/// Whisper STFT hop length.
const WHISPER_HOP_LENGTH: usize = 160;
/// Maximum number of output tokens produced per chunk (Whisper canonical).
const WHISPER_MAX_TOKENS: usize = 448;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors that can occur during Whisper inference.
#[derive(Debug, Error)]
#[cfg_attr(alef, alef(skip))]
pub enum TranscriptionError {
    /// ONNX Runtime returned an error during session build or inference.
    #[error("ONNX Runtime error: {0}")]
    Ort(#[from] ort::Error),
    /// Tokenizer load or decode failed.
    #[error("tokenizer error: {0}")]
    Tokenizer(String),
    /// A tensor shape was not as expected.
    #[error("tensor shape error: {0}")]
    Shape(String),
    /// An I/O error occurred (model file missing, etc.).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// The encoder produced no output.
    #[error("model produced no output")]
    NoOutput,
    /// A required special token was missing from the tokenizer vocabulary.
    #[error("special token not found in tokenizer: {0}")]
    MissingSpecialToken(String),
    /// The mel spectrogram computation failed.
    #[error("mel spectrogram error: {0}")]
    MelSpec(String),
}

// ---------------------------------------------------------------------------
// Special-token IDs resolved once at load time
// ---------------------------------------------------------------------------

/// Token IDs for the four-token Whisper decode prompt.
#[derive(Debug, Clone)]
struct SpecialTokens {
    /// `<|startoftranscript|>`
    start_of_transcript: u32,
    /// `<|endoftext|>`
    end_of_text: u32,
    /// `<|transcribe|>`
    transcribe: u32,
    /// `<|notimestamps|>`
    no_timestamps: u32,
    /// Language token IDs, keyed by ISO-639-1 code (e.g. `"en"` → token id).
    language_ids: HashMap<String, u32>,
}

impl SpecialTokens {
    /// Resolve all special tokens from the tokenizer.
    ///
    /// Language codes follow Whisper's naming convention: `<|en|>`, `<|de|>`, …
    fn resolve(tokenizer: &Tokenizer) -> Result<Self, TranscriptionError> {
        let resolve = |token: &str| -> Result<u32, TranscriptionError> {
            tokenizer
                .token_to_id(token)
                .ok_or_else(|| TranscriptionError::MissingSpecialToken(token.to_string()))
        };

        let start_of_transcript = resolve("<|startoftranscript|>")?;
        let end_of_text = resolve("<|endoftext|>")?;
        let transcribe = resolve("<|transcribe|>")?;
        let no_timestamps = resolve("<|notimestamps|>")?;

        // Build a language→id map by probing a curated list of ISO-639-1 codes.
        // All multilingual Whisper models have `<|en|>`, `<|de|>`, etc. in their
        // vocabulary. We resolve any code that the tokenizer actually knows.
        let language_codes = [
            "af", "am", "ar", "as", "az", "ba", "be", "bg", "bn", "bo", "br", "bs", "ca", "cs", "cy", "da", "de", "el",
            "en", "es", "et", "eu", "fa", "fi", "fo", "fr", "gl", "gu", "ha", "haw", "he", "hi", "hr", "ht", "hu",
            "hy", "id", "is", "it", "ja", "jw", "ka", "kk", "km", "kn", "ko", "la", "lb", "lo", "lt", "lv", "mg", "mi",
            "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl", "nn", "no", "oc", "pa", "pl", "ps", "pt", "ro", "ru",
            "sa", "sd", "si", "sk", "sl", "sn", "so", "sq", "sr", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl",
            "tr", "tt", "uk", "ur", "uz", "vi", "yi", "yo", "zh",
        ];

        let mut language_ids = HashMap::new();
        for code in language_codes {
            let token = format!("<|{code}|>");
            if let Some(id) = tokenizer.token_to_id(&token) {
                language_ids.insert(code.to_string(), id);
            }
        }

        tracing::debug!(
            start_of_transcript,
            end_of_text,
            transcribe,
            no_timestamps,
            language_count = language_ids.len(),
            "Resolved Whisper special tokens",
        );

        Ok(Self {
            start_of_transcript,
            end_of_text,
            transcribe,
            no_timestamps,
            language_ids,
        })
    }

    /// Look up the language token ID for `lang` (e.g. `"en"`).
    ///
    /// Falls back to English when the language code is unrecognised.
    fn language_id(&self, lang: &str) -> u32 {
        if let Some(&id) = self.language_ids.get(lang) {
            return id;
        }
        tracing::warn!(language = lang, "Unknown language code; falling back to English",);
        *self.language_ids.get("en").unwrap_or(&self.start_of_transcript)
    }
}

// ---------------------------------------------------------------------------
// Session builder helpers
// ---------------------------------------------------------------------------

/// Build an ONNX Runtime session from a model file path.
///
/// Uses the same builder configuration as `reranking/mod.rs`:
/// all-graph optimization, intra-thread budget from the concurrency
/// resolver, and the bundled ORT execution providers.
fn build_session(path: &std::path::Path) -> Result<Session, TranscriptionError> {
    crate::ort_discovery::ensure_ort_available();
    let thread_budget = crate::core::config::concurrency::resolve_thread_budget(None);

    let mut builder = Session::builder()?;
    builder = builder
        .with_optimization_level(GraphOptimizationLevel::All)
        .map_err(|e| ort::Error::new(e.message()))?;
    builder = builder
        .with_intra_threads(thread_budget)
        .map_err(|e| ort::Error::new(e.message()))?;
    builder = builder
        .with_inter_threads(1)
        .map_err(|e| ort::Error::new(e.message()))?;
    builder = crate::ort_discovery::apply_execution_providers(builder, None)?;
    let session = builder.commit_from_file(path)?;
    Ok(session)
}

// ---------------------------------------------------------------------------
// WhisperEngine
// ---------------------------------------------------------------------------

/// Whisper ONNX inference engine.
///
/// Holds three sessions (encoder, decoder, decoder_with_past) and a tokenizer.
/// Call [`WhisperEngine::transcribe`] to produce a transcript from PCM audio.
#[cfg_attr(alef, alef(skip))]
pub struct WhisperEngine {
    encoder: Session,
    decoder: Session,
    decoder_with_past: Session,
    tokenizer: Tokenizer,
    special_tokens: SpecialTokens,
    mel_frontend: BatchLogMelSpectrogram,
    n_mels: u32,
}

// SAFETY: WhisperEngine is Send + Sync because:
// 1. Tokenizer is Send + Sync (confirmed in the tokenizers crate).
// 2. Session::run() is thread-safe per the ONNX Runtime C API documentation;
//    its &mut self signature is an API artifact — run_inner(&self) does the work.
// 3. BatchLogMelSpectrogram is Send + Sync (all fields are Send + Sync).
// 4. All other fields are immutable after construction.
#[allow(unsafe_code)]
unsafe impl Send for WhisperEngine {}
#[allow(unsafe_code)]
unsafe impl Sync for WhisperEngine {}

impl WhisperEngine {
    /// Load a Whisper engine from the given model paths.
    ///
    /// Builds three ONNX sessions and resolves special-token IDs from the
    /// bundled tokenizer. This is a blocking, CPU-heavy operation — callers
    /// on an async runtime should wrap it in `tokio::task::spawn_blocking`.
    pub fn load(paths: &WhisperModelPaths) -> Result<Self, TranscriptionError> {
        tracing::debug!(
            encoder = ?paths.encoder,
            decoder = ?paths.decoder,
            decoder_with_past = ?paths.decoder_with_past,
            n_mels = paths.n_mels,
            "Loading WhisperEngine sessions",
        );

        let encoder = build_session(&paths.encoder)?;
        let decoder = build_session(&paths.decoder)?;
        // For non-sharded models (Tiny, Base) decoder_with_past is a different
        // file.  For sharded models (Small+) it points to the same merged file
        // but is still loaded as an independent Session object so that both
        // I/O name sets can be queried independently.
        let decoder_with_past = build_session(&paths.decoder_with_past)?;

        // Log I/O names discovered from the live sessions — useful for debugging
        // mismatches between model revisions.
        tracing::debug!(
            inputs = ?encoder.inputs().iter().map(|i| i.name().to_string()).collect::<Vec<_>>(),
            outputs = ?encoder.outputs().iter().map(|o| o.name().to_string()).collect::<Vec<_>>(),
            "Encoder session I/O",
        );
        tracing::debug!(
            inputs = ?decoder.inputs().iter().map(|i| i.name().to_string()).collect::<Vec<_>>(),
            outputs = ?decoder.outputs().iter().map(|o| o.name().to_string()).collect::<Vec<_>>(),
            "Decoder (no past) session I/O",
        );
        tracing::debug!(
            inputs = ?decoder_with_past.inputs().iter().map(|i| i.name().to_string()).collect::<Vec<_>>(),
            outputs = ?decoder_with_past.outputs().iter().map(|o| o.name().to_string()).collect::<Vec<_>>(),
            "Decoder (with past) session I/O",
        );

        let tokenizer =
            Tokenizer::from_file(&paths.tokenizer).map_err(|e| TranscriptionError::Tokenizer(e.to_string()))?;

        let special_tokens = SpecialTokens::resolve(&tokenizer)?;

        let mel_frontend = BatchLogMelSpectrogram::new(BatchLogMelConfig {
            sample_rate: WHISPER_SAMPLE_RATE,
            n_fft: WHISPER_N_FFT,
            win_length: WHISPER_N_FFT,
            hop_length: WHISPER_HOP_LENGTH,
            n_mels: paths.n_mels as usize,
            f_min: 0.0,
            f_max: None,
            htk: false,
            norm: true,
            preemphasis: 0.0,
            center: true,
            log_zero_guard: 1e-10_f32,
            pad_to: 0,
            normalize_per_feature: false,
        })
        .map_err(|e| TranscriptionError::MelSpec(e.to_string()))?;

        Ok(Self {
            encoder,
            decoder,
            decoder_with_past,
            tokenizer,
            special_tokens,
            mel_frontend,
            n_mels: paths.n_mels,
        })
    }

    /// Transcribe PCM audio to a string.
    ///
    /// The `pcm` input **must** already be 16 kHz mono f32 as produced by
    /// [`crate::transcription::decode::decode_audio_to_pcm`]. Passing audio
    /// at a different sample rate will produce garbage output without an error.
    ///
    /// For audio longer than 30 seconds the input is split into 30-second
    /// chunks; each chunk is transcribed independently and the results are
    /// joined with a single space.
    ///
    /// The `_timestamps` parameter is accepted for API completeness but has
    /// no effect in this implementation — v1 always uses `<|notimestamps|>`.
    pub fn transcribe(
        &self,
        pcm: &PcmAudio,
        language: Option<&str>,
        _timestamps: bool,
    ) -> Result<String, TranscriptionError> {
        if pcm.samples.is_empty() {
            return Ok(String::new());
        }

        let lang = language.unwrap_or("en");

        // Split into 30-second chunks and transcribe each.
        let mut parts: Vec<String> = Vec::new();
        let mut offset = 0_usize;

        loop {
            let remaining = pcm.samples.len() - offset;
            if remaining == 0 {
                break;
            }

            let chunk_end = (offset + WHISPER_CHUNK_SAMPLES).min(pcm.samples.len());
            let chunk = &pcm.samples[offset..chunk_end];

            let text = self.transcribe_chunk(chunk, lang)?;
            if !text.is_empty() {
                parts.push(text);
            }

            offset += WHISPER_CHUNK_SAMPLES;
            if offset >= pcm.samples.len() {
                break;
            }
        }

        Ok(parts.join(" "))
    }

    // -----------------------------------------------------------------------
    // Private implementation
    // -----------------------------------------------------------------------

    /// Transcribe a single chunk of PCM (at most 30 seconds of audio).
    ///
    /// The chunk is zero-padded to exactly [`WHISPER_CHUNK_SAMPLES`] samples
    /// so that the encoder always receives a `[1, n_mels, 3000]` tensor.
    fn transcribe_chunk(&self, chunk: &[f32], lang: &str) -> Result<String, TranscriptionError> {
        // Pad or truncate to exactly 480 000 samples.
        let padded = if chunk.len() == WHISPER_CHUNK_SAMPLES {
            chunk.to_vec()
        } else {
            let mut v = chunk.to_vec();
            v.resize(WHISPER_CHUNK_SAMPLES, 0.0_f32);
            v
        };

        // -----------------------------------------------------------------------
        // Step 1: compute log-mel spectrogram
        // -----------------------------------------------------------------------
        let mel_flat = self.compute_log_mel(&padded)?;

        // -----------------------------------------------------------------------
        // Step 2: encoder pass
        // -----------------------------------------------------------------------
        let encoder_hidden_states = self.run_encoder(mel_flat)?;

        // -----------------------------------------------------------------------
        // Step 3: build initial prompt tokens
        // -----------------------------------------------------------------------
        let lang_id = self.special_tokens.language_id(lang);
        let prompt: Vec<i64> = vec![
            self.special_tokens.start_of_transcript as i64,
            lang_id as i64,
            self.special_tokens.transcribe as i64,
            self.special_tokens.no_timestamps as i64,
        ];

        // -----------------------------------------------------------------------
        // Steps 4–5: greedy decode loop
        // -----------------------------------------------------------------------
        let token_ids = self.greedy_decode(prompt, &encoder_hidden_states)?;

        // -----------------------------------------------------------------------
        // Step 6: decode token IDs to text
        // -----------------------------------------------------------------------
        let text = self
            .tokenizer
            .decode(&token_ids, /* skip_special_tokens */ true)
            .map_err(|e| TranscriptionError::Tokenizer(e.to_string()))?;

        Ok(text.trim().to_string())
    }

    /// Compute the Whisper log-mel spectrogram for a 30-second padded chunk.
    ///
    /// Returns a flat Vec<f32> laid out row-major `[n_mels, 3000]` ready for
    /// wrapping into a `[1, n_mels, 3000]` tensor.
    ///
    /// We use `compute_flat()` (which returns raw `Vec<f32>`) to avoid touching
    /// mel_spec's ndarray 0.16 types — ort requires ndarray 0.17 types.
    fn compute_log_mel(&self, samples: &[f32]) -> Result<Vec<f32>, TranscriptionError> {
        // `BatchLogMelSpectrogram::compute_flat` returns rows (n_mels) × cols (n_frames)
        // laid out row-major in `data`, using natural log internally with a guard.
        // Whisper expects log10; we convert via division by ln(10).
        // We then apply the canonical Whisper normalisation:
        //   log_spec = max(log10_spec, max(log10_spec) - 8)
        //   log_spec = (log_spec + 4) / 4
        const LN10: f32 = std::f32::consts::LN_10;

        let output = self
            .mel_frontend
            .compute_flat(samples)
            .map_err(|e| TranscriptionError::MelSpec(e.to_string()))?;

        let n_mels = output.rows;
        let n_frames = output.cols;
        let mut flat = output.data;

        // flat is row-major: flat[mel * n_frames + frame]

        // Convert ln → log10.
        for v in flat.iter_mut() {
            *v /= LN10;
        }

        // Whisper normalisation across the whole chunk.
        let max_val = flat.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let floor = max_val - 8.0_f32;
        for v in flat.iter_mut() {
            *v = (v.max(floor) + 4.0_f32) / 4.0_f32;
        }

        // Pad or trim to exactly WHISPER_N_FRAMES columns.
        let target_frames = WHISPER_N_FRAMES;
        let log_mel_flat = if n_frames == target_frames {
            flat
        } else if n_frames < target_frames {
            // Pad each mel row with zeros to reach target_frames.
            let mut padded = vec![0.0_f32; n_mels * target_frames];
            for mel_idx in 0..n_mels {
                let src_start = mel_idx * n_frames;
                let dst_start = mel_idx * target_frames;
                padded[dst_start..dst_start + n_frames].copy_from_slice(&flat[src_start..src_start + n_frames]);
            }
            padded
        } else {
            // Trim each mel row to target_frames.
            let mut trimmed = vec![0.0_f32; n_mels * target_frames];
            for mel_idx in 0..n_mels {
                let src_start = mel_idx * n_frames;
                let dst_start = mel_idx * target_frames;
                trimmed[dst_start..dst_start + target_frames]
                    .copy_from_slice(&flat[src_start..src_start + target_frames]);
            }
            trimmed
        };

        Ok(log_mel_flat)
    }

    /// Run the encoder and return the `last_hidden_state` value.
    fn run_encoder(&self, mel_flat: Vec<f32>) -> Result<Value, TranscriptionError> {
        let n_mels = self.n_mels as usize;
        // Build [1, n_mels, WHISPER_N_FRAMES] array using ndarray 0.17 (ort's version).
        let mel_nd = Array3::from_shape_vec((1, n_mels, WHISPER_N_FRAMES), mel_flat)
            .map_err(|e| TranscriptionError::Shape(e.to_string()))?;

        let mel_value: Value = Value::from_array(mel_nd)?.into();

        // SAFETY: ort::Session::run() takes &mut self but delegates to
        // run_inner(&self), which is thread-safe per the ONNX Runtime C API
        // documentation (OrtApi::Run is explicitly documented as thread-safe
        // for concurrent calls on the same session). The *mut cast is sound
        // because run_inner performs no mutation of the Session struct.
        #[allow(unsafe_code)]
        let outputs = unsafe {
            let ptr = &self.encoder as *const Session as *mut Session;
            (*ptr).run(ort::inputs!["input_features" => mel_value])
        }?;

        // Find `last_hidden_state` by name; fall back to the first output if the
        // model revision uses a different name.
        let encoder_output_name = self
            .encoder
            .outputs()
            .first()
            .map(|o| o.name().to_string())
            .unwrap_or_else(|| "last_hidden_state".to_string());

        let hidden: Value = outputs
            .into_iter()
            .find(|(name, _)| *name == "last_hidden_state" || *name == encoder_output_name)
            .map(|(_, v)| v)
            .ok_or(TranscriptionError::NoOutput)?;

        Ok(hidden)
    }

    /// Run the greedy decode loop.
    ///
    /// Returns the token IDs produced **after** the prompt (i.e. the transcribed
    /// tokens), excluding the final `<|endoftext|>` token.
    fn greedy_decode(&self, prompt: Vec<i64>, encoder_hidden_states: &Value) -> Result<Vec<u32>, TranscriptionError> {
        let eot = self.special_tokens.end_of_text;

        // Discover decoder I/O names once from the live sessions.
        let dec_input_names: Vec<String> = self.decoder.inputs().iter().map(|i| i.name().to_string()).collect();
        let dec_output_names: Vec<String> = self.decoder.outputs().iter().map(|o| o.name().to_string()).collect();
        let dwp_input_names: Vec<String> = self
            .decoder_with_past
            .inputs()
            .iter()
            .map(|i| i.name().to_string())
            .collect();
        let dwp_output_names: Vec<String> = self
            .decoder_with_past
            .outputs()
            .iter()
            .map(|o| o.name().to_string())
            .collect();

        tracing::debug!(?dec_input_names, ?dec_output_names, "Decoder I/O names");
        tracing::debug!(?dwp_input_names, ?dwp_output_names, "Decoder-with-past I/O names");

        // Detect the exact name the model uses for encoder hidden states.
        let enc_hs_input_name = dec_input_names
            .iter()
            .find(|n| n.contains("encoder_hidden_states"))
            .cloned()
            .unwrap_or_else(|| "encoder_hidden_states".to_string());

        let logits_output_name = dec_output_names
            .iter()
            .find(|n| *n == "logits")
            .cloned()
            .unwrap_or_else(|| "logits".to_string());

        let dwp_logits_output_name = dwp_output_names
            .iter()
            .find(|n| *n == "logits")
            .cloned()
            .unwrap_or_else(|| "logits".to_string());

        // -----------------------------------------------------------------------
        // Step 0: initial decoder pass (no past key-values)
        // -----------------------------------------------------------------------
        let prompt_len = prompt.len();
        let input_ids_0 =
            Array2::from_shape_vec((1, prompt_len), prompt).map_err(|e| TranscriptionError::Shape(e.to_string()))?;
        let ids_value_0: Value = Value::from_array(input_ids_0)?.into();

        let enc_hs_clone = clone_value_f32(encoder_hidden_states)?;

        let step0_inputs = ort::inputs![
            "input_ids" => ids_value_0,
            &enc_hs_input_name => enc_hs_clone,
        ];

        // SAFETY: see run_encoder for the full SAFETY argument. Same pattern.
        #[allow(unsafe_code)]
        let step0_outputs: ort::session::SessionOutputs = unsafe {
            let ptr = &self.decoder as *const Session as *mut Session;
            (*ptr).run(step0_inputs)
        }?;

        // Extract logits and greedy-argmax the last position.
        let first_token = {
            let logits_val = step0_outputs
                .iter()
                .find(|(name, _)| *name == logits_output_name)
                .map(|(_, v)| v)
                .ok_or(TranscriptionError::NoOutput)?;
            greedy_argmax_last(&logits_val)?
        };

        if first_token == eot {
            return Ok(Vec::new());
        }
        let mut generated: Vec<u32> = vec![first_token];

        // Collect KV-cache outputs from step 0.
        // Present-key-values are named "present.{i}.{decoder|encoder}.{key|value}".
        // The matching decoder_with_past inputs are named
        // "past_key_values.{i}.{decoder|encoder}.{key|value}".
        //
        // Key architectural detail: the DWP session only outputs decoder KV
        // (present.*.decoder.*) — not encoder KV — because encoder cross-attention
        // keys/values are constant across all decode steps. We therefore:
        //   1. Extract encoder KV from step-0 outputs and never update them.
        //   2. Extract decoder KV from step-0 outputs and update them each step.
        //   3. Feed both to every DWP step.
        let step0_non_logits: Vec<(String, Value)> = step0_outputs
            .into_iter()
            .filter(|(name, _)| *name != logits_output_name)
            .map(|(name, val)| {
                let input_name = name.replacen("present", "past_key_values", 1);
                (input_name, val)
            })
            .collect();

        // Separate encoder KV (constant) from decoder KV (updated each step).
        let mut encoder_kvs: Vec<(String, Value)> = Vec::new();
        let mut decoder_kvs: Vec<(String, Value)> = Vec::new();
        for (name, val) in step0_non_logits {
            if name.contains(".encoder.") {
                encoder_kvs.push((name, val));
            } else {
                decoder_kvs.push((name, val));
            }
        }

        // -----------------------------------------------------------------------
        // Steps 1…N: decoder_with_past loop
        // -----------------------------------------------------------------------
        let dwp_wants_enc_hs = dwp_input_names.iter().any(|n| n.contains("encoder_hidden_states"));

        for _ in 1..WHISPER_MAX_TOKENS {
            let last_token = *generated.last().expect("generated is non-empty; qed");

            let last_id_arr = Array2::from_shape_vec((1, 1), vec![last_token as i64])
                .map_err(|e| TranscriptionError::Shape(e.to_string()))?;
            let ids_val: Value = Value::from_array(last_id_arr)?.into();

            let mut dwp_inputs = ort::inputs!["input_ids" => ids_val];

            if dwp_wants_enc_hs {
                let enc_hs_c = clone_value_f32(encoder_hidden_states)?;
                dwp_inputs.push((enc_hs_input_name.as_str().into(), enc_hs_c.into()));
            }

            // Append decoder past key-value tensors (updated at each step).
            for (kv_name, kv_val) in &decoder_kvs {
                let kv_clone = clone_value_f32(kv_val)?;
                dwp_inputs.push((kv_name.as_str().into(), kv_clone.into()));
            }

            // Append encoder past key-value tensors (constant; never change).
            for (kv_name, kv_val) in &encoder_kvs {
                let kv_clone = clone_value_f32(kv_val)?;
                dwp_inputs.push((kv_name.as_str().into(), kv_clone.into()));
            }

            // SAFETY: see run_encoder for the full SAFETY argument. Same pattern.
            #[allow(unsafe_code)]
            let step_outputs: ort::session::SessionOutputs = unsafe {
                let ptr = &self.decoder_with_past as *const Session as *mut Session;
                (*ptr).run(dwp_inputs)
            }?;

            let next_token = {
                let logits_val = step_outputs
                    .iter()
                    .find(|(name, _)| *name == dwp_logits_output_name)
                    .map(|(_, v)| v)
                    .ok_or(TranscriptionError::NoOutput)?;
                greedy_argmax_last(&logits_val)?
            };

            if next_token == eot {
                break;
            }
            generated.push(next_token);

            // Update decoder KV-cache from DWP outputs.
            // DWP outputs only decoder KV (present.*.decoder.*); encoder KV
            // remains unchanged across all steps.
            let new_decoder_kvs: Vec<(String, Value)> = step_outputs
                .into_iter()
                .filter(|(name, _)| *name != dwp_logits_output_name)
                .map(|(name, val)| {
                    let input_name = name.replacen("present", "past_key_values", 1);
                    (input_name, val)
                })
                .collect();

            if !new_decoder_kvs.is_empty() {
                decoder_kvs = new_decoder_kvs;
            }
            // If new_decoder_kvs is empty the merged-decoder caches KV
            // internally — keep the previous decoder_kvs unchanged.
        }

        Ok(generated)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract the token with the highest logit from the last position of a
/// `[batch=1, seq_len, vocab_size]` logits tensor.
///
/// Whisper vocab is ~51865 tokens. A plain argmax over `Vec<f32>` is fast
/// enough; no softmax is required for greedy decoding.
fn greedy_argmax_last(logits: &Value) -> Result<u32, TranscriptionError> {
    let tensor = logits.try_extract_array::<f32>().map_err(TranscriptionError::Ort)?;
    let shape = tensor.shape();
    if shape.len() < 3 {
        return Err(TranscriptionError::Shape(format!(
            "Expected logits tensor rank 3, got rank {}",
            shape.len()
        )));
    }
    let seq_len = shape[shape.len() - 2];
    let vocab_size = shape[shape.len() - 1];
    let last_pos_offset = (seq_len - 1) * vocab_size;

    let flat: Vec<f32> = tensor.iter().cloned().collect();
    if flat.len() < last_pos_offset + vocab_size {
        return Err(TranscriptionError::Shape(format!(
            "Logits flat length {} too short for offset {} + vocab {}",
            flat.len(),
            last_pos_offset,
            vocab_size
        )));
    }
    let last_logits = &flat[last_pos_offset..last_pos_offset + vocab_size];

    let best = last_logits
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(idx, _)| idx as u32)
        .ok_or_else(|| TranscriptionError::Shape("logits slice was empty".to_string()))?;

    Ok(best)
}

/// Clone an `ort::value::Value` by extracting its f32 data and rebuilding a
/// new tensor with the same shape.
///
/// This is necessary because `ort::value::Value` is not `Clone` and ORT
/// session inputs consume values by move.
fn clone_value_f32(value: &Value) -> Result<Value, TranscriptionError> {
    let arr = value.try_extract_array::<f32>().map_err(TranscriptionError::Ort)?;
    let shape: Vec<usize> = arr.shape().to_vec();
    let flat: Vec<f32> = arr.iter().cloned().collect();
    let owned = ndarray::ArrayD::from_shape_vec(ndarray::IxDyn(&shape), flat)
        .map_err(|e| TranscriptionError::Shape(e.to_string()))?;
    let result: Value = Value::from_array(owned)?.into();
    Ok(result)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greedy_argmax_last_picks_highest() {
        // Manually construct a [1, 2, 4] logits tensor.
        let data = Array3::from_shape_vec(
            (1, 2, 4),
            vec![
                // position 0: max at index 1
                0.1_f32, 0.9, 0.2, 0.0, // position 1: max at index 3
                0.1, 0.2, 0.3, 0.8,
            ],
        )
        .unwrap();
        let val: Value = Value::from_array(data).unwrap().into();
        let tok = greedy_argmax_last(&val).unwrap();
        assert_eq!(tok, 3, "expected argmax at index 3 (last position)");
    }

    #[test]
    fn greedy_argmax_last_single_position() {
        let data = Array3::from_shape_vec((1, 1, 5), vec![0.0_f32, 0.0, 100.0, 0.0, 0.0]).unwrap();
        let val: Value = Value::from_array(data).unwrap().into();
        let tok = greedy_argmax_last(&val).unwrap();
        assert_eq!(tok, 2);
    }

    #[test]
    fn special_tokens_resolve_from_tiny_vocab() {
        // This test requires the Tiny model to be cached locally.  If not, skip.
        use crate::core::config::transcription::WhisperModel;
        use crate::transcription::model::ensure_whisper_model;

        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("xberg")
            .join("whisper");

        let paths = match ensure_whisper_model(WhisperModel::Tiny, Some(&cache_dir), false, false) {
            Ok(p) => p,
            Err(_) => return, // model not cached — skip
        };

        let tokenizer = Tokenizer::from_file(&paths.tokenizer).expect("tokenizer load");
        let st = SpecialTokens::resolve(&tokenizer).expect("special token resolution");

        assert!(st.end_of_text > 0, "end_of_text token should be a valid non-zero ID");
        assert!(
            st.language_ids.contains_key("en"),
            "English language token must be present"
        );
    }
}
