```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	preserve := true
	result, err := xberg.ExtractFileSync("document.pdf", &xberg.ExtractionConfig{
		TokenReduction: &xberg.TokenReductionConfig{
			Mode:                  "moderate",
			PreserveImportantWords: &preserve,
		},
	})
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Content))
}
```
