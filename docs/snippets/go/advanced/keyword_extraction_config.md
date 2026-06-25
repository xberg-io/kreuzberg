```go title="Go"
package main

import (
	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	maxKeywords := int32(10)
	minScore := 0.3
	language := "en"

	config := &xberg.ExtractionConfig{
		Keywords: &xberg.KeywordConfig{
			Algorithm:   xberg.KeywordAlgorithm_YAKE,
			MaxKeywords: &maxKeywords,
			MinScore:    &minScore,
			Language:    &language,
		},
	}
	_ = config
}
```
