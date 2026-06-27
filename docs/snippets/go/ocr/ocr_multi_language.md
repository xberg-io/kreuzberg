```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	config := xberg.ExtractionConfig{
		Ocr: &xberg.OcrConfig{
			Backend:  "tesseract",
			Language: "eng+deu+fra",
		},
	}
	input := xberg.ExtractInputFromURI("multilingual.pdf")
	result, err := xberg.Extract(*input, config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println(result.Results[0].Content)
}
```
