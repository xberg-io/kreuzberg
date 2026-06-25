//! Djot event parsing and content extraction.
//!
//! Handles parsing of jotdown events into tables.

mod table_extraction;

pub(crate) use table_extraction::extract_tables_from_events;
