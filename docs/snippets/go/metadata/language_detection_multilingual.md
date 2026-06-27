```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	minConfidence := 0.8
	cfg := xberg.ExtractionConfig{
		LanguageDetection: &xberg.LanguageDetectionConfig{
			Enabled:        true,
			MinConfidence:  &minConfidence,
			DetectMultiple: true,
		},
	}

	input := xberg.ExtractInputFromURI("multilingual_document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	fmt.Printf("Detected languages: %v\n", result.Results[0].DetectedLanguages)
	// Output: [eng fra deu]
}
```
