//! Inline DOCX XML parser.
//!
//! Vendored and adapted from [docx-lite](https://github.com/v-lawyer/docx-lite) v0.2.0
//! (MIT OR Apache-2.0, V-Lawyer Team). See ATTRIBUTIONS.md for details.
//!
//! Changes from upstream:
//! - `Paragraph::to_text()` joins runs with `" "` instead of `""` (fixes #359)
//! - Adapted to use kreuzberg's existing `quick-xml` and `zip` versions
//! - Removed file-path based APIs (we only need bytes/reader)

use std::collections::HashMap;
use std::io::{Cursor, Read, Seek};

use quick_xml::Reader;
use quick_xml::events::Event;

// --- Types ---

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
    pub tables: Vec<Table>,
    pub lists: Vec<ListItem>,
    pub headers: Vec<HeaderFooter>,
    pub footers: Vec<HeaderFooter>,
    pub footnotes: Vec<Note>,
    pub endnotes: Vec<Note>,
}

#[derive(Debug, Clone, Default)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub style: Option<String>,
    pub numbering_id: Option<i64>,
    pub numbering_level: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct Run {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, Default)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Default)]
pub struct TableCell {
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub level: u32,
    pub list_type: ListType,
    pub number: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    Bullet,
    Numbered,
}

