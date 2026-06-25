```go title="Go"
package main

import (
	"fmt"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	preserveImportant := true
	config := xberg.ExtractionConfig{
		TokenReduction: &xberg.TokenReductionOptions{
			Mode:                   "moderate",
			PreserveImportantWords: &preserveImportant,
		},
	}

	fmt.Printf("Mode: %s, Preserve Important Words: %v\n",
		config.TokenReduction.Mode,
		*config.TokenReduction.PreserveImportantWords)
}
```
