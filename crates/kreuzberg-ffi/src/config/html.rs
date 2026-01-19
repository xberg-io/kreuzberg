//! HTML options parsing from JSON
//!
//! Handles the complex nested structure of HTML conversion options.

use html_to_markdown_rs::options::{
    CodeBlockStyle, ConversionOptions, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, PreprocessingPreset,
    WhitespaceMode,
};

type FfiResult<T> = std::result::Result<T, String>;

/// Parse enum value from optional JSON value
fn parse_enum<T, F>(value: Option<&serde_json::Value>, parse_fn: F) -> FfiResult<Option<T>>
where
    F: Fn(&str) -> FfiResult<T>,
{
    if let Some(raw) = value {
        let text = raw
            .as_str()
            .ok_or_else(|| "Expected string for enum field".to_string())?;
        return parse_fn(text).map(Some);
    }
    Ok(None)
}

/// Parse HeadingStyle from string
fn parse_heading_style(value: &str) -> FfiResult<HeadingStyle> {
    match value.to_lowercase().as_str() {
        "atx" => Ok(HeadingStyle::Atx),
        "underlined" => Ok(HeadingStyle::Underlined),
        "atx_closed" => Ok(HeadingStyle::AtxClosed),
        other => Err(format!(
            "Invalid heading_style '{}'. Expected one of: atx, underlined, atx_closed",
            other
        )),
    }
}

/// Parse ListIndentType from string
fn parse_list_indent_type(value: &str) -> FfiResult<ListIndentType> {
    match value.to_lowercase().as_str() {
        "spaces" => Ok(ListIndentType::Spaces),
        "tabs" => Ok(ListIndentType::Tabs),
        other => Err(format!(
            "Invalid list_indent_type '{}'. Expected 'spaces' or 'tabs'",
            other
        )),
    }
}

/// Parse HighlightStyle from string
fn parse_highlight_style(value: &str) -> FfiResult<HighlightStyle> {
    match value.to_lowercase().as_str() {
        "double_equal" | "==" | "highlight" => Ok(HighlightStyle::DoubleEqual),
        "html" => Ok(HighlightStyle::Html),
        "bold" => Ok(HighlightStyle::Bold),
        "none" => Ok(HighlightStyle::None),
        other => Err(format!(
            "Invalid highlight_style '{}'. Expected one of: double_equal, html, bold, none",
            other
        )),
    }
}

/// Parse WhitespaceMode from string
fn parse_whitespace_mode(value: &str) -> FfiResult<WhitespaceMode> {
    match value.to_lowercase().as_str() {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        other => Err(format!(
            "Invalid whitespace_mode '{}'. Expected 'normalized' or 'strict'",
            other
        )),
    }
}

/// Parse NewlineStyle from string
fn parse_newline_style(value: &str) -> FfiResult<NewlineStyle> {
    match value.to_lowercase().as_str() {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        other => Err(format!(
            "Invalid newline_style '{}'. Expected 'spaces' or 'backslash'",
            other
        )),
    }
}

/// Parse CodeBlockStyle from string
fn parse_code_block_style(value: &str) -> FfiResult<CodeBlockStyle> {
    match value.to_lowercase().as_str() {
        "indented" => Ok(CodeBlockStyle::Indented),
        "backticks" => Ok(CodeBlockStyle::Backticks),
        "tildes" => Ok(CodeBlockStyle::Tildes),
        other => Err(format!(
            "Invalid code_block_style '{}'. Expected 'indented', 'backticks', or 'tildes'",
            other
        )),
    }
}

/// Parse PreprocessingPreset from string
#[allow(dead_code)]
fn parse_preprocessing_preset(value: &str) -> FfiResult<PreprocessingPreset> {
    match value.to_lowercase().as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "standard" => Ok(PreprocessingPreset::Standard),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        other => Err(format!(
            "Invalid preprocessing.preset '{}'. Expected one of: minimal, standard, aggressive",
            other
        )),
    }
}

