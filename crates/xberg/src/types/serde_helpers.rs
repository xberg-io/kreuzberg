//! Custom serde serialization helpers for Vec<Arc<T>>.

/// Module for serializing Vec<Arc<T>> with transparent Arc handling.
///
/// Serializes a Vec<Arc<T>> as Vec<T> for compatibility, while preserving
/// Arc semantics for memory efficiency.
///
/// # Arc Sharing Semantics
///
/// **Important**: Arc sharing semantics are **NOT** preserved across serialization.
/// When deserializing, each element's Arc is independently created with `Arc::new()`.
/// This is important for `PageContent` where tables/images may be shared across pages.
///
/// Example with shared tables:
/// ```ignore
/// let shared_table = Arc::new(Table { /* ... */ });
/// let page_contents = vec![
///     PageContent { tables: vec![Arc::clone(&shared_table)], ... },
///     PageContent { tables: vec![Arc::clone(&shared_table)], ... },
/// ];
/// // In-memory: both pages' tables point to the same Arc
///
/// let json = serde_json::to_string(&page_contents)?;
/// let deserialized = serde_json::from_str::<Vec<PageContent>>(&json)?;
/// // After deserialization: each page has independent Arc instances,
/// // even though the table data is identical
/// ```
///
/// Design rationale:
/// - JSON has no mechanism to represent shared references
/// - Preserving sharing would require complex metadata and deduplication
/// - Current approach is simple, predictable, and maintains compatibility
/// - In-memory sharing (via Arc) is an implementation detail for the Rust side
///
/// If in-memory sharing is required after deserialization, implement custom
/// deduplication logic using hashing or content comparison.
pub mod serde_vec_arc {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::sync::Arc;

    /// Serialize Vec<Arc<T>> by serializing each T directly.
    ///
    /// Each element is unwrapped from its Arc and serialized independently.
    /// No sharing metadata is included in the serialized output.
    pub(crate) fn serialize<S, T>(vec: &[Arc<T>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: serde::Serialize,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(vec.len()))?;
        for arc_item in vec {
            seq.serialize_element(&**arc_item)?;
        }
        seq.end()
    }

    /// Deserialize Vec<T> and wrap each element in Arc.
    ///
    /// Each element is independently wrapped in a new Arc.
    /// Sharing relationships from before serialization are lost.
    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<Arc<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let vec: Vec<T> = Deserialize::deserialize(deserializer)?;
        Ok(vec.into_iter().map(Arc::new).collect())
    }
}
