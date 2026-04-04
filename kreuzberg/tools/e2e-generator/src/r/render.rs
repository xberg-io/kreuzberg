use serde_json::{Map, Value};

/// Render a JSON value as R code
pub fn render_r_value(value: &Value) -> String {
    match value {
        Value::Null => "NULL".into(),
        Value::Bool(b) => if *b { "TRUE" } else { "FALSE" }.into(),
        Value::Number(n) => render_number_value(n),
        Value::String(s) => render_r_string(s),
        Value::Array(items) => {
            if items.is_empty() {
                "c()".into()
            } else {
                let inner = items.iter().map(render_r_value).collect::<Vec<_>>().join(", ");
                format!("c({inner})")
            }
        }
        Value::Object(map) => render_r_list(map),
    }
}

/// Render a string as an R string literal (using double quotes, escaping as needed)
pub fn render_r_string(text: &str) -> String {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("\"{escaped}\"")
}

/// Render a vector of strings as an R character vector: c("a", "b", "c")
pub fn render_string_vector(items: &[String]) -> String {
    if items.is_empty() {
        "character(0)".into()
    } else {
        let inner = items.iter().map(|s| render_r_string(s)).collect::<Vec<_>>().join(", ");
        format!("c({inner})")
    }
}

/// Render an optional string as R NULL or a string literal
pub fn render_optional_string(value: Option<&String>) -> String {
    match value {
        Some(text) => render_r_string(text),
        None => "NULL".into(),
    }
}

/// Render a numeric literal with underscore separators for readability (R uses L suffix for integers)
pub fn render_numeric_literal(value: u64) -> String {
    // R doesn't use underscore separators; just use plain numbers with L for integer
    format!("{}L", value)
}

/// Render a JSON number as R code
fn render_number_value(number: &serde_json::Number) -> String {
    if let Some(v) = number.as_u64() {
        format!("{}L", v)
    } else if let Some(v) = number.as_i64() {
        format!("{}L", v)
    } else if let Some(v) = number.as_f64() {
        // Ensure we always have a decimal point
        let s = v.to_string();
        if s.contains('.') { s } else { format!("{}.0", s) }
    } else {
        number.to_string()
    }
}

/// Render a JSON object as an R named list: list(key1 = val1, key2 = val2)
pub fn render_r_list(map: &Map<String, Value>) -> String {
    if map.is_empty() {
        return "list()".into();
    }
    let pairs = map
        .iter()
        .map(|(key, value)| {
            let r_key = sanitize_r_name(key);
            format!("{} = {}", r_key, render_r_value(value))
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("list({pairs})")
}

/// Render a config expression from a JSON map, returning None if empty
pub fn render_config_expression(config: &Map<String, Value>) -> Option<String> {
    if config.is_empty() {
        None
    } else {
        Some(render_r_value(&Value::Object(config.clone())))
    }
}

/// Sanitize a string for use as an R identifier/name
pub fn sanitize_identifier(input: &str) -> String {
    let mut output = String::new();
    for (idx, ch) in input.chars().enumerate() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
            if idx == 0 && ch.is_ascii_digit() {
                output.push('_');
            }
            output.push(ch);
        } else {
            output.push('_');
        }
    }
    if output.is_empty() { "fixture".into() } else { output }
}

/// Sanitize a key for use as an R list name (replace hyphens with underscores)
fn sanitize_r_name(key: &str) -> String {
    // R names can contain dots and underscores but not hyphens
    key.replace('-', "_")
}

/// Escape content for embedding inside an R string literal
pub fn escape_r_string_content(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Convert a snake_case string to Title Case
pub fn to_title_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
