```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, xberg.ExtractionConfig{})
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	content := result.Results[0].Content
	tables := result.Results[0].Tables
	images := result.Results[0].Images
	metadata := result.Results[0].Metadata

	fmt.Printf("Content: %d characters\n", len(content))
	fmt.Printf("Tables: %d\n", len(tables))
	fmt.Printf("Images: %d\n", len(images))

	if metadata != nil {
		fmt.Print("Metadata keys: ")
		for key := range metadata {
			fmt.Print(key + " ")
		}
		fmt.Println()
	}
}
```
