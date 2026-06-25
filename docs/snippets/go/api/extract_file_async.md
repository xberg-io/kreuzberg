```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/v5"
)

func main() {
	result, err := xberg.ExtractFile("document.pdf", nil, xberg.ExtractionConfig{})
	if err != nil {
		log.Fatalf("extraction failed: %v", err)
	}

	println("Content:", result.Content)
	println("MIME type:", result.MimeType)
}
```
