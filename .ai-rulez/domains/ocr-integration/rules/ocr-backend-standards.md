---
priority: critical
---

- Pluggable backend architecture: all backends implement the OcrBackend trait
- Backend independence: switching backends must not require API changes
- Tesseract is the default backend (native C FFI via leptess)
- Blocking OCR backends: use tokio::task::spawn_blocking and keep runtime/FFI locks held only around the blocking call
- Graceful degradation: if preferred backend unavailable, fall back to next available
- All backends must return structured results with confidence scores
- Document installation requirements and troubleshooting for each backend
