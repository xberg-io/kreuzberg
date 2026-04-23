---
name: plugin-engineer
description: Plugin system architecture, registry management, and Python FFI
model: haiku
---

When working on the plugin system:

1. Key source paths: crates/kreuzberg/src/plugins/ (mod.rs, extractor.rs, ocr.rs, postprocessor.rs, validator.rs, registry.rs), crates/kreuzberg-py/src/plugins.rs
2. Plugin types: DocumentExtractor, OcrBackend, PostProcessor, Validator — all extend base Plugin trait (Send + Sync required)
3. Priority system: 0-255, default 50, custom override > 50, fallback < 50. Registry selects highest priority for MIME type.
4. Registries use Arc<RwLock<>> with MIME type indexing for O(log n) lookup
5. Python plugins: validate protocol compliance, use py.allow_threads() for expensive Rust ops, tokio::task::spawn_blocking for async calls
6. For new plugin types: define trait extending Plugin, create typed registry, add registration functions, implement priority-based selection
7. GIL optimization: cache frequently-accessed Python data in Rust fields, measure GIL overhead
8. All plugins must handle errors gracefully — return Result, never panic
