---
priority: high
---

- Track confidence scores on all OCR results — expose in API
- Image preprocessing (denoise, deskew, binarize) should improve accuracy by 10-30%
- PSM mode selection: auto-detect layout, allow user override (single block, single line, sparse text, etc.)
- Language detection: validate requested languages are available, provide install hints if not
- Multi-language support: allow multiple languages per OCR request
- Test OCR accuracy against ground-truth documents in CI
