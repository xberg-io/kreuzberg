```go title="Go"
package main

import (
	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	enableQualityProcessing := true

	config := &xberg.ExtractionConfig{
		EnableQualityProcessing: &enableQualityProcessing,
	}
	_ = config
}
```
