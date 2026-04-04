// Re-export from the canonical implementation in crate::utils::string_utils.
// The text module previously maintained its own duplicate encoding cache (HashMap,
// DefaultHasher, flat 1000-entry cap). All functionality now lives in
// crate::utils::string_utils which uses LRU eviction, AHasher, and env-configurable
// limits.
pub use crate::utils::string_utils::{calculate_text_confidence, fix_mojibake, safe_decode};
