```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	config, err := xberg.LoadExtractionConfigFromFile("xberg.toml")
	if err != nil {
		log.Fatalf("load config failed: %v", err)
	}

	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, *config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Printf("Detected MIME: %s", result.Results[0].MimeType)
}
```
