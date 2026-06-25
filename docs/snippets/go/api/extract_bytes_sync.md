```go title="Go"
package main

import (
	"log"
	"os"

	"github.com/xberg-io/xberg/v5"
)

func main() {
	content, err := os.ReadFile("document.pdf")
	if err != nil {
		log.Fatalf("failed to read file: %v", err)
	}

	result, err := xberg.ExtractBytesSync(content, "application/pdf", xberg.ExtractionConfig{})
	if err != nil {
		log.Fatalf("extraction failed: %v", err)
	}

	println("Content:", result.Content)
}
```
