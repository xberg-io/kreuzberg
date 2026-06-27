```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	cfg := xberg.ExtractionConfig{
		Ocr: &xberg.OcrConfig{
			Backend:  "paddle-ocr",
			Language: "en",
		},
	}

	input := xberg.ExtractInputFromURI("scanned.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	for _, element := range result.Results[0].OcrElements {
		fmt.Printf("Text: %s\n", element.Text)
		fmt.Printf("Confidence: %.2f\n", element.Confidence.Recognition)
		fmt.Printf("Geometry: %+v\n", element.Geometry)
		if element.Rotation != nil {
			fmt.Printf("Rotation: %.1f°\n", element.Rotation.AngleDegrees)
		}
		fmt.Println()
	}
}
```
