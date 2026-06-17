//! PDF form field extraction using the pdf_oxide backend.
//!
//! Maps pdf_oxide's `FormField` types to Kreuzberg's `PdfFormField` model,
//! extracting field names, types, values, bounding boxes, and metadata.
//!
//! Currently supports **AcroForm only**. XFA-only forms (no AcroForm) return an
//! empty vector. Mixed AcroForm+XFA documents extract from the AcroForm layer.

use super::OxideDocument;
use crate::types::{BoundingBox, PdfFormField, FormFieldType};

/// Extract form fields from a PDF document using pdf_oxide.
///
/// Calls `FormExtractor::extract_fields` to get all AcroForm fields from the document,
/// then maps each field's type, value, and bounding box to Kreuzberg's types.
///
/// This function has a fast-path: if the document has no AcroForm catalog entry,
/// it returns an empty vector immediately without further processing.
///
/// # Arguments
///
/// * `doc` - The opened PDF document
///
/// # Returns
///
/// A `Vec<PdfFormField>` containing all successfully extracted form fields,
/// or an empty vector if the document has no AcroForm or extraction fails.
pub(crate) fn extract_form_fields(doc: &OxideDocument) -> Vec<PdfFormField> {
    match pdf_oxide::extractors::forms::FormExtractor::extract_fields(&doc.doc) {
        Ok(oxide_fields) => {
            oxide_fields
                .into_iter()
                .map(map_form_field)
                .collect()
        }
        Err(e) => {
            tracing::debug!("pdf_oxide form field extraction failed: {e}");
            Vec::new()
        }
    }
}

/// Maps a single pdf_oxide FormField to a Kreuzberg PdfFormField.
///
/// Converts all field properties including type, value, bounds, and metadata.
/// Type mapping and value extraction handle various field subtypes (text, checkbox, radio, choice, signature, button).
fn map_form_field(oxide_field: pdf_oxide::extractors::forms::FormField) -> PdfFormField {
    let field_type = map_field_type(&oxide_field.field_type, oxide_field.flags);
    let value = map_field_value(&oxide_field.value);
    let default_value = oxide_field
        .default_value
        .as_ref()
        .and_then(map_field_value);
    let bbox = oxide_field.bounds.map(|bounds| BoundingBox {
        x0: bounds[0],
        y0: bounds[1],
        x1: bounds[2],
        y1: bounds[3],
    });

    PdfFormField {
        name: oxide_field.name,
        full_name: oxide_field.full_name,
        field_type,
        value,
        default_value,
        flags: oxide_field.flags.unwrap_or(0),
        page: None, // Page assignment is done downstream via spatial analysis
        bbox,
        max_length: oxide_field.max_length,
        tooltip: oxide_field.tooltip,
    }
}

/// Maps pdf_oxide's `FieldType` to Kreuzberg's `FormFieldType`.
///
/// Matches the PDF form field type hierarchy:
/// - Button (/Btn) includes checkboxes, radio buttons, and push buttons
/// - Text (/Tx) is single- or multi-line text input
/// - Choice (/Ch) is dropdown or list box
/// - Signature (/Sig) is digital signature
/// - Unknown for unrecognized types
///
/// For Button fields, further classifies using field flags per ISO 32000-1 §12.7.4.2:
/// - PUSH_BUTTON (bit 16): returns `Button`
/// - RADIO (bit 15): returns `Radio`
/// - Neither: returns `Checkbox` (default button subtype)
fn map_field_type(oxide_type: &pdf_oxide::extractors::forms::FieldType, flags: Option<u32>) -> FormFieldType {
    use pdf_oxide::extractors::forms::{FieldType, field_flags};
    match oxide_type {
        FieldType::Button => {
            // Classify button subtypes via field flags.
            if let Some(f) = flags {
                if f & field_flags::PUSH_BUTTON != 0 {
                    FormFieldType::Button
                } else if f & field_flags::RADIO != 0 {
                    FormFieldType::Radio
                } else {
                    // Default button subtype is checkbox
                    FormFieldType::Checkbox
                }
            } else {
                // No flags available; default to checkbox
                FormFieldType::Checkbox
            }
        }
        FieldType::Text => FormFieldType::Text,
        FieldType::Choice => FormFieldType::Choice,
        FieldType::Signature => FormFieldType::Signature,
        FieldType::Unknown(_) => FormFieldType::Unknown,
    }
}

