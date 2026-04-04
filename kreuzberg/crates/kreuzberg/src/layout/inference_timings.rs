/// Thread-local side-channel for passing granular inference timings from model
/// implementations back to the [`crate::layout::engine::LayoutEngine`] caller.
///
/// This avoids changing the [`crate::layout::models::LayoutModel`] trait signature
/// while still providing per-step timing data to callers that need it.
///
/// Usage:
/// - Model implementations (e.g. `RtDetrModel`) call [`set`] with measured timings.
/// - `LayoutEngine::detect_timed()` calls [`take`] after the model returns to retrieve them.
use std::cell::Cell;

thread_local! {
    static PREPROCESS_MS: Cell<f64> = const { Cell::new(0.0) };
    static ONNX_MS: Cell<f64> = const { Cell::new(0.0) };
}

/// Record granular timings from the current inference call.
///
/// Called by model implementations immediately before returning results.
pub fn set(preprocess_ms: f64, onnx_ms: f64) {
    PREPROCESS_MS.with(|c| c.set(preprocess_ms));
    ONNX_MS.with(|c| c.set(onnx_ms));
}

/// Retrieve and reset the timings recorded by the most recent [`set`] call.
///
/// Returns `(preprocess_ms, onnx_ms)`. Resets both values to 0 after reading.
pub fn take() -> (f64, f64) {
    let pre = PREPROCESS_MS.with(|c| {
        let v = c.get();
        c.set(0.0);
        v
    });
    let onnx = ONNX_MS.with(|c| {
        let v = c.get();
        c.set(0.0);
        v
    });
    (pre, onnx)
}
