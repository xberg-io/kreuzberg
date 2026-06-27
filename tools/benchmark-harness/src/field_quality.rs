//! Field-level extraction quality runner.
//!
//! Evaluates three distinct structured-extraction surfaces that are dropped by the
//! main `compare` pipeline (which reduces results to `.content`):
//!
//! - **FormFields** — PDF AcroForm / XFA field extraction scored against a flat
//!   `{ "field_name": "value" }` JSON ground truth.
//! - **Formula** — LaTeX formula recognition scored against a
//!   `{ "formulas": ["...", ...] }` JSON ground truth using bag-of-words token F1.
//! - **Structured** — Optional; scores `result.formulas` / structured output against
//!   a JSON ground truth loaded from the `datasets` module when `--dataset` is given.
//!
//! Results are printed in a P/R/F1 table per fixture plus an average row, mirroring
//! the style of `comparison::print_comparison_table`. Results are NOT fed into the
//! SF1/TF1 guardrails system.

use crate::fixture::{Fixture, GroundTruth};
use crate::json_quality::{
    NumericTolerance, field_precision_recall_f1_normalized, flatten_form_fields, latex_token_f1, type_correctness_rate,
};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

/// Which structured-extraction surface to score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// PDF form-field extraction (AcroForm / XFA).
    FormFields,
    /// LaTeX formula recognition from layout-guided extraction.
    Formula,
    /// Structured JSON extraction (requires `--dataset`).
    Structured,
}

impl Mode {
    /// Parse a mode string (case-insensitive).
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "form-fields" | "formfields" | "form_fields" => Some(Self::FormFields),
            "formula" | "formulas" => Some(Self::Formula),
            "structured" => Some(Self::Structured),
            _ => None,
        }
    }
}

/// Arguments for the `field-quality` subcommand.
pub struct Args {
    /// Directory containing fixture JSON files.
    pub fixtures: PathBuf,
    /// Which mode to run.
    pub mode: Mode,
    /// Optional dataset name for `Structured` mode (e.g. "cord", "sroie").
    pub dataset: Option<String>,
    /// Optional substring filter on fixture file stem.
    pub filter: Option<String>,
}

// ── Per-fixture result ────────────────────────────────────────────────────────

struct FixtureRow {
    name: String,
    precision: f64,
    recall: f64,
    f1: f64,
    /// Additional metric label + value (e.g. type_correctness_rate for FormFields).
    extra: Option<(String, f64)>,
}

// ── Main runner ───────────────────────────────────────────────────────────────

/// Run the field-quality benchmark for the given `args`.
///
/// Async because the binary's `main` is `#[tokio::main]`; extraction futures are
/// awaited directly (building a nested runtime would panic).
pub async fn run(args: Args) -> Result<()> {
    match args.mode {
        Mode::FormFields => run_form_fields(&args).await,
        Mode::Formula => run_formula(&args).await,
        Mode::Structured => run_structured(&args).await,
    }
}

// ── FormFields mode ───────────────────────────────────────────────────────────

async fn run_form_fields(args: &Args) -> Result<()> {
    let fixtures = load_fixtures_with_gt(
        &args.fixtures,
        args.filter.as_deref(),
        |gt| gt.fields_json.is_some(),
        "fields_json",
    )?;

    if fixtures.is_empty() {
        eprintln!(
            "No fixtures with ground_truth.fields_json found in {}",
            args.fixtures.display()
        );
        return Ok(());
    }

    let tol = NumericTolerance::default();

    let mut rows: Vec<FixtureRow> = Vec::new();

    for (fixture_path, fixture) in &fixtures {
        let fixture_dir = fixture_path.parent().unwrap_or(Path::new("."));
        let gt = fixture.ground_truth.as_ref().expect("checked above");

        // Resolve and parse the GT fields JSON
        let fields_json_path = fixture_dir.join(gt.fields_json.as_ref().expect("checked above"));
        let gt_value: serde_json::Value = load_json_file(&fields_json_path)
            .with_context(|| format!("failed to load fields_json from {}", fields_json_path.display()))?;

        // Extract the document — form fields are on by default in PdfConfig
        let doc_path = fixture.resolve_document_path(fixture_dir);
        let extraction_config = xberg::ExtractionConfig {
            pdf_options: Some(xberg::PdfConfig::default()),
            ..Default::default()
        };

        let result = crate::extract_xberg_file(&doc_path, &extraction_config)
            .await
            .with_context(|| format!("extraction failed for {}", doc_path.display()))?;

        // Flatten form fields to JSON and score
        let pred_value = flatten_form_fields(&result.form_fields);
        let metrics = field_precision_recall_f1_normalized(&pred_value, &gt_value, &tol);
        let type_rate = type_correctness_rate(&pred_value, &gt_value);

        let name = fixture_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();

        rows.push(FixtureRow {
            name,
            precision: metrics.precision,
            recall: metrics.recall,
            f1: metrics.f1,
            extra: Some(("type_rate".to_string(), type_rate)),
        });
    }

    print_table(
        &rows,
        "Form-Field Extraction Quality (P/R/F1 + type_rate)",
        &["precision", "recall", "f1", "type_rate"],
    );
    Ok(())
}

