//! Shared FFI functions for parsing HTML-to-Markdown conversion options.
//!
//! This module provides C FFI functions for parsing enum values from strings,
//! eliminating duplication across language bindings (Node.js, Python, Ruby).
//! Each enum parser returns an i32 discriminant (or -1 for invalid input).

use std::ffi::{CStr, c_char};
use std::ptr;

/// Parse HeadingStyle from string to discriminant.
///
/// Valid values: "atx", "underlined", "atx_closed" | "atx-closed"
/// Returns: 0 = Atx, 1 = Underlined, 2 = AtxClosed, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_heading_style(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "atx" => 0,
        "underlined" => 1,
        "atx_closed" | "atx-closed" => 2,
        _ => -1,
    }
}

/// Convert HeadingStyle discriminant to string.
///
/// Returns: pointer to static string, or NULL for invalid discriminant
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_heading_style_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"atx".as_ptr(),
        1 => c"underlined".as_ptr(),
        2 => c"atx_closed".as_ptr(),
        _ => ptr::null(),
    }
}

/// Parse CodeBlockStyle from string to discriminant.
///
/// Valid values: "indented", "backticks", "tildes"
/// Returns: 0 = Indented, 1 = Backticks, 2 = Tildes, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_code_block_style(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "indented" => 0,
        "backticks" => 1,
        "tildes" => 2,
        _ => -1,
    }
}

/// Convert CodeBlockStyle discriminant to string.
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_code_block_style_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"indented".as_ptr(),
        1 => c"backticks".as_ptr(),
        2 => c"tildes".as_ptr(),
        _ => ptr::null(),
    }
}

/// Parse HighlightStyle from string to discriminant.
///
/// Valid values: "double_equal" | "==" | "double-equal", "html", "bold", "none"
/// Returns: 0 = DoubleEqual, 1 = Html, 2 = Bold, 3 = None, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_highlight_style(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "double_equal" | "==" | "double-equal" => 0,
        "html" => 1,
        "bold" => 2,
        "none" => 3,
        _ => -1,
    }
}

/// Convert HighlightStyle discriminant to string.
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_highlight_style_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"double_equal".as_ptr(),
        1 => c"html".as_ptr(),
        2 => c"bold".as_ptr(),
        3 => c"none".as_ptr(),
        _ => ptr::null(),
    }
}

/// Parse ListIndentType from string to discriminant.
///
/// Valid values: "spaces", "tabs"
/// Returns: 0 = Spaces, 1 = Tabs, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_list_indent_type(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "spaces" => 0,
        "tabs" => 1,
        _ => -1,
    }
}

/// Convert ListIndentType discriminant to string.
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_list_indent_type_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"spaces".as_ptr(),
        1 => c"tabs".as_ptr(),
        _ => ptr::null(),
    }
}

/// Parse WhitespaceMode from string to discriminant.
///
/// Valid values: "default", "preserve", "preserve_inner", "collapse"
/// Returns: 0 = Default, 1 = Preserve, 2 = PreserveInner, 3 = Collapse, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_whitespace_mode(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "default" => 0,
        "preserve" => 1,
        "preserve_inner" | "preserve-inner" => 2,
        "collapse" => 3,
        _ => -1,
    }
}

/// Convert WhitespaceMode discriminant to string.
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_whitespace_mode_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"default".as_ptr(),
        1 => c"preserve".as_ptr(),
        2 => c"preserve_inner".as_ptr(),
        3 => c"collapse".as_ptr(),
        _ => ptr::null(),
    }
}

/// Parse NewlineStyle from string to discriminant.
///
/// Valid values: "default", "spaces", "backslash"
/// Returns: 0 = Default, 1 = Spaces, 2 = Backslash, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_newline_style(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "default" => 0,
        "spaces" => 1,
        "backslash" => 2,
        _ => -1,
    }
}

/// Convert NewlineStyle discriminant to string.
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_newline_style_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"default".as_ptr(),
        1 => c"spaces".as_ptr(),
        2 => c"backslash".as_ptr(),
        _ => ptr::null(),
    }
}

/// Parse PreprocessingPreset from string to discriminant.
///
/// Valid values: "none", "conservative", "aggressive"
/// Returns: 0 = None, 1 = Conservative, 2 = Aggressive, -1 = Invalid
///
/// # Safety
///
/// - `value` must be a valid null-terminated C string or NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_parse_preprocessing_preset(value: *const c_char) -> i32 {
    if value.is_null() {
        return -1;
    }

    let c_str = match unsafe { CStr::from_ptr(value) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match c_str.to_lowercase().as_str() {
        "none" => 0,
        "conservative" => 1,
        "aggressive" => 2,
        _ => -1,
    }
}

