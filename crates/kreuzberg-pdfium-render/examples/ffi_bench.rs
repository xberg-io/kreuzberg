//! FFI cost benchmark: character-indexed API vs segment API.
//!
//! Usage: cargo run --example ffi_bench -p kreuzberg-pdfium-render --release -- <pdf_path>

use pdfium_render::prelude::*;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ffi_bench <pdf_path>");
        std::process::exit(1);
    }
    let pdf_path = &args[1];

    let bindings = Pdfium::bind_to_library(std::env::var("PDFIUM_LIB").unwrap_or_else(|_| {
        // Try common locations
        for path in &[
            "/Users/naamanhirschfeld/.local/kreuzberg-ffi/lib/libpdfium.dylib",
            "libpdfium.dylib",
        ] {
            if std::path::Path::new(path).exists() {
                return path.to_string();
            }
        }
        "libpdfium.dylib".to_string()
    }))
    .or_else(|_| Pdfium::bind_to_system_library())
    .expect("Failed to bind pdfium — set PDFIUM_LIB env var");
    let pdfium = Pdfium::new(bindings);
    let doc = pdfium.load_pdf_from_file(pdf_path, None).expect("Failed to load PDF");
    let pages = doc.pages();
    let page_count = pages.len();

    println!("PDF: {} ({} pages)", pdf_path, page_count);

    // Warm up (load pages into pdfium cache)
    for i in 0..page_count.min(3) {
        if let Ok(page) = pages.get(i) {
            let _ = page.text().map(|t| t.all());
        }
    }

    const RUNS: usize = 3;

    // === A: page.text().all() ===
    let mut total_chars = 0usize;
    let mut best_a = std::time::Duration::MAX;
    for _ in 0..RUNS {
        total_chars = 0;
        let t = Instant::now();
        for i in 0..page_count {
            if let Ok(page) = pages.get(i)
                && let Ok(text_api) = page.text()
            {
                let ft = text_api.all();
                total_chars += ft.chars().count();
            }
        }
        best_a = best_a.min(t.elapsed());
    }
    println!("\nA. page.text().all(): {:?} ({} chars)", best_a, total_chars);

    // === B: Segment iteration (current approach) ===
    let mut total_segments = 0usize;
    let mut best_b = std::time::Duration::MAX;
    for _ in 0..RUNS {
        total_segments = 0;
        let t = Instant::now();
        for i in 0..page_count {
            if let Ok(page) = pages.get(i)
                && let Ok(text_api) = page.text()
            {
                let segments = text_api.segments();
                let seg_count = segments.len();
                total_segments += seg_count;
                for j in 0..seg_count {
                    if let Ok(seg) = segments.get(j) {
                        let _bounds = seg.bounds();
                        if let Ok(chars) = seg.chars() {
                            let _count = chars.len();
                            // Sample font from first non-ws char
                            for ch in chars.iter() {
                                let uv = ch.unicode_value();
                                if char::from_u32(uv).is_some_and(|c| c.is_whitespace()) {
                                    continue;
                                }
                                let _fs = ch.scaled_font_size();
                                let _info = ch.font_info();
                                let _origin = ch.origin();
                                break;
                            }
                        }
                    }
                }
            }
        }
        best_b = best_b.min(t.elapsed());
    }
    println!("B. Segment iteration: {:?} ({} segments)", best_b, total_segments);

    // === C: Character-indexed full (font_size + bounds + font_info per char) ===
    let mut total_char_calls = 0usize;
    let mut best_c = std::time::Duration::MAX;
    for _ in 0..RUNS {
        total_char_calls = 0;
        let t = Instant::now();
        for i in 0..page_count {
            if let Ok(page) = pages.get(i)
                && let Ok(text_api) = page.text()
            {
                let chars = text_api.chars();
                for ch in chars.iter() {
                    let _fs = ch.scaled_font_size();
                    let _bounds = ch.tight_bounds();
                    let _info = ch.font_info();
                    total_char_calls += 1;
                }
            }
        }
        best_c = best_c.min(t.elapsed());
    }
    println!(
        "C. Char-indexed full: {:?} ({} chars × 3 calls)",
        best_c, total_char_calls
    );

    // === D: Character-indexed font_size only ===
    let mut best_d = std::time::Duration::MAX;
    for _ in 0..RUNS {
        let t = Instant::now();
        for i in 0..page_count {
            if let Ok(page) = pages.get(i)
                && let Ok(text_api) = page.text()
            {
                let chars = text_api.chars();
                for ch in chars.iter() {
                    let _fs = ch.scaled_font_size();
                }
            }
        }
        best_d = best_d.min(t.elapsed());
    }
    println!("D. Char-indexed fs only: {:?}", best_d);

    // === E: Character-indexed bounds only ===
    let mut best_e = std::time::Duration::MAX;
    for _ in 0..RUNS {
        let t = Instant::now();
        for i in 0..page_count {
            if let Ok(page) = pages.get(i)
                && let Ok(text_api) = page.text()
            {
                let chars = text_api.chars();
                for ch in chars.iter() {
                    let _bounds = ch.tight_bounds();
                }
            }
        }
        best_e = best_e.min(t.elapsed());
    }
    println!("E. Char-indexed bounds only: {:?}", best_e);

    // Summary
    println!("\n=== Summary ===");
    println!("Pages:            {}", page_count);
    println!("Total chars:      {}", total_chars);
    println!("Total segments:   {}", total_segments);
    println!("Chars/page:       {:.0}", total_chars as f64 / page_count as f64);
    println!("Segments/page:    {:.0}", total_segments as f64 / page_count as f64);
    println!();
    println!(
        "A. text().all():       {:>10.2?}  ({:.2}ms/page)",
        best_a,
        best_a.as_secs_f64() * 1000.0 / page_count as f64
    );
    println!(
        "B. Segments:           {:>10.2?}  ({:.2}ms/page)",
        best_b,
        best_b.as_secs_f64() * 1000.0 / page_count as f64
    );
    println!(
        "C. Chars full:         {:>10.2?}  ({:.2}ms/page)",
        best_c,
        best_c.as_secs_f64() * 1000.0 / page_count as f64
    );
    println!(
        "D. Chars fs only:      {:>10.2?}  ({:.2}ms/page)",
        best_d,
        best_d.as_secs_f64() * 1000.0 / page_count as f64
    );
    println!(
        "E. Chars bounds only:  {:>10.2?}  ({:.2}ms/page)",
        best_e,
        best_e.as_secs_f64() * 1000.0 / page_count as f64
    );
    if best_b.as_nanos() > 0 {
        println!(
            "\nC/B ratio (char-full vs segments): {:.1}x",
            best_c.as_secs_f64() / best_b.as_secs_f64()
        );
        println!(
            "D/B ratio (char-fs vs segments):   {:.1}x",
            best_d.as_secs_f64() / best_b.as_secs_f64()
        );
    }
}