// ── Formula mode ──────────────────────────────────────────────────────────────

async fn run_formula(args: &Args) -> Result<()> {
    let fixtures = load_fixtures_with_gt(
        &args.fixtures,
        args.filter.as_deref(),
        |gt| gt.formulas_json.is_some(),
        "formulas_json",
    )?;

    if fixtures.is_empty() {
        eprintln!(
            "No fixtures with ground_truth.formulas_json found in {}",
            args.fixtures.display()
        );
        return Ok(());
    }

    let mut rows: Vec<FixtureRow> = Vec::new();

    for (fixture_path, fixture) in &fixtures {
        let fixture_dir = fixture_path.parent().unwrap_or(Path::new("."));
        let gt = fixture.ground_truth.as_ref().expect("checked above");

        // Resolve and parse the GT formulas JSON: { "formulas": ["...", ...] }
        let formulas_json_path = fixture_dir.join(gt.formulas_json.as_ref().expect("checked above"));
        let gt_value: serde_json::Value = load_json_file(&formulas_json_path)
            .with_context(|| format!("failed to load formulas_json from {}", formulas_json_path.display()))?;
        let gt_formulas = parse_formulas_array(&gt_value, &formulas_json_path)?;

        // Extract with layout-enabled config so formula detection fires
        let doc_path = fixture.resolve_document_path(fixture_dir);
        let extraction_config = build_layout_config();

        let result = crate::extract_xberg_file(&doc_path, &extraction_config)
            .await
            .with_context(|| format!("extraction failed for {}", doc_path.display()))?;

        let extracted_formulas: Vec<String> = result.formulas.iter().map(|f| f.latex.clone()).collect();
        let metrics = latex_token_f1(&extracted_formulas, &gt_formulas);

        let name = fixture_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();

        rows.push(FixtureRow {
            name,
            precision: metrics.precision,
            recall: metrics.recall,
            f1: metrics.f1,
            extra: None,
        });
    }

    print_table(
        &rows,
        "Formula Extraction Quality (LaTeX Token F1)",
        &["precision", "recall", "f1"],
    );
    Ok(())
}

// ── Structured mode ───────────────────────────────────────────────────────────

