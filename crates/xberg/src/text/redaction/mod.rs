//! Redaction & anonymisation engine.
//!
//! The engine is invoked from the Late-stage post-processor at
//! [`crate::plugins::processor::builtin::redaction`]. It runs the pure-Rust
//! pattern engine (and optionally a NER backend for PERSON / ORGANIZATION /
//! LOCATION) over [`ExtractionResult::content`](crate::ExtractionResult::content)
//! and rewrites every textual field in place. The original text is dropped at
//! the end of the pipeline; the audit trail lives in
//! [`ExtractionResult::redaction_report`](crate::ExtractionResult::redaction_report).

pub mod engine;
pub mod patterns;
pub mod strategy;

pub use engine::redact;
