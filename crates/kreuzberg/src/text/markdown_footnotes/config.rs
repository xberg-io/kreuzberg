//! Configuration for markdown footnote parsing.

use serde::{Deserialize, Serialize};

/// Configuration for markdown footnote and citation parsing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct FootnoteConfig {
    /// Whether to parse the structured citation block (default: true).
    ///
    /// When enabled, the parser will look for and extract citations from
    /// the block after `---` + `<!-- citations ... -->`.
    #[serde(default = "default_parse_citations")]
    pub parse_citations: bool,
}

fn default_parse_citations() -> bool {
    true
}

impl Default for FootnoteConfig {
    fn default() -> Self {
        Self { parse_citations: true }
    }
}

impl FootnoteConfig {
    /// Create a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to parse the citation block.
    pub fn with_parse_citations(mut self, enabled: bool) -> Self {
        self.parse_citations = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FootnoteConfig::default();
        assert!(config.parse_citations);
    }

    #[test]
    fn test_builder_parse_citations() {
        let config = FootnoteConfig::new().with_parse_citations(false);
        assert!(!config.parse_citations);
    }

    #[test]
    fn test_serde_roundtrip() {
        let config = FootnoteConfig { parse_citations: false };
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: FootnoteConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.parse_citations, config.parse_citations);
    }
}
