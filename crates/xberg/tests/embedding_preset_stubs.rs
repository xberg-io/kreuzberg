// Tests that the no-feature stubs compile and return empty/None.
// Only active when `embedding-presets` is absent; run with:
//   cargo test -p xberg --test embedding_preset_stubs --no-default-features

#[cfg(not(feature = "embedding-presets"))]
mod tests {
    #[test]
    fn get_returns_none_without_feature() {
        assert!(xberg::get_embedding_preset("all-minilm-l6-v2").is_none());
    }

    #[test]
    fn list_returns_empty_without_feature() {
        assert!(xberg::list_embedding_presets().is_empty());
    }
}
