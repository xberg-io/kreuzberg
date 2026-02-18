pub(crate) mod internal {
    // We want to make the PdfFormFieldPrivate trait private while providing a blanket
    // implementation of PdfFormFieldCommon for any type T where T: PdfFormFieldPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfFormFieldPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{
        FPDF_ANNOTATION, FPDF_FORMFLAG_CHOICE_COMBO, FPDF_FORMFLAG_CHOICE_EDIT, FPDF_FORMFLAG_CHOICE_MULTI_SELECT,
        FPDF_FORMFLAG_NOEXPORT, FPDF_FORMFLAG_NONE, FPDF_FORMFLAG_READONLY, FPDF_FORMFLAG_REQUIRED,
        FPDF_FORMFLAG_TEXT_MULTILINE, FPDF_FORMFLAG_TEXT_PASSWORD, FPDF_FORMHANDLE, FPDF_OBJECT_NAME,
        FPDF_OBJECT_STREAM, FPDF_OBJECT_STRING, FPDF_WCHAR,
    };
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::{PdfiumError, PdfiumInternalError};
    use crate::pdf::appearance_mode::PdfAppearanceMode;
    use crate::pdf::document::page::field::PdfFormFieldCommon;
    use crate::utils::dates::date_time_to_pdf_string;
    use crate::utils::mem::create_byte_buffer;
    use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
    use bitflags::bitflags;
    use chrono::Utc;

    bitflags! {
        /// Flags specifying various characteristics of one form field. For more details,
        /// refer to Section 8.6.2 of The PDF Reference (Sixth Edition, PDF Format 1.7),
        /// starting on page 674.
        pub struct PdfFormFieldFlags: u32 {
            /// No flags are set for this form field.
            const None = FPDF_FORMFLAG_NONE;

            /// If set, the user may not change the value of the field. Any associated widget
            /// annotations will not interact with the user; that is, they will not respond
            /// to mouse clicks or change their appearance in response to mouse motions.
            /// This flag is useful for fields whose values are computed or imported from
            /// a database.
            const ReadOnly = FPDF_FORMFLAG_READONLY;

            /// If set, the field must have a value at the time it is exported by a
            /// "submit form" action.
            ///
            /// For more information on "submit form" actions, refer to Section 8.6.4 of
            /// The PDF Reference (Sixth Edition, PDF Format 1.7), starting on page 702.
            const Required = FPDF_FORMFLAG_REQUIRED;

            /// If set, the field must not be exported by any "submit form" action.
            ///
            /// For more information on "submit form" actions, refer to Section 8.6.4 of
            /// The PDF Reference (Sixth Edition, PDF Format 1.7), starting on page 702.
            const NoExport = FPDF_FORMFLAG_NOEXPORT;

            /// If set, the field can contain multiple lines of text; if clear,
            /// the field's text is restricted to a single line.
            const TextMultiline = FPDF_FORMFLAG_TEXT_MULTILINE;

            /// If set, the field is intended for entering a secure password that should not
            /// be echoed visibly to the screen. Characters typed from the keyboard
            /// should instead be echoed in some unreadable form, such as asterisks or
            /// bullet characters.
            ///
            /// To protect password confidentiality, viewer applications should never
            /// store the value of the text field in the PDF file if this flag is set.
            const TextPassword = FPDF_FORMFLAG_TEXT_PASSWORD;

            /// If set, the text entered in the field represents the path name of a
            /// file whose contents are to be submitted as the value of the field.
            ///
            /// This flag was added in PDF version 1.4.
            const TextFileSelect = 1 << 21; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, the text entered in the field is not spell-checked.
            ///
            /// This flag was added in PDF version 1.4.
            const TextDoNotSpellCheck = 1 << 23; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, the field does not scroll (horizontally for single-line fields,
            /// vertically for multiple-line fields) to accommodate more text than fits
            /// within its annotation rectangle. Once the field is full, no further text
            /// is accepted.
            ///
            /// This flag was added in PDF version 1.4.
            const TextDoNotScroll = 1 << 24; // Not directly exposed by Pdfium, but we can support it inline.

            /// Meaningful only if the `MaxLen` entry is present in the text field
            /// dictionary (see Table 8.78) _and_ if the Multiline, Password, and FileSelect
            /// flags are clear. If set, the field is automatically divided into as many
            /// equally-spaced positions ("combs") as the value of `MaxLen`, and the text
            /// is laid out into those combs.
            ///
            /// For more information on this setting, refer to Table 8.77 of
            /// The PDF Reference (Sixth Edition, PDF Format 1.7), on page 691.
            ///
            /// This flag was added in PDF version 1.5.
            const TextComb = 1 << 25; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, the value of this field should be represented as a rich text
            /// string. If the field has a value, the `RV` entry of the field dictionary
            /// specifies the rich text string.
            ///
            /// This flag was added in PDF version 1.5.
            const TextRichText = 1 << 26; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, the field is a combo box; if clear, the field is a list box.
            const ChoiceCombo = FPDF_FORMFLAG_CHOICE_COMBO;

            /// If set, the combo box includes an editable text box as well as a drop-
            /// down list; if clear, it includes only a drop-down list. This flag is
            /// meaningful only if the `ChoiceCombo` flag is set.
            const ChoiceEdit = FPDF_FORMFLAG_CHOICE_EDIT;

            /// If set, the option items of the combo box or list box should be sorted
            /// alphabetically.
            ///
            /// This flag is intended for use by form authoring tools, not by PDF viewer
            /// applications. Viewers should simply display the options in the order in which
            /// they occur in the `Opt` array.
            const ChoiceSort = 1 << 20; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, more than one of the option items of the combo box or list box
            /// may be selected simultaneously; if clear, no more than one item at a time
            /// may be selected.
            ///
            /// This flag was added in PDF version 1.4.
            const ChoiceMultiSelect = FPDF_FORMFLAG_CHOICE_MULTI_SELECT;

            /// If set, text entered in the combo box field is not spell-checked.
            ///
            /// This flag is meaningful only if the `ChoiceCombo` and `ChoiceEdit` flags
            /// are both set.
            ///
            /// This flag was added in PDF version 1.4.
            const ChoiceDoNotSpellCheck = 1 << 23;

            /// If set, the new value for the combo box or list box is committed as soon
            /// as a selection is made with the pointing device. This option enables
            /// applications to perform an action once a selection is made, without requiring
            /// the user to exit the field. If clear, the new value is not committed until
            /// the user exits the field.
            const ChoiceCommitOnSelectionChange = 1 << 27;

            /// If set, exactly one radio button must be selected at all
            /// times; clicking the currently selected button has no effect. If clear,
            /// clicking the selected button deselects it, leaving no button selected.
            ///
            /// This flag is only applicable to radio buttons.
            const ButtonNoToggleToOff = 1 << 15; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, the field is a set of radio buttons; if clear, the field is a check box.
            /// This flag is meaningful only if the `ButtonIsPushbutton` flag is clear.
            const ButtonIsRadio = 1 << 16; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, the field is a push button that does not retain a permanent value.
            const ButtonIsPushButton = 1 << 17; // Not directly exposed by Pdfium, but we can support it inline.

            /// If set, a group of radio buttons within a radio button field that
            /// use the same value for the on state will turn on and off in unison; that is if
            /// one is checked, they are all checked. If clear, the buttons are mutually
            /// exclusive, i.e. the same behavior as HTML radio buttons.
            ///
            /// This flag was added in PDF version 1.5.
            const ButtonIsRadiosInUnison = 1 << 26; // Not directly exposed by Pdfium, but we can support it inline.
        }
    }

    /// Internal crate-specific functionality common to all [PdfFormField] objects.
    pub trait PdfFormFieldPrivate<'a>: PdfFormFieldCommon {
        /// Returns the internal `FPDF_FORMHANDLE` handle for this [PdfFormField].
        fn form_handle(&self) -> FPDF_FORMHANDLE;

        /// Returns the internal `FPDF_ANNOTATION` handle for this [PdfFormField].
        fn annotation_handle(&self) -> FPDF_ANNOTATION;

        /// Returns the [PdfiumLibraryBindings] used by this [PdfFormField].
        fn bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Internal implementation of [PdfFormFieldCommon::name()].
        fn name_impl(&self) -> Option<String> {
            // Retrieving the field name from Pdfium is a two-step operation. First, we call
            // FPDFAnnot_GetFormFieldName() with a null buffer; this will retrieve the length of
            // the field name text in bytes. If the length is zero, then the field name is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetFormFieldName() again with a pointer to the buffer;
            // this will write the field name to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetFormFieldName(
                self.form_handle(),
                self.annotation_handle(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The field name is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetFormFieldName(
                    self.form_handle(),
                    self.annotation_handle(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Internal implementation of `value()` function shared by value-carrying form field widgets
        /// such as text fields. Not exposed directly by [PdfFormFieldCommon].
        fn value_impl(&self) -> Option<String> {
            // Retrieving the field value from Pdfium is a two-step operation. First, we call
            // FPDFAnnot_GetFormFieldValue() with a null buffer; this will retrieve the length of
            // the form value text in bytes. If the length is zero, then the form value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetFormFieldValue() again with a pointer to the buffer;
            // this will write the field value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetFormFieldValue(
                self.form_handle(),
                self.annotation_handle(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The field value is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetFormFieldValue(
                    self.form_handle(),
                    self.annotation_handle(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Internal implementation of `set_value()` function shared by value-carrying form
        /// field widgets such as text fields. Not exposed directly by [PdfFormFieldCommon].
        #[inline]
        fn set_value_impl(&mut self, value: &str) -> Result<(), PdfiumError> {
            self.bindings()
                .to_result(self.bindings().FPDFAnnot_SetStringValue_str(
                    self.annotation_handle(),
                    "M",
                    &date_time_to_pdf_string(Utc::now()),
                ))
                .and_then(|_| {
                    self.bindings().to_result(self.bindings().FPDFAnnot_SetStringValue_str(
                        self.annotation_handle(),
                        "V",
                        value,
                    ))
                })
        }

        /// Internal implementation of `export_value()` function shared by on/off form field widgets
        /// such as checkbox and radio button fields. Not exposed directly by [PdfFormFieldCommon].
        fn export_value_impl(&self) -> Option<String> {
            // Retrieving the export value from Pdfium is a two-step operation. First, we call
            // FPDFAnnot_GetFormFieldExportValue() with a null buffer; this will retrieve the length of
            // the export value text in bytes. If the length is zero, then the export value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetFormFieldExportValue() again with a pointer to the buffer;
            // this will write the export value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetFormFieldExportValue(
                self.form_handle(),
                self.annotation_handle(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The field value is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetFormFieldExportValue(
                    self.form_handle(),
                    self.annotation_handle(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Internal implementation of `is_checked()` function shared by checkable form field widgets
        /// such as radio buttons and checkboxes. Not exposed directly by [PdfFormFieldCommon].
        ///
        /// Note Pdfium does not consider appearance streams when determining if a checkable form
        /// field is currently selected. As a result, this function may not return the expected
        /// result for fields with custom appearance streams.
        fn is_checked_impl(&self) -> Result<bool, PdfiumError> {
            Ok(self.bindings().is_true(
                self.bindings()
                    .FPDFAnnot_IsChecked(self.form_handle(), self.annotation_handle()),
            ))
        }

        /// Internal implementation of `index_in_group()` function shared by checkable form field
        /// widgets such as radio buttons and checkboxes. Not exposed directly by [PdfFormFieldCommon].
        fn index_in_group_impl(&self) -> u32 {
            let result = self
                .bindings()
                .FPDFAnnot_GetFormControlIndex(self.form_handle(), self.annotation_handle());

            if result < 0 {
                // Pdfium uses a -1 value to signal an error.

                0
            } else {
                result as u32
            }
        }

        /// Returns the string value associated with the given key in the annotation dictionary
        /// of the [PdfPageAnnotation] containing this [PdfFormField], if any.
        fn get_string_value(&self, key: &str) -> Option<String> {
            if !self
                .bindings()
                .is_true(self.bindings().FPDFAnnot_HasKey(self.annotation_handle(), key))
            {
                // The key does not exist.

                return None;
            }

            let t = self.bindings().FPDFAnnot_GetValueType(self.annotation_handle(), key) as u32;

            if t != FPDF_OBJECT_STRING && t != FPDF_OBJECT_NAME && t != FPDF_OBJECT_STREAM {
                // The key exists, but the value associated with the key is not a string
                // or a type with an underlying string representation.

                return None;
            }

            // Retrieving the string value from Pdfium is a two-step operation. First, we call
            // FPDFAnot_GetStringValue() with a null buffer; this will retrieve the length of
            // the value in bytes, assuming the key exists. If the length is zero, then there
            // is no such key, or the key's value is not a string.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnot_GetStringValue() again with a pointer to the buffer;
            // this will write the string value into the buffer.

            let buffer_length =
                self.bindings()
                    .FPDFAnnot_GetStringValue(self.annotation_handle(), key, std::ptr::null_mut(), 0);

            if buffer_length <= 2 {
                // A buffer length of 2 indicates that the string value for the given key is
                // an empty UTF16-LE string, so there is no point in retrieving it.

                return None;
            }

            let mut buffer = create_byte_buffer(buffer_length as usize);

            let result = self.bindings().FPDFAnnot_GetStringValue(
                self.annotation_handle(),
                key,
                buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                buffer_length,
            );

            assert_eq!(result, buffer_length);

            Some(get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default())
        }

        /// Sets the string value associated with the given key in the annotation dictionary
        /// of the [PdfPageAnnotation] containing this [PdfFormField].
        fn set_string_value(&mut self, key: &str, value: &str) -> Result<(), PdfiumError> {
            // Attempt to update the modification date first, before we apply the given value update.
            // That way, if updating the date fails, we can fail early.

            #[allow(clippy::collapsible_if)] // Prefer to keep the intent clear
            if key != "M"
            // Don't update the modification date if the key we have been given to update
            // is itself the modification date!
            {
                self.set_string_value("M", &date_time_to_pdf_string(Utc::now()))?;
            }

            // With the modification date updated, we can now update the key and value
            // we were given.

            if self.bindings().is_true(self.bindings().FPDFAnnot_SetStringValue_str(
                self.annotation_handle(),
                key,
                value,
            )) {
                Ok(())
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::Unknown))
            }
        }

        /// Internal implementation of [PdfFormFieldCommon::appearance_mode_value()].
        fn appearance_mode_value_impl(&self, appearance_mode: PdfAppearanceMode) -> Option<String> {
            // Retrieving the appearance mode value from Pdfium is a two-step operation.
            // First, we call FPDFAnnot_GetAP() with a null buffer; this will retrieve the length of
            // the appearance mode value text in bytes. If the length is zero, then the
            // appearance mode value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetAP() again with a pointer to the buffer;
            // this will write the appearance mode value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetAP(
                self.annotation_handle(),
                appearance_mode.as_pdfium(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The appearance mode value is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetAP(
                    self.annotation_handle(),
                    appearance_mode.as_pdfium(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Returns the currently set appearance stream for this form field, if any.
        fn appearance_stream_impl(&self) -> Option<String> {
            self.get_string_value("AS")
        }

        /// Returns all the flags currently set on this form field.
        #[inline]
        fn get_flags_impl(&self) -> PdfFormFieldFlags {
            PdfFormFieldFlags::from_bits_truncate(
                self.bindings()
                    .FPDFAnnot_GetFormFieldFlags(self.form_handle(), self.annotation_handle()) as u32,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pdf::document::page::field::private::internal::{PdfFormFieldFlags, PdfFormFieldPrivate};
    use crate::prelude::*;
    use crate::utils::test::{test_bind_to_pdfium, test_fixture_path};

    #[test]
    fn test_get_form_field_flags() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();
        let document = pdfium.load_pdf_from_file(&test_fixture_path("form-test.pdf"), None)?;
        let page = document.pages().first()?;
        let annotation = page
            .annotations()
            .iter()
            .find(|annotation| annotation.as_form_field().is_some())
            .unwrap();
        let field = annotation.as_form_field().unwrap();
        let text = field.as_text_field().unwrap();

        let flags = text.get_flags_impl();

        assert!(!flags.contains(PdfFormFieldFlags::ReadOnly));
        assert!(!text.is_read_only());

        assert!(!flags.contains(PdfFormFieldFlags::Required));
        assert!(!text.is_required());

        assert!(!flags.contains(PdfFormFieldFlags::NoExport));
        assert!(text.is_exported_on_submit());

        assert!(!flags.contains(PdfFormFieldFlags::TextMultiline));
        assert!(!text.is_multiline());

        assert!(!flags.contains(PdfFormFieldFlags::TextPassword));
        assert!(!text.is_password());

        assert!(flags.contains(PdfFormFieldFlags::TextDoNotSpellCheck));
        assert!(!text.is_spell_checked());

        Ok(())
    }
}
