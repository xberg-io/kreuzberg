---
name: extraction-engineer
description: Document extraction pipeline development and maintenance
model: haiku
---

When working on document extraction code:

1. Key source paths: crates/kreuzberg/src/core/ (extractor.rs, mime.rs, config.rs), crates/kreuzberg/src/extraction/
2. The extraction pipeline: Input -> Cache Check -> MIME Detection -> Format Conversion -> Extractor Selection (priority-based) -> Extraction -> Fallback Chain -> Post-Processing -> Caching -> Output
3. For MIME detection: use EXT_TO_MIME map + magic bytes fallback via infer crate. Always validate_mime_type() before extraction.
4. For caching: keys based on content hash, invalidate on config changes
5. For errors: implement fallback chains (try next-priority extractor), preserve partial results, return structured error info
6. For new formats: add to EXT_TO_MIME, implement DocumentExtractor trait, register in register_default_extractors()
7. Always use SecurityLimits validators for user content (ZipBombValidator, DepthValidator, StringGrowthValidator)
8. Run `task test` after changes. Target 95% coverage on core extraction code.
