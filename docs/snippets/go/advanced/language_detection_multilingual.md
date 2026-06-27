```go title="Go"
package main

import (
	"fmt"
	"log"
	"strings"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	enabled := true
	detectMultiple := true
	minConfidence := 0.8

	cfg := xberg.ExtractionConfig{
		LanguageDetection: &xberg.LanguageDetectionConfig{
			Enabled:        &enabled,
			MinConfidence:  &minConfidence,
			DetectMultiple: &detectMultiple,
		},
	}

	input := xberg.ExtractInputFromURI("multilingual_document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("Processing failed: %v", err)
	}

	languages := result.Results[0].DetectedLanguages
	if len(languages) > 0 {
		fmt.Printf("Detected %d language(s): %s\n", len(languages), strings.Join(languages, ", "))
	} else {
		fmt.Println("No languages detected")
	}

	fmt.Printf("Total content: %d characters\n", len(result.Results[0].Content))
	fmt.Printf("MIME type: %s\n", result.Results[0].MimeType)
}
```
