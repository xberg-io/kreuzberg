```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/kreuzberg/packages/go/v5"
)

func main() {
	config, err := kreuzberg.LoadExtractionConfigFromFile("")
	if err != nil {
		log.Fatalf("discover config failed: %v", err)
	}

	result, err := kreuzberg.ExtractFileSync("document.pdf", config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Printf("Content length: %d", len(result.Content))
}
```
