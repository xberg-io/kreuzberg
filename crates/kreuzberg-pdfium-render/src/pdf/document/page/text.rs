//! Defines the [PdfPageText] struct, exposing functionality related to the
//! collection of Unicode characters visible on a single [PdfPage].

pub mod char;
pub mod chars;
pub mod search;
pub mod segment;
pub mod segments;

use crate::bindgen::{FPDF_TEXTPAGE, FPDF_WCHAR, FPDF_WIDESTRING};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::PdfPage;
use crate::pdf::document::page::annotation::PdfPageAnnotation;
use crate::pdf::document::page::annotation::PdfPageAnnotationCommon;
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::text::PdfPageTextObject;
use crate::pdf::document::page::text::chars::{PdfPageTextCharIndex, PdfPageTextChars};
use crate::pdf::document::page::text::search::{PdfPageTextSearch, PdfSearchOptions};
use crate::pdf::document::page::text::segments::PdfPageTextSegments;
use crate::pdf::points::PdfPoints;
use crate::pdf::rect::PdfRect;
use crate::utils::mem::{create_byte_buffer, create_sized_buffer};
use crate::utils::utf16le::{get_pdfium_utf16le_bytes_from_str, get_string_from_pdfium_utf16le_bytes};
use bytemuck::cast_slice;
use std::fmt::{Display, Formatter};
use std::os::raw::{c_double, c_int};
use std::ptr::null_mut;

