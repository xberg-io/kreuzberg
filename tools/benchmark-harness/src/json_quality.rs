//! JSON extraction quality metrics for structured document extraction.
//!
//! Provides metrics for evaluating JSON-schema-driven extraction:
//! - **schema_validity_rate** — fraction of predictions passing JSON Schema validation
//! - **field_precision_recall_f1** — leaf-level P/R/F1 between predicted and ground truth (strict)
//! - **field_precision_recall_f1_normalized** — leaf-level P/R/F1 with case-fold + numeric-as-string tolerance
//! - **type_correctness_rate** — percentage of leaves with matching JSON types
//! - **numeric_match** — numeric leaves within tolerance (configurable per type)
//! - **exact_match** — whole-record exact equality
//! - **flatten_form_fields** — flatten `PdfFormField` slice to a `{name: value}` JSON object
//! - **latex_token_f1** — bag-of-words F1 over normalized LaTeX token sets

use serde_json::Value;

/// Configuration for numeric matching tolerance.
#[derive(Debug, Clone)]
pub struct NumericTolerance {
    /// Tolerance for currency values (default ±1%)
    pub currency_percent: f64,
    /// Tolerance for decimal numbers (default ±1%)
    pub decimal_percent: f64,
}

impl Default for NumericTolerance {
    fn default() -> Self {
        Self {
            currency_percent: 0.01,
            decimal_percent: 0.01,
        }
    }
}

/// Precision, Recall, F1 triple.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metrics {
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
}

/// Check if a JSON value is valid against a schema (draft-07).
///
/// # Arguments
/// * `value` - The JSON value to validate
/// * `schema` - The JSON Schema (draft-07)
///
/// # Returns
/// `true` if valid, `false` otherwise
pub fn is_valid_against_schema(value: &Value, schema: &Value) -> bool {
    jsonschema::is_valid(schema, value)
}

/// Compute schema validity rate: fraction of predictions passing validation.
///
/// # Arguments
/// * `predictions` - Array of predicted JSON values
/// * `schema` - JSON Schema to validate against
///
/// # Returns
/// Validity rate in [0.0, 1.0]
pub fn schema_validity_rate(predictions: &[Value], schema: &Value) -> f64 {
    if predictions.is_empty() {
        return 0.0;
    }

    let valid = predictions
        .iter()
        .filter(|pred| is_valid_against_schema(pred, schema))
        .count();

    valid as f64 / predictions.len() as f64
}

/// Extract all leaf values (scalars) from a JSON value with their paths.
fn collect_leaves<'a>(value: &'a Value, prefix: &str) -> Vec<(String, &'a Value)> {
    let mut leaves = Vec::new();

    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                collect_leaves_recursive(val, &path, &mut leaves);
            }
        }
        Value::Array(arr) => {
            for (idx, val) in arr.iter().enumerate() {
                let path = format!("{}[{}]", prefix, idx);
                collect_leaves_recursive(val, &path, &mut leaves);
            }
        }
        _ => {
            leaves.push((prefix.to_string(), value));
        }
    }

    leaves
}

fn collect_leaves_recursive<'a>(value: &'a Value, path: &str, leaves: &mut Vec<(String, &'a Value)>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_path = format!("{}.{}", path, key);
                collect_leaves_recursive(val, &new_path, leaves);
            }
        }
        Value::Array(arr) => {
            for (idx, val) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                collect_leaves_recursive(val, &new_path, leaves);
            }
        }
        _ => {
            leaves.push((path.to_string(), value));
        }
    }
}

/// Compute field-level P/R/F1 between predicted and ground truth JSON.
///
/// Matches leaves by path; computes true positives (matching leaves) and false
/// positives/negatives (present only in one side).
///
/// # Arguments
/// * `predicted` - Predicted JSON object
/// * `ground_truth` - Ground truth JSON object
///
/// # Returns
/// `Metrics { precision, recall, f1 }`
pub fn field_precision_recall_f1(predicted: &Value, ground_truth: &Value) -> Metrics {
    let pred_leaves: std::collections::HashMap<String, _> = collect_leaves(predicted, "").into_iter().collect();
    let gt_leaves: std::collections::HashMap<String, _> = collect_leaves(ground_truth, "").into_iter().collect();

    let mut tp = 0usize;
    let mut fp = 0usize;
    let mut fn_count = 0usize;

    // True positives and false positives
    for (path, pred_val) in &pred_leaves {
        if let Some(gt_val) = gt_leaves.get(path) {
            if pred_val == gt_val {
                tp += 1;
            } else {
                fp += 1;
            }
        } else {
            fp += 1;
        }
    }

    // False negatives
    for path in gt_leaves.keys() {
        if !pred_leaves.contains_key(path) {
            fn_count += 1;
        }
    }

    let precision = if tp + fp == 0 {
        0.0
    } else {
        tp as f64 / (tp + fp) as f64
    };

    let recall = if tp + fn_count == 0 {
        0.0
    } else {
        tp as f64 / (tp + fn_count) as f64
    };

    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    };

    Metrics { precision, recall, f1 }
}

