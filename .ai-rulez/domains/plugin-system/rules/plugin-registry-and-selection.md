---
priority: critical
---

- Separate typed registry per plugin type (ExtractorRegistry, OcrRegistry, etc.)
- Thread safety: Arc<RwLock<>> for all registries
- Priority system: 0-255, default 50, custom > 50, fallback < 50
- Selection: highest priority plugin matching the MIME type wins
- MIME type indexing for O(log n) lookup
- Conflict resolution: if equal priority, prefer Rust-native over FFI plugins
- Dynamic registration: plugins can be added/removed at runtime
- Validate plugin before registration (check trait compliance, supported formats)
