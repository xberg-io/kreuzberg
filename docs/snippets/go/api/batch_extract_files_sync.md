```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/v5"
)

func main() {
	items := []xberg.BatchFileItem{
		{Path: "doc1.pdf"},
		{Path: "doc2.docx"},
		{Path: "doc3.pptx"},
	}

	results, err := xberg.BatchExtractFilesSync(items, xberg.ExtractionConfig{})
	if err != nil {
		log.Fatalf("batch extraction failed: %v", err)
	}

	for i, result := range results {
		println("Doc", i, "content length:", len(result.Content))
	}
}
```
