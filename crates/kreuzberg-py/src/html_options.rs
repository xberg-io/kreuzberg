//! HTML to Markdown conversion options parsing
//!
//! Provides functions to parse and validate HTML conversion options from Python dictionaries.
//! Delegates enum parsing to FFI layer functions and handles conversion option assembly.

#![allow(unsafe_code)]

use html_to_markdown_rs::options::{
    CodeBlockStyle, ConversionOptions, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, PreprocessingPreset,
    WhitespaceMode,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;

/// Parse a Python dictionary into ConversionOptions.
///
/// Extracts all known HTML conversion options from a Python dictionary and
/// constructs a ConversionOptions struct with the parsed values.
///
/// Returns a tuple of (parsed_options, stored_dict) for later reference.
pub fn parse_html_options_dict(
    options: Option<Bound<'_, PyDict>>,
) -> PyResult<(Option<ConversionOptions>, Option<Py<PyDict>>)> {
    if let Some(dict) = options {
        let parsed = parse_html_options(&dict)?;
        Ok((Some(parsed), Some(dict.unbind())))
    } else {
        Ok((None, None))
    }
}

/// Parse all HTML conversion options from a Python dictionary.
///
/// Iterates through known option keys and extracts their values, delegating
/// enum parsing to specialized functions.
fn parse_html_options(dict: &Bound<'_, PyDict>) -> PyResult<ConversionOptions> {
    let mut opts = ConversionOptions::default();

    if let Some(value) = dict.get_item("heading_style")? {
        let style: String = value.extract()?;
        opts.heading_style = parse_heading_style(&style)?;
    }

    if let Some(value) = dict.get_item("list_indent_type")? {
        let value: String = value.extract()?;
        opts.list_indent_type = parse_list_indent_type(&value)?;
    }

    if let Some(value) = dict.get_item("list_indent_width")? {
        opts.list_indent_width = value.extract()?;
    }

    if let Some(value) = dict.get_item("bullets")? {
        opts.bullets = value.extract()?;
    }

    if let Some(value) = dict.get_item("strong_em_symbol")? {
        let symbol: String = value.extract()?;
        let mut chars = symbol.chars();
        let ch = chars
            .next()
            .ok_or_else(|| PyValueError::new_err("strong_em_symbol must not be empty"))?;
        opts.strong_em_symbol = ch;
    }

    if let Some(value) = dict.get_item("escape_asterisks")? {
        opts.escape_asterisks = value.extract()?;
    }
    if let Some(value) = dict.get_item("escape_underscores")? {
        opts.escape_underscores = value.extract()?;
    }
    if let Some(value) = dict.get_item("escape_misc")? {
        opts.escape_misc = value.extract()?;
    }
    if let Some(value) = dict.get_item("escape_ascii")? {
        opts.escape_ascii = value.extract()?;
    }

    if let Some(value) = dict.get_item("code_language")? {
        opts.code_language = value.extract()?;
    }

    if let Some(value) = dict.get_item("autolinks")? {
        opts.autolinks = value.extract()?;
    }

    if let Some(value) = dict.get_item("default_title")? {
        opts.default_title = value.extract()?;
    }

    if let Some(value) = dict.get_item("br_in_tables")? {
        opts.br_in_tables = value.extract()?;
    }

    if let Some(value) = dict.get_item("hocr_spatial_tables")? {
        opts.hocr_spatial_tables = value.extract()?;
    }

    if let Some(value) = dict.get_item("highlight_style")? {
        let style: String = value.extract()?;
        opts.highlight_style = parse_highlight_style(&style)?;
    }

    if let Some(value) = dict.get_item("extract_metadata")? {
        opts.extract_metadata = value.extract()?;
    }

    if let Some(value) = dict.get_item("whitespace_mode")? {
        let mode: String = value.extract()?;
        opts.whitespace_mode = parse_whitespace_mode(&mode)?;
    }

    if let Some(value) = dict.get_item("strip_newlines")? {
        opts.strip_newlines = value.extract()?;
    }

    if let Some(value) = dict.get_item("wrap")? {
        opts.wrap = value.extract()?;
    }

    if let Some(value) = dict.get_item("wrap_width")? {
        opts.wrap_width = value.extract()?;
    }

    if let Some(value) = dict.get_item("convert_as_inline")? {
        opts.convert_as_inline = value.extract()?;
    }

    if let Some(value) = dict.get_item("sub_symbol")? {
        opts.sub_symbol = value.extract()?;
    }

    if let Some(value) = dict.get_item("sup_symbol")? {
        opts.sup_symbol = value.extract()?;
    }

    if let Some(value) = dict.get_item("newline_style")? {
        let style: String = value.extract()?;
        opts.newline_style = parse_newline_style(&style)?;
    }

    if let Some(value) = dict.get_item("code_block_style")? {
        let style: String = value.extract()?;
        opts.code_block_style = parse_code_block_style(&style)?;
    }

    if let Some(value) = dict.get_item("keep_inline_images_in")? {
        opts.keep_inline_images_in = value.extract()?;
    }

    if let Some(value) = dict.get_item("encoding")? {
        opts.encoding = value.extract()?;
    }

    if let Some(value) = dict.get_item("debug")? {
        opts.debug = value.extract()?;
    }

    if let Some(value) = dict.get_item("strip_tags")? {
        opts.strip_tags = value.extract()?;
    }

    if let Some(value) = dict.get_item("preserve_tags")? {
        opts.preserve_tags = value.extract()?;
    }

    if let Some(value) = dict.get_item("preprocessing")? {
        let pre_dict: Bound<'_, PyDict> = value.cast::<PyDict>()?.clone();
        let mut preprocessing = opts.preprocessing.clone();

        if let Some(v) = pre_dict.get_item("enabled")? {
            preprocessing.enabled = v.extract()?;
        }

        if let Some(v) = pre_dict.get_item("preset")? {
            let preset: String = v.extract()?;
            preprocessing.preset = parse_preprocessing_preset(&preset)?;
        }

        if let Some(v) = pre_dict.get_item("remove_navigation")? {
            preprocessing.remove_navigation = v.extract()?;
        }

        if let Some(v) = pre_dict.get_item("remove_forms")? {
            preprocessing.remove_forms = v.extract()?;
        }

        opts.preprocessing = preprocessing;
    }

    Ok(opts)
}

/// Parse heading style string to HeadingStyle enum.
/// Delegates to FFI layer for parsing logic.
fn parse_heading_style(value: &str) -> PyResult<HeadingStyle> {
    let c_value = CString::new(value)
        .map_err(|_| PyValueError::new_err(format!("Invalid heading_style '{}'. Contains null bytes", value)))?;

    let discriminant = unsafe { kreuzberg_ffi::kreuzberg_parse_heading_style(c_value.as_ptr()) };

    match discriminant {
        0 => Ok(HeadingStyle::Atx),
        1 => Ok(HeadingStyle::Underlined),
        2 => Ok(HeadingStyle::AtxClosed),
        _ => Err(PyValueError::new_err(format!(
            "Invalid heading_style '{}'. Expected one of: atx, underlined, atx_closed",
            value
        ))),
    }
}

/// Parse list indent type string to ListIndentType enum.
/// Delegates to FFI layer for parsing logic.
fn parse_list_indent_type(value: &str) -> PyResult<ListIndentType> {
    let c_value = CString::new(value)
        .map_err(|_| PyValueError::new_err(format!("Invalid list_indent_type '{}'. Contains null bytes", value)))?;

    let discriminant = unsafe { kreuzberg_ffi::kreuzberg_parse_list_indent_type(c_value.as_ptr()) };

    match discriminant {
        0 => Ok(ListIndentType::Spaces),
        1 => Ok(ListIndentType::Tabs),
        _ => Err(PyValueError::new_err(format!(
            "Invalid list_indent_type '{}'. Expected 'spaces' or 'tabs'",
            value
        ))),
    }
}

/// Parse highlight style string to HighlightStyle enum.
/// Delegates to FFI layer for parsing logic.
fn parse_highlight_style(value: &str) -> PyResult<HighlightStyle> {
    let c_value = CString::new(value)
        .map_err(|_| PyValueError::new_err(format!("Invalid highlight_style '{}'. Contains null bytes", value)))?;

    let discriminant = unsafe { kreuzberg_ffi::kreuzberg_parse_highlight_style(c_value.as_ptr()) };

    match discriminant {
        0 => Ok(HighlightStyle::DoubleEqual),
        1 => Ok(HighlightStyle::Html),
        2 => Ok(HighlightStyle::Bold),
        3 => Ok(HighlightStyle::None),
        _ => Err(PyValueError::new_err(format!(
            "Invalid highlight_style '{}'. Expected one of: double_equal, html, bold, none",
            value
        ))),
    }
}

/// Parse whitespace mode string to WhitespaceMode enum.
fn parse_whitespace_mode(value: &str) -> PyResult<WhitespaceMode> {
    match value.to_lowercase().as_str() {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        other => Err(PyValueError::new_err(format!(
            "Invalid whitespace_mode '{}'. Expected 'normalized' or 'strict'",
            other
        ))),
    }
}

/// Parse newline style string to NewlineStyle enum.
fn parse_newline_style(value: &str) -> PyResult<NewlineStyle> {
    match value.to_lowercase().as_str() {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        other => Err(PyValueError::new_err(format!(
            "Invalid newline_style '{}'. Expected 'spaces' or 'backslash'",
            other
        ))),
    }
}

/// Parse code block style string to CodeBlockStyle enum.
/// Delegates to FFI layer for parsing logic.
fn parse_code_block_style(value: &str) -> PyResult<CodeBlockStyle> {
    let c_value = CString::new(value)
        .map_err(|_| PyValueError::new_err(format!("Invalid code_block_style '{}'. Contains null bytes", value)))?;

    let discriminant = unsafe { kreuzberg_ffi::kreuzberg_parse_code_block_style(c_value.as_ptr()) };

    match discriminant {
        0 => Ok(CodeBlockStyle::Indented),
        1 => Ok(CodeBlockStyle::Backticks),
        2 => Ok(CodeBlockStyle::Tildes),
        _ => Err(PyValueError::new_err(format!(
            "Invalid code_block_style '{}'. Expected 'indented', 'backticks', or 'tildes'",
            value
        ))),
    }
}

/// Parse preprocessing preset string to PreprocessingPreset enum.
fn parse_preprocessing_preset(value: &str) -> PyResult<PreprocessingPreset> {
    match value.to_lowercase().as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "standard" => Ok(PreprocessingPreset::Standard),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        other => Err(PyValueError::new_err(format!(
            "Invalid preprocessing.preset '{}'. Expected one of: minimal, standard, aggressive",
            other
        ))),
    }
}
