```go title="Go"
package main

import (
	"log"
	"os"

	"github.com/xberg-io/xberg/v5"
)

func main() {
	doc1, _ := os.ReadFile("doc1.pdf")
	doc2, _ := os.ReadFile("doc2.docx")

	items := []xberg.BatchBytesItem{
		{Content: doc1, MimeType: "application/pdf"},
		{Content: doc2, MimeType: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"},
	}

	results, err := xberg.BatchExtractBytesSync(items, xberg.ExtractionConfig{})
	if err != nil {
		log.Fatalf("batch extraction failed: %v", err)
	}

	println("Processed", len(results), "documents")
}
```
