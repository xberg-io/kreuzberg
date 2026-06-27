```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	preserve := true
	cfg := xberg.ExtractionConfig{
		TokenReduction: &xberg.TokenReductionConfig{
			Mode:                  "moderate",
			PreserveImportantWords: &preserve,
		},
	}
	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Results[0].Content))
}
```
