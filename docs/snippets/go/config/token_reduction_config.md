```go title="Go"
package main

import (
	"fmt"

	"github.com/xberg-io/kreuzberg/packages/go/v5"
)

func main() {
	preserveImportant := true
	config := kreuzberg.ExtractionConfig{
		TokenReduction: &kreuzberg.TokenReductionOptions{
			Mode:                   "moderate",
			PreserveImportantWords: &preserveImportant,
		},
	}

	fmt.Printf("Mode: %s, Preserve Important Words: %v\n",
		config.TokenReduction.Mode,
		*config.TokenReduction.PreserveImportantWords)
}
```
