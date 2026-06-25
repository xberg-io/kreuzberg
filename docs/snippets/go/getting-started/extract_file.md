```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	useCache := true
	enableQP := true

	config := &xberg.ExtractionConfig{
		UseCache:                &useCache,
		EnableQualityProcessing: &enableQP,
	}

	result, err := xberg.ExtractFileSync("contract.pdf", config)
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
