//! Check for PDF Artifact content marks (headers, footers, watermarks).
//! Usage: cargo run --example artifact_check -p kreuzberg-pdfium-render --release -- <pdf> [max_pages]

use pdfium_render::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pdf_path = args.get(1).expect("Usage: artifact_check <pdf> [max_pages]");
    let max_pages: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);

    let pdfium = Pdfium;
    let doc = pdfium.load_pdf_from_file(pdf_path, None).expect("Failed to load PDF");

    println!("PDF: {} ({} pages)", pdf_path, doc.pages().len());
    println!();

    let page_count = doc.pages().len() as usize;
    for page_idx in 0..page_count.min(max_pages) {
        let page = doc.pages().get(page_idx as i32).expect("get page");
        let mut artifact_count = 0;
        let mut artifact_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for obj in page.objects().iter() {
            for mark in obj.content_marks().iter() {
                if mark.name().as_deref() == Some("Artifact") {
                    artifact_count += 1;
                    let atype = mark.param_string_value("Type").unwrap_or_else(|| "(none)".to_string());
                    *artifact_types.entry(atype).or_insert(0) += 1;
                }
            }
        }

        println!(
            "Page {}: {} objects, {} artifacts",
            page_idx,
            page.objects().len(),
            artifact_count
        );
        for (atype, count) in &artifact_types {
            println!("  Type={}: {} objects", atype, count);
        }

        // Also check text chars → text_object → artifact chain
        if let Ok(text_api) = page.text() {
            let chars = text_api.chars();
            let total_chars = chars.len();
            let mut artifact_chars = 0;
            let mut checked = 0;
            // Sample every 10th char to keep it fast
            for i in (0..total_chars).step_by(10) {
                if let Ok(ch) = chars.get(i)
                    && let Ok(text_obj) = ch.text_object()
                {
                    checked += 1;
                    for mark in text_obj.content_marks().iter() {
                        if mark.name().as_deref() == Some("Artifact") {
                            artifact_chars += 1;
                            break;
                        }
                    }
                }
            }
            println!(
                "  Text: {} total chars, sampled {}, {} artifact chars (every 10th)",
                total_chars, checked, artifact_chars
            );
        }
        println!();
    }
}
