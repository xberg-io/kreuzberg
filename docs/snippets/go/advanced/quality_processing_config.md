```go title="Go"
package main

import (
	"github.com/xberg-io/kreuzberg/packages/go/v5"
)

func main() {
	enableQualityProcessing := true

	config := &kreuzberg.ExtractionConfig{
		EnableQualityProcessing: &enableQualityProcessing,
	}
	_ = config
}
```
