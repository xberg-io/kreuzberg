```go title="Go"
package main

import (
	"github.com/kreuzberg-dev/kreuzberg/packages/go/v5"
)

enableQualityProcessing := true

config := &kreuzberg.ExtractionConfig{
	EnableQualityProcessing: &enableQualityProcessing,
}
```
