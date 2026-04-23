use kreuzberg::{ExtractionConfig, extract_file};
use kreuzberg::core::config::OcrConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = ExtractionConfig::default();
    config.force_ocr = true;
    config.ocr = Some(OcrConfig::default());
    config.images = Some(kreuzberg::core::config::ImageExtractionConfig {
        extract_images: true,
        ..Default::default()
    });

    // Using a docx that likely has images
    let docx_path = "../../test_documents/docx/word_sample.docx";
    println!("Extracting {} with force_ocr=true...", docx_path);

    let result = extract_file(docx_path, None, &config).await?;

    println!("--- Content ---");
    println!("{}", result.content);
    println!("---------------");

    if !result.processing_warnings.is_empty() {
        println!("--- Warnings ---");
        for warning in &result.processing_warnings {
            println!("{}: {}", warning.source, warning.message);
        }
        println!("---------------");
    }

    if let Some(images) = result.images {
        println!("Extracted {} images", images.len());
        for (i, img) in images.iter().enumerate() {
            println!("Image {}: Format = {}, Size = {} bytes", i, img.format, img.data.len());
            if let Some(ocr) = &img.ocr_result {
                println!("Image {}: OCR content length = {}", i, ocr.content.len());
                println!("Image {}: OCR content: {:?}", i, ocr.content);
            } else {
                println!("Image {}: No OCR result", i);
            }
        }
    } else {
        println!("No images extracted.");
    }

    Ok(())
}
