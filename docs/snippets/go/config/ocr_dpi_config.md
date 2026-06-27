```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	targetDpi := int32(300)
	config := xberg.ExtractionConfig{
		Ocr: &xberg.OcrConfig{
			Backend: "tesseract",
			TesseractConfig: &xberg.TesseractConfig{
				Preprocessing: &xberg.ImagePreprocessingConfig{
					TargetDpi: &targetDpi,
				},
			},
		},
	}
	input := xberg.ExtractInputFromURI("scanned.pdf")
	result, err := xberg.Extract(*input, config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Results[0].Content))
}
```
