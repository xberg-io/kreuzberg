---
priority: high
---

- Cache keys: content-hash based (hash of file bytes + config), not path-based
- Invalidate cache when extraction config changes (output format, OCR settings, etc.)
- Check cache before any extraction — cache hits should skip all processing
- Concurrent batch processing: use configurable worker pool, default to CPU count
- Stream large files instead of loading into memory — use AsyncRead where possible
- Monitor cache hit rates — target >80% for repeated extractions
