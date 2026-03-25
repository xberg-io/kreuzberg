---
description: "plugin architecture patterns"
priority: critical
---

## priority: critical

# Plugin Architecture & Registration

## Plugin Types

| Type | Trait | Location |
|------|-------|----------|
| Document Extractor | `DocumentExtractor: Plugin` | `plugins/extractor/trait.rs` |
| OCR Backend | `OcrBackend: Plugin` | `plugins/ocr/trait.rs` |
| Post Processor | `PostProcessor: Plugin` | `plugins/processor/trait.rs` |
| Validator | `Validator: Plugin` | `plugins/validator/trait.rs` |

## DocumentExtractor Implementation

```rust
use crate::plugins::{DocumentExtractor, Plugin};
use async_trait::async_trait;

pub struct MyExtractor;

impl Plugin for MyExtractor {
    fn name(&self) -> &str { "my-extractor" }
    fn version(&self) -> String { env!("CARGO_PKG_VERSION").to_string() }
}

#[async_trait]
impl DocumentExtractor for MyExtractor {
    async fn extract_bytes(&self, content: &[u8], mime_type: &str, config: &ExtractionConfig)
        -> Result<ExtractionResult> { /* ... */ }

    fn supported_mime_types(&self) -> &[&str] { &["application/x-custom"] }
    fn priority(&self) -> i32 { 50 }

    // WASM support (optional)
    fn as_sync_extractor(&self) -> Option<&dyn SyncExtractor> { None }
}
```

## Priority System

| Range | Use |
|-------|-----|
| 0-25 | Fallback/low-quality |
| 26-49 | Alternative extractors |
| **50** | **Default (built-in)** |
| 51-75 | Premium/enhanced |
| 76-100 | Specialized/high-priority |

Registry selects **highest priority** extractor for each MIME type. Override built-ins with priority > 50.

## Registration

```rust
// In extractors/mod.rs → register_default_extractors()
let registry = get_document_extractor_registry();
let mut registry = registry.write()
    .map_err(|e| KreuzbergError::Other(format!("Registry lock poisoned: {}", e)))?;
registry.register(Arc::new(MyExtractor::new()))?;
```

## Feature-Gated Registration

```rust
#[cfg(feature = "office")]
{
    registry.register(Arc::new(DocxExtractor::new()))?;
    registry.register(Arc::new(PptxExtractor::new()))?;
}
```

## PostProcessor Pattern

```rust
impl PostProcessor for MyProcessor {
    async fn process(&self, result: &mut ExtractionResult, config: &ExtractionConfig)
        -> Result<()> {
        result.content = process_content(&result.content);
        Ok(())
    }
    fn stage(&self) -> ProcessorStage { ProcessorStage::Middle }
}
```

Stages: `Early` → `Middle` → `Late`. Failures isolated (don't block others).

## Critical Rules

1. All plugins **MUST be `Send + Sync`**
2. Feature gate with `#[cfg(feature = "...")]` for optional formats
3. Use `#[async_trait]` for `DocumentExtractor`
4. Initialization via `ensure_initialized()` (lazy, called before first extraction)
5. Plugin names: kebab-case (e.g., `"pdf-extractor"`)