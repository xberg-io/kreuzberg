//! Batch extraction functions
//!
//! Handles batch extraction of multiple files or byte arrays.

use crate::config::{parse_extraction_config, parse_file_extraction_config};
use crate::error_handling::{kreuzberg_error, runtime_error};
use crate::result::extraction_result_to_ruby;

use std::path::PathBuf;

use magnus::value::ReprValue;
use magnus::{Error, RArray, RHash, RString, Ruby, Value, scan_args::scan_args, TryConvert};

/// Batch extract content from multiple files (synchronous)
///
/// Accepts `paths` as an RArray of strings.
/// Optional keyword arg `file_configs`: RArray of config hashes (or nil per element),
/// must match paths length. When absent, all items get `None` config.
pub fn batch_extract_files_sync(args: &[Value]) -> Result<RArray, Error> {
    let ruby = Ruby::get().expect("Ruby not initialized");
    let args = scan_args::<(RArray,), (), (), (), RHash, ()>(args)?;
    let (paths_array,) = args.required;
    let opts = Some(args.keywords);

    let config = parse_extraction_config(&ruby, opts)?;

    let paths: Vec<String> = paths_array.to_vec::<String>()?;

    let items = build_file_items(&ruby, &paths, opts)?;

    let results = kreuzberg::batch_extract_file_sync(items, &config).map_err(kreuzberg_error)?;

    let results_array = ruby.ary_new();
    for result in results {
        results_array.push(extraction_result_to_ruby(&ruby, result)?)?;
    }

    Ok(results_array)
}

/// Batch extract content from multiple files (asynchronous)
pub fn batch_extract_files(args: &[Value]) -> Result<RArray, Error> {
    let ruby = Ruby::get().expect("Ruby not initialized");
    let args = scan_args::<(RArray,), (), (), (), RHash, ()>(args)?;
    let (paths_array,) = args.required;
    let opts = Some(args.keywords);

    let config = parse_extraction_config(&ruby, opts)?;

    let paths: Vec<String> = paths_array.to_vec::<String>()?;

    let items = build_file_items(&ruby, &paths, opts)?;

    let runtime = tokio::runtime::Runtime::new()
        .map_err(|e| runtime_error(format!("Failed to create Tokio runtime: {}", e)))?;

    let results = runtime
        .block_on(async { kreuzberg::batch_extract_file(items, &config).await })
        .map_err(kreuzberg_error)?;

    let results_array = ruby.ary_new();
    for result in results {
        results_array.push(extraction_result_to_ruby(&ruby, result)?)?;
    }

    Ok(results_array)
}

/// Batch extract content from multiple byte arrays (synchronous)
///
/// Accepts `bytes_array` and `mime_types` as required positional args.
/// Optional keyword arg `file_configs`: RArray of config hashes (or nil per element).
pub fn batch_extract_bytes_sync(args: &[Value]) -> Result<RArray, Error> {
    let ruby = Ruby::get().expect("Ruby not initialized");
    let args = scan_args::<(RArray, RArray), (), (), (), RHash, ()>(args)?;
    let (bytes_array, mime_types_array) = args.required;
    let opts = Some(args.keywords);

    let config = parse_extraction_config(&ruby, opts)?;

    let bytes_vec: Vec<RString> = bytes_array
        .into_iter()
        .map(RString::try_convert)
        .collect::<Result<_, _>>()?;
    let mime_types: Vec<String> = mime_types_array.to_vec::<String>()?;

    if bytes_vec.len() != mime_types.len() {
        return Err(runtime_error(format!(
            "bytes_array and mime_types must have the same length: {} vs {}",
            bytes_vec.len(),
            mime_types.len()
        )));
    }

    let items = build_bytes_items(&ruby, &bytes_vec, &mime_types, opts)?;

    let results = kreuzberg::batch_extract_bytes_sync(items, &config).map_err(kreuzberg_error)?;

    let results_array = ruby.ary_new();
    for result in results {
        results_array.push(extraction_result_to_ruby(&ruby, result)?)?;
    }

    Ok(results_array)
}

