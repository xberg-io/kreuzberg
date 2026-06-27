```go title="Go"
package main

import (
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	extractMetadata := true
	config := xberg.ExtractionConfig{
		PdfOptions: &xberg.PdfConfig{
			ExtractImages:   true,
			ExtractMetadata: &extractMetadata,
			Passwords:       []string{"password1", "password2"},
			Hierarchy:       &xberg.HierarchyConfig{},
		},
	}
	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Results[0].Content))
}
```
