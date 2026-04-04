//! GC-guarded Ruby value wrapper for plugin registrations
//!
//! Keeps Ruby values alive across plugin registrations by informing the Ruby GC.

use magnus::{Ruby, Value};

/// Keeps Ruby values alive across plugin registrations by informing the GC.
///
/// This prevents Ruby objects (like Procs) from being garbage collected while
/// they're being used as plugin callbacks.
pub struct GcGuardedValue {
    value: Value,
}

impl GcGuardedValue {
    /// Create a new GC-guarded value
    pub fn new(value: Value) -> Self {
        let ruby = Ruby::get().expect("Ruby not initialized");
        ruby.gc_register_address(&value);
        Self { value }
    }

    /// Get the wrapped value
    pub fn value(&self) -> Value {
        self.value
    }
}

impl Drop for GcGuardedValue {
    fn drop(&mut self) {
        if let Ok(ruby) = Ruby::get() {
            ruby.gc_unregister_address(&self.value);
        }
    }
}
