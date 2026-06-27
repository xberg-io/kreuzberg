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

	// Iterate over tables
	for _, table := range result.Results[0].Tables {
		fmt.Printf("Table with %d rows\n", len(table.Cells))
		fmt.Println(table.Markdown) // Markdown representation

		// Access cells
		for _, row := range table.Cells {
			fmt.Println(row)
		}
	}
}
```
