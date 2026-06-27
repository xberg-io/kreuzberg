```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	cfg := xberg.ExtractionConfig{
		TokenReduction: &xberg.TokenReductionConfig{
			Mode:             "moderate",
			PreserveMarkdown: true,
		},
	}

	input := xberg.ExtractInputFromURI("verbose_document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	fmt.Printf("Original tokens: %v\n", result.Results[0].Metadata.Additional["original_token_count"])
	fmt.Printf("Reduced tokens: %v\n", result.Results[0].Metadata.Additional["token_count"])
	fmt.Printf("Reduction ratio: %v\n", result.Results[0].Metadata.Additional["token_reduction_ratio"])
}
```