/// The collection of Unicode characters visible on a single [PdfPage].
///
/// Use the [PdfPageText::all()] function to easily return all characters in the containing
/// [PdfPage] in the order in which they are defined in the PDF file.
///
/// Use the [PdfPageText::search()] function to initialise a new [PdfPageTextSearch] object,
/// yielding the results of searching for a target string within the character collection.
///
/// In complex custom layouts, the order in which characters are defined in the document
/// and the order in which they appear visually during rendering (and thus the order in
/// which they are read by a user) may not necessarily match.
///
/// [PdfPageText] implements both the [ToString] and the [Display] traits.
pub struct PdfPageText<'a> {
    text_page_handle: FPDF_TEXTPAGE,
    page: &'a PdfPage<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageText<'a> {
    pub(crate) fn from_pdfium(
        text_page_handle: FPDF_TEXTPAGE,
        page: &'a PdfPage<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageText {
            text_page_handle,
            page,
            bindings,
        }
    }

    /// Returns the internal `FPDF_TEXTPAGE` handle for this [PdfPageText].
    #[inline]
    pub(crate) fn text_page_handle(&self) -> FPDF_TEXTPAGE {
        self.text_page_handle
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfPageText].
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the total number of characters in all text segments in the containing [PdfPage].
    ///
    /// The character count includes whitespace and newlines, and so may differ slightly
    /// from the result of calling `PdfPageText::all().len()`.
    #[inline]
    pub fn len(&self) -> i32 {
        self.bindings.FPDFText_CountChars(self.text_page_handle())
    }

    /// Returns `true` if there are no characters in any text box collection in the containing [PdfPage].
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a collection of all the `PdfPageTextSegment` text segments in the containing [PdfPage].
    #[inline]
    pub fn segments(&self) -> PdfPageTextSegments<'_> {
        PdfPageTextSegments::new(self, 0, self.len(), self.bindings())
    }

    /// Returns a subset of the `PdfPageTextSegment` text segments in the containing [PdfPage].
    /// Only text segments containing characters in the given index range will be included.
    #[inline]
    pub fn segments_subset(&self, start: PdfPageTextCharIndex, count: PdfPageTextCharIndex) -> PdfPageTextSegments<'_> {
        PdfPageTextSegments::new(self, start as i32, count as i32, self.bindings())
    }

    /// Returns a collection of all the `PdfPageTextChar` characters in the containing [PdfPage].
    #[inline]
    pub fn chars(&self) -> PdfPageTextChars<'_> {
        PdfPageTextChars::new(
            self.page.document_handle(),
            self.page.page_handle(),
            self.text_page_handle(),
            (0..self.len()).collect(),
            self.bindings(),
        )
    }

    /// Returns a collection of all the `PdfPageTextChar` characters in the given [PdfPageTextObject].
    ///
    /// The return result will be empty if the given [PdfPageTextObject] is not attached to the
    /// containing [PdfPage].
    #[inline]
    pub fn chars_for_object(&self, object: &PdfPageTextObject) -> Result<PdfPageTextChars<'_>, PdfiumError> {
        Ok(PdfPageTextChars::new(
            self.page.document_handle(),
            self.page.page_handle(),
            self.text_page_handle(),
            self.chars()
                .iter()
                .filter(|char| {
                    self.bindings
                        .FPDFText_GetTextObject(self.text_page_handle(), char.index() as i32)
                        == object.object_handle()
                })
                .map(|char| char.index() as i32)
                .collect(),
            self.bindings(),
        ))
    }

    /// Returns a collection of all the `PdfPageTextChar` characters in the given [PdfPageAnnotation].
    ///
    /// The return result will be empty if the given [PdfPageAnnotation] is not attached to the
    /// containing [PdfPage].
    #[inline]
    pub fn chars_for_annotation(&self, annotation: &PdfPageAnnotation) -> Result<PdfPageTextChars<'_>, PdfiumError> {
        self.chars_inside_rect(annotation.bounds()?)
            .map_err(|_| PdfiumError::NoCharsInAnnotation)
    }

    /// Returns a collection of all the `PdfPageTextChar` characters that lie within the bounds of
    /// the given [PdfRect] in the containing [PdfPage].
    #[inline]
    pub fn chars_inside_rect(&self, rect: PdfRect) -> Result<PdfPageTextChars<'_>, PdfiumError> {
        let tolerance_x = rect.width() / 2.0;
        let tolerance_y = rect.height() / 2.0;
        let center_height = rect.bottom() + tolerance_y;

        match (
            Self::get_char_index_near_point(
                self.text_page_handle(),
                rect.left(),
                tolerance_x,
                center_height,
                tolerance_y,
                self.bindings(),
            ),
            Self::get_char_index_near_point(
                self.text_page_handle(),
                rect.right(),
                tolerance_x,
                center_height,
                tolerance_y,
                self.bindings(),
            ),
        ) {
            (Some(start), Some(end)) => Ok(PdfPageTextChars::new(
                self.page.document_handle(),
                self.page.page_handle(),
                self.text_page_handle(),
                (start as i32..=end as i32 + 1).collect(),
                self.bindings,
            )),
            (Some(start), None) => Ok(PdfPageTextChars::new(
                self.page.document_handle(),
                self.page.page_handle(),
                self.text_page_handle(),
                (start as i32..=start as i32 + 1).collect(),
                self.bindings,
            )),
            (None, Some(end)) => Ok(PdfPageTextChars::new(
                self.page.document_handle(),
                self.page.page_handle(),
                self.text_page_handle(),
                (end as i32..=end as i32 + 1).collect(),
                self.bindings,
            )),
            _ => Err(PdfiumError::NoCharsInRect),
        }
    }

    /// Returns the character near to the given x and y positions on the containing [PdfPage],
    /// if any. The returned character will be no further from the given positions than the given
    /// tolerance values.
    pub(crate) fn get_char_index_near_point(
        text_page_handle: FPDF_TEXTPAGE,
        x: PdfPoints,
        tolerance_x: PdfPoints,
        y: PdfPoints,
        tolerance_y: PdfPoints,
        bindings: &dyn PdfiumLibraryBindings,
    ) -> Option<PdfPageTextCharIndex> {
        match bindings.FPDFText_GetCharIndexAtPos(
            text_page_handle,
            x.value as c_double,
            y.value as c_double,
            tolerance_x.value as c_double,
            tolerance_y.value as c_double,
        ) {
            -1 => None, // No character at position within tolerances
            -3 => None, // An error occurred, but we'll eat it
            index => Some(index as PdfPageTextCharIndex),
        }
    }

    /// Returns all characters that lie within the containing [PdfPage], in the order in which
    /// they are defined in the document, concatenated into a single string.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    pub fn all(&self) -> String {
        self.inside_rect(self.page.page_size())
    }

    /// Returns all characters that lie within the bounds of the given [PdfRect] in the
    /// containing [PdfPage], in the order in which they are defined in the document,
    /// concatenated into a single string.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    pub fn inside_rect(&self, rect: PdfRect) -> String {
        // Retrieving the bounded text from Pdfium is a two-step operation. First, we call
        // FPDFText_GetBoundedText() with a null buffer; this will retrieve the length of
        // the bounded text in _characters_ (not _bytes_!). If the length is zero, then there is
        // no text within the given rectangle's boundaries.

        // If the length is non-zero, then we reserve a buffer (sized in words rather than bytes,
        // to allow for two bytes per character) and call FPDFText_GetBoundedText() again with a
        // pointer to the buffer; this will write the bounded text to the buffer in UTF16-LE format.

        let left = rect.left().value as f64;

        let top = rect.top().value as f64;

        let right = rect.right().value as f64;

        let bottom = rect.bottom().value as f64;

        let chars_count =
            self.bindings()
                .FPDFText_GetBoundedText(self.text_page_handle(), left, top, right, bottom, null_mut(), 0);

        if chars_count == 0 {
            // No text lies within the given rectangle.

            return String::new();
        }

        let mut buffer = create_sized_buffer(chars_count as usize);

        let result = self.bindings().FPDFText_GetBoundedText(
            self.text_page_handle(),
            left,
            top,
            right,
            bottom,
            buffer.as_mut_ptr(),
            chars_count,
        );

        assert_eq!(result, chars_count);

        get_string_from_pdfium_utf16le_bytes(cast_slice(buffer.as_slice()).to_vec()).unwrap_or_default()
    }

    /// Returns all characters assigned to the given [PdfPageTextObject] in this [PdfPageText] object,
    /// concatenated into a single string.
    pub fn for_object(&self, object: &PdfPageTextObject) -> String {
        // Retrieving the string value from Pdfium is a two-step operation. First, we call
        // FPDFTextObj_GetText() with a null buffer; this will retrieve the length of
        // the text in bytes, assuming the page object exists. If the length is zero,
        // then there is no text.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFTextObj_GetText() again with a pointer to the buffer;
        // this will write the text for the page object into the buffer.

        let buffer_length =
            self.bindings()
                .FPDFTextObj_GetText(object.object_handle(), self.text_page_handle(), null_mut(), 0);

        if buffer_length == 0 {
            // There is no text.

            return String::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings().FPDFTextObj_GetText(
            object.object_handle(),
            self.text_page_handle(),
            buffer.as_mut_ptr() as *mut FPDF_WCHAR,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default()
    }

    /// Returns all characters that lie within the bounds of the given [PdfPageAnnotation] in the
    /// containing [PdfPage], in the order in which they are defined in the document,
    /// concatenated into a single string.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    #[inline]
    pub fn for_annotation(&self, annotation: &PdfPageAnnotation) -> Result<String, PdfiumError> {
        let bounds = annotation.bounds()?;

        Ok(self.inside_rect(bounds))
    }

    /// Starts a search for the given text string, returning a new [PdfPageTextSearch]
    /// object that can be used to step through the search results.
    #[inline]
    pub fn search(&self, text: &str, options: &PdfSearchOptions) -> Result<PdfPageTextSearch<'_>, PdfiumError> {
        self.search_from(text, options, 0)
    }

    /// Starts a search for the given test string from the given character position,
    /// returning a new [PdfPageTextSearch] object that can be used to step through
    /// the search results.
    pub fn search_from(
        &self,
        text: &str,
        options: &PdfSearchOptions,
        index: PdfPageTextCharIndex,
    ) -> Result<PdfPageTextSearch<'_>, PdfiumError> {
        if text.is_empty() {
            Err(PdfiumError::TextSearchTargetIsEmpty)
        } else {
            Ok(PdfPageTextSearch::from_pdfium(
                self.bindings().FPDFText_FindStart(
                    self.text_page_handle(),
                    get_pdfium_utf16le_bytes_from_str(text).as_ptr() as FPDF_WIDESTRING,
                    options.as_pdfium(),
                    index as c_int,
                ),
                self,
                self.bindings(),
            ))
        }
    }
}

