//! PDF table detection comprehensive test.
//!
//! This test file analyzes table detection across all PDF test documents
//! to understand the current state of table detection and identify false positives.
//!
//! Run with:
//!   cargo test --features pdf,ocr --test pdf_table_detection -- --ignored --nocapture 2>&1 | head -1000
//!
//! This will extract tables from all PDFs and log:
//! - Filename
//! - Number of tables detected
//! - Dimensions of each table (rows x cols)
//! - First 3 cells of each table (to verify legitimacy)

#![cfg(feature = "pdf")]

mod helpers;

use helpers::*;
use kreuzberg::core::config::{ExtractionConfig, OcrConfig};
use kreuzberg::extract_file_sync;

/// All PDF filenames in test_documents/pdf/.
#[allow(dead_code)]
const ALL_PDFS: &[&str] = &[
    "100_g_networking_technology_overview_slides_toronto_august_2016.pdf",
    "5_level_paging_and_5_level_ept_intel_revision_1_1_may_2017.pdf",
    "a_brief_introduction_to_neural_networks_neuronalenetze_en_zeta2_2col_dkrieselcom.pdf",
    "a_brief_introduction_to_the_standard_annotation_language_sal_2006.pdf",
    "a_catalogue_of_optimizing_transformations_1971_allen_catalog.pdf",
    "a_comparison_of_programming_languages_in_economics_16_jun_2014.pdf",
    "a_comprehensive_study_of_convergent_and_commutative_replicated_data_types.pdf",
    "a_comprehensive_study_of_main_memory_partitioning_and_its_application_to_large_scale_comparison_and_radix_sort_sigmod14_i.pdf",
    "a_course_in_machine_learning_ciml_v0_9_all.pdf",
    "algebra_topology_differential_calculus_and_optimization_theory_for_computer_science_and_machine_learning_2019_math_deep.pdf",
    "an_introduction_to_statistical_learning_with_applications_in_r_islr_sixth_printing.pdf",
    "assembly_language_for_beginners_al4_b_en.pdf",
    "bayesian_data_analysis_third_edition_13th_feb_2020.pdf",
    "code_and_formula.pdf",
    "copy_protected.pdf",
    "embedded_images_tables.pdf",
    "fake_memo.pdf",
    "fundamentals_of_deep_learning_2014.pdf",
    "google_doc_document.pdf",
    "image_only_german_pdf.pdf",
    "intel_64_and_ia_32_architectures_software_developer_s_manual_combined_volumes_1_4_june_2021_325462_sdm_vol_1_2abcd_3abcd.pdf",
    "large.pdf",
    "medium.pdf",
    "multi_page_tables.pdf",
    "multi_page.pdf",
    "non_ascii_text.pdf",
    "non_searchable.pdf",
    "ocr_test_rotated_180.pdf",
    "ocr_test_rotated_270.pdf",
    "ocr_test_rotated_90.pdf",
    "ocr_test.pdf",
    "password_protected.pdf",
    "perfect_hash_functions_slides.pdf",
    "program_design_in_the_unix_environment.pdf",
    "proof_of_concept_or_gtfo_v13_october_18th_2016.pdf",
    "right_to_left_01.pdf",
    "sample_contract.pdf",
    "scanned.pdf",
    "searchable.pdf",
    "sharable_web_guide.pdf",
    "simple.pdf",
    "table_document.pdf",
    "tatr.pdf",
    "test_article.pdf",
    "the_hideous_name_1985_pike85hideous.pdf",
    "tiny.pdf",
    "with_images.pdf",
    "xerox_alta_link_series_mfp_sag_en_us_2.pdf",
];

/// Format cell content for display (truncate long text)
fn format_cell(cell: &str) -> String {
    let max_len = 50;
    if cell.len() > max_len {
        // Find a valid UTF-8 boundary at or before max_len
        let truncated = &cell[..cell.floor_char_boundary(max_len)];
        format!("{truncated}...")
    } else {
        cell.to_string()
    }
}

