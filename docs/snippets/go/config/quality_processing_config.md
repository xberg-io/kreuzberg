```go title="Go"
package main

import (
	"fmt"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v5"
)

func main() {
	config := &kreuzberg.ExtractionConfig{
		EnableQualityProcessing: true,  // Default
	}

	fmt.Printf("Quality processing enabled: %v\n", config.EnableQualityProcessing)
}
```
