//! Dump segment bounding boxes for a PDF page to understand segment granularity.
//! Usage: cargo run --example seg_dump -p kreuzberg-pdfium-render --release -- <pdf> [page]

use pdfium_render::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pdf_path = args.get(1).expect("Usage: seg_dump <pdf> [page_num]");
    let page_num: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);

    let bindings = Pdfium::bind_to_library("/Users/naamanhirschfeld/.local/kreuzberg-ffi/lib/libpdfium.dylib")
        .or_else(|_| Pdfium::bind_to_system_library())
        .expect("Failed to bind pdfium");
    let pdfium = Pdfium::new(bindings);
    let doc = pdfium.load_pdf_from_file(pdf_path, None).expect("Failed to load PDF");
    let pages = doc.pages();

    let page = pages.get(page_num as i32).expect("Page not found");
    let text_api = page.text().expect("No text");
    let segs = text_api.segments();
    let count = segs.len();
    let page_h = page.height().value;

    println!("Page {}: {} segments, page_height={:.1}", page_num, count, page_h);
    println!(
        "{:>4} {:>7} {:>7} {:>7} {:>7} {:>5}  text_preview",
        "idx", "left", "bottom", "right", "top", "h"
    );

    let mut prev_bottom = page_h;
    for i in 0..count {
        if let Ok(seg) = segs.get(i) {
            let b = seg.bounds();
            let left = b.left().value;
            let bottom = b.bottom().value;
            let right = b.right().value;
            let top = b.top().value;
            let h = top - bottom;

            // Gap from previous segment
            let gap = prev_bottom - top;
            let gap_marker = if gap > h * 1.5 { " <<GAP>>" } else { "" };

            let text = seg.text();
            let preview: String = text.chars().take(50).collect();

            println!(
                "{:>4} {:>7.1} {:>7.1} {:>7.1} {:>7.1} {:>5.1}  {}{}",
                i, left, bottom, right, top, h, preview, gap_marker
            );
            prev_bottom = bottom;
        }
    }
}
