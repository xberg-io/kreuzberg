use std::borrow::Cow;

/// Converts XML tag name bytes to a string, avoiding allocation when possible.
#[inline]
pub fn xml_tag_name(name: &[u8]) -> Cow<'_, str> {
    String::from_utf8_lossy(name)
}
