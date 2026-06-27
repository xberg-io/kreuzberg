---
description: OCR backend integration and image processing
---

- Multiple backends: Tesseract (C FFI via leptonica/tesseract-sys), PaddleOCR (ONNX Runtime), Candle OCR, VLM OCR, and custom plugin OCR backends
- Backend selection: priority-based with fallback — Tesseract default, PaddleOCR for CJK, plugin/VLM backends as configured
- Image preprocessing: deskew, binarization, noise removal, contrast enhancement — applied before OCR
- PSM modes: configurable page segmentation (single block, single line, sparse text) per use case
- Table detection: identify table regions → cell extraction → row/column reconstruction → Markdown table output
- hOCR: parse Tesseract hOCR output for word-level bounding boxes, confidence scores, reading order
- Language management: auto-detect document language, load appropriate Tesseract traineddata, support multi-language documents
- Caching: cache OCR results by image hash + backend + language + PSM mode
- Confidence tracking: per-word and per-page confidence scores, flag low-confidence regions for review
