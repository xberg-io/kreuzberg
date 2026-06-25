```go title="Go"
package main

import "github.com/xberg-io/xberg/packages/go/v5"

func main() {
	psm := int32(3)

	_ = xberg.ExtractionConfig{
		Ocr: &xberg.OcrConfig{
			Backend:  "tesseract",
			Language: "eng+fra",
			TesseractConfig: &xberg.TesseractConfig{
				Psm: &psm,
			},
		},
	}
}
```