/// Maps pdf_oxide's `FieldValue` to a String representation.
///
/// Handles:
/// - Text: returned as-is
/// - Boolean: converted to "true" or "false" (e.g., checkbox Boolean(true) → "true")
/// - Name: value returned as-is (e.g., radio Name("Opt") → "Opt", dropdown selections)
/// - Array: values joined with ", " (multi-select list boxes)
/// - None: returns None
fn map_field_value(oxide_value: &pdf_oxide::extractors::forms::FieldValue) -> Option<String> {
    use pdf_oxide::extractors::forms::FieldValue;
    match oxide_value {
        FieldValue::Text(s) => Some(s.clone()),
        FieldValue::Boolean(b) => Some(b.to_string()),
        FieldValue::Name(n) => Some(n.clone()),
        FieldValue::Array(arr) => {
            if arr.is_empty() {
                None
            } else {
                Some(arr.join(", "))
            }
        }
        FieldValue::None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdf_oxide::extractors::forms::{FieldType, FieldValue};

    #[test]
    fn test_map_field_type_button_push_button() {
        use pdf_oxide::extractors::forms::field_flags;
        let flags = field_flags::PUSH_BUTTON;
        assert_eq!(map_field_type(&FieldType::Button, Some(flags)), FormFieldType::Button);
    }

    #[test]
    fn test_map_field_type_button_radio() {
        use pdf_oxide::extractors::forms::field_flags;
        let flags = field_flags::RADIO;
        assert_eq!(map_field_type(&FieldType::Button, Some(flags)), FormFieldType::Radio);
    }

    #[test]
    fn test_map_field_type_button_checkbox_no_flags() {
        assert_eq!(map_field_type(&FieldType::Button, None), FormFieldType::Checkbox);
    }

    #[test]
    fn test_map_field_type_button_checkbox_default() {
        // When a button field has no PUSH_BUTTON or RADIO flag, it defaults to checkbox.
        assert_eq!(map_field_type(&FieldType::Button, Some(0)), FormFieldType::Checkbox);
    }

    #[test]
    fn test_map_field_type_text() {
        assert_eq!(map_field_type(&FieldType::Text, None), FormFieldType::Text);
    }

    #[test]
    fn test_map_field_type_choice() {
        assert_eq!(map_field_type(&FieldType::Choice, None), FormFieldType::Choice);
    }

    #[test]
    fn test_map_field_type_signature() {
        assert_eq!(map_field_type(&FieldType::Signature, None), FormFieldType::Signature);
    }

    #[test]
    fn test_map_field_type_unknown() {
        assert_eq!(map_field_type(&FieldType::Unknown("custom".to_string()), None), FormFieldType::Unknown);
    }

    #[test]
    fn test_map_field_value_text() {
        let value = FieldValue::Text("hello".to_string());
        assert_eq!(map_field_value(&value), Some("hello".to_string()));
    }

    #[test]
    fn test_map_field_value_boolean_true() {
        let value = FieldValue::Boolean(true);
        assert_eq!(map_field_value(&value), Some("true".to_string()));
    }

    #[test]
    fn test_map_field_value_boolean_false() {
        let value = FieldValue::Boolean(false);
        assert_eq!(map_field_value(&value), Some("false".to_string()));
    }

    #[test]
    fn test_map_field_value_name() {
        let value = FieldValue::Name("Yes".to_string());
        assert_eq!(map_field_value(&value), Some("Yes".to_string()));
    }

    #[test]
    fn test_map_field_value_array() {
        let value = FieldValue::Array(vec!["option1".to_string(), "option2".to_string()]);
        assert_eq!(map_field_value(&value), Some("option1, option2".to_string()));
    }

    #[test]
    fn test_map_field_value_empty_array() {
        let value = FieldValue::Array(vec![]);
        assert_eq!(map_field_value(&value), None);
    }

    #[test]
    fn test_map_field_value_none() {
        let value = FieldValue::None;
        assert_eq!(map_field_value(&value), None);
    }

    // === Button-specific field mapping tests ===

    #[test]
    fn test_checkbox_boolean_true_maps_to_true_string() {
        let value = FieldValue::Boolean(true);
        let mapped = map_field_value(&value).expect("Some");
        assert_eq!(mapped, "true", "checkbox Boolean(true) should map to 'true'");
    }

    #[test]
    fn test_radio_name_value_maps_as_is() {
        let value = FieldValue::Name("Opt".to_string());
        let mapped = map_field_value(&value).expect("Some");
        assert_eq!(mapped, "Opt", "radio Name('Opt') should map to 'Opt'");
    }

    // === Type + value integration tests ===

    #[test]
    fn test_checkbox_field_classification_and_value() {
        // Checkbox: Button field with no PUSH_BUTTON or RADIO flag, Boolean(true) value.
        let field_type = map_field_type(&FieldType::Button, Some(0));
        let value = map_field_value(&FieldValue::Boolean(true));
        assert_eq!(field_type, FormFieldType::Checkbox);
        assert_eq!(value, Some("true".to_string()));
    }

    #[test]
    fn test_radio_field_classification_and_value() {
        // Radio: Button field with RADIO flag, Name("Choice") value.
        use pdf_oxide::extractors::forms::field_flags;
        let field_type = map_field_type(&FieldType::Button, Some(field_flags::RADIO));
        let value = map_field_value(&FieldValue::Name("Choice".to_string()));
        assert_eq!(field_type, FormFieldType::Radio);
        assert_eq!(value, Some("Choice".to_string()));
    }

    #[test]
    fn test_push_button_field_classification() {
        // Push button: Button field with PUSH_BUTTON flag.
        use pdf_oxide::extractors::forms::field_flags;
        let field_type = map_field_type(&FieldType::Button, Some(field_flags::PUSH_BUTTON));
        assert_eq!(field_type, FormFieldType::Button);
    }
}
