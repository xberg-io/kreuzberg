---
priority: critical
---

- All plugins must implement the base Plugin trait: Send + Sync + 'static required
- Plugin types: DocumentExtractor, OcrBackend, PostProcessor, Validator
- Async execution: use async trait methods for non-blocking operations
- Lifecycle: init() -> process() -> cleanup(). Init must validate all requirements.
- Never panic in plugin code — all errors must be returned as Result
- Consistent result format: all extractors return ExtractionResult with text, metadata, and confidence
