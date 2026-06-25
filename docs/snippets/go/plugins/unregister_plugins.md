```go title="Go"
package main

import (
	"errors"
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	// Unregister a specific document extractor
	if err := xberg.UnregisterDocumentExtractor("custom-json-extractor"); err != nil {
		var validErr *xberg.ValidationError
		if errors.As(err, &validErr) {
			log.Printf("validation error: %v", err)
		} else {
			log.Fatalf("unregister document extractor: %v", err)
		}
	}

	// Unregister a specific post-processor
	if err := xberg.UnregisterPostProcessor("word_count"); err != nil {
		var validErr *xberg.ValidationError
		if errors.As(err, &validErr) {
			log.Printf("validation error: %v", err)
		} else {
			log.Fatalf("unregister post processor: %v", err)
		}
	}

	// Unregister a specific OCR backend
	if err := xberg.UnregisterOCRBackend("cloud-ocr"); err != nil {
		var validErr *xberg.ValidationError
		if errors.As(err, &validErr) {
			log.Printf("validation error: %v", err)
		} else {
			log.Fatalf("unregister OCR backend: %v", err)
		}
	}

	// Unregister a specific validator
	if err := xberg.UnregisterValidator("min_length_validator"); err != nil {
		var validErr *xberg.ValidationError
		if errors.As(err, &validErr) {
			log.Printf("validation error: %v", err)
		} else {
			log.Fatalf("unregister validator: %v", err)
		}
	}

	fmt.Println("Plugins unregistered successfully")
}
```
