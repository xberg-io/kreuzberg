//! Safety wrapper for NIF operations.
//!
//! Wraps extraction calls with `catch_unwind` to prevent panics in native C
//! libraries (pdfium, tesseract) from crashing the BEAM VM. Instead, panics
//! are caught and returned as `{:error, reason}` tuples.

use std::panic::{self, AssertUnwindSafe};

/// Run a closure that may panic (e.g., from native C FFI) and convert panics
/// to a string error message. Logs the panic at error level with a backtrace.
///
/// Returns `Ok(T)` on success, `Err(String)` if the closure panicked.
pub fn catch_native_panic<F, T>(operation: &str, f: F) -> Result<T, String>
where
    F: FnOnce() -> T,
{
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => Ok(result),
        Err(payload) => {
            let panic_msg = if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic payload".to_string()
            };

            let backtrace = std::backtrace::Backtrace::force_capture();

            tracing::error!(
                operation = operation,
                panic_message = %panic_msg,
                backtrace = %backtrace,
                "Native library panic caught in Elixir NIF — returning error instead of crashing BEAM"
            );

            Err(format!("Native library panic during {}: {}", operation, panic_msg))
        }
    }
}
