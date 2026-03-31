//! Check if char API and page.text().all() return chars in the same order.
//!
//! Usage: cargo run --example char_order_check -p kreuzberg-pdfium-render --release -- <pdf_path>

use pdfium_render::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pdf_path = args.get(1).expect("Usage: char_order_check <pdf_path>");

    let bindings = Pdfium::bind_to_library("/Users/naamanhirschfeld/.local/kreuzberg-ffi/lib/libpdfium.dylib")
        .or_else(|_| Pdfium::bind_to_system_library())
        .expect("Failed to bind pdfium");
    let pdfium = Pdfium::new(bindings);
    let doc = pdfium.load_pdf_from_file(pdf_path, None).expect("Failed to load PDF");
    let pages = doc.pages();

    // Check first page only
    let page = pages.get(0).expect("No pages");
    let text_api = page.text().expect("No text");

    let full_text = text_api.all();
    let full_chars: Vec<char> = full_text.chars().collect();
    let all_chars = text_api.chars();

    println!("Page 0:");
    println!("  full_text chars: {}", full_chars.len());
    println!("  char API count:  {}", all_chars.len());
    println!();

    // Compare first 50 chars
    println!("First 50 chars comparison:");
    println!("  {:>4} {:>6} {:>6}  match?", "idx", "all()", "chars()");
    let mut mismatches = 0;
    for (i, ch) in all_chars.iter().enumerate().take(50) {
        let api_char = ch.unicode_char().unwrap_or('?');
        let full_char = full_chars.get(i).copied().unwrap_or('⊘');
        let matches = api_char == full_char;
        if !matches {
            mismatches += 1;
        }
        println!(
            "  {:>4} {:>6} {:>6}  {}",
            i,
            format!("{:?}", full_char),
            format!("{:?}", api_char),
            if matches { "✓" } else { "✗ MISMATCH" }
        );
    }
    println!("\nMismatches in first 50: {}", mismatches);

    // Count total mismatches
    let mut total_mismatch = 0;
    let min_len = full_chars.len().min(all_chars.len());
    for (i, ch) in all_chars.iter().enumerate().take(min_len) {
        let api_char = ch.unicode_char().unwrap_or('?');
        let full_char = full_chars[i];
        if api_char != full_char {
            total_mismatch += 1;
        }
    }
    println!("Total mismatches in {} chars: {}", min_len, total_mismatch);
}
