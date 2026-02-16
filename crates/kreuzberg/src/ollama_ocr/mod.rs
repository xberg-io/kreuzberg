//! Ollama OCR backend using vision models.
//!
//! Sends images to a local or remote Ollama instance via the `/api/chat` endpoint
//! and extracts text from the model's response. Works out of the box with any
//! Ollama-hosted vision model (e.g., `glm-ocr`, `llava`, `moondream`).
//!
//! # Quick Start
//!
//! Enable the `ollama-ocr` feature, then configure:
//!
//! ```toml
//! [dependencies]
//! kreuzberg = { version = "4.3", features = ["ollama-ocr"] }
//! ```
//!
//! The backend auto-registers as `"ollama"` and connects to `localhost:11434` by default.
//!
//! # Custom Configuration
//!
//! ```rust,no_run
//! use kreuzberg::ollama_ocr::OllamaOcrBackend;
//! use kreuzberg::plugins::register_ocr_backend;
//! use std::sync::Arc;
//!
//! let backend = OllamaOcrBackend::builder()
//!     .endpoint("https://ollama.example.com")
//!     .model("llava")
//!     .prompt("Extract all text from this image verbatim.")
//!     .build();
//!
//! register_ocr_backend(Arc::new(backend)).unwrap();
//! ```

mod backend;

pub use backend::{OllamaOcrBackend, OllamaOcrBuilder};
