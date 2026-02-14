//! XML extraction functions.
//!
//! Provides memory-efficient streaming XML parsing using `quick-xml`. Can handle
//! multi-GB XML files without loading them entirely into memory.
//!
//! # Features
//!
//! - **Streaming parser**: Processes XML files in constant memory
//! - **Element tracking**: Counts total elements and unique element names
//! - **Contextual text extraction**: Extracts text with element names as context for better quality
//! - **Whitespace handling**: Optional whitespace preservation
//!
//! # Example
//!
//! ```rust
//! use kreuzberg::extraction::xml::parse_xml;
//!
//! # fn example() -> kreuzberg::Result<()> {
//! let xml = b"<root><item>Hello</item><item>World</item></root>";
//! let result = parse_xml(xml, false)?; // false = trim whitespace
//!
//! // Content includes element names as context
//! assert!(result.content.contains("item: Hello"));
//! assert!(result.content.contains("item: World"));
//! assert_eq!(result.element_count, 3);
//! # Ok(())
//! # }
//! ```
use crate::error::{KreuzbergError, Result};
use crate::types::XmlExtractionResult;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::borrow::Cow;
use std::collections::HashSet;

pub fn parse_xml(xml_bytes: &[u8], preserve_whitespace: bool) -> Result<XmlExtractionResult> {
    let mut reader = Reader::from_reader(xml_bytes);
    reader.config_mut().trim_text(!preserve_whitespace);
    reader.config_mut().check_end_names = false;

    let mut content = String::new();
    let mut element_count = 0usize;
    let mut unique_elements_set = HashSet::new();
    let mut buf = Vec::new();
    let mut element_stack: Vec<String> = Vec::new();
    let mut last_was_element_tag = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let name: Cow<str> = String::from_utf8_lossy(&name_bytes);
                let name_owned = name.into_owned();
                element_count += 1;
                unique_elements_set.insert(name_owned.clone());

                // Extract attribute values as text content
                for attr in e.attributes().flatten() {
                    let attr_value: Cow<str> = String::from_utf8_lossy(&attr.value);
                    let trimmed_value = attr_value.trim();
                    if !trimmed_value.is_empty() {
                        let attr_key: Cow<str> = String::from_utf8_lossy(attr.key.as_ref());
                        if !content.is_empty() && !content.ends_with('\n') {
                            content.push('\n');
                        }
                        content.push_str(&name_owned);
                        content.push('[');
                        content.push_str(&attr_key);
                        content.push_str("]: ");
                        content.push_str(trimmed_value);
                        content.push('\n');
                    }
                }

                element_stack.push(name_owned);
                last_was_element_tag = true;
            }
            Ok(Event::Empty(e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let name: Cow<str> = String::from_utf8_lossy(&name_bytes);
                let name_owned = name.into_owned();
                element_count += 1;
                unique_elements_set.insert(name_owned.clone());

                // For self-closing tags, add element name and attributes
                if !content.is_empty() && !content.ends_with('\n') {
                    content.push('\n');
                }

                // Extract attribute values
                let mut has_attrs = false;
                for attr in e.attributes().flatten() {
                    let attr_value: Cow<str> = String::from_utf8_lossy(&attr.value);
                    let trimmed_value = attr_value.trim();
                    if !trimmed_value.is_empty() {
                        let attr_key: Cow<str> = String::from_utf8_lossy(attr.key.as_ref());
                        content.push_str(&name_owned);
                        content.push('[');
                        content.push_str(&attr_key);
                        content.push_str("]: ");
                        content.push_str(trimmed_value);
                        content.push('\n');
                        has_attrs = true;
                    }
                }

                if !has_attrs {
                    content.push_str(&name_owned);
                    content.push('\n');
                }
                last_was_element_tag = true;
            }
            Ok(Event::End(_e)) => {
                // Pop matching element from stack
                element_stack.pop();
                last_was_element_tag = true;
            }
            Ok(Event::Text(e)) => {
                let text_cow: Cow<str> = String::from_utf8_lossy(e.as_ref());
                let trimmed = if preserve_whitespace {
                    text_cow.to_string()
                } else {
                    text_cow.trim().to_string()
                };

                if !trimmed.is_empty() {
                    // Add element context if we just opened a new element
                    if last_was_element_tag && !element_stack.is_empty() {
                        if !content.is_empty() && !content.ends_with('\n') {
                            content.push('\n');
                        }
                        let elem_name = &element_stack[element_stack.len() - 1];
                        content.push_str(elem_name);
                        content.push_str(": ");
                    }

                    content.push_str(&trimmed);
                    content.push('\n');
                    last_was_element_tag = false;
                }
            }
            Ok(Event::CData(e)) => {
                let text_cow: Cow<str> = String::from_utf8_lossy(&e);

                // Add element context if we just opened a new element
                if last_was_element_tag && !element_stack.is_empty() {
                    if !content.is_empty() && !content.ends_with('\n') {
                        content.push('\n');
                    }
                    let elem_name = &element_stack[element_stack.len() - 1];
                    content.push_str(elem_name);
                    content.push_str(": ");
                }

                content.push_str(&text_cow);
                content.push('\n');
                last_was_element_tag = false;
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(KreuzbergError::parsing(format!(
                    "XML parsing error at position {}: {}",
                    reader.buffer_position(),
                    e
                )));
            }
            _ => {}
        }
        buf.clear();
    }

    let content = content.trim().to_string();
    let mut unique_elements: Vec<String> = unique_elements_set.into_iter().collect();
    unique_elements.sort();

    Ok(XmlExtractionResult {
        content,
        element_count,
        unique_elements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_xml() {
        let xml = b"<root><item>Hello</item><item>World</item></root>";
        let result = parse_xml(xml, false).unwrap();
        // Now includes element names as context
        assert!(result.content.contains("item: Hello"));
        assert!(result.content.contains("item: World"));
        assert_eq!(result.element_count, 3);
        assert!(result.unique_elements.contains(&"root".to_string()));
        assert!(result.unique_elements.contains(&"item".to_string()));
        assert_eq!(result.unique_elements.len(), 2);
    }

    #[test]
    fn test_xml_with_cdata() {
        let xml = b"<root><![CDATA[Special <characters> & data]]></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("Special <characters> & data"));
        assert_eq!(result.element_count, 1);
    }

    #[test]
    fn test_malformed_xml_lenient() {
        let xml = b"<root><item>Unclosed<item2>Content</root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(!result.content.is_empty());
        assert!(result.content.contains("Content"));
    }

    #[test]
    fn test_empty_xml() {
        let xml = b"<root></root>";
        let result = parse_xml(xml, false).unwrap();
        assert_eq!(result.content, "");
        assert_eq!(result.element_count, 1);
        assert_eq!(result.unique_elements.len(), 1);
    }

    #[test]
    fn test_whitespace_handling() {
        let xml = b"<root>  <item>  Text  </item>  </root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("item: Text"));
    }

    #[test]
    fn test_preserve_whitespace() {
        let xml = b"<root>  Text with   spaces  </root>";
        let result_trimmed = parse_xml(xml, false).unwrap();
        let result_preserved = parse_xml(xml, true).unwrap();
        assert!(result_trimmed.content.contains("Text with   spaces"));
        assert!(result_preserved.content.len() >= result_trimmed.content.len());
    }

    #[test]
    fn test_element_counting() {
        let xml = b"<root><a/><b/><c/><b/><d/></root>";
        let result = parse_xml(xml, false).unwrap();
        assert_eq!(result.element_count, 6);
        assert_eq!(result.unique_elements.len(), 5);
        assert!(result.unique_elements.contains(&"b".to_string()));
    }

    #[test]
    fn test_xml_with_attributes() {
        let xml = br#"<root id="1"><item type="test">Content</item></root>"#;
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("item: Content"));
        assert_eq!(result.element_count, 2);
    }

    #[test]
    fn test_xml_with_namespaces() {
        let xml = b"<ns:root xmlns:ns=\"http://example.com\"><ns:item>Text</ns:item></ns:root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("Text"));
        assert!(result.element_count >= 2);
    }

    #[test]
    fn test_xml_with_comments() {
        let xml = b"<root><!-- Comment --><item>Text</item></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("item: Text"));
        assert_eq!(result.element_count, 2);
    }

    #[test]
    fn test_xml_with_processing_instructions() {
        let xml = b"<?xml version=\"1.0\"?><root><item>Text</item></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("item: Text"));
        assert_eq!(result.element_count, 2);
    }

    #[test]
    fn test_xml_with_mixed_content() {
        let xml = b"<root>Text before<item>nested</item>Text after</root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("Text before"));
        assert!(result.content.contains("item: nested"));
        assert!(result.content.contains("Text after"));
    }

    #[test]
    fn test_xml_empty_bytes() {
        let xml = b"";
        let result = parse_xml(xml, false).unwrap();
        assert_eq!(result.content, "");
        assert_eq!(result.element_count, 0);
        assert!(result.unique_elements.is_empty());
    }

    #[test]
    fn test_xml_only_whitespace() {
        let xml = b"   \n\t  ";
        let result = parse_xml(xml, false).unwrap();
        assert_eq!(result.content, "");
        assert_eq!(result.element_count, 0);
    }

    #[test]
    fn test_xml_with_nested_elements() {
        let xml = b"<root><parent><child><grandchild>Deep</grandchild></child></parent></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("Deep"));
        assert!(result.content.contains("grandchild: Deep"));
        assert_eq!(result.element_count, 4);
        assert_eq!(result.unique_elements.len(), 4);
    }

    #[test]
    fn test_xml_with_special_characters() {
        let xml = b"<root>&lt;&gt;&amp;&quot;&apos;</root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.element_count >= 1);
    }

    #[test]
    fn test_xml_self_closing_tags() {
        let xml = b"<root><item1/><item2/><item3/></root>";
        let result = parse_xml(xml, false).unwrap();
        assert_eq!(result.element_count, 4);
        assert_eq!(result.unique_elements.len(), 4);
    }

    #[test]
    fn test_xml_multiple_text_nodes() {
        let xml = b"<root>First<a/>Second<b/>Third</root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("First"));
        assert!(result.content.contains("Second"));
        assert!(result.content.contains("Third"));
    }

    #[test]
    fn test_xml_with_newlines() {
        let xml = b"<root>\n  <item>\n    Text\n  </item>\n</root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("item: Text"));
    }

    #[test]
    fn test_xml_large_cdata() {
        let large_text = "A".repeat(10000);
        let xml = format!("<root><![CDATA[{}]]></root>", large_text);
        let result = parse_xml(xml.as_bytes(), false).unwrap();
        assert!(result.content.contains(&large_text));
    }

    #[test]
    fn test_xml_unique_elements_sorted() {
        let xml = b"<root><z/><a/><m/><b/></root>";
        let result = parse_xml(xml, false).unwrap();
        let expected = vec!["a", "b", "m", "root", "z"];
        assert_eq!(result.unique_elements, expected);
    }

    #[test]
    fn test_xml_result_structure() {
        let xml = b"<root><item>Test</item></root>";
        let result = parse_xml(xml, false).unwrap();

        assert!(!result.content.is_empty());
        assert!(result.element_count > 0);
        assert!(!result.unique_elements.is_empty());
    }

    #[test]
    fn test_xml_with_multiple_cdata_sections() {
        let xml = b"<root><![CDATA[First]]>Text<![CDATA[Second]]></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("First"));
        assert!(result.content.contains("Text"));
        assert!(result.content.contains("Second"));
    }

    #[test]
    fn test_xml_preserve_whitespace_flag() {
        let xml = b"<root>  A  B  </root>";
        let without_preserve = parse_xml(xml, false).unwrap();
        let with_preserve = parse_xml(xml, true).unwrap();

        assert!(!without_preserve.content.starts_with(' '));

        assert!(with_preserve.content.len() >= without_preserve.content.len());
    }

    #[test]
    fn test_xml_element_count_accuracy() {
        let xml = b"<root><a><b><c/></b></a><d/></root>";
        let result = parse_xml(xml, false).unwrap();
        assert_eq!(result.element_count, 5);
    }

    #[test]
    fn test_xml_with_invalid_utf8() {
        let xml = b"<root><item>Valid text \xFF invalid</item></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("Valid text"));
        assert_eq!(result.element_count, 2);
    }

    #[test]
    fn test_xml_cdata_with_invalid_utf8() {
        let xml = b"<root><![CDATA[Text \xFF more text]]></root>";
        let result = parse_xml(xml, false).unwrap();
        assert!(result.content.contains("Text"));
        assert!(result.content.contains("more text"));
        assert_eq!(result.element_count, 1);
    }

    #[test]
    fn test_xml_element_name_with_invalid_utf8() {
        let xml = b"<root><item\xFF>Content</item\xFF></root>";
        let result = parse_xml(xml, false);
        let _ = result;
    }
}