impl<'a> Display for PdfPageText<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.all().as_str())
    }
}

impl<'a> Drop for PdfPageText<'a> {
    /// Closes the [PdfPageText] collection, releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings().FPDFText_ClosePage(self.text_page_handle());
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::ffi::OsStr;
    use std::fs;

    use crate::prelude::*;
    use crate::utils::test::{test_bind_to_pdfium, test_fixture_path};

    #[test]
    fn test_overlapping_chars_results() -> Result<(), PdfiumError> {
        // Test to make sure the result of the .chars_for_object() function returns the
        // correct results in the event of overlapping text objects.
        // For more details, see: https://github.com/ajrcarey/pdfium-render/issues/98

        let pdfium = test_bind_to_pdfium();

        // Create a new document with three overlapping text objects.

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document.pages_mut().create_page_at_start(PdfPagePaperSize::a4())?;

        let font = document.fonts_mut().courier();

        let txt1 = page.objects_mut().create_text_object(
            PdfPoints::ZERO,
            PdfPoints::ZERO,
            "AAAAAA",
            font,
            PdfPoints::new(10.0),
        )?;

        let txt2 = page.objects_mut().create_text_object(
            PdfPoints::ZERO,
            PdfPoints::ZERO,
            "BBBBBB",
            font,
            PdfPoints::new(10.0),
        )?;

        let txt3 = page.objects_mut().create_text_object(
            PdfPoints::ZERO,
            PdfPoints::ZERO,
            "CDCDCDE",
            font,
            PdfPoints::new(10.0),
        )?;

        let page_text = page.text()?;

        // Check the results for all three objects are not affected by overlapping.

        assert!(test_one_overlapping_text_object_results(&txt1, &page_text, "AAAAAA")?);
        assert!(test_one_overlapping_text_object_results(&txt2, &page_text, "BBBBBB")?);
        assert!(test_one_overlapping_text_object_results(&txt3, &page_text, "CDCDCDE")?);

        Ok(())
    }