/// Compute type correctness rate: fraction of matched leaves with matching JSON types.
///
/// Only considers leaves that exist in both predicted and ground truth.
///
/// # Arguments
/// * `predicted` - Predicted JSON object
/// * `ground_truth` - Ground truth JSON object
///
/// # Returns
/// Type correctness rate in [0.0, 1.0]
pub fn type_correctness_rate(predicted: &Value, ground_truth: &Value) -> f64 {
    let pred_leaves: std::collections::HashMap<String, _> = collect_leaves(predicted, "").into_iter().collect();
    let gt_leaves: std::collections::HashMap<String, _> = collect_leaves(ground_truth, "").into_iter().collect();

    let mut matched = 0usize;
    let mut type_correct = 0usize;

    for (path, pred_val) in &pred_leaves {
        if let Some(gt_val) = gt_leaves.get(path) {
            matched += 1;
            if same_json_type(pred_val, gt_val) {
                type_correct += 1;
            }
        }
    }

    if matched == 0 {
        0.0
    } else {
        type_correct as f64 / matched as f64
    }
}

fn same_json_type(a: &Value, b: &Value) -> bool {
    matches!(
        (a, b),
        (Value::Null, Value::Null)
            | (Value::Bool(_), Value::Bool(_))
            | (Value::Number(_), Value::Number(_))
            | (Value::String(_), Value::String(_))
            | (Value::Array(_), Value::Array(_))
            | (Value::Object(_), Value::Object(_))
    )
}

/// Check if two numeric values match within tolerance.
///
/// For integers, checks exact equality. For floats, uses percentage tolerance.
///
/// # Arguments
/// * `predicted` - Predicted numeric value
/// * `ground_truth` - Ground truth numeric value
/// * `tolerance` - Tolerance configuration
///
/// # Returns
/// `true` if values match within tolerance
pub fn numeric_match(predicted: &Value, ground_truth: &Value, tolerance: &NumericTolerance) -> bool {
    let pred_num = match predicted {
        Value::Number(n) => n.as_f64(),
        _ => return false,
    };

    let gt_num = match ground_truth {
        Value::Number(n) => n.as_f64(),
        _ => return false,
    };

    match (pred_num, gt_num) {
        (Some(p), Some(g)) => {
            let percent_diff = ((p - g).abs() / g.abs()).min(1.0);
            // Values >= 1.0 are treated as currency-scale (prices, totals, counts),
            // letting the looser currency_percent govern. Sub-unit decimals stick
            // with the tighter decimal_percent budget. When the two tolerances are
            // identical (Default::default), the choice is a no-op.
            let effective = if g.abs() >= 1.0 {
                tolerance.currency_percent.max(tolerance.decimal_percent)
            } else {
                tolerance.decimal_percent
            };
            percent_diff <= effective
        }
        _ => false,
    }
}

/// Check if two JSON values are exactly equal.
pub fn exact_match(predicted: &Value, ground_truth: &Value) -> bool {
    predicted == ground_truth
}

// ── Normalized helpers ────────────────────────────────────────────────────────

