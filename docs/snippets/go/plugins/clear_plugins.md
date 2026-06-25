```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	// Clear all plugins of a specific type
	if err := xberg.ClearPostProcessors(); err != nil {
		log.Fatalf("clear post processors: %v", err)
	}
	log.Println("Post processors cleared")

	if err := xberg.ClearValidators(); err != nil {
		log.Fatalf("clear validators: %v", err)
	}
	log.Println("Validators cleared")

	if err := xberg.ClearOCRBackends(); err != nil {
		log.Fatalf("clear OCR backends: %v", err)
	}
	log.Println("OCR backends cleared")

	if err := xberg.ClearDocumentExtractors(); err != nil {
		log.Fatalf("clear document extractors: %v", err)
	}
	log.Println("Document extractors cleared")
}
```
