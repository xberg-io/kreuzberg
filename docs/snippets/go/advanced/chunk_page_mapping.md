```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg"
)

func main() {
	maxChars := uint(500)
	overlap := uint(50)
	cfg := xberg.ExtractionConfig{
		Chunking: &xberg.ChunkingConfig{
			MaxCharacters: &maxChars,
			Overlap:       &overlap,
		},
	}

	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatal(err)
	}

	for _, chunk := range result.Results[0].Chunks {
		first := chunk.Metadata.FirstPage
		last := chunk.Metadata.LastPage
		if first == nil {
			continue
		}
		pageRange := fmt.Sprintf("Page %d", *first)
		if last != nil && *first != *last {
			pageRange = fmt.Sprintf("Pages %d-%d", *first, *last)
		}

		preview := chunk.Content
		if len(preview) > 50 {
			preview = preview[:50]
		}
		fmt.Printf("Chunk: %s... (%s)\n", preview, pageRange)
	}
}
```