async fn run_structured(args: &Args) -> Result<()> {
    let dataset_name = match &args.dataset {
        Some(d) => d.as_str(),
        None => {
            eprintln!(
                "Structured mode requires --dataset <name> (e.g. cord, sroie, funsd, docile, vrdu). \
                Skipping."
            );
            return Ok(());
        }
    };

    // Load StructuredFixtures from the datasets module.
    let fixtures_dir = &args.fixtures;
    let fixtures = load_structured_dataset(dataset_name, fixtures_dir);

    let fixtures = match fixtures {
        Ok(f) if f.is_empty() => {
            eprintln!("Dataset '{}' loaded 0 fixtures — check path.", dataset_name);
            return Ok(());
        }
        Ok(f) => f,
        Err(e) => {
            eprintln!("Dataset '{}' not available ({}). Skipping.", dataset_name, e);
            return Ok(());
        }
    };

    // Apply name filter if requested
    let filter = args.filter.as_deref();
    let fixtures: Vec<_> = fixtures
        .into_iter()
        .filter(|f| {
            filter
                .map(|pat| {
                    f.document_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.contains(pat))
                        .unwrap_or(false)
                })
                .unwrap_or(true)
        })
        .collect();

    let tol = NumericTolerance::default();

    let mut rows: Vec<FixtureRow> = Vec::new();

    for fixture in &fixtures {
        let extraction_config = xberg::ExtractionConfig::default();

        let result = match crate::extract_xberg_file(&fixture.document_path, &extraction_config).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  ERROR {}: {}", fixture.document_path.display(), e);
                continue;
            }
        };

        // Build a flat JSON representation of the structured output for comparison.
        // Here we use the content as a single-field value for a rough P/R/F1 signal.
        // Richer structured output (e.g., tables) can be added when datasets provide
        // JSON ground truth keyed to specific field paths.
        let pred_value = serde_json::json!({ "content": result.content });
        let gt_value = &fixture.ground_truth;

        let metrics = field_precision_recall_f1_normalized(&pred_value, gt_value, &tol);

        let name = fixture
            .document_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();

        rows.push(FixtureRow {
            name,
            precision: metrics.precision,
            recall: metrics.recall,
            f1: metrics.f1,
            extra: None,
        });
    }

    print_table(
        &rows,
        &format!("Structured Extraction Quality — dataset: {dataset_name}"),
        &["precision", "recall", "f1"],
    );
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Load all fixtures from `fixtures_dir`, optionally filtered by stem substring,
/// keeping only those where the `GroundTruth` satisfies `predicate`.
fn load_fixtures_with_gt(
    fixtures_dir: &Path,
    filter: Option<&str>,
    predicate: impl Fn(&GroundTruth) -> bool,
    field_name: &str,
) -> Result<Vec<(PathBuf, Fixture)>> {
    if !fixtures_dir.exists() {
        bail!("fixtures directory does not exist: {}", fixtures_dir.display());
    }

    let mut results = Vec::new();
    collect_fixture_files(fixtures_dir, &mut results)?;

    // Parse each JSON file as a Fixture and filter
    let mut out = Vec::new();
    for path in results {
        // Apply stem filter
        if let Some(pat) = filter {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if !stem.contains(pat) {
                continue;
            }
        }

        let contents =
            std::fs::read_to_string(&path).with_context(|| format!("failed to read fixture {}", path.display()))?;
        let fixture: Fixture =
            serde_json::from_str(&contents).with_context(|| format!("failed to parse fixture {}", path.display()))?;

        // Check if this fixture has the required GT field
        let has_gt = fixture.ground_truth.as_ref().map(&predicate).unwrap_or(false);

        if has_gt {
            out.push((path, fixture));
        }
    }

    eprintln!(
        "field-quality: found {} fixture(s) with {} in {}",
        out.len(),
        field_name,
        fixtures_dir.display()
    );
    Ok(out)
}

/// Recursively collect all `.json` files under `dir`.
fn collect_fixture_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("failed to read directory {}", dir.display()))? {
        let entry = entry.with_context(|| format!("failed to read entry in {}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_files(&path, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            out.push(path);
        }
    }
    Ok(())
}

/// Load and deserialize a JSON file.
fn load_json_file(path: &Path) -> Result<serde_json::Value> {
    let contents = std::fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&contents).with_context(|| format!("failed to parse JSON in {}", path.display()))
}

/// Parse the `"formulas"` array from a `{ "formulas": [...] }` JSON value.
fn parse_formulas_array(value: &serde_json::Value, path: &Path) -> Result<Vec<String>> {
    let arr = value.get("formulas").and_then(|v| v.as_array()).with_context(|| {
        format!(
            "formulas_json at {} must have a top-level \"formulas\" array",
            path.display()
        )
    })?;

    arr.iter()
        .map(|v| {
            v.as_str()
                .map(String::from)
                .with_context(|| format!("formula entry in {} must be a string", path.display()))
        })
        .collect()
}

