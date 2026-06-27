```go title="disk_cache.go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	useCache := true
	namespace := "documents"
	ttl := uint64(7 * 86400)

	config := xberg.ExtractionConfig{
		UseCache:       &useCache,
		CacheNamespace: &namespace,
		CacheTTLSecs:   &ttl,
	}

	fmt.Println("First extraction (will be cached)...")
	input1 := xberg.ExtractInputFromURI("document.pdf")
	result1, err := xberg.Extract(*input1, config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}
	if len(result1.Results) > 0 {
		fmt.Printf("  - Content length: %d\n", len(result1.Results[0].Content))
	}

	fmt.Println("\nSecond extraction (from cache)...")
	input2 := xberg.ExtractInputFromURI("document.pdf")
	result2, err := xberg.Extract(*input2, config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}
	if len(result2.Results) > 0 {
		fmt.Printf("  - Content length: %d\n", len(result2.Results[0].Content))
	}

	if len(result1.Results) > 0 && len(result2.Results) > 0 {
		fmt.Printf("\nResults are identical: %v\n", result1.Results[0].Content == result2.Results[0].Content)
	}
}
```