/// Batch extract content from multiple byte arrays (asynchronous)
pub fn batch_extract_bytes(args: &[Value]) -> Result<RArray, Error> {
    let ruby = Ruby::get().expect("Ruby not initialized");
    let args = scan_args::<(RArray, RArray), (), (), (), RHash, ()>(args)?;
    let (bytes_array, mime_types_array) = args.required;
    let opts = Some(args.keywords);

    let config = parse_extraction_config(&ruby, opts)?;

    let bytes_vec: Vec<RString> = bytes_array
        .into_iter()
        .map(RString::try_convert)
        .collect::<Result<_, _>>()?;
    let mime_types: Vec<String> = mime_types_array.to_vec::<String>()?;

    if bytes_vec.len() != mime_types.len() {
        return Err(runtime_error(format!(
            "bytes_array and mime_types must have the same length: {} vs {}",
            bytes_vec.len(),
            mime_types.len()
        )));
    }

    let items = build_bytes_items(&ruby, &bytes_vec, &mime_types, opts)?;

    let runtime = tokio::runtime::Runtime::new()
        .map_err(|e| runtime_error(format!("Failed to create Tokio runtime: {}", e)))?;

    let results = runtime
        .block_on(async { kreuzberg::batch_extract_bytes(items, &config).await })
        .map_err(kreuzberg_error)?;

    let results_array = ruby.ary_new();
    for result in results {
        results_array.push(extraction_result_to_ruby(&ruby, result)?)?;
    }

    Ok(results_array)
}

/// Build file items from paths and optional file_configs keyword arg.
///
/// If `file_configs` keyword is present in opts, zip with paths.
/// Otherwise, all items get `None` config.
fn build_file_items(
    ruby: &Ruby,
    paths: &[String],
    opts: Option<RHash>,
) -> Result<Vec<(PathBuf, Option<kreuzberg::FileExtractionConfig>)>, Error> {
    let file_configs_array: Option<RArray> = opts
        .and_then(|kw| kw.get(ruby.to_symbol("file_configs")))
        .and_then(|v: Value| {
            match v.equal(ruby.qnil()) {
                Ok(true) => None,
                Ok(false) => RArray::try_convert(v).ok(),
                Err(_) => None,
            }
        });

    match file_configs_array {
        Some(fc_array) => {
            if fc_array.len() != paths.len() {
                return Err(runtime_error(format!(
                    "file_configs must have the same length as paths: {} vs {}",
                    fc_array.len(),
                    paths.len()
                )));
            }
            let mut items = Vec::with_capacity(paths.len());
            for (i, path) in paths.iter().enumerate() {
                let fc_val = fc_array.entry::<Value>(i as isize)?;
                let file_config = parse_file_extraction_config(fc_val)?;
                items.push((PathBuf::from(path), file_config));
            }
            Ok(items)
        }
        None => Ok(paths
            .iter()
            .map(|p| (PathBuf::from(p), None))
            .collect()),
    }
}

/// Build bytes items from byte arrays, mime types, and optional file_configs keyword arg.
fn build_bytes_items(
    ruby: &Ruby,
    bytes_vec: &[RString],
    mime_types: &[String],
    opts: Option<RHash>,
) -> Result<Vec<(Vec<u8>, String, Option<kreuzberg::FileExtractionConfig>)>, Error> {
    let file_configs_array: Option<RArray> = opts
        .and_then(|kw| kw.get(ruby.to_symbol("file_configs")))
        .and_then(|v: Value| {
            match v.equal(ruby.qnil()) {
                Ok(true) => None,
                Ok(false) => RArray::try_convert(v).ok(),
                Err(_) => None,
            }
        });

    match file_configs_array {
        Some(fc_array) => {
            if fc_array.len() != bytes_vec.len() {
                return Err(runtime_error(format!(
                    "file_configs must have the same length as bytes_array: {} vs {}",
                    fc_array.len(),
                    bytes_vec.len()
                )));
            }
            let mut items = Vec::with_capacity(bytes_vec.len());
            for (i, (bytes, mime)) in bytes_vec.iter().zip(mime_types.iter()).enumerate() {
                let fc_val = fc_array.entry::<Value>(i as isize)?;
                let file_config = parse_file_extraction_config(fc_val)?;
                items.push((
                    unsafe { bytes.as_slice() }.to_vec(),
                    mime.clone(),
                    file_config,
                ));
            }
            Ok(items)
        }
        None => Ok(bytes_vec
            .iter()
            .zip(mime_types.iter())
            .map(|(bytes, mime)| {
                (
                    unsafe { bytes.as_slice() }.to_vec(),
                    mime.clone(),
                    None,
                )
            })
            .collect()),
    }
}
