```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/kreuzberg/packages/go/v5"
)

func main() {
	config, err := kreuzberg.LoadExtractionConfigFromFile("kreuzberg.toml")
	if err != nil {
		log.Fatalf("load config failed: %v", err)
	}

	result, err := kreuzberg.ExtractFileSync("document.pdf", config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Printf("Detected MIME: %s", result.MimeType)
}
```