    fn test_one_overlapping_text_object_results(
        object: &PdfPageObject,
        page_text: &PdfPageText,
        expected: &str,
    ) -> Result<bool, PdfiumError> {
        if let Some(txt) = object.as_text_object() {
            assert_eq!(txt.text().trim(), expected);
            assert_eq!(page_text.for_object(txt).trim(), expected);

            for (index, char) in txt.chars(page_text)?.iter().enumerate() {
                assert_eq!(txt.text().chars().nth(index), char.unicode_char());
                assert_eq!(expected.chars().nth(index), char.unicode_char());
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[test]
    fn test_text_chars_results_equality() -> Result<(), PdfiumError> {
        // For all available test documents, check that the results of
        // PdfPageObjectText::text() and PdfPageObjectText::chars() match.

        let pdfium = test_bind_to_pdfium();

        let fixture_dir = test_fixture_path("");
        let samples = fs::read_dir(&fixture_dir)
            .unwrap()
            .filter_map(|entry| match entry {
                Ok(e) => Some(e.path()),
                Err(_) => None,
            })
            .filter(|path| path.extension() == Some(OsStr::new("pdf")))
            .collect::<Vec<_>>();

        assert!(!samples.is_empty());

        for sample in samples {
            println!("Testing all text objects in file {}", sample.display());

            let document = pdfium.load_pdf_from_file(&sample, None)?;

            for page in document.pages().iter() {
                let text = page.text()?;

                for object in page.objects().iter() {
                    if let Some(obj) = object.as_text_object() {
                        let chars = obj
                            .chars(&text)?
                            .iter()
                            .filter_map(|char| char.unicode_string())
                            .join("");

                        assert_eq!(obj.text().trim(), chars.replace("\0", "").trim());
                    }
                }
            }
        }

        Ok(())
    }
}