#[test]
#[ignore]
fn test_table_detection_false_positives() {
    if !test_documents_available() {
        println!("Skipping: test_documents not available");
        return;
    }

    let non_table_pdfs = vec![
        "simple.pdf",
        "fake_memo.pdf",
        "google_doc_document.pdf",
        "searchable.pdf",
    ];

    println!("\n");
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║     False Positive Analysis - Non-Table Documents              ║");
    println!("║  These documents should NOT have tables detected               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    let mut false_positives = 0;
    let mut correct_negatives = 0;

    for filename in non_table_pdfs {
        let path = get_test_file_path(&format!("pdf/{}", filename));

        if !path.exists() {
            println!("[SKIP] {} - file not found", filename);
            continue;
        }

        let config = ExtractionConfig {
            ocr: Some(OcrConfig {
                backend: "tesseract".to_string(),
                language: "eng".to_string(),
                ..Default::default()
            }),
            force_ocr: false,
            ..Default::default()
        };

        match extract_file_sync(&path, None, &config) {
            Ok(result) => {
                if result.tables.is_empty() {
                    println!("  [CORRECT] {} - no tables detected", filename);
                    correct_negatives += 1;
                } else {
                    println!(
                        "  [FALSE POSITIVE] {} - detected {} tables (should have none)",
                        filename,
                        result.tables.len()
                    );
                    false_positives += 1;

                    for (idx, table) in result.tables.iter().enumerate() {
                        let rows = table.cells.len();
                        let cols = if rows > 0 { table.cells[0].len() } else { 0 };
                        println!("    Table {}: {} rows × {} cols", idx + 1, rows, cols);

                        if rows > 0 && cols > 0 {
                            let preview_rows = rows.min(2);
                            let preview_cols = cols.min(2);
                            for r in 0..preview_rows {
                                let mut row_str = String::from("      | ");
                                for c in 0..preview_cols {
                                    let cell_content = table.cells[r].get(c).map(|s| s.as_str()).unwrap_or("");
                                    row_str.push_str(&format!("{} | ", format_cell(cell_content)));
                                }
                                if preview_cols < cols {
                                    row_str.push_str("... |");
                                }
                                println!("{}", row_str);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("  [ERROR] {}: {}", filename, e);
            }
        }
    }

    println!();
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                    False Positive Summary                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Correct negatives (no tables):  {}", correct_negatives);
    println!("False positives (detected):    {}", false_positives);
    if false_positives > 0 {
        println!();
        println!("WARNING: Detected {} false-positive tables!", false_positives);
        println!("These should be investigated to improve detection accuracy.");
    }
    println!();
}

/// Focused test on specific PDFs known to have tables
#[test]
#[ignore]
fn test_table_detection_focus_on_table_documents() {
    if !test_documents_available() {
        println!("Skipping: test_documents not available");
        return;
    }

    let table_pdfs = vec![
        "embedded_images_tables.pdf",
        "multi_page_tables.pdf",
        "table_document.pdf",
        "multi_page.pdf",
    ];

    println!("\n");
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║      Focused Table Detection on Known Table Documents          ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    for filename in table_pdfs {
        let path = get_test_file_path(&format!("pdf/{}", filename));

        if !path.exists() {
            println!("[SKIP] {} - file not found", filename);
            continue;
        }

        println!("Analyzing: {}", filename);
        println!();

        let config = ExtractionConfig {
            ocr: Some(OcrConfig {
                backend: "tesseract".to_string(),
                language: "eng".to_string(),
                ..Default::default()
            }),
            force_ocr: false,
            ..Default::default()
        };

        match extract_file_sync(&path, None, &config) {
            Ok(result) => {
                println!("  Tables detected: {}", result.tables.len());

                if result.tables.is_empty() {
                    println!("  No tables detected - possible false negative");
                }

                for (idx, table) in result.tables.iter().enumerate() {
                    let rows = table.cells.len();
                    let cols = if rows > 0 { table.cells[0].len() } else { 0 };

                    println!();
                    println!("  Table {} (page {}):", idx + 1, table.page_number);
                    println!("    Dimensions: {} rows × {} cols", rows, cols);
                    println!("    Cell count: {}", rows * cols);

                    // Full preview (up to 10x10)
                    if rows > 0 && cols > 0 {
                        let preview_rows = rows.min(10);
                        let preview_cols = cols.min(10);
                        println!("    Full preview:");
                        for r in 0..preview_rows {
                            let mut row_str = String::from("      | ");
                            for c in 0..preview_cols {
                                let cell_content = table.cells[r].get(c).map(|s| s.as_str()).unwrap_or("");
                                row_str.push_str(&format!("{} | ", format_cell(cell_content)));
                            }
                            if preview_cols < cols {
                                row_str.push_str("... |");
                            }
                            println!("{}", row_str);
                        }
                        if preview_rows < rows {
                            println!("      | ... |");
                        }
                    }

                    println!();
                    println!("    Markdown:");
                    println!("{}", table.markdown);
                    println!();
                }
            }
            Err(e) => {
                println!("  ERROR: {}", e);
            }
        }

        println!("─────────────────────────────────────────────────────────────────");
        println!();
    }
}
