```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	fmt.Println("Xberg CGO bindings loaded successfully")

	input := xberg.ExtractInputFromURI("sample.pdf")
	result, err := xberg.Extract(*input, xberg.ExtractionConfig{})
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	fmt.Println("Installation verified!")
	fmt.Printf("Extracted %d characters\n", len(result.Results[0].Content))
}
```
