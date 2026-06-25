```go title="Go"
package main

import (
	"fmt"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	config := &xberg.ExtractionConfig{
		Keywords: &xberg.KeywordConfig{
			Algorithm:  "YAKE",
			MaxKeywords: 10,
			MinScore:   0.3,
			NgramRange: "1,3",
			Language:   "en",
		},
	}

	fmt.Printf("Keywords config: Algorithm=%s, MaxKeywords=%d, MinScore=%f\n",
		config.Keywords.Algorithm,
		config.Keywords.MaxKeywords,
		config.Keywords.MinScore)
}
```
