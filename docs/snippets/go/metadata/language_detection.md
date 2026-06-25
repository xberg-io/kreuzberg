```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	enabled := true
	minConfidence := 0.9
	result, err := xberg.ExtractFileSync("document.pdf", nil, xberg.ExtractionConfig{
		LanguageDetection: &xberg.LanguageDetectionConfig{
			Enabled:        &enabled,
			MinConfidence:  &minConfidence,
			DetectMultiple: true,
		},
	})
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Content))
}
```
