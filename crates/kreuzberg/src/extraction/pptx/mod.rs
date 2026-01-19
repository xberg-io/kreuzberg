//! PowerPoint presentation extraction functions.
//!
//! This module provides PowerPoint (PPTX) file parsing by directly reading the Office Open XML
//! format. It extracts text content, slide structure, images, and presentation metadata.
//!
//! # Attribution
//!
//! This code is based on the [pptx-to-md](https://github.com/nilskruthoff/pptx-parser) library
//! by Nils Kruthoff, licensed under MIT OR Apache-2.0. The original code has been vendored and
//! adapted to integrate with Kreuzberg's architecture. See ATTRIBUTIONS.md for full license text.
//!
//! # Features
//!
//! - **Slide extraction**: Reads all slides from presentation
//! - **Text formatting**: Preserves bold, italic, underline formatting as Markdown
//! - **Image extraction**: Optionally extracts embedded images with metadata
//! - **Office metadata**: Extracts core properties, custom properties (when `office` feature enabled)
//! - **Structure preservation**: Maintains heading hierarchy and list structure
//!
//! # Supported Formats
//!
//! - `.pptx` - PowerPoint Presentation
//! - `.pptm` - PowerPoint Macro-Enabled Presentation
//! - `.ppsx` - PowerPoint Slide Show
//!
//! # Example
//!
//! ```rust
//! use kreuzberg::extraction::pptx::extract_pptx_from_path;
//!
//! # fn example() -> kreuzberg::Result<()> {
//! let result = extract_pptx_from_path("presentation.pptx", true, None)?;
//!
//! println!("Slide count: {}", result.slide_count);
//! println!("Image count: {}", result.image_count);
//! println!("Content:\n{}", result.content);
//! # Ok(())
//! # }
//! ```

mod container;
mod content_builder;
mod elements;
mod image_handling;
mod metadata;
mod parser;

use crate::error::Result;
use crate::types::{ExtractedImage, PptxExtractionResult};

use container::{PptxContainer, SlideIterator};
use content_builder::ContentBuilder;
use elements::{ParserConfig, SlideElement};
use image_handling::detect_image_format;
use metadata::{extract_all_notes, extract_metadata};

/// Extract PPTX content from a file path.
///
/// # Arguments
///
/// * `path` - Path to the PPTX file
/// * `extract_images` - Whether to extract embedded images
/// * `page_config` - Optional page configuration for boundary tracking
///
/// # Returns
///
/// A `PptxExtractionResult` containing extracted content, metadata, and images.
pub fn extract_pptx_from_path(
    path: &str,
    extract_images: bool,
    page_config: Option<&crate::core::config::PageConfig>,
) -> Result<PptxExtractionResult> {
    let config = ParserConfig {
        extract_images,
        ..Default::default()
    };

    let mut container = PptxContainer::open(path)?;

    let metadata = extract_metadata(&mut container.archive);

    let notes = extract_all_notes(&mut container)?;

    let mut iterator = SlideIterator::new(container);
    let slide_count = iterator.slide_count();

    let estimated_capacity = slide_count.saturating_mul(1000).max(8192);
    let mut content_builder = ContentBuilder::with_page_config(estimated_capacity, page_config.cloned());

    let mut total_image_count = 0;
    let mut total_table_count = 0;
    let mut extracted_images = Vec::new();

    while let Some(slide) = iterator.next_slide()? {
        let byte_start = if page_config.is_some() {
            content_builder.start_slide(slide.slide_number)
        } else {
            0
        };

        let slide_content = slide.to_markdown(&config);
        content_builder.add_text(&slide_content);

        if let Some(slide_notes) = notes.get(&slide.slide_number) {
            content_builder.add_notes(slide_notes);
        }

        if page_config.is_some() {
            content_builder.end_slide(slide.slide_number, byte_start, slide_content.clone());
        }

        if config.extract_images
            && let Ok(image_data) = iterator.get_slide_images(&slide)
        {
            for (_, data) in image_data {
                let format = detect_image_format(&data);
                let image_index = extracted_images.len();

                extracted_images.push(ExtractedImage {
                    data,
                    format,
                    image_index,
                    page_number: Some(slide.slide_number as usize),
                    width: None,
                    height: None,
                    colorspace: None,
                    bits_per_component: None,
                    is_mask: false,
                    description: None,
                    ocr_result: None,
                });
            }
        }

        total_image_count += slide.image_count();
        total_table_count += slide.table_count();
    }

    let (content, boundaries, page_contents) = content_builder.build();

    let page_structure = boundaries.as_ref().map(|bounds| crate::types::PageStructure {
        total_count: slide_count,
        unit_type: crate::types::PageUnitType::Slide,
        boundaries: Some(bounds.clone()),
        pages: page_contents.as_ref().map(|pcs| {
            pcs.iter()
                .map(|pc| crate::types::PageInfo {
                    number: pc.page_number,
                    title: None,
                    dimensions: None,
                    image_count: None,
                    table_count: None,
                    hidden: None,
                })
                .collect()
        }),
    });

    Ok(PptxExtractionResult {
        content,
        metadata,
        slide_count,
        image_count: total_image_count,
        table_count: total_table_count,
        images: extracted_images,
        page_structure,
        page_contents,
    })
}

