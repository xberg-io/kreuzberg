```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg"
)

func main() {
	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, xberg.ExtractionConfig{})
	if err != nil {
		log.Fatal(err)
	}

	if result.Results[0].Metadata.Pages == nil || result.Results[0].Metadata.Pages.Boundaries == nil {
		return
	}

	contentBytes := []byte(result.Results[0].Content)
	for i, boundary := range result.Results[0].Metadata.Pages.Boundaries {
		if i >= 3 {
			break
		}
		pageText := string(contentBytes[boundary.ByteStart:boundary.ByteEnd])
		preview := pageText
		if len(preview) > 100 {
			preview = preview[:100]
		}

		fmt.Printf("Page %d:\n", boundary.PageNumber)
		fmt.Printf("  Byte range: %d-%d\n", boundary.ByteStart, boundary.ByteEnd)
		fmt.Printf("  Preview: %s...\n", preview)
	}
}
```
