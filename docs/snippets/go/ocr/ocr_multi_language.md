```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	result, err := xberg.ExtractFileSync("multilingual.pdf", nil, xberg.ExtractionConfig{
		Ocr: &xberg.OcrConfig{
			Backend:  "tesseract",
			Language: "eng+deu+fra",
		},
	})
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println(result.Content)
}
```