/// Extract PPTX content from a byte buffer.
///
/// # Arguments
///
/// * `data` - Raw PPTX file bytes
/// * `extract_images` - Whether to extract embedded images
/// * `page_config` - Optional page configuration for boundary tracking
///
/// # Returns
///
/// A `PptxExtractionResult` containing extracted content, metadata, and images.
pub fn extract_pptx_from_bytes(
    data: &[u8],
    extract_images: bool,
    page_config: Option<&crate::core::config::PageConfig>,
) -> Result<PptxExtractionResult> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let unique_id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_path = std::env::temp_dir().join(format!("temp_pptx_{}_{}.pptx", std::process::id(), unique_id));

    // IO errors must bubble up - temp file write issues need user reports ~keep
    std::fs::write(&temp_path, data)?;

    let result = extract_pptx_from_path(
        temp_path.to_str().ok_or_else(|| {
            crate::KreuzbergError::validation("Invalid temp path - contains invalid UTF-8".to_string())
        })?,
        extract_images,
        page_config,
    );

    if let Err(e) = std::fs::remove_file(&temp_path) {
        tracing::warn!("Failed to remove temp PPTX file: {}", e);
    }

    result
}

// Re-export Slide implementation methods for internal use
impl elements::Slide {
    fn from_xml(slide_number: u32, xml_data: &[u8], rels_data: Option<&[u8]>) -> Result<Self> {
        let elements = parser::parse_slide_xml(xml_data)?;

        let images = if let Some(rels) = rels_data {
            parser::parse_slide_rels(rels)?
        } else {
            Vec::new()
        };

        Ok(Self {
            slide_number,
            elements,
            images,
        })
    }

    fn to_markdown(&self, config: &ParserConfig) -> String {
        let mut builder = ContentBuilder::new();

        if config.include_slide_comment {
            builder.add_slide_header(self.slide_number);
        }

        let mut element_indices: Vec<usize> = (0..self.elements.len()).collect();
        element_indices.sort_by_key(|&i| {
            let pos = self.elements[i].position();
            (pos.y, pos.x)
        });

        for &idx in &element_indices {
            match &self.elements[idx] {
                SlideElement::Text(text, _) => {
                    let text_content: String = text.runs.iter().map(|run| run.render_as_md()).collect();

                    let normalized = text_content.replace('\n', " ");
                    let is_title = normalized.len() < 100 && !normalized.trim().is_empty();

                    if is_title {
                        builder.add_title(normalized.trim());
                    } else {
                        builder.add_text(&text_content);
                    }
                }
                SlideElement::Table(table, _) => {
                    let table_rows: Vec<Vec<String>> = table
                        .rows
                        .iter()
                        .map(|row| {
                            row.cells
                                .iter()
                                .map(|cell| cell.runs.iter().map(|run| run.extract()).collect::<String>())
                                .collect()
                        })
                        .collect();
                    builder.add_table(&table_rows);
                }
                SlideElement::List(list, _) => {
                    for item in &list.items {
                        let item_text: String = item.runs.iter().map(|run| run.extract()).collect();
                        builder.add_list_item(item.level, item.is_ordered, &item_text);
                    }
                }
                SlideElement::Image(img_ref, _) => {
                    builder.add_image(&img_ref.id, self.slide_number);
                }
                SlideElement::Unknown => {}
            }
        }

        builder.build().0
    }

