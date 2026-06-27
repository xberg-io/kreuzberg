//! Redaction & anonymisation engine.
//!
//! The engine is invoked from the Late-stage post-processor at
//! [`crate::plugins::processor::builtin::redaction`]. It runs the pure-Rust
//! pattern engine (and optionally a NER backend for PERSON / ORGANIZATION /
//! LOCATION) over [`ExtractedDocument::content`](crate::ExtractedDocument::content)
//! and rewrites every textual field in place. The original text is dropped at
//! the end of the pipeline; the audit trail lives in
//! [`ExtractedDocument::redaction_report`](crate::ExtractedDocument::redaction_report).

pub mod engine;
pub mod patterns;
pub mod strategy;

pub use engine::redact;