#[derive(Debug, Clone, Default)]
pub struct HeaderFooter {
    pub paragraphs: Vec<Paragraph>,
    pub tables: Vec<Table>,
    pub header_type: HeaderFooterType,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum HeaderFooterType {
    #[default]
    Default,
    First,
    Even,
    Odd,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub note_type: NoteType,
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteType {
    Footnote,
    Endnote,
}

// --- Impls ---

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_text(&self) -> String {
        let mut text = String::new();

        let mut list_index = 0;
        for paragraph in &self.paragraphs {
            if let (Some(_num_id), Some(_level)) = (paragraph.numbering_id, paragraph.numbering_level) {
                let para_text = paragraph.to_text();
                if !para_text.is_empty() {
                    text.push_str(&para_text);
                    text.push('\n');
                }
                list_index += 1;
                let _ = list_index; // suppress unused warning
            } else {
                let para_text = paragraph.to_text();
                if !para_text.is_empty() {
                    text.push_str(&para_text);
                    text.push('\n');
                }
            }
        }

        for table in &self.tables {
            for row in &table.rows {
                for cell in &row.cells {
                    for paragraph in &cell.paragraphs {
                        let para_text = paragraph.to_text();
                        if !para_text.is_empty() {
                            text.push_str(&para_text);
                            text.push('\t');
                        }
                    }
                }
                text.push('\n');
            }
            text.push('\n');
        }

        text
    }
}

impl Paragraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Join text runs with a space separator.
    ///
    /// In DOCX, separate `<w:r>` elements within the same paragraph represent
    /// distinct text runs (e.g. due to formatting changes). These runs need a
    /// space between them to produce readable text.
    ///
    /// Empty runs are filtered out to avoid double spaces.
    pub fn to_text(&self) -> String {
        self.runs
            .iter()
            .map(|run| run.text.as_str())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn add_run(&mut self, run: Run) {
        self.runs.push(run);
    }
}

impl Run {
    pub fn new(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HeaderFooter {
    pub fn extract_text(&self) -> String {
        let mut text = String::new();

        for paragraph in &self.paragraphs {
            let para_text = paragraph.to_text();
            if !para_text.is_empty() {
                text.push_str(&para_text);
                text.push('\n');
            }
        }

        for table in &self.tables {
            for row in &table.rows {
                for cell in &row.cells {
                    for paragraph in &cell.paragraphs {
                        let para_text = paragraph.to_text();
                        if !para_text.is_empty() {
                            text.push_str(&para_text);
                            text.push('\t');
                        }
                    }
                }
                text.push('\n');
            }
        }

        text
    }
}

// --- Parser ---

struct DocxParser<R: Read + Seek> {
    archive: zip::ZipArchive<R>,
}

impl<R: Read + Seek> DocxParser<R> {
    fn new(reader: R) -> Result<Self, DocxParseError> {
        let archive = zip::ZipArchive::new(reader)?;
        Ok(Self { archive })
    }

    fn parse(mut self) -> Result<Document, DocxParseError> {
        let mut document = Document::new();

        let document_xml = self.read_file("word/document.xml")?;
        self.parse_document_xml(&document_xml, &mut document)?;

        if let Ok(numbering_xml) = self.read_file("word/numbering.xml") {
            let numbering_defs = self.parse_numbering(&numbering_xml)?;
            self.process_lists(&mut document, &numbering_defs);
        }

        self.parse_headers_footers(&mut document)?;

        if let Ok(footnotes_xml) = self.read_file("word/footnotes.xml") {
            self.parse_notes(&footnotes_xml, &mut document.footnotes, NoteType::Footnote)?;
        }

        if let Ok(endnotes_xml) = self.read_file("word/endnotes.xml") {
            self.parse_notes(&endnotes_xml, &mut document.endnotes, NoteType::Endnote)?;
        }

        Ok(document)
    }

    fn read_file(&mut self, path: &str) -> Result<String, DocxParseError> {
        let mut file = self
            .archive
            .by_name(path)
            .map_err(|_| DocxParseError::FileNotFound(path.to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn parse_document_xml(&self, xml: &str, document: &mut Document) -> Result<(), DocxParseError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_run: Option<Run> = None;
        let mut current_table: Option<Table> = None;
        let mut current_row: Option<TableRow> = None;
        let mut current_cell: Option<TableCell> = None;
        let mut in_text = false;
        let mut in_table = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"w:p" => {
                        if in_table {
                            if current_cell.is_none() {
                                current_cell = Some(TableCell::default());
                            }
                        } else {
                            current_paragraph = Some(Paragraph::new());
                        }
                    }
                    b"w:numPr" => {
                        if let Some(ref mut para) = current_paragraph {
                            para.numbering_id = Some(1);
                            para.numbering_level = Some(0);
                        }
                    }
                    b"w:r" => {
                        current_run = Some(Run::default());
                    }
                    b"w:t" => {
                        in_text = true;
                    }
                    b"w:tbl" => {
                        in_table = true;
                        current_table = Some(Table::new());
                    }
                    b"w:tr" => {
                        current_row = Some(TableRow::default());
                    }
                    b"w:tc" => {
                        current_cell = Some(TableCell::default());
                    }
                    b"w:b" => {
                        if let Some(ref mut run) = current_run {
                            run.bold = true;
                        }
                    }
                    b"w:i" => {
                        if let Some(ref mut run) = current_run {
                            run.italic = true;
                        }
                    }
                    b"w:u" => {
                        if let Some(ref mut run) = current_run {
                            run.underline = true;
                        }
                    }
                    _ => {}
                },
                Ok(Event::Text(e)) => {
                    if in_text {
                        if let Some(ref mut run) = current_run {
                            let text = e.decode()?.into_owned();
                            run.text.push_str(&text);
                        }
                    }
                }
                Ok(Event::End(ref e)) => match e.name().as_ref() {
                    b"w:t" => {
                        in_text = false;
                    }
                    b"w:r" => {
                        if let Some(run) = current_run.take() {
                            if in_table {
                                if let Some(ref mut cell) = current_cell {
                                    if cell.paragraphs.is_empty() {
                                        cell.paragraphs.push(Paragraph::new());
                                    }
                                    if let Some(para) = cell.paragraphs.last_mut() {
                                        para.add_run(run);
                                    }
                                }
                            } else if let Some(ref mut para) = current_paragraph {
                                para.add_run(run);
                            }
                        }
                    }
                    b"w:p" => {
                        if in_table {
                            // handled via cell
                        } else if let Some(para) = current_paragraph.take() {
                            document.paragraphs.push(para);
                        }
                    }
                    b"w:tc" => {
                        if let Some(cell) = current_cell.take() {
                            if let Some(ref mut row) = current_row {
                                row.cells.push(cell);
                            }
                        }
                    }
                    b"w:tr" => {
                        if let Some(row) = current_row.take() {
                            if let Some(ref mut table) = current_table {
                                table.rows.push(row);
                            }
                        }
                    }
                    b"w:tbl" => {
                        in_table = false;
                        if let Some(table) = current_table.take() {
                            document.tables.push(table);
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_numbering(&self, xml: &str) -> Result<HashMap<i64, ListType>, DocxParseError> {
        let mut numbering_defs = HashMap::new();
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut current_num_id: Option<i64> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"w:num" {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"w:numId" {
                                if let Ok(id_str) = std::str::from_utf8(&attr.value) {
                                    current_num_id = id_str.parse().ok();
                                }
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"w:num" {
                        if let Some(id) = current_num_id {
                            numbering_defs.insert(id, ListType::Bullet);
                        }
                        current_num_id = None;
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(numbering_defs)
    }

    fn process_lists(&self, document: &mut Document, numbering_defs: &HashMap<i64, ListType>) {
        for paragraph in &document.paragraphs {
            if let (Some(num_id), Some(level)) = (paragraph.numbering_id, paragraph.numbering_level) {
                let list_type = numbering_defs.get(&num_id).cloned().unwrap_or(ListType::Bullet);

                let list_item = ListItem {
                    level: level as u32,
                    list_type,
                    number: None,
                    text: paragraph.to_text(),
                };

                document.lists.push(list_item);
            }
        }
    }

    fn parse_headers_footers(&mut self, document: &mut Document) -> Result<(), DocxParseError> {
        for i in 1..=3 {
            let header_path = format!("word/header{}.xml", i);
            if let Ok(header_xml) = self.read_file(&header_path) {
                let mut header = HeaderFooter::default();
                self.parse_header_footer_content(&header_xml, &mut header)?;
                document.headers.push(header);
            }

            let footer_path = format!("word/footer{}.xml", i);
            if let Ok(footer_xml) = self.read_file(&footer_path) {
                let mut footer = HeaderFooter::default();
                self.parse_header_footer_content(&footer_xml, &mut footer)?;
                document.footers.push(footer);
            }
        }

        Ok(())
    }

    fn parse_header_footer_content(&self, xml: &str, header_footer: &mut HeaderFooter) -> Result<(), DocxParseError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_run: Option<Run> = None;
        let mut in_text = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"w:p" => current_paragraph = Some(Paragraph::new()),
                    b"w:r" => current_run = Some(Run::default()),
                    b"w:t" => in_text = true,
                    _ => {}
                },
                Ok(Event::Text(e)) => {
                    if in_text {
                        if let Some(ref mut run) = current_run {
                            let text = e.decode()?.into_owned();
                            run.text.push_str(&text);
                        }
                    }
                }
                Ok(Event::End(ref e)) => match e.name().as_ref() {
                    b"w:t" => in_text = false,
                    b"w:r" => {
                        if let Some(run) = current_run.take() {
                            if let Some(ref mut para) = current_paragraph {
                                para.add_run(run);
                            }
                        }
                    }
                    b"w:p" => {
                        if let Some(para) = current_paragraph.take() {
                            header_footer.paragraphs.push(para);
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_notes(&self, xml: &str, notes: &mut Vec<Note>, note_type: NoteType) -> Result<(), DocxParseError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut current_note: Option<Note> = None;
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_run: Option<Run> = None;
        let mut in_text = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"w:footnote" | b"w:endnote" => {
                        let mut id = String::new();
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"w:id" {
                                id = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        current_note = Some(Note {
                            id,
                            note_type: note_type.clone(),
                            paragraphs: Vec::new(),
                        });
                    }
                    b"w:p" => current_paragraph = Some(Paragraph::new()),
                    b"w:r" => current_run = Some(Run::default()),
                    b"w:t" => in_text = true,
                    _ => {}
                },
                Ok(Event::Text(e)) => {
                    if in_text {
                        if let Some(ref mut run) = current_run {
                            let text = e.decode()?.into_owned();
                            run.text.push_str(&text);
                        }
                    }
                }
                Ok(Event::End(ref e)) => match e.name().as_ref() {
                    b"w:t" => in_text = false,
                    b"w:r" => {
                        if let Some(run) = current_run.take() {
                            if let Some(ref mut para) = current_paragraph {
                                para.add_run(run);
                            }
                        }
                    }
                    b"w:p" => {
                        if let Some(para) = current_paragraph.take() {
                            if let Some(ref mut note) = current_note {
                                note.paragraphs.push(para);
                            }
                        }
                    }
                    b"w:footnote" | b"w:endnote" => {
                        if let Some(note) = current_note.take() {
                            if note.id != "-1" && note.id != "0" {
                                notes.push(note);
                            }
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }
}

// --- Error ---

#[derive(Debug, thiserror::Error)]
enum DocxParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("Required file not found in DOCX: {0}")]
    FileNotFound(String),
}

// quick-xml's unescape returns an encoding error type
impl From<quick_xml::encoding::EncodingError> for DocxParseError {
    fn from(e: quick_xml::encoding::EncodingError) -> Self {
        DocxParseError::Xml(quick_xml::Error::Encoding(e))
    }
}

// --- Public API ---

/// Parse a DOCX document from bytes and return the structured document.
pub fn parse_document(bytes: &[u8]) -> crate::error::Result<Document> {
    let cursor = Cursor::new(bytes);
    let parser = DocxParser::new(cursor)
        .map_err(|e| crate::error::KreuzbergError::parsing(format!("DOCX parsing failed: {}", e)))?;
    parser
        .parse()
        .map_err(|e| crate::error::KreuzbergError::parsing(format!("DOCX parsing failed: {}", e)))
}

/// Extract text from DOCX bytes.
pub fn extract_text_from_bytes(bytes: &[u8]) -> crate::error::Result<String> {
    let doc = parse_document(bytes)?;
    Ok(doc.extract_text())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test for #359: DOCX list items missing whitespace between text runs.
    #[test]
    fn test_paragraph_to_text_joins_runs_with_space() {
        let mut para = Paragraph::new();
        para.add_run(Run::new("Hello".to_string()));
        para.add_run(Run::new("World".to_string()));
        assert_eq!(para.to_text(), "Hello World");
    }

    #[test]
    fn test_paragraph_to_text_single_run() {
        let mut para = Paragraph::new();
        para.add_run(Run::new("Hello".to_string()));
        assert_eq!(para.to_text(), "Hello");
    }

    #[test]
    fn test_paragraph_to_text_empty_runs_filtered() {
        let mut para = Paragraph::new();
        para.add_run(Run::new("Hello".to_string()));
        para.add_run(Run::new(String::new()));
        para.add_run(Run::new("World".to_string()));
        assert_eq!(para.to_text(), "Hello World");
    }

    #[test]
    fn test_paragraph_to_text_no_runs() {
        let para = Paragraph::new();
        assert_eq!(para.to_text(), "");
    }

    #[test]
    fn test_paragraph_to_text_three_runs() {
        let mut para = Paragraph::new();
        para.add_run(Run::new("The".to_string()));
        para.add_run(Run::new("quick".to_string()));
        para.add_run(Run::new("fox".to_string()));
        assert_eq!(para.to_text(), "The quick fox");
    }
}
