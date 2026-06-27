```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg"
)

func main() {
	trueVal := true
	maxChars := uint(1000)
	overlap := uint(200)
	config := xberg.ExtractionConfig{
		UseCache:                &trueVal,
		EnableQualityProcessing: &trueVal,
		Ocr: &xberg.OcrConfig{
			Backend:   "tesseract",
			Language:  "eng",
		},
		Chunking: &xberg.ChunkingConfig{
			MaxCharacters: &maxChars,
			Overlap:       &overlap,
		},
	}

	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, config)
	if err != nil {
		log.Fatalf("extraction failed: %v", err)
	}

	println("Content length:", len(result.Results[0].Content))
	println("Chunks:", len(result.Results[0].Chunks))
}
```
