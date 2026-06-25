//! Djot attribute parsing utilities.
//!
//! Handles parsing of Djot attributes from jotdown events and string syntax.

/// Render attributes to djot attribute syntax.
///
/// Converts Xberg's Attributes struct back to djot attribute syntax:
/// {.class #id key="value"}
pub(crate) fn render_attributes(attrs: &crate::types::Attributes) -> String {
    let mut parts = Vec::new();

    if let Some(ref id) = attrs.id {
        parts.push(format!("#{}", id));
    }

    for class in &attrs.classes {
        parts.push(format!(".{}", class));
    }

    for (key, value) in &attrs.key_values {
        parts.push(format!("{}=\"{}\"", key, value));
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!("{{{}}}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_attributes_with_all_parts() {
        let attrs = crate::types::Attributes {
            id: Some("my-id".to_string()),
            classes: vec!["class1".to_string(), "class2".to_string()],
            key_values: vec![("data-test".to_string(), "value".to_string())],
        };

        let rendered = render_attributes(&attrs);
        assert!(rendered.contains("#my-id"));
        assert!(rendered.contains(".class1"));
        assert!(rendered.contains(".class2"));
        assert!(rendered.contains("data-test"));
    }

    #[test]
    fn test_render_attributes_empty() {
        let attrs = crate::types::Attributes {
            id: None,
            classes: vec![],
            key_values: Vec::new(),
        };

        let rendered = render_attributes(&attrs);
        assert_eq!(rendered, "");
    }
}
