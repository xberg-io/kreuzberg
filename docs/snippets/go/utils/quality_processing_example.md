```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	cfg := xberg.ExtractionConfig{
		EnableQualityProcessing: true,
	}

	input := xberg.ExtractInputFromURI("scanned_document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	qualityScore := 0.0
	if result.Results[0].QualityScore != nil {
		qualityScore = *result.Results[0].QualityScore
	}

	if qualityScore < 0.5 {
		fmt.Printf("Warning: Low quality extraction (%.2f)\n", qualityScore)
		fmt.Println("Consider re-scanning with higher DPI or adjusting OCR settings")
	} else {
		fmt.Printf("Quality score: %.2f\n", qualityScore)
	}
}
```
