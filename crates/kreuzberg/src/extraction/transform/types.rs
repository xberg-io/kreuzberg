//! Type definitions for transformation operations.

/// Metadata about a detected list item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItemMetadata {
    /// Type of list (Bullet, Numbered, etc.)
    pub list_type: ListType,
    /// Starting byte offset in the content string
    pub byte_start: usize,
    /// Ending byte offset in the content string
    pub byte_end: usize,
    /// List item indent level
    pub indent_level: u32,
}

/// Type of list detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListType {
    /// Bullet points (-, *, •, etc.)
    Bullet,
    /// Numbered lists (1., 2., etc.)
    Numbered,
    /// Lettered lists (a., b., A., B., etc.)
    Lettered,
    /// Indented items
    Indented,
}