/// Normalize a string for comparison: trim whitespace, lowercase, collapse runs of
/// internal whitespace to a single space.
fn normalize_string(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// Compare two leaf values with normalization:
/// - Both `Value::Number` → `numeric_match`
/// - Both `Value::String` → normalize (trim, case-fold, collapse whitespace) then compare;
///   if both parse as `f64`, fall back to `numeric_match` so GT numbers stored as quoted
///   strings (e.g. `"60.000"`) match their numeric counterparts.
/// - Otherwise → strict `==`
fn values_match_normalized(pred: &Value, gt: &Value, tol: &NumericTolerance) -> bool {
    match (pred, gt) {
        (Value::Number(_), Value::Number(_)) => numeric_match(pred, gt, tol),
        (Value::String(p), Value::String(g)) => {
            let pn = normalize_string(p);
            let gn = normalize_string(g);
            if pn == gn {
                return true;
            }
            // Both parse as f64? Fall back to numeric tolerance.
            if let (Ok(pf), Ok(gf)) = (pn.parse::<f64>(), gn.parse::<f64>()) {
                let pred_num =
                    Value::Number(serde_json::Number::from_f64(pf).unwrap_or_else(|| serde_json::Number::from(0)));
                let gt_num =
                    Value::Number(serde_json::Number::from_f64(gf).unwrap_or_else(|| serde_json::Number::from(0)));
                return numeric_match(&pred_num, &gt_num, tol);
            }
            false
        }
        // One is a number, the other is a string representation of a number.
        (Value::Number(n), Value::String(s)) | (Value::String(s), Value::Number(n)) => {
            let sn = normalize_string(s);
            if let (Some(nf), Ok(sf)) = (n.as_f64(), sn.parse::<f64>()) {
                let a = Value::Number(serde_json::Number::from_f64(nf).unwrap_or_else(|| serde_json::Number::from(0)));
                let b = Value::Number(serde_json::Number::from_f64(sf).unwrap_or_else(|| serde_json::Number::from(0)));
                numeric_match(&a, &b, tol)
            } else {
                false
            }
        }
        _ => pred == gt,
    }
}

/// Compute field-level P/R/F1 between predicted and ground truth JSON, using
/// normalized value comparison.
///
/// Path matching is identical to [`field_precision_recall_f1`]; a path-matched leaf
/// counts as a true positive when [`values_match_normalized`] holds:
/// - Both numeric → within tolerance
/// - Both strings → normalize (trim, case-fold, collapse whitespace), then compare;
///   if both parse as `f64`, fall back to numeric tolerance
/// - Number vs quoted number string → numeric tolerance
/// - Otherwise → strict equality
///
/// # Arguments
/// * `predicted` - Predicted JSON object
/// * `ground_truth` - Ground truth JSON object
/// * `tol` - Numeric tolerance configuration
///
/// # Returns
/// `Metrics { precision, recall, f1 }`
pub fn field_precision_recall_f1_normalized(
    predicted: &Value,
    ground_truth: &Value,
    tol: &NumericTolerance,
) -> Metrics {
    let pred_leaves: std::collections::HashMap<String, _> = collect_leaves(predicted, "").into_iter().collect();
    let gt_leaves: std::collections::HashMap<String, _> = collect_leaves(ground_truth, "").into_iter().collect();

    let mut tp = 0usize;
    let mut fp = 0usize;
    let mut fn_count = 0usize;

    // True positives and false positives
    for (path, pred_val) in &pred_leaves {
        if let Some(gt_val) = gt_leaves.get(path) {
            if values_match_normalized(pred_val, gt_val, tol) {
                tp += 1;
            } else {
                fp += 1;
            }
        } else {
            fp += 1;
        }
    }

    // False negatives (paths only in GT)
    for path in gt_leaves.keys() {
        if !pred_leaves.contains_key(path) {
            fn_count += 1;
        }
    }

    let precision = if tp + fp == 0 {
        0.0
    } else {
        tp as f64 / (tp + fp) as f64
    };

    let recall = if tp + fn_count == 0 {
        0.0
    } else {
        tp as f64 / (tp + fn_count) as f64
    };

    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    };

    Metrics { precision, recall, f1 }
}

/// Flatten a `PdfFormField` slice into a flat JSON object `{ full_name: value_string }`.
///
/// Key precedence: `full_name` (fallback to `name`).
/// Value precedence: `value`, then `default_value`, then `""`.
///
/// The resulting object has one leaf per field and is suitable for feeding into
/// [`collect_leaves`] / [`field_precision_recall_f1_normalized`].
pub fn flatten_form_fields(fields: &[kreuzberg::PdfFormField]) -> Value {
    let mut map = serde_json::Map::new();
    for field in fields {
        let key = if field.full_name.is_empty() {
            field.name.clone()
        } else {
            field.full_name.clone()
        };
        let val = field
            .value
            .as_deref()
            .or(field.default_value.as_deref())
            .unwrap_or("")
            .to_string();
        map.insert(key, Value::String(val));
    }
    Value::Object(map)
}

// ── LaTeX token F1 ───────────────────────────────────────────────────────────

