```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	enabled := true
	minConfidence := 0.9
	config := xberg.ExtractionConfig{
		LanguageDetection: &xberg.LanguageDetectionConfig{
			Enabled:        &enabled,
			MinConfidence:  &minConfidence,
			DetectMultiple: true,
		},
	}
	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Results[0].Content))
}
```
