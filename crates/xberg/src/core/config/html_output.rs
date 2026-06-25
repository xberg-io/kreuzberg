//! HTML output configuration.
//!
//! Controls how `OutputFormat::Html` renders an `InternalDocument`:
//! which built-in theme to use, whether to embed the CSS in a `<style>`
//! block, and optional user-supplied CSS (inline string or file path).

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

fn default_class_prefix() -> String {
    "kb-".to_string()
}

fn default_true() -> bool {
    true
}

/// Configuration for styled HTML output.
///
/// When set on [`crate::core::config::ExtractionConfig::html_output`] alongside
/// `output_format = OutputFormat::Html`, the pipeline builds a
/// [`StyledHtmlRenderer`](crate::rendering::StyledHtmlRenderer) instead of
/// the plain comrak-based renderer.
///
/// # Example
///
/// ```rust
/// use xberg::core::config::{HtmlOutputConfig, HtmlTheme};
///
/// let config = HtmlOutputConfig {
///     theme: HtmlTheme::GitHub,
///     css: Some(".kb-p { font-size: 1.1rem; }".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlOutputConfig {
    /// Inline CSS string injected into the output after the theme stylesheet.
    /// Concatenated after `css_file` content when both are set.
    #[serde(default)]
    pub css: Option<String>,

    /// Path to a CSS file loaded once at renderer construction time.
    /// Concatenated before `css` when both are set.
    #[serde(default)]
    pub css_file: Option<PathBuf>,

    /// Built-in colour/typography theme. Default: [`HtmlTheme::Unstyled`].
    #[serde(default)]
    pub theme: HtmlTheme,

    /// CSS class prefix applied to every emitted class name.
    ///
    /// Default: `"kb-"`. Change this if your host application already uses
    /// classes that start with `kb-`.
    #[serde(default = "default_class_prefix")]
    pub class_prefix: String,

    /// When `true` (default), write the resolved CSS into a `<style>` block
    /// immediately after the opening `<div class="{prefix}doc">`.
    ///
    /// Set to `false` to emit only the structural markup and wire up your
    /// own stylesheet targeting the `kb-*` class names.
    #[serde(default = "default_true")]
    pub embed_css: bool,
}

impl Default for HtmlOutputConfig {
    fn default() -> Self {
        Self {
            css: None,
            css_file: None,
            theme: HtmlTheme::Unstyled,
            class_prefix: default_class_prefix(),
            embed_css: true,
        }
    }
}

/// Built-in HTML theme selection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HtmlTheme {
    /// Sensible defaults: system font stack, neutral colours, readable line
    /// measure. CSS custom properties (`--kb-*`) are all defined so user CSS
    /// can override individual values.
    Default,
    /// GitHub Markdown-inspired palette and spacing.
    GitHub,
    /// Dark background, light text.
    Dark,
    /// Minimal light theme with generous whitespace.
    Light,
    /// No built-in stylesheet emitted. CSS custom properties are still defined
    /// on `:root` so user stylesheets can reference `var(--kb-*)` tokens.
    #[default]
    Unstyled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values() {
        let cfg = HtmlOutputConfig::default();
        assert_eq!(cfg.class_prefix, "kb-");
        assert!(cfg.embed_css);
        assert!(cfg.css.is_none());
        assert!(cfg.css_file.is_none());
        assert_eq!(cfg.theme, HtmlTheme::Unstyled);
    }

    #[test]
    fn serde_roundtrip() {
        let cfg = HtmlOutputConfig {
            css: Some(".kb-p { color: red; }".to_string()),
            theme: HtmlTheme::GitHub,
            embed_css: false,
            ..Default::default()
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: HtmlOutputConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.css, cfg.css);
        assert_eq!(back.theme, HtmlTheme::GitHub);
        assert!(!back.embed_css);
    }

    #[test]
    fn theme_serde() {
        assert_eq!(serde_json::to_string(&HtmlTheme::GitHub).unwrap(), "\"github\"");
        let t: HtmlTheme = serde_json::from_str("\"dark\"").unwrap();
        assert_eq!(t, HtmlTheme::Dark);
    }
}
