```go title="Go"
package main

import (
	"log"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v5"
)

func main() {
	lang := "en"
	cfg := &kreuzberg.ExtractionConfig{
		OCR: &kreuzberg.OCRConfig{
			Backend:  "paddle-ocr",
			Language: &lang,
			// PaddleOcr: &kreuzberg.PaddleOcrConfig{ModelTier: "server"}, // for max accuracy
		},
	}

	result, err := kreuzberg.ExtractFileSync("scanned.pdf", cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}
	log.Println(len(result.Content))
}
```
