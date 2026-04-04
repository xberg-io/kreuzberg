//! Type conversion utilities
//!
//! This module provides functions for converting between Rust types and Elixir terms,
//! including JSON serialization/deserialization and term encoding/decoding.

use rustler::types::map::map_new;
use rustler::{Encoder, Env, Term};
use std::collections::HashMap;

/// Convert a Rust ExtractionResult to an Elixir term
///
/// This function converts the kreuzberg ExtractionResult struct into a map
/// that can be returned to Elixir code.
pub fn convert_extraction_result_to_term<'a>(
    env: Env<'a>,
    result: &kreuzberg::types::ExtractionResult,
) -> Result<Term<'a>, String> {
    // Create a JSON representation and convert to Elixir term
    let result_json = serde_json::to_value(result).map_err(|e| format!("Failed to serialize result: {}", e))?;

    // Convert JSON to Elixir term
    let term = json_to_term(env, &result_json).map_err(|e| format!("Failed to convert to Elixir term: {}", e))?;

    Ok(term)
}

/// Convert a serde_json::Value to a Rustler Term
///
/// Recursively converts JSON values to Elixir terms:
/// - null -> nil
/// - boolean -> true/false
/// - number -> integer or float
/// - string -> binary
/// - array -> list
/// - object -> map
pub fn json_to_term<'a>(env: Env<'a>, value: &serde_json::Value) -> Result<Term<'a>, String> {
    match value {
        serde_json::Value::Null => Ok(rustler::types::atom::nil().encode(env)),
        serde_json::Value::Bool(b) => Ok(b.encode(env)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.encode(env))
            } else if let Some(u) = n.as_u64() {
                Ok(u.encode(env))
            } else if let Some(f) = n.as_f64() {
                Ok(f.encode(env))
            } else {
                Err("Invalid number".to_string())
            }
        }
        serde_json::Value::String(s) => Ok(s.encode(env)),
        serde_json::Value::Array(arr) => {
            let mut terms = Vec::new();
            for item in arr {
                terms.push(json_to_term(env, item)?);
            }
            Ok(terms.encode(env))
        }
        serde_json::Value::Object(obj) => {
            let mut map = map_new(env);
            for (k, v) in obj {
                let key = k.encode(env);
                let val = json_to_term(env, v)?;
                map = map
                    .map_put(key, val)
                    .map_err(|_| "Failed to put map entry".to_string())?;
            }
            Ok(map)
        }
    }
}

/// Convert an Elixir term to a serde_json::Value
///
/// Recursively converts Elixir terms to JSON values for deserialization.
/// Handles atoms, booleans, numbers, strings, lists, and maps.
pub fn term_to_json(term: Term) -> Result<serde_json::Value, String> {
    // Handle nil (atom)
    if let Ok(atom_str) = term.atom_to_string() {
        return Ok(match atom_str.as_str() {
            "nil" => serde_json::Value::Null,
            "true" => serde_json::Value::Bool(true),
            "false" => serde_json::Value::Bool(false),
            other => serde_json::Value::String(other.to_string()),
        });
    }

    // Handle booleans
    if let Ok(b) = term.decode::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }

    // Handle integers
    if let Ok(i) = term.decode::<i64>() {
        return Ok(serde_json::Value::Number(serde_json::Number::from(i)));
    }

    // Handle floats
    if let Ok(f) = term.decode::<f64>()
        && let Some(num) = serde_json::Number::from_f64(f)
    {
        return Ok(serde_json::Value::Number(num));
    }

    // Handle strings
    if let Ok(s) = term.decode::<String>() {
        return Ok(serde_json::Value::String(s));
    }

    // Handle lists
    if let Ok(list) = term.decode::<Vec<Term>>() {
        let items: Result<Vec<_>, _> = list.into_iter().map(term_to_json).collect();
        return Ok(serde_json::Value::Array(items?));
    }

    // Handle maps
    if let Ok(map) = term.decode::<HashMap<String, Term>>() {
        let mut obj = serde_json::Map::new();
        for (k, v) in map {
            obj.insert(k, term_to_json(v)?);
        }
        return Ok(serde_json::Value::Object(obj));
    }

    Err("Unable to convert term to JSON".to_string())
}

/// Helper function to describe the type of a Term for error messages
pub fn describe_term_type(term: Term) -> String {
    if term.decode::<bool>().is_ok() {
        return "boolean".to_string();
    }
    if term.decode::<i64>().is_ok() {
        return "integer".to_string();
    }
    if term.decode::<f64>().is_ok() {
        return "float".to_string();
    }
    if term.decode::<String>().is_ok() {
        return "string".to_string();
    }
    if term.decode::<Vec<Term>>().is_ok() {
        return "list".to_string();
    }
    if term.decode::<HashMap<String, Term>>().is_ok() {
        return "map".to_string();
    }
    if term.atom_to_string().is_ok() {
        return "atom".to_string();
    }
    "unknown type".to_string()
}
