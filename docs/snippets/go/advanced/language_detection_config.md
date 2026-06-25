```go title="Go"
package main

import (
	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	enabled := true
	detectMultiple := false
	minConfidence := 0.8

	config := &xberg.ExtractionConfig{
		LanguageDetection: &xberg.LanguageDetectionConfig{
			Enabled:        &enabled,
			MinConfidence:  &minConfidence,
			DetectMultiple: &detectMultiple,
		},
	}
	_ = config
}
```
