//! Defines the [PdfParagraph] struct, exposing functionality related to a group of
//! styled text strings that should be laid out together on a `PdfPage` as single paragraph.

use crate::bindgen::FPDF_PAGEOBJECT;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::PdfDocument;
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::text::PdfPageTextObject;
use crate::pdf::document::page::object::{PdfPageObject, PdfPageObjectCommon};
use crate::pdf::font::PdfFont;
use crate::pdf::points::PdfPoints;
use itertools::Itertools;
use maybe_owned::MaybeOwned;
use std::cmp::Ordering;

/// A single styled string in a [PdfParagraph].
pub struct PdfStyledString<'a> {
    text: String,
    font: MaybeOwned<'a, PdfFont<'a>>,
    font_size: PdfPoints,
}

impl<'a> PdfStyledString<'a> {
    /// Creates a new [PdfStyledString] from the given arguments.
    #[inline]
    pub fn new(text: String, font: &'a PdfFont<'a>, font_size: PdfPoints) -> Self {
        PdfStyledString {
            text,
            font: MaybeOwned::Borrowed(font),
            font_size,
        }
    }

    /// Creates a new [PdfStyledString] from the given [PdfPageTextObject].
    #[inline]
    pub fn from_text_object(text_object: &'a PdfPageTextObject<'a>) -> Self {
        PdfStyledString {
            text: text_object.text(),
            font: MaybeOwned::Owned(text_object.font()),
            font_size: text_object.unscaled_font_size(),
        }
    }

    /// Adds the given string to the text in this [PdfStyledString]. The given separator will be used
    /// to separate the existing text in this [PdfStyledString] from the given string.
    #[inline]
    pub(crate) fn push(&mut self, text: impl ToString, separator: &str) {
        if !self.text.ends_with(separator) {
            self.text.push_str(separator);
        }

        self.text.push_str(text.to_string().as_str());
    }

    /// Returns the text in this [PdfStyledString].
    #[inline]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// Returns the [PdfFont] used to style this [PdfStyledString].
    #[inline]
    pub fn font(&self) -> &PdfFont<'_> {
        self.font.as_ref()
    }

    /// Returns the font size used to style this [PdfStyledString].
    #[inline]
    pub fn font_size(&self) -> PdfPoints {
        self.font_size
    }

    /// Returns `true` if the font and font size of this [PdfStyledString] is the same as
    /// that of the given string.
    #[inline]
    pub fn does_match_string_styling(&self, other: &PdfStyledString) -> bool {
        self.does_match_raw_styling(other.font_size(), other.font())
    }

    /// Returns `true` if the font and font size of this [PdfStyledString] is the same as
    /// that of the given [PdfPageTextObject].
    #[inline]
    pub fn does_match_object_styling(&self, other: &PdfPageTextObject) -> bool {
        self.does_match_raw_styling(other.unscaled_font_size(), &other.font())
    }

    fn does_match_raw_styling(&self, other_font_size: PdfPoints, other_font: &PdfFont) -> bool {
        // It's more expensive to try to match the fonts based on name, so we try to match
        // based on FPDF_FONT handles first.

        if self.font_size() != other_font_size {
            return false;
        }

        let this_font = self.font();

        if this_font.handle() != other_font.handle() {
            return false;
        }

        let this_font_name = this_font.family();

        let other_font_name = other_font.family();

        if this_font_name.is_empty() && other_font_name.is_empty() {
            // We can't distinguish based on font names, and the sizes and font handles are identical,
            // so best guess is the styling matches.

            return true;
        }

        (!this_font_name.is_empty() || !other_font_name.is_empty()) && this_font_name == other_font_name
    }

    /// Creates a new [PdfPageTextObject] from this styled string, using the Pdfium bindings in
    /// the given document.
    #[inline]
    pub fn as_text_object(&self, document: &PdfDocument<'a>) -> Result<PdfPageTextObject<'a>, PdfiumError> {
        PdfPageTextObject::new(document, self.text(), self.font(), self.font_size())
    }
}

