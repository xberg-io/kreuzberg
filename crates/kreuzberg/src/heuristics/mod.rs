//! Document-processing heuristics: chunking decisions, confidence scoring, multi-document
//! boundary detection, and structured-extraction call-mode selection.
//!
//! Infrastructure only: the algorithms ship with generic, fully-overridable defaults.
//! Tuned threshold/weight values belong to downstream consumers via config, not here.
//!
//! # Modules
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`config`] | [`HeuristicsConfig`] — all thresholds as public fields + `Default` |
//! | [`error`] | [`HeuristicsError`] and `Result<T>` alias |
//! | [`decision`] | [`ChunkingDecision`], [`ChunkPlan`], [`PageRange`], [`check_format_limits`] |
//! | [`thresholds`] | [`calculate_chunk_plan`], [`calculate_plan_from_overrides`] |
//! | [`analyzer`] | [`DocumentMetadata`], [`UserChunkConfig`], [`analyze_document`] |
//! | [`multidoc`] | [`MultidocInput`], [`DocumentBoundary`], [`detect_boundaries`] |
//! | [`confidence`] | [`ConfidenceSignals`], [`ExtractionConfidence`], [`score_confidence`] |
//! | [`structured`] | [`StructuredCallMode`], [`StructuredInput`], [`StructuredThresholds`], [`choose_call_mode`] |

pub mod analyzer;
pub mod confidence;
pub mod config;
pub mod decision;
pub mod error;
pub mod multidoc;
pub mod structured;
pub mod thresholds;

// Convenience re-exports so callers can `use kreuzberg::heuristics::*` for the
// most common types.
pub use analyzer::{DocumentMetadata, UserChunkConfig, analyze_document, analyze_with_user_chunks};
pub use confidence::{ConfidenceSignals, ConfidenceWeights, ExtractionConfidence, SchemaCompliance, score_confidence};
pub use config::HeuristicsConfig;
pub use decision::{
    ChunkInfo, ChunkPlan, ChunkingDecision, ChunkingReason, NoChunkingReason, PageRange, check_format_limits,
};
pub use error::{HeuristicsError, Result};
pub use multidoc::{
    BoundaryReason, DocumentBoundary, MultidocInput, MultidocThresholds, PageSignals,
    boundaries_from_extraction_result, detect_boundaries,
};
pub use structured::{StructuredCallMode, StructuredInput, StructuredThresholds, choose_call_mode};
pub use thresholds::{calculate_chunk_plan, calculate_plan_from_overrides};
