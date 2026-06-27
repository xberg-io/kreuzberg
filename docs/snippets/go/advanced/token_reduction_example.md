```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	preserveMarkdown := true
	mode := "moderate"

	cfg := xberg.ExtractionConfig{
		TokenReduction: &xberg.TokenReductionConfig{
			Mode:             &mode,
			PreserveMarkdown: &preserveMarkdown,
		},
	}

	input := xberg.ExtractInputFromURI("verbose_document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extraction failed: %v", err)
	}

	original := 0
	reduced := 0
	ratio := 0.0

	if val, ok := result.Results[0].Metadata["original_token_count"]; ok {
		original = val.(int)
	}

	if val, ok := result.Results[0].Metadata["token_count"]; ok {
		reduced = val.(int)
	}

	if val, ok := result.Results[0].Metadata["token_reduction_ratio"]; ok {
		ratio = val.(float64)
	}

	fmt.Printf("Reduced from %d to %d tokens\n", original, reduced)
	fmt.Printf("Reduction: %.1f%%\n", ratio*100)
}
```
