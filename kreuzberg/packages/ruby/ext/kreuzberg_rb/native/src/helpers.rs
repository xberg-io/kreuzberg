//! Helper utilities for Ruby value conversion and manipulation
//!
//! Provides utilities for converting between Ruby and JSON values,
//! accessing keyword arguments, and managing cache directories.

use magnus::{Error, RArray, RHash, Ruby, Symbol, Value, TryConvert, IntoValue};
use magnus::value::ReprValue;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error_handling::runtime_error;

/// Convert Ruby Symbol or String to Rust String
pub fn symbol_to_string(value: Value) -> Result<String, Error> {
    if let Some(symbol) = Symbol::from_value(value) {
        Ok(symbol.name()?.to_string())
    } else {
        String::try_convert(value)
    }
}

/// Get keyword argument from hash (supports both symbol and string keys)
pub fn get_kw(ruby: &Ruby, hash: RHash, name: &str) -> Option<Value> {
    hash.get(name).or_else(|| {
        let sym = ruby.intern(name);
        hash.get(sym)
    })
}

/// Set a hash entry with a string key
pub fn set_hash_entry(_ruby: &Ruby, hash: &RHash, key: &str, value: Value) -> Result<(), Error> {
    hash.aset(key, value)?;
    Ok(())
}

/// Convert serde_json Value to Ruby Value
pub fn json_value_to_ruby(ruby: &Ruby, value: &serde_json::Value) -> Result<Value, Error> {
    Ok(match value {
        serde_json::Value::Null => ruby.qnil().as_value(),
        serde_json::Value::Bool(b) => {
            if *b {
                ruby.qtrue().as_value()
            } else {
                ruby.qfalse().as_value()
            }
        }
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                ruby.integer_from_i64(i).into_value_with(ruby)
            } else if let Some(u) = num.as_u64() {
                ruby.integer_from_u64(u).into_value_with(ruby)
            } else if let Some(f) = num.as_f64() {
                ruby.float_from_f64(f).into_value_with(ruby)
            } else {
                ruby.qnil().as_value()
            }
        }
        serde_json::Value::String(s) => ruby.str_new(s).into_value_with(ruby),
        serde_json::Value::Array(items) => {
            let ary = ruby.ary_new();
            for item in items {
                ary.push(json_value_to_ruby(ruby, item)?)?;
            }
            ary.into_value_with(ruby)
        }
        serde_json::Value::Object(map) => {
            let hash = ruby.hash_new();
            for (key, val) in map {
                let key_value = ruby.str_new(key).into_value_with(ruby);
                let val_value = json_value_to_ruby(ruby, val)?;
                hash.aset(key_value, val_value)?;
            }
            hash.into_value_with(ruby)
        }
    })
}

/// Convert Ruby key (String or Symbol) to Rust String
pub fn ruby_key_to_string(value: Value) -> Result<String, Error> {
    if let Ok(sym) = Symbol::try_convert(value) {
        Ok(sym.name()?.to_string())
    } else {
        String::try_convert(value)
    }
}

/// Convert Ruby Value to serde_json Value
pub fn ruby_value_to_json(value: Value) -> Result<serde_json::Value, Error> {
    let ruby = Ruby::get().expect("Ruby not initialized");

    if value.equal(ruby.qnil())? {
        return Ok(serde_json::Value::Null);
    }

    if value.equal(ruby.qtrue())? {
        return Ok(serde_json::Value::Bool(true));
    }

    if value.equal(ruby.qfalse())? {
        return Ok(serde_json::Value::Bool(false));
    }

    if let Ok(integer) = i64::try_convert(value) {
        return Ok(serde_json::Value::Number(integer.into()));
    }

    if let Ok(unsigned) = u64::try_convert(value) {
        return Ok(serde_json::Value::Number(serde_json::Number::from(unsigned)));
    }

    if let Ok(float) = f64::try_convert(value)
        && let Some(num) = serde_json::Number::from_f64(float)
    {
        return Ok(serde_json::Value::Number(num));
    }

    if let Ok(sym) = Symbol::try_convert(value) {
        return Ok(serde_json::Value::String(sym.name()?.to_string()));
    }

    if let Ok(string) = String::try_convert(value) {
        return Ok(serde_json::Value::String(string));
    }

    if let Ok(array) = RArray::try_convert(value) {
        let mut values = Vec::with_capacity(array.len());
        for item in array.into_iter() {
            values.push(ruby_value_to_json(item)?);
        }
        return Ok(serde_json::Value::Array(values));
    }

    if let Ok(hash) = RHash::try_convert(value) {
        let mut map = serde_json::Map::new();
        hash.foreach(|key: Value, val: Value| {
            let key_string = ruby_key_to_string(key)?;
            let json_value = ruby_value_to_json(val)?;
            map.insert(key_string, json_value);
            Ok(magnus::r_hash::ForEach::Continue)
        })?;

        return Ok(serde_json::Value::Object(map));
    }

    Err(runtime_error("Unsupported Ruby value for JSON conversion"))
}

/// Get the cache root directory
pub fn cache_root_dir() -> Result<PathBuf, Error> {
    std::env::current_dir()
        .map(|dir| dir.join(".kreuzberg"))
        .map_err(|e| runtime_error(format!("Failed to get current directory: {}", e)))
}

/// Get all cache directories (root and subdirectories)
pub fn cache_directories(root: &Path) -> Result<Vec<PathBuf>, Error> {
    if !root.exists() {
        return Ok(vec![]);
    }

    let mut dirs = vec![root.to_path_buf()];
    let entries = fs::read_dir(root).map_err(|e| runtime_error(format!("Failed to read cache root: {}", e)))?;

    for entry in entries {
        let entry = entry.map_err(|e| runtime_error(format!("Failed to read cache directory entry: {}", e)))?;
        if entry
            .file_type()
            .map_err(|e| runtime_error(format!("Failed to determine cache entry type: {}", e)))?
            .is_dir()
        {
            dirs.push(entry.path());
        }
    }

    Ok(dirs)
}