/// A single fragment in a [PdfParagraph]. The fragment may later be split into sub-fragments when
/// assembling the [PdfParagraph] into lines.
enum PdfParagraphFragment<'a> {
    StyledString(PdfStyledString<'a>),
    LineBreak(PdfLineAlignment),
    NonTextObject(FPDF_PAGEOBJECT),
}

/// Controls the overflow behaviour of a [PdfParagraph] that, due to changes in its content,
/// needs to overflow the maximum bounds of the original page objects from which it was defined.
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(unused)]
pub enum PdfParagraphOverflowBehaviour {
    /// The maximum line width will be adjusted so that the paragraph's height stays the same.
    FixHeightExpandWidth,

    /// The paragraph's height will expand so that the paragraph's maximum width stays the same.
    FixWidthExpandHeight,

    /// Content overflowing the paragraph's width and height will be clipped.
    Clip,
}

/// Controls the line alignment behaviour of a [PdfParagraph].
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(unused)]
pub enum PdfParagraphAlignment {
    /// All lines will be non-justified, aligned to the left.
    LeftAlign,

    /// All lines will be non-justified, aligned to the right.
    RightAlign,

    /// All lines will be non-justified and centered.
    Center,

    /// All lines except the last will be justified.
    Justify,

    /// All lines, including the last, will be justified.
    ForceJustify,
}

/// The paragraph-relative alignment of a single [PdfLine].
#[derive(Copy, Clone, Debug, PartialEq)]
enum PdfLineAlignment {
    None,
    LeftAlign,
    RightAlign,
    Center,
    Justify,
}

/// A span of paragraph fragments that make up one line in a [PdfParagraph].
struct PdfLine<'a> {
    alignment: PdfLineAlignment,
    bottom: PdfPoints,
    left: PdfPoints,
    width: PdfPoints,
    fragments: Vec<PdfParagraphFragment<'a>>,
}

impl<'a> PdfLine<'a> {
    #[inline]
    fn new(
        alignment: PdfLineAlignment,
        bottom: PdfPoints,
        left: PdfPoints,
        width: PdfPoints,
        fragments: Vec<PdfParagraphFragment<'a>>,
    ) -> Self {
        PdfLine {
            alignment,
            bottom,
            left,
            width,
            fragments,
        }
    }
}

/// A group of [PdfPageTextObject] objects contained in the same `PdfPageObjects` collection
/// that should be laid out together as a single paragraph.
///
/// Text layout in PDF files is handled entirely by text objects. Each text object contains
/// a single span of text that is styled consistently and can be at most a single line long.
/// Multiple text objects stitched together visually at the time the page is generated are
/// interpreted by the reader as paragraphs, but there is no concept in the PDF file format
/// of a multi-line text block, and there is no native functionality for retrieving a single
/// paragraph from its constituent text objects. This makes it difficult to work with long spans
/// of text.
///
/// The [PdfParagraph] is an attempt to improve multi-line text handling. Paragraphs can
/// be created from existing groups of page objects, or created by scratch; once created, text in
/// a paragraph can be edited and re-formatted, and then used to generate a group of text objects
/// that can be placed on a page.
pub struct PdfParagraph<'a> {
    fragments: Vec<PdfParagraphFragment<'a>>,
    bottom: Option<PdfPoints>,
    left: Option<PdfPoints>,
    max_width: Option<PdfPoints>,
    #[allow(unused)]
    max_height: Option<PdfPoints>,
    overflow: PdfParagraphOverflowBehaviour,
    alignment: PdfParagraphAlignment,
    #[allow(unused)]
    first_line_indent: PdfPoints,
}

impl<'a> PdfParagraph<'a> {
    // TODO: lifetime issues, using iterator is a possibility but PdfPage::objects().iter()
    // and PdfPageGroupObject::iter() return iterators over PdfPageObject<'a> whereas
    // &[PdfPageObject<'a>] returns an iterator over &PdfPageObject<'a>

