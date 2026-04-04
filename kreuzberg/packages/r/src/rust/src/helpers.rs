//! R <-> Rust type conversion utilities

use crate::error::to_r_error;
use extendr_api::prelude::*;
use serde_json::Value;

/// Convert a serde_json::Value to an R object
pub fn json_to_robj(value: &Value) -> extendr_api::Result<Robj> {
    match value {
        Value::Null => Ok(().into()),
        Value::Bool(b) => Ok(b.into_robj()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok((i as i32).into_robj())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_robj())
            } else {
                Ok(().into())
            }
        }
        Value::String(s) => {
            // R strings (CHARSXP) are C strings and cannot contain embedded NUL bytes.
            // Strip them to avoid conversion errors.
            let sanitized = s.replace('\0', "");
            Ok(sanitized.into_robj())
        }
        Value::Array(arr) => {
            let items: Vec<Robj> = arr.iter().map(json_to_robj).collect::<extendr_api::Result<Vec<_>>>()?;
            Ok(List::from_values(items).into_robj())
        }
        Value::Object(map) => {
            let names: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
            let values: Vec<Robj> = map.values().map(json_to_robj).collect::<extendr_api::Result<Vec<_>>>()?;
            let list = List::from_names_and_values(names, values).map_err(to_r_error)?;
            Ok(list.into_robj())
        }
    }
}
