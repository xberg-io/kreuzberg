---
priority: high
---

- hOCR parsing: extract word-level bounding boxes, confidence scores, and text content
- Preserve spatial relationships from hOCR output for layout reconstruction
- Table detection: use cell boundary detection (line detection + intersection analysis)
- Validate grid structure before treating detected regions as tables
- OCR each cell individually for better accuracy
- Convert tables to markdown format with proper column alignment
