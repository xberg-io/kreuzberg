```go title="Go"
package main

import (
	"fmt"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	maxChars := 1024
	maxOverlap := 100
	batchSize := int32(32)
	config := &xberg.ExtractionConfig{
		Chunking: &xberg.ChunkingConfig{
			MaxChars:   &maxChars,
			MaxOverlap: &maxOverlap,
			Embedding: &xberg.EmbeddingConfig{
				Model:                   "balanced",
				Normalize:               true,
				BatchSize:               &batchSize,
				ShowDownloadProgress:    false,
			},
		},
	}

	fmt.Printf("Config: MaxChars=%d, MaxOverlap=%d, Model=%s\n",
		*config.Chunking.MaxChars,
		*config.Chunking.MaxOverlap,
		config.Chunking.Embedding.Model)
}
```