/// Build a layout-enabled `ExtractionConfig` for formula extraction.
///
/// The harness depends on `xberg` with `features = ["full"]`, which includes
/// `layout-detection`, so the layout config is always available.
/// Config for formula extraction: GLM-OCR paired mode populates
/// `ExtractedDocument.formulas` only when layout detection runs over OCR'd pages,
/// so force OCR through the `candle-glm-ocr` backend (mirrors the
/// `CandleGlmOcrLayout` benchmark pipeline). Requires the harness to be built
/// with `--features glm-ocr-bench`.
fn build_layout_config() -> xberg::ExtractionConfig {
    xberg::ExtractionConfig {
        force_ocr: true,
        ocr: Some(xberg::core::config::OcrConfig {
            backend: "candle-glm-ocr".to_string(),
            language: vec!["en".to_string()],
            ..Default::default()
        }),
        layout: Some(xberg::LayoutDetectionConfig::default()),
        // GLM-OCR inference on CPU is slow; 10 minutes is sufficient for
        // multi-page formula documents without risking infinite hangs.
        extraction_timeout_secs: Some(600),
        ..Default::default()
    }
}

/// Load a named public dataset, returning a `Result<Vec<StructuredFixture>>`.
/// Returns an error if the dataset is unknown or not found on disk.
fn load_structured_dataset(
    name: &str,
    root: &Path,
) -> std::result::Result<Vec<crate::datasets::StructuredFixture>, crate::datasets::DatasetError> {
    use crate::datasets::{DatasetError, Split};
    match name {
        "cord" => crate::datasets::load_cord(root, Split::Test),
        "sroie" => crate::datasets::load_sroie(root, Split::Test),
        "funsd" => crate::datasets::load_funsd(root, Split::Test),
        "docile" => crate::datasets::load_docile(root, Split::Test),
        "vrdu" => crate::datasets::load_vrdu(root, Split::Test),
        other => Err(DatasetError::Other(format!("unknown dataset '{other}'"))),
    }
}

// ── Table printing ────────────────────────────────────────────────────────────

/// Print a P/R/F1 table (mirroring `comparison::print_comparison_table` style).
fn print_table(rows: &[FixtureRow], title: &str, columns: &[&str]) {
    let has_extra = rows.iter().any(|r| r.extra.is_some());
    let col_count = if has_extra { columns.len() } else { columns.len().min(3) };
    let row_width = 30 + col_count * 12;

    eprintln!("\n{}", title);
    eprintln!("{}", "=".repeat(title.len()));

    // Header
    eprint!("{:<30}", "Fixture");
    for col in columns.iter().take(col_count) {
        eprint!(" {:>10}", col);
    }
    eprintln!();
    eprintln!("{}", "-".repeat(row_width));

    // Data rows
    for row in rows {
        eprint!("{:<30}", truncate(&row.name, 30));
        eprint!(" {:>9.1}%", row.precision * 100.0);
        eprint!(" {:>9.1}%", row.recall * 100.0);
        eprint!(" {:>9.1}%", row.f1 * 100.0);
        if let Some((_, v)) = &row.extra {
            eprint!(" {:>9.1}%", v * 100.0);
        }
        eprintln!();
    }

    if rows.is_empty() {
        eprintln!("  (no results)");
        return;
    }

    // Average row
    eprintln!("{}", "-".repeat(row_width));
    eprint!("{:<30}", "AVERAGE");
    let avg_p = avg_metric(rows, |r| r.precision);
    let avg_r = avg_metric(rows, |r| r.recall);
    let avg_f1 = avg_metric(rows, |r| r.f1);
    eprint!(" {:>9.1}%", avg_p * 100.0);
    eprint!(" {:>9.1}%", avg_r * 100.0);
    eprint!(" {:>9.1}%", avg_f1 * 100.0);
    if has_extra {
        let avg_extra = avg_metric(rows, |r| r.extra.as_ref().map(|(_, v)| *v).unwrap_or(f64::NAN));
        eprint!(" {:>9.1}%", avg_extra * 100.0);
    }
    eprintln!();
}

fn avg_metric(rows: &[FixtureRow], f: impl Fn(&FixtureRow) -> f64) -> f64 {
    let vals: Vec<f64> = rows.iter().map(f).filter(|v| !v.is_nan()).collect();
    if vals.is_empty() {
        f64::NAN
    } else {
        vals.iter().sum::<f64>() / vals.len() as f64
    }
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max { s } else { &s[..max] }
}
