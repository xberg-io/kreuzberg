```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	config, err := xberg.LoadExtractionConfigFromFile("")
	if err != nil {
		log.Fatalf("discover config failed: %v", err)
	}

	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, *config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Printf("Content length: %d", len(result.Results[0].Content))
}
```
