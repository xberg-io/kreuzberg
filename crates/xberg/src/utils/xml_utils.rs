#[cfg(any(feature = "xml", feature = "office"))]
use std::borrow::Cow;

/// Converts XML tag name bytes to a string, avoiding allocation when possible.
#[cfg(any(feature = "xml", feature = "office"))]
#[inline]
pub(crate) fn xml_tag_name(name: &[u8]) -> Cow<'_, str> {
    String::from_utf8_lossy(name)
}
