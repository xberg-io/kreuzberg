```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v4"
)

func main() {
	useCache := true
	enableQP := true

	config := &kreuzberg.ExtractionConfig{
		UseCache:                &useCache,
		EnableQualityProcessing: &enableQP,
	}

	result, err := kreuzberg.ExtractFileSync("contract.pdf", config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	fmt.Printf("Extracted %d characters\n", len(result.Content))
	if result.QualityScore != nil {
		fmt.Printf("Quality score: %.2f\n", *result.QualityScore)
	}
	fmt.Printf("Processing time: %v\n", result.ProcessingTime)
}
```
