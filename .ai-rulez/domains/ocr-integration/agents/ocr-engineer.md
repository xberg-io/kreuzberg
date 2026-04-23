---
name: ocr-engineer
description: OCR pipeline development, backend integration, and table reconstruction
model: haiku
---

When working on OCR code:

1. Key source paths: crates/kreuzberg/src/ocr/ (processor.rs, tesseract_backend.rs, hocr.rs, cache.rs, language_registry.rs, table/)
2. The OCR pipeline: Image Detection -> Preprocessing (denoise, deskew, binarize) -> Backend Selection -> OCR Execution -> hOCR Parsing -> Table Reconstruction -> Caching -> Return
3. Backends: Tesseract (default, native C FFI via leptess), PaddleOCR (ONNX via ort), EasyOCR (Python via PyO3)
4. For Python backends: use tokio::task::spawn_blocking, minimize GIL hold time with py.allow_threads(), cache Python data in Rust fields
5. For table detection: detect via line/cell boundary detection, validate grid structure, OCR each cell, output as markdown
6. For language management: validate against LanguageRegistry, check tessdata availability
7. Cache OCR results with key = hash(image_bytes + language + config)
8. hOCR parsing: use the hocr module to extract word-level bounding boxes and confidence scores
