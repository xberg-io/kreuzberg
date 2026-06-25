```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	cfg := xberg.ExtractionConfig{
		Ocr: &xberg.OcrConfig{
			Backend:  "paddle-ocr",
			Language: "en",
		},
	}

	result, err := xberg.ExtractFileSync("scanned.pdf", nil, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}
	log.Println(len(result.Content))
}
```