/// Parse HTML conversion options from JSON value
pub fn parse_html_options(value: &serde_json::Value) -> FfiResult<ConversionOptions> {
    let mut opts = ConversionOptions::default();
    let obj = value
        .as_object()
        .ok_or_else(|| "html_options must be an object".to_string())?;

    if let Some(val) = obj.get("heading_style") {
        opts.heading_style = parse_enum(Some(val), parse_heading_style)?.unwrap_or(opts.heading_style);
    }

    if let Some(val) = obj.get("list_indent_type") {
        opts.list_indent_type = parse_enum(Some(val), parse_list_indent_type)?.unwrap_or(opts.list_indent_type);
    }

    if let Some(val) = obj.get("list_indent_width") {
        opts.list_indent_width = val
            .as_u64()
            .map(|v| v as usize)
            .ok_or_else(|| "list_indent_width must be an integer".to_string())?;
    }

    if let Some(val) = obj.get("bullets") {
        opts.bullets = val
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "bullets must be a string".to_string())?;
    }

    if let Some(val) = obj.get("strong_em_symbol") {
        let symbol = val
            .as_str()
            .ok_or_else(|| "strong_em_symbol must be a string".to_string())?;
        let mut chars = symbol.chars();
        opts.strong_em_symbol = chars
            .next()
            .ok_or_else(|| "strong_em_symbol must not be empty".to_string())?;
    }

    if let Some(val) = obj.get("escape_asterisks") {
        opts.escape_asterisks = val
            .as_bool()
            .ok_or_else(|| "escape_asterisks must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("escape_underscores") {
        opts.escape_underscores = val
            .as_bool()
            .ok_or_else(|| "escape_underscores must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("escape_misc") {
        opts.escape_misc = val
            .as_bool()
            .ok_or_else(|| "escape_misc must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("escape_ascii") {
        opts.escape_ascii = val
            .as_bool()
            .ok_or_else(|| "escape_ascii must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("code_language") {
        opts.code_language = val
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "code_language must be a string".to_string())?;
    }

    if let Some(val) = obj.get("autolinks") {
        opts.autolinks = val.as_bool().ok_or_else(|| "autolinks must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("default_title") {
        opts.default_title = val
            .as_bool()
            .ok_or_else(|| "default_title must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("br_in_tables") {
        opts.br_in_tables = val
            .as_bool()
            .ok_or_else(|| "br_in_tables must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("hocr_spatial_tables") {
        opts.hocr_spatial_tables = val
            .as_bool()
            .ok_or_else(|| "hocr_spatial_tables must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("highlight_style") {
        opts.highlight_style = parse_enum(Some(val), parse_highlight_style)?.unwrap_or(opts.highlight_style);
    }

    if let Some(val) = obj.get("extract_metadata") {
        opts.extract_metadata = val
            .as_bool()
            .ok_or_else(|| "extract_metadata must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("whitespace_mode") {
        opts.whitespace_mode = parse_enum(Some(val), parse_whitespace_mode)?.unwrap_or(opts.whitespace_mode);
    }

    if let Some(val) = obj.get("strip_newlines") {
        opts.strip_newlines = val
            .as_bool()
            .ok_or_else(|| "strip_newlines must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("wrap") {
        opts.wrap = val.as_bool().ok_or_else(|| "wrap must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("wrap_width") {
        opts.wrap_width = val
            .as_u64()
            .map(|v| v as usize)
            .ok_or_else(|| "wrap_width must be an integer".to_string())?;
    }

    if let Some(val) = obj.get("convert_as_inline") {
        opts.convert_as_inline = val
            .as_bool()
            .ok_or_else(|| "convert_as_inline must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("sub_symbol") {
        opts.sub_symbol = val
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "sub_symbol must be a string".to_string())?;
    }

    if let Some(val) = obj.get("sup_symbol") {
        opts.sup_symbol = val
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "sup_symbol must be a string".to_string())?;
    }

    if let Some(val) = obj.get("newline_style") {
        opts.newline_style = parse_enum(Some(val), parse_newline_style)?.unwrap_or(opts.newline_style);
    }

    if let Some(val) = obj.get("code_block_style") {
        opts.code_block_style = parse_enum(Some(val), parse_code_block_style)?.unwrap_or(opts.code_block_style);
    }

    if let Some(val) = obj.get("keep_inline_images_in") {
        opts.keep_inline_images_in = val
            .as_array()
            .ok_or_else(|| "keep_inline_images_in must be an array".to_string())?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| "keep_inline_images_in entries must be strings".to_string())
            })
            .collect::<FfiResult<Vec<_>>>()?;
    }

    if let Some(val) = obj.get("encoding") {
        opts.encoding = val
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "encoding must be a string".to_string())?;
    }

    if let Some(val) = obj.get("debug") {
        opts.debug = val.as_bool().ok_or_else(|| "debug must be a boolean".to_string())?;
    }

    if let Some(val) = obj.get("strip_tags") {
        opts.strip_tags = val
            .as_array()
            .ok_or_else(|| "strip_tags must be an array".to_string())?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| "strip_tags entries must be strings".to_string())
            })
            .collect::<FfiResult<Vec<_>>>()?;
    }

    if let Some(val) = obj.get("preserve_tags") {
        opts.preserve_tags = val
            .as_array()
            .ok_or_else(|| "preserve_tags must be an array".to_string())?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| "preserve_tags entries must be strings".to_string())
            })
            .collect::<FfiResult<Vec<_>>>()?;
    }

    Ok(opts)
}