/// Normalize a single LaTeX string for token-level comparison:
/// strip surrounding `$` / `$$` delimiters, collapse whitespace.
fn normalize_latex(s: &str) -> String {
    let s = s.trim();
    // Strip `$$...$$` then `$...$`
    let s = s.strip_prefix("$$").and_then(|s| s.strip_suffix("$$")).unwrap_or(s);
    let s = s.strip_prefix('$').and_then(|s| s.strip_suffix('$')).unwrap_or(s);
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Tokenize a normalized LaTeX string on whitespace.
fn tokenize_latex(s: &str) -> Vec<String> {
    normalize_latex(s)
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect()
}

/// Compute bag-of-words multiset F1 between two sets of LaTeX formula strings.
///
/// Each formula is normalized (strip `$`/`$$`, collapse whitespace) and tokenized on
/// whitespace. Precision and recall are computed over the concatenated token bags
/// (all extracted formulas vs all GT formulas), using multiset intersection.
///
/// Returns `Metrics { precision, recall, f1 }`.
pub fn latex_token_f1(extracted: &[String], gt: &[String]) -> Metrics {
    let pred_tokens: Vec<String> = extracted.iter().flat_map(|s| tokenize_latex(s)).collect();
    let gt_tokens: Vec<String> = gt.iter().flat_map(|s| tokenize_latex(s)).collect();

    if pred_tokens.is_empty() && gt_tokens.is_empty() {
        return Metrics {
            precision: 1.0,
            recall: 1.0,
            f1: 1.0,
        };
    }
    if pred_tokens.is_empty() || gt_tokens.is_empty() {
        return Metrics {
            precision: 0.0,
            recall: 0.0,
            f1: 0.0,
        };
    }

    // Build multiset counts
    let mut pred_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    let mut gt_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for t in &pred_tokens {
        *pred_counts.entry(t.as_str()).or_insert(0) += 1;
    }
    for t in &gt_tokens {
        *gt_counts.entry(t.as_str()).or_insert(0) += 1;
    }

    // Multiset intersection: min(pred_count, gt_count) for each token
    let intersection: usize = gt_counts
        .iter()
        .map(|(tok, &gc)| {
            let pc = pred_counts.get(tok).copied().unwrap_or(0);
            pc.min(gc)
        })
        .sum();

    let precision = intersection as f64 / pred_tokens.len() as f64;
    let recall = intersection as f64 / gt_tokens.len() as f64;
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };

    Metrics { precision, recall, f1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_validity_rate_all_valid() {
        let schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "number" }
            }
        });

        let predictions = vec![json!({"name": "Alice", "age": 30}), json!({"name": "Bob", "age": 25})];

        let rate = schema_validity_rate(&predictions, &schema);
        assert_eq!(rate, 1.0);
    }

    #[test]
    fn test_schema_validity_rate_partial_valid() {
        let schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });

        let predictions = vec![
            json!({"name": "Alice"}),
            json!({"age": 30}), // missing required field
        ];

        let rate = schema_validity_rate(&predictions, &schema);
        assert!(rate > 0.0 && rate < 1.0);
    }

    #[test]
    fn test_field_precision_recall_f1_perfect_match() {
        let pred = json!({
            "name": "Alice",
            "age": 30,
            "city": "NYC"
        });

        let gt = json!({
            "name": "Alice",
            "age": 30,
            "city": "NYC"
        });

        let metrics = field_precision_recall_f1(&pred, &gt);
        assert_eq!(metrics.precision, 1.0);
        assert_eq!(metrics.recall, 1.0);
        assert_eq!(metrics.f1, 1.0);
    }

    #[test]
    fn test_field_precision_recall_f1_partial_match() {
        let pred = json!({
            "name": "Alice",
            "age": 30,
            "extra": "field"
        });

        let gt = json!({
            "name": "Alice",
            "age": 30,
            "city": "NYC"
        });

        let metrics = field_precision_recall_f1(&pred, &gt);
        assert!(metrics.precision < 1.0);
        assert!(metrics.recall < 1.0);
        assert!(metrics.f1 > 0.0);
    }

    #[test]
    fn test_field_precision_recall_f1_nested_objects() {
        let pred = json!({
            "name": "Alice",
            "address": {
                "city": "NYC",
                "zip": "10001"
            }
        });

        let gt = json!({
            "name": "Alice",
            "address": {
                "city": "NYC",
                "zip": "10001"
            }
        });

        let metrics = field_precision_recall_f1(&pred, &gt);
        assert_eq!(metrics.f1, 1.0);
    }

    #[test]
    fn test_field_precision_recall_f1_arrays() {
        let pred = json!({
            "items": ["apple", "banana"]
        });

        let gt = json!({
            "items": ["apple", "banana"]
        });

        let metrics = field_precision_recall_f1(&pred, &gt);
        assert_eq!(metrics.f1, 1.0);
    }

    #[test]
    fn test_type_correctness_rate_all_correct() {
        let pred = json!({
            "name": "Alice",
            "age": 30
        });

        let gt = json!({
            "name": "Bob",
            "age": 25
        });

        let rate = type_correctness_rate(&pred, &gt);
        assert_eq!(rate, 1.0);
    }

    #[test]
    fn test_type_correctness_rate_mixed() {
        let pred = json!({
            "name": "Alice",
            "age": "30" // wrong type: string instead of number
        });

        let gt = json!({
            "name": "Bob",
            "age": 25
        });

        let rate = type_correctness_rate(&pred, &gt);
        assert!(rate < 1.0 && rate > 0.0);
    }

    #[test]
    fn test_numeric_match_within_tolerance() {
        let tol = NumericTolerance::default();

        let pred = json!(101.0);
        let gt = json!(100.0);

        let result = numeric_match(&pred, &gt, &tol);
        assert!(result); // 1% difference is within default tolerance
    }

    #[test]
    fn test_numeric_match_outside_tolerance() {
        let tol = NumericTolerance::default();

        let pred = json!(150.0);
        let gt = json!(100.0);

        let result = numeric_match(&pred, &gt, &tol);
        assert!(!result); // 50% difference exceeds tolerance
    }

    #[test]
    fn test_numeric_match_currency() {
        let tol = NumericTolerance {
            currency_percent: 0.02, // 2% tolerance for currency
            decimal_percent: 0.01,
        };

        let pred = json!(102.0);
        let gt = json!(100.0);

        let result = numeric_match(&pred, &gt, &tol);
        assert!(result);
    }

    #[test]
    fn test_exact_match_identical() {
        let a = json!({
            "name": "Alice",
            "items": ["apple", "banana"]
        });

        let b = json!({
            "name": "Alice",
            "items": ["apple", "banana"]
        });

        assert!(exact_match(&a, &b));
    }

    #[test]
    fn test_exact_match_different() {
        let a = json!({
            "name": "Alice"
        });

        let b = json!({
            "name": "Bob"
        });

        assert!(!exact_match(&a, &b));
    }

    // ── Normalized field P/R/F1 tests ─────────────────────────────────────────

    #[test]
    fn test_normalized_string_case_fold() {
        // "ALICE" vs "alice" — should match after normalization
        let pred = json!({ "name": "ALICE" });
        let gt = json!({ "name": "alice" });
        let tol = NumericTolerance::default();
        let m = field_precision_recall_f1_normalized(&pred, &gt, &tol);
        assert_eq!(m.f1, 1.0, "case-folded strings must match");
    }

    #[test]
    fn test_normalized_string_whitespace_collapse() {
        let pred = json!({ "city": "  New   York  " });
        let gt = json!({ "city": "New York" });
        let tol = NumericTolerance::default();
        let m = field_precision_recall_f1_normalized(&pred, &gt, &tol);
        assert_eq!(m.f1, 1.0, "whitespace-collapsed strings must match");
    }

    #[test]
    fn test_normalized_numeric_as_string() {
        // GT stores number as quoted string "60.000", pred has JSON number 60.0
        let pred = json!({ "amount": 60.0 });
        let gt = json!({ "amount": "60.000" });
        let tol = NumericTolerance::default();
        let m = field_precision_recall_f1_normalized(&pred, &gt, &tol);
        assert_eq!(m.f1, 1.0, "number vs quoted number string must match within tolerance");
    }

    #[test]
    fn test_normalized_strict_fallback_for_non_numeric_strings() {
        // "foo" vs "bar" — no numeric parse possible, must not match
        let pred = json!({ "label": "foo" });
        let gt = json!({ "label": "bar" });
        let tol = NumericTolerance::default();
        let m = field_precision_recall_f1_normalized(&pred, &gt, &tol);
        assert!(m.f1 < 1.0, "non-matching strings must not produce f1=1");
    }

    #[test]
    fn test_normalized_perfect_match_numbers() {
        let pred = json!({ "total": 100.5, "count": 3 });
        let gt = json!({ "total": 100.5, "count": 3 });
        let tol = NumericTolerance::default();
        let m = field_precision_recall_f1_normalized(&pred, &gt, &tol);
        assert_eq!(m.f1, 1.0);
    }

    // ── flatten_form_fields tests ─────────────────────────────────────────────

    #[test]
    fn test_flatten_form_fields_uses_full_name_and_value() {
        use kreuzberg::{FormFieldType, PdfFormField};
        let fields = vec![PdfFormField {
            name: "leaf".to_string(),
            full_name: "root.leaf".to_string(),
            field_type: FormFieldType::Text,
            value: Some("hello".to_string()),
            default_value: None,
            flags: 0,
            page: None,
            bbox: None,
            max_length: None,
            tooltip: None,
        }];
        let flat = flatten_form_fields(&fields);
        assert_eq!(flat["root.leaf"], Value::String("hello".to_string()));
    }

    #[test]
    fn test_flatten_form_fields_falls_back_to_default_value() {
        use kreuzberg::{FormFieldType, PdfFormField};
        let fields = vec![PdfFormField {
            name: "leaf".to_string(),
            full_name: "root.leaf".to_string(),
            field_type: FormFieldType::Text,
            value: None,
            default_value: Some("default_val".to_string()),
            flags: 0,
            page: None,
            bbox: None,
            max_length: None,
            tooltip: None,
        }];
        let flat = flatten_form_fields(&fields);
        assert_eq!(flat["root.leaf"], Value::String("default_val".to_string()));
    }

    #[test]
    fn test_flatten_form_fields_empty_string_when_no_value() {
        use kreuzberg::{FormFieldType, PdfFormField};
        let fields = vec![PdfFormField {
            name: "leaf".to_string(),
            full_name: "root.leaf".to_string(),
            field_type: FormFieldType::Checkbox,
            value: None,
            default_value: None,
            flags: 0,
            page: None,
            bbox: None,
            max_length: None,
            tooltip: None,
        }];
        let flat = flatten_form_fields(&fields);
        assert_eq!(flat["root.leaf"], Value::String(String::new()));
    }

    #[test]
    fn test_flatten_form_fields_empty_full_name_falls_back_to_name() {
        use kreuzberg::{FormFieldType, PdfFormField};
        let fields = vec![PdfFormField {
            name: "solo".to_string(),
            full_name: String::new(),
            field_type: FormFieldType::Text,
            value: Some("v".to_string()),
            default_value: None,
            flags: 0,
            page: None,
            bbox: None,
            max_length: None,
            tooltip: None,
        }];
        let flat = flatten_form_fields(&fields);
        assert_eq!(flat["solo"], Value::String("v".to_string()));
    }

    // ── latex_token_f1 tests ──────────────────────────────────────────────────

    #[test]
    fn test_latex_token_f1_perfect_match() {
        let extracted = vec!["E = mc^2".to_string()];
        let gt = vec!["E = mc^2".to_string()];
        let m = latex_token_f1(&extracted, &gt);
        assert_eq!(m.f1, 1.0);
    }

    #[test]
    fn test_latex_token_f1_strips_dollar_delimiters() {
        // Both have `$$` wrappers — tokens should be the same after stripping
        let extracted = vec!["$$E = mc^2$$".to_string()];
        let gt = vec!["E = mc^2".to_string()];
        let m = latex_token_f1(&extracted, &gt);
        assert_eq!(m.f1, 1.0, "dollar delimiters must be stripped before comparison");
    }

    #[test]
    fn test_latex_token_f1_partial_overlap() {
        let extracted = vec!["a b c".to_string()];
        let gt = vec!["a b d".to_string()];
        let m = latex_token_f1(&extracted, &gt);
        // intersection = {a, b}, pred_len = 3, gt_len = 3
        // precision = 2/3, recall = 2/3, f1 = 2/3
        let expected_f1 = 2.0 / 3.0;
        assert!(
            (m.f1 - expected_f1).abs() < 1e-9,
            "expected f1={:.4}, got {:.4}",
            expected_f1,
            m.f1
        );
    }

    #[test]
    fn test_latex_token_f1_both_empty() {
        let m = latex_token_f1(&[], &[]);
        assert_eq!(m.f1, 1.0, "vacuously empty must score 1.0");
    }

    #[test]
    fn test_latex_token_f1_extracted_empty() {
        let gt = vec!["x^2".to_string()];
        let m = latex_token_f1(&[], &gt);
        assert_eq!(m.f1, 0.0);
    }
}