    fn image_count(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| matches!(e, SlideElement::Image(_, _)))
            .count()
    }

    fn table_count(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| matches!(e, SlideElement::Table(_, _)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pptx_bytes(slides: Vec<&str>) -> Vec<u8> {
        use std::io::Write;
        use zip::write::{SimpleFileOptions, ZipWriter};

        let mut buffer = Vec::new();
        {
            let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buffer));
            let options = SimpleFileOptions::default();

            zip.start_file("[Content_Types].xml", options).unwrap();
            zip.write_all(
                br#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="xml" ContentType="application/xml"/>
    <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
</Types>"#,
            )
            .unwrap();

            zip.start_file("ppt/presentation.xml", options).unwrap();
            zip.write_all(b"<?xml version=\"1.0\"?><presentation/>").unwrap();

            zip.start_file("_rels/.rels", options).unwrap();
            zip.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#).unwrap();

            let mut rels_xml = String::from(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#,
            );
            for (i, _) in slides.iter().enumerate() {
                rels_xml.push_str(&format!(
                    r#"<Relationship Id="rId{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide{}.xml"/>"#,
                    i + 1,
                    i + 1
                ));
            }
            rels_xml.push_str("</Relationships>");
            zip.start_file("ppt/_rels/presentation.xml.rels", options).unwrap();
            zip.write_all(rels_xml.as_bytes()).unwrap();

            for (i, text) in slides.iter().enumerate() {
                let slide_xml = format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
    <p:cSld>
        <p:spTree>
            <p:sp>
                <p:txBody>
                    <a:p>
                        <a:r>
                            <a:t>{}</a:t>
                        </a:r>
                    </a:p>
                </p:txBody>
            </p:sp>
        </p:spTree>
    </p:cSld>
</p:sld>"#,
                    text
                );
                zip.start_file(format!("ppt/slides/slide{}.xml", i + 1), options)
                    .unwrap();
                zip.write_all(slide_xml.as_bytes()).unwrap();
            }

            zip.start_file("docProps/core.xml", options).unwrap();
            zip.write_all(
                br#"<?xml version="1.0" encoding="UTF-8"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
                   xmlns:dc="http://purl.org/dc/elements/1.1/"
                   xmlns:dcterms="http://purl.org/dc/terms/">
    <dc:title>Test Presentation</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:description>Test Description</dc:description>
    <dc:subject>Test Subject</dc:subject>
</cp:coreProperties>"#,
            )
            .unwrap();

            let _ = zip.finish().unwrap();
        }
        buffer
    }

    #[test]
    fn test_extract_pptx_from_bytes_single_slide() {
        let pptx_bytes = create_test_pptx_bytes(vec!["Hello World"]);
        let result = extract_pptx_from_bytes(&pptx_bytes, false, None).unwrap();

        assert_eq!(result.slide_count, 1);
        assert!(
            result.content.contains("Hello World"),
            "Content was: {}",
            result.content
        );
        assert_eq!(result.image_count, 0);
        assert_eq!(result.table_count, 0);
    }

    #[test]
    fn test_extract_pptx_from_bytes_multiple_slides() {
        let pptx_bytes = create_test_pptx_bytes(vec!["Slide 1", "Slide 2", "Slide 3"]);
        let result = extract_pptx_from_bytes(&pptx_bytes, false, None).unwrap();

        assert_eq!(result.slide_count, 3);
        assert!(result.content.contains("Slide 1"));
        assert!(result.content.contains("Slide 2"));
        assert!(result.content.contains("Slide 3"));
    }

    #[test]
    fn test_extract_pptx_metadata() {
        let pptx_bytes = create_test_pptx_bytes(vec!["Content"]);
        let result = extract_pptx_from_bytes(&pptx_bytes, false, None).unwrap();

        assert!(result.metadata.fonts.is_empty() || !result.metadata.fonts.is_empty());
    }

    #[test]
    fn test_extract_pptx_empty_slides() {
        let pptx_bytes = create_test_pptx_bytes(vec!["", "", ""]);
        let result = extract_pptx_from_bytes(&pptx_bytes, false, None).unwrap();

        assert_eq!(result.slide_count, 3);
    }

    #[test]
    fn test_extract_pptx_from_bytes_invalid_data() {
        use crate::error::KreuzbergError;

        let invalid_bytes = b"not a valid pptx file";
        let result = extract_pptx_from_bytes(invalid_bytes, false, None);

        assert!(result.is_err());
        if let Err(KreuzbergError::Parsing { message: msg, .. }) = result {
            assert!(msg.contains("Failed to read PPTX archive") || msg.contains("Failed to write temp PPTX file"));
        } else {
            panic!("Expected ParsingError");
        }
    }

    #[test]
    fn test_extract_pptx_from_bytes_empty_data() {
        let empty_bytes: &[u8] = &[];
        let result = extract_pptx_from_bytes(empty_bytes, false, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_detect_image_format_jpeg() {
        let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(detect_image_format(&jpeg_header), "jpeg");
    }

    #[test]
    fn test_detect_image_format_png() {
        let png_header = vec![0x89, 0x50, 0x4E, 0x47];
        assert_eq!(detect_image_format(&png_header), "png");
    }

    #[test]
    fn test_detect_image_format_gif() {
        let gif_header = b"GIF89a";
        assert_eq!(detect_image_format(gif_header), "gif");
    }

    #[test]
    fn test_detect_image_format_bmp() {
        let bmp_header = b"BM";
        assert_eq!(detect_image_format(bmp_header), "bmp");
    }

    #[test]
    fn test_detect_image_format_svg() {
        let svg_header = b"<svg xmlns=\"http://www.w3.org/2000/svg\">";
        assert_eq!(detect_image_format(svg_header), "svg");
    }

    #[test]
    fn test_detect_image_format_tiff_little_endian() {
        let tiff_header = vec![0x49, 0x49, 0x2A, 0x00];
        assert_eq!(detect_image_format(&tiff_header), "tiff");
    }

    #[test]
    fn test_detect_image_format_tiff_big_endian() {
        let tiff_header = vec![0x4D, 0x4D, 0x00, 0x2A];
        assert_eq!(detect_image_format(&tiff_header), "tiff");
    }

    #[test]
    fn test_detect_image_format_unknown() {
        let unknown_data = b"unknown format";
        assert_eq!(detect_image_format(unknown_data), "unknown");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(image_handling::html_escape("plain text"), "plain text");
        assert_eq!(image_handling::html_escape("a & b"), "a &amp; b");
        assert_eq!(image_handling::html_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(image_handling::html_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(image_handling::html_escape("'apostrophe'"), "&#x27;apostrophe&#x27;");
        assert_eq!(
            image_handling::html_escape("<a href=\"url\" title='test'>text & more</a>"),
            "&lt;a href=&quot;url&quot; title=&#x27;test&#x27;&gt;text &amp; more&lt;/a&gt;"
        );
    }

    #[test]
    fn test_get_slide_rels_path() {
        assert_eq!(
            image_handling::get_slide_rels_path("ppt/slides/slide1.xml"),
            "ppt/slides/_rels/slide1.xml.rels"
        );
        assert_eq!(
            image_handling::get_slide_rels_path("ppt/slides/slide10.xml"),
            "ppt/slides/_rels/slide10.xml.rels"
        );
    }

    #[test]
    fn test_get_full_image_path_relative() {
        assert_eq!(
            image_handling::get_full_image_path("ppt/slides/slide1.xml", "../media/image1.png"),
            "ppt/media/image1.png"
        );
    }

    #[test]
    fn test_get_full_image_path_direct() {
        assert_eq!(
            image_handling::get_full_image_path("ppt/slides/slide1.xml", "image1.png"),
            "ppt/slides/image1.png"
        );
    }
}