    // /// Creates a set of one or more [PdfParagraph] objects from the objects on the given [PdfPage].
    // #[inline]
    // pub fn from_page(page: &'a PdfPage<'a>) -> Vec<PdfParagraph<'a>> {
    //     let objects = page.objects().iter().collect::<Vec<_>>();
    //
    //     Self::from_iter(objects.as_slice())
    // }
    //
    // #[inline]
    // pub fn from_group(group: &'a PdfPageGroupObject<'a>) -> Vec<PdfParagraph<'a>> {
    //     let objects = group.iter().collect::<Vec<_>>();
    //
    //     Self::from_iter(objects.as_slice())
    // }

    /// Creates a set of one or more [PdfParagraph] objects from the given slice of page objects.
    pub fn from_objects(objects: &'a [PdfPageObject<'a>]) -> Vec<PdfParagraph<'a>> {
        let mut lines = Vec::new();

        let mut current_line_fragments = Vec::new();

        let mut objects_bottom = None;

        let mut objects_top = None;

        let mut objects_left = None;

        let mut objects_right = None;

        // Extract positions from all given objects, so we can attempt to arrange them
        // in reading order irrespective of their original positions.

        let positioned_objects = objects
            .iter()
            .map(|object| {
                let object_bottom = object.bounds().map(|bounds| bounds.bottom()).unwrap_or(PdfPoints::ZERO);

                match objects_bottom {
                    Some(paragraph_bottom) => {
                        if paragraph_bottom > object_bottom {
                            objects_bottom = Some(object_bottom);
                        }
                    }
                    None => objects_bottom = Some(object_bottom),
                };

                let object_top = object.bounds().map(|bounds| bounds.top()).unwrap_or(PdfPoints::ZERO);

                match objects_top {
                    Some(paragraph_top) => {
                        if paragraph_top < object_top {
                            objects_top = Some(object_top);
                        }
                    }
                    None => objects_top = Some(object_top),
                }

                let _object_height = object.bounds().map(|bounds| bounds.height()).unwrap_or(PdfPoints::ZERO);

                let object_left = object.bounds().map(|bounds| bounds.left()).unwrap_or(PdfPoints::ZERO);

                match objects_left {
                    Some(paragraph_left) => {
                        if paragraph_left > object_left {
                            objects_left = Some(object_left);
                        }
                    }
                    None => objects_left = Some(object_left),
                }

                let object_right = object.bounds().map(|bounds| bounds.right()).unwrap_or(PdfPoints::ZERO);

                match objects_right {
                    Some(paragraph_right) => {
                        if paragraph_right < object_right {
                            objects_right = Some(object_right);
                        }
                    }
                    None => objects_right = Some(object_right),
                }

                (object_bottom, object_top, object_left, object_right, object)
            })
            .sorted_by(|a, b| {
                let (a_bottom, _a_top, _, a_right) = (a.0, a.1, a.2, a.3);

                let (_b_bottom, b_top, b_left, _) = (b.0, b.1, b.2, b.3);

                // Keep track of the paragraph maximum bounds as we examine objects.

                // Sort by position: vertically first, then horizontally.

                if b_top > a_bottom {
                    // Object a is in a line lower down the page than object b.

                    Ordering::Greater
                } else if b_top < a_bottom {
                    // Object a is in a line higher up the page than object b.

                    Ordering::Less
                } else if a_right < b_left {
                    // Objects a and b are on the same line, and object a is closer to the left edge
                    // of the line than object b.

                    Ordering::Less
                } else {
                    // Objects a and b are on the same line, and object a is closer to the right edge
                    // of the line than object b.

                    Ordering::Greater
                }
            })
            .collect::<Vec<_>>();

        let paragraph_left = objects_left.unwrap_or(PdfPoints::ZERO);
        let paragraph_right = objects_right.unwrap_or(paragraph_left);

        let mut current_line_bottom = PdfPoints::ZERO;
        let mut current_line_left = PdfPoints::ZERO;
        let current_line_right = PdfPoints::ZERO;
        let mut current_line_alignment = PdfLineAlignment::None;

        let mut last_object_bottom = None;
        let mut last_object_height = None;
        let mut last_object_left = None;
        let mut last_object_right = None;

        for (bottom, top, left, right, object) in positioned_objects.iter() {
            let top = *top;

            let bottom = *bottom;

            let left = *left;

            let right = *right;

            if last_object_left.is_none() || left < last_object_left.unwrap() {
                // We're at the start of a new line. Does this line break indicate a new paragraph?

                let next_line_alignment = Self::guess_line_alignment(
                    last_object_left,
                    last_object_right,
                    left,
                    right,
                    paragraph_left,
                    paragraph_right,
                );

                if next_line_alignment != current_line_alignment
                    || last_object_bottom.unwrap_or(PdfPoints::ZERO) - last_object_height.unwrap_or(PdfPoints::ZERO)
                        > top
                {
                    // Yes, this line break probably indicates a new paragraph.

                    lines.push(PdfLine::new(
                        current_line_alignment,
                        current_line_bottom,
                        current_line_left,
                        right - current_line_left,
                        current_line_fragments,
                    ));

                    current_line_fragments = vec![PdfParagraphFragment::LineBreak(current_line_alignment)];
                    current_line_left = left;
                    current_line_bottom = bottom;
                    current_line_alignment = next_line_alignment;
                } else {
                    // The line break probably just represents a carriage-return rather than the
                    // deliberate end of a paragraph.
                }
            }

            last_object_left = Some(left);
            last_object_right = Some(right);
            last_object_bottom = Some(bottom);
            last_object_height = Some(top - bottom);

            if let Some(object) = object.as_text_object() {
                // If the styling of this object is the same as the last styled string fragment,
                // then append the text of this object to the last fragment; otherwise, start a
                // new text fragment.

                if let Some(PdfParagraphFragment::StyledString(last_string)) = current_line_fragments.last_mut() {
                    if last_string.does_match_object_styling(object) {
                        // The styles of the two text objects are the same. They should be
                        // merged them into the same styled string - but should they
                        // be part of the same word, or separate words?

                        let separator = if let Ok(bounds) = object.bounds() {
                            if let Some(last_object_right) = last_object_right {
                                if last_object_right > bounds.left() {
                                    // The last and current objects are touching.
                                    // Assume they're part of the same word, despite being
                                    // in separate objects.

                                    ""
                                } else {
                                    // The last and current objects are separated.

                                    " "
                                }
                            } else {
                                // We're at the start of a line.

                                ""
                            }
                        } else {
                            // Cannot measure the bounds of the current object; by default,
                            // assume it's separated from the last object.

                            " "
                        };

                        last_string.push(object.text(), separator);
                    } else {
                        // The styles of the two text objects are different, so they can't be merged.

                        current_line_fragments.push(PdfParagraphFragment::StyledString(
                            PdfStyledString::from_text_object(object),
                        ));
                    }
                } else {
                    // The last fragment wasn't a string fragment, so we have to start a new fragment.

                    current_line_fragments.push(PdfParagraphFragment::StyledString(PdfStyledString::from_text_object(
                        object,
                    )));
                }
            } else {
                current_line_fragments.push(PdfParagraphFragment::NonTextObject(object.object_handle()));
            }
        }

        lines.push(PdfLine::new(
            current_line_alignment,
            current_line_bottom,
            current_line_left,
            current_line_right - current_line_left,
            current_line_fragments,
        ));

        // Assemble lines into paragraphs.

        let mut paragraphs = Vec::new();

        let mut current_paragraph_fragments = Vec::new();

        let mut current_paragraph_bottom = None;

        let mut current_paragraph_left = None;

        let mut current_paragraph_right = None;

        let mut current_paragraph_first_line_left = None;

        let mut last_line_alignment = lines
            .first()
            .map(|line| line.alignment)
            .unwrap_or(PdfLineAlignment::None);

        let mut first_line_alignment = last_line_alignment;

        for mut line in lines.drain(..) {
            if line.alignment != last_line_alignment {
                // TODO: this won't work as expected for non-force-justified paragraphs
                // where the last line in the paragraph is left-aligned, not justified

                // Finalize the current paragraph...

                if !current_paragraph_fragments.is_empty() {
                    paragraphs.push(Self::paragraph_from_lines(
                        current_paragraph_fragments,
                        current_paragraph_bottom,
                        current_paragraph_left,
                        current_paragraph_right,
                        current_paragraph_first_line_left,
                        first_line_alignment,
                        last_line_alignment,
                    ));

                    // ... and start a new paragraph.

                    current_paragraph_fragments = Vec::new();
                    current_paragraph_bottom = None;
                    current_paragraph_left = None;
                    current_paragraph_right = None;
                    current_paragraph_first_line_left = None;
                    first_line_alignment = last_line_alignment
                }
            }

            current_paragraph_fragments.append(&mut line.fragments);

            last_line_alignment = line.alignment;

            if let Some(paragraph_left) = current_paragraph_left {
                if line.left < paragraph_left {
                    current_paragraph_left = Some(line.left);
                }
            } else {
                current_paragraph_left = Some(line.left);
            }

            if let Some(paragraph_right) = current_paragraph_right {
                if line.left + line.width > paragraph_right {
                    current_paragraph_right = Some(line.left + line.width);
                }
            } else {
                current_paragraph_right = Some(line.left + line.width);
            }

            if let Some(paragraph_bottom) = current_paragraph_bottom {
                if line.bottom < paragraph_bottom {
                    current_paragraph_bottom = Some(line.bottom);
                }
            } else {
                current_paragraph_bottom = Some(line.bottom);
            }

            if current_paragraph_first_line_left.is_none() {
                current_paragraph_first_line_left = Some(line.left);
            }
        }

        // Finalize the last paragraph.

        paragraphs.push(Self::paragraph_from_lines(
            current_paragraph_fragments,
            current_paragraph_bottom,
            current_paragraph_left,
            current_paragraph_right,
            current_paragraph_first_line_left,
            first_line_alignment,
            last_line_alignment,
        ));

        paragraphs
    }

    fn paragraph_from_lines(
        fragments: Vec<PdfParagraphFragment<'a>>,
        bottom: Option<PdfPoints>,
        left: Option<PdfPoints>,
        right: Option<PdfPoints>,
        first_line_left: Option<PdfPoints>,
        first_line_alignment: PdfLineAlignment,
        last_line_alignment: PdfLineAlignment,
    ) -> PdfParagraph<'a> {
        PdfParagraph {
            fragments,
            bottom,
            left,
            max_width: match (left, right) {
                (Some(left), Some(right)) => Some(right - left),
                _ => None,
            },
            max_height: None,
            overflow: PdfParagraphOverflowBehaviour::FixWidthExpandHeight,
            alignment: if first_line_alignment == last_line_alignment
                && first_line_alignment == PdfLineAlignment::Justify
            {
                // Every line in the paragraph, including the last line, is justified.

                PdfParagraphAlignment::ForceJustify
            } else {
                match first_line_alignment {
                    PdfLineAlignment::None | PdfLineAlignment::LeftAlign => PdfParagraphAlignment::LeftAlign,
                    PdfLineAlignment::RightAlign => PdfParagraphAlignment::RightAlign,
                    PdfLineAlignment::Center => PdfParagraphAlignment::Center,
                    PdfLineAlignment::Justify => PdfParagraphAlignment::Justify,
                }
            },
            first_line_indent: match (first_line_left, left) {
                (Some(first_line_left), Some(left)) => first_line_left - left,
                _ => PdfPoints::ZERO,
            },
        }
    }

    fn guess_line_alignment(
        previous_line_left: Option<PdfPoints>,
        previous_line_right: Option<PdfPoints>,
        line_left: PdfPoints,
        line_right: PdfPoints,
        paragraph_left: PdfPoints,
        paragraph_right: PdfPoints,
    ) -> PdfLineAlignment {
        const ALIGNMENT_THRESHOLD: f32 = 2.0;

        // Is this line in alignment with the previous line?

        if let (Some(previous_line_left), Some(previous_line_right)) = (previous_line_left, previous_line_right) {
            let is_aligned_left = (previous_line_left.value - line_left.value).abs() < ALIGNMENT_THRESHOLD;

            let is_aligned_right = (previous_line_right.value - line_right.value).abs() < ALIGNMENT_THRESHOLD;

            match (is_aligned_left, is_aligned_right) {
                (true, true) => PdfLineAlignment::Justify,
                (true, false) => PdfLineAlignment::LeftAlign,
                (false, true) => PdfLineAlignment::RightAlign,
                (false, false) => PdfLineAlignment::Center,
            }
        } else {
            let is_aligned_left = (paragraph_left.value - line_left.value).abs() < ALIGNMENT_THRESHOLD;

            let is_aligned_right = (paragraph_right.value - line_right.value).abs() < ALIGNMENT_THRESHOLD;

            match (is_aligned_left, is_aligned_right) {
                (true, true) => PdfLineAlignment::Justify,
                (true, false) => PdfLineAlignment::LeftAlign,
                (false, true) => PdfLineAlignment::RightAlign,
                (false, false) => PdfLineAlignment::Center,
            }
        }
    }

    /// Creates a new, empty [PdfParagraph] with the given maximum line width,
    /// overflow, and alignment settings.
    #[inline]
    pub fn empty(
        maximum_width: PdfPoints,
        overflow: PdfParagraphOverflowBehaviour,
        alignment: PdfParagraphAlignment,
    ) -> Self {
        PdfParagraph {
            fragments: vec![],
            bottom: None,
            left: None,
            max_width: Some(maximum_width),
            max_height: None,
            overflow,
            alignment,
            first_line_indent: PdfPoints::ZERO,
        }
    }

    /// Returns `true` if this [PdfParagraph] contains no fragments.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

    /// Adds a new fragment containing the given styled string to this paragraph.
    #[inline]
    pub fn push(&mut self, string: PdfStyledString<'a>) {
        // If the styling of this object is the same as the last styled string fragment,
        // then append the text of this object to the last fragment; otherwise, start a
        // new text fragment.

        if let Some(PdfParagraphFragment::StyledString(last_string)) = self.fragments.last_mut() {
            if last_string.does_match_string_styling(&string) {
                // The styles of the two styled strings are the same. Merge them into the same
                // styled string.

                last_string.push(string.text(), " ");
            } else {
                // The styles of the two styled strings are different, so they can't be merged.

                self.fragments.push(PdfParagraphFragment::StyledString(string));
            }
        } else {
            // The last fragment wasn't a string fragment.

            self.fragments.push(PdfParagraphFragment::StyledString(string));
        }
    }

    /// Returns the maximum line width of this paragraph.
    #[inline]
    pub fn maximum_width(&self) -> PdfPoints {
        self.max_width.unwrap_or(PdfPoints::ZERO)
    }

    /// Sets the maximum line width of this paragraph to the given value.
    #[inline]
    pub fn set_maximum_width(&mut self, width: PdfPoints) {
        self.max_width = Some(width);
    }

    /// Sets the maximum height of this paragraph to the given value.
    #[inline]
    pub fn set_maximum_height(&mut self, height: PdfPoints) {
        self.max_height = Some(height);
    }

    /// Returns the text contained within all text fragments in this paragraph.
    #[inline]
    pub fn text(&self) -> String {
        self.fragments
            .iter()
            .filter_map(|fragment| match fragment {
                PdfParagraphFragment::StyledString(string) => Some(string.text.as_str()),
                PdfParagraphFragment::LineBreak(_) => Some("\n"),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Returns the text contained within all text fragments in this paragraph,
    /// separating each text fragment with the given separator.
    pub fn text_separated(&self, separator: &str) -> String {
        self.fragments
            .iter()
            .filter_map(|fragment| match fragment {
                PdfParagraphFragment::StyledString(string) => Some(string.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// Assembles the fragments in this paragraph into lines, taking into account the paragraph's
    /// current sizing, overflow, indent, and alignment settings. Consumes the paragraph and
    /// returns the assembled lines.
    fn into_lines(self) -> Vec<PdfLine<'a>> {
        let mut lines: Vec<PdfLine<'a>> = Vec::new();
        let mut current_fragments: Vec<PdfParagraphFragment<'a>> = Vec::new();
        let mut current_width = PdfPoints::ZERO;
        let current_bottom = self.bottom.unwrap_or(PdfPoints::ZERO);
        let current_left = self.left.unwrap_or(PdfPoints::ZERO);

        let effective_max_width = self.max_width.unwrap_or(PdfPoints::new(f32::MAX));

        for fragment in self.fragments {
            match fragment {
                PdfParagraphFragment::LineBreak(alignment) => {
                    if !current_fragments.is_empty() {
                        lines.push(PdfLine::new(
                            alignment,
                            current_bottom,
                            current_left,
                            current_width,
                            std::mem::take(&mut current_fragments),
                        ));
                        current_width = PdfPoints::ZERO;
                    }
                }
                PdfParagraphFragment::StyledString(ref styled) => {
                    // Estimate width from text length and font size
                    let estimated_width = PdfPoints::new(styled.text().len() as f32 * styled.font_size().value * 0.5);

                    if current_width.value + estimated_width.value > effective_max_width.value
                        && !current_fragments.is_empty()
                    {
                        // Line break needed
                        lines.push(PdfLine::new(
                            PdfLineAlignment::None,
                            current_bottom,
                            current_left,
                            current_width,
                            std::mem::take(&mut current_fragments),
                        ));
                        current_width = PdfPoints::ZERO;
                    }

                    current_width = PdfPoints::new(current_width.value + estimated_width.value);
                    current_fragments.push(fragment);
                }
                PdfParagraphFragment::NonTextObject(_) => {
                    current_fragments.push(fragment);
                }
            }
        }

        // Flush remaining fragments
        if !current_fragments.is_empty() {
            lines.push(PdfLine::new(
                PdfLineAlignment::None,
                current_bottom,
                current_left,
                current_width,
                current_fragments,
            ));
        }

        lines
    }

    /// Assembles the fragments in this paragraph into lines, taking into account the paragraph's
    /// current sizing, overflow, indent, and alignment settings, and generates new page objects for
    /// each line, adding all generated page objects to a new [PdfPageGroupObject].
    ///
    /// Note: Full implementation requires a `PdfDocument` reference to create text objects.
    /// This is currently a placeholder for future layout rendering support.
    pub fn as_group(&self) -> Result<(), PdfiumError> {
        Err(PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::Unknown))
    }
}

#[cfg(test)]
mod tests {
    use crate::pdf::document::page::paragraph::PdfParagraph;
    use crate::prelude::*;
    use crate::utils::test::{test_bind_to_pdfium, test_fixture_path};

    #[test]
    fn test_paragraph_construction() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        let document = pdfium.load_pdf_from_file(&test_fixture_path("text-test.pdf"), None)?;

        let page = document.pages().get(0)?;

        let objects = page.objects().iter().collect::<Vec<_>>();

        let paragraphs = PdfParagraph::from_objects(objects.as_slice());

        // Verify that paragraphs were constructed from the page objects
        assert!(
            !paragraphs.is_empty(),
            "Expected at least one paragraph from page objects"
        );

        // Verify text extraction works for each paragraph
        for paragraph in paragraphs.iter() {
            let text = paragraph.text();
            // Each paragraph should produce some text (the test fixture has text content)
            assert!(
                !text.trim().is_empty() || paragraph.is_empty(),
                "Non-empty paragraph should produce non-empty text"
            );
        }

        // Verify text_separated also works
        for paragraph in paragraphs.iter() {
            let separated = paragraph.text_separated(" ");
            let plain = paragraph.text();
            // Both text methods should return content for non-empty paragraphs
            if !paragraph.is_empty() {
                assert!(
                    !separated.is_empty() || !plain.is_empty(),
                    "Text extraction should return content for non-empty paragraphs"
                );
            }
        }

        Ok(())
    }
}