/// Convert PreprocessingPreset discriminant to string.
#[unsafe(no_mangle)]
pub extern "C" fn kreuzberg_preprocessing_preset_to_string(discriminant: i32) -> *const c_char {
    match discriminant {
        0 => c"none".as_ptr(),
        1 => c"conservative".as_ptr(),
        2 => c"aggressive".as_ptr(),
        _ => ptr::null(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_heading_style_parsing() {
        unsafe {
            let atx = CString::new("atx").unwrap();
            assert_eq!(kreuzberg_parse_heading_style(atx.as_ptr()), 0);

            let underlined = CString::new("underlined").unwrap();
            assert_eq!(kreuzberg_parse_heading_style(underlined.as_ptr()), 1);

            let atx_closed = CString::new("atx_closed").unwrap();
            assert_eq!(kreuzberg_parse_heading_style(atx_closed.as_ptr()), 2);

            let atx_closed_dash = CString::new("atx-closed").unwrap();
            assert_eq!(kreuzberg_parse_heading_style(atx_closed_dash.as_ptr()), 2);

            let invalid = CString::new("invalid").unwrap();
            assert_eq!(kreuzberg_parse_heading_style(invalid.as_ptr()), -1);

            assert_eq!(kreuzberg_parse_heading_style(ptr::null()), -1);
        }
    }

    #[test]
    fn test_heading_style_to_string() {
        unsafe {
            assert_eq!(
                CStr::from_ptr(kreuzberg_heading_style_to_string(0)).to_str().unwrap(),
                "atx"
            );
            assert_eq!(
                CStr::from_ptr(kreuzberg_heading_style_to_string(1)).to_str().unwrap(),
                "underlined"
            );
            assert_eq!(
                CStr::from_ptr(kreuzberg_heading_style_to_string(2)).to_str().unwrap(),
                "atx_closed"
            );
            assert!(kreuzberg_heading_style_to_string(99).is_null());
        }
    }

    #[test]
    fn test_code_block_style_parsing() {
        unsafe {
            let indented = CString::new("indented").unwrap();
            assert_eq!(kreuzberg_parse_code_block_style(indented.as_ptr()), 0);

            let backticks = CString::new("backticks").unwrap();
            assert_eq!(kreuzberg_parse_code_block_style(backticks.as_ptr()), 1);

            let tildes = CString::new("tildes").unwrap();
            assert_eq!(kreuzberg_parse_code_block_style(tildes.as_ptr()), 2);

            let invalid = CString::new("invalid").unwrap();
            assert_eq!(kreuzberg_parse_code_block_style(invalid.as_ptr()), -1);
        }
    }

    #[test]
    fn test_highlight_style_parsing() {
        unsafe {
            let double_eq = CString::new("double_equal").unwrap();
            assert_eq!(kreuzberg_parse_highlight_style(double_eq.as_ptr()), 0);

            let eq_symbols = CString::new("==").unwrap();
            assert_eq!(kreuzberg_parse_highlight_style(eq_symbols.as_ptr()), 0);

            let html = CString::new("html").unwrap();
            assert_eq!(kreuzberg_parse_highlight_style(html.as_ptr()), 1);

            let bold = CString::new("bold").unwrap();
            assert_eq!(kreuzberg_parse_highlight_style(bold.as_ptr()), 2);

            let none = CString::new("none").unwrap();
            assert_eq!(kreuzberg_parse_highlight_style(none.as_ptr()), 3);
        }
    }

    #[test]
    fn test_list_indent_type_parsing() {
        unsafe {
            let spaces = CString::new("spaces").unwrap();
            assert_eq!(kreuzberg_parse_list_indent_type(spaces.as_ptr()), 0);

            let tabs = CString::new("tabs").unwrap();
            assert_eq!(kreuzberg_parse_list_indent_type(tabs.as_ptr()), 1);

            let invalid = CString::new("invalid").unwrap();
            assert_eq!(kreuzberg_parse_list_indent_type(invalid.as_ptr()), -1);
        }
    }

    #[test]
    fn test_whitespace_mode_parsing() {
        unsafe {
            let default = CString::new("default").unwrap();
            assert_eq!(kreuzberg_parse_whitespace_mode(default.as_ptr()), 0);

            let preserve = CString::new("preserve").unwrap();
            assert_eq!(kreuzberg_parse_whitespace_mode(preserve.as_ptr()), 1);

            let preserve_inner = CString::new("preserve_inner").unwrap();
            assert_eq!(kreuzberg_parse_whitespace_mode(preserve_inner.as_ptr()), 2);

            let collapse = CString::new("collapse").unwrap();
            assert_eq!(kreuzberg_parse_whitespace_mode(collapse.as_ptr()), 3);
        }
    }

    #[test]
    fn test_newline_style_parsing() {
        unsafe {
            let default = CString::new("default").unwrap();
            assert_eq!(kreuzberg_parse_newline_style(default.as_ptr()), 0);

            let spaces = CString::new("spaces").unwrap();
            assert_eq!(kreuzberg_parse_newline_style(spaces.as_ptr()), 1);

            let backslash = CString::new("backslash").unwrap();
            assert_eq!(kreuzberg_parse_newline_style(backslash.as_ptr()), 2);
        }
    }

    #[test]
    fn test_preprocessing_preset_parsing() {
        unsafe {
            let none = CString::new("none").unwrap();
            assert_eq!(kreuzberg_parse_preprocessing_preset(none.as_ptr()), 0);

            let conservative = CString::new("conservative").unwrap();
            assert_eq!(kreuzberg_parse_preprocessing_preset(conservative.as_ptr()), 1);

            let aggressive = CString::new("aggressive").unwrap();
            assert_eq!(kreuzberg_parse_preprocessing_preset(aggressive.as_ptr()), 2);
        }
    }
}
