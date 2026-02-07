```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v4"
)

func main() {
	lang := "en"
	cfg := &kreuzberg.ExtractionConfig{
		OCR: &kreuzberg.OCRConfig{
			Backend:  "paddle-ocr",
			Language: &lang,
		},
	}

	result, err := kreuzberg.ExtractFileSync("scanned.pdf", cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	for _, element := range result.OcrElements {
		fmt.Printf("Text: %s\n", element.Text)
		fmt.Printf("Confidence: %.2f\n", element.Confidence.Recognition)
		fmt.Printf("Geometry: %+v\n", element.Geometry)
		if element.Rotation != nil {
			fmt.Printf("Rotation: %.1f°\n", element.Rotation.Angle)
		}
		fmt.Println()
	}
}
```
