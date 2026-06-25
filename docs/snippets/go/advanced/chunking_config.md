```go title="Go"
package main

import (
	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	maxChars := 1000
	maxOverlap := 200
	normalize := true
	batchSize := int32(32)

	config := &xberg.ExtractionConfig{
		Chunking: &xberg.ChunkingConfig{
			MaxChars:   &maxChars,
			MaxOverlap: &maxOverlap,
			Embedding: &xberg.EmbeddingConfig{
				Model:     xberg.EmbeddingModelType_Preset("all-minilm-l6-v2"),
				Normalize: &normalize,
				BatchSize: &batchSize,
			},
		},
	}
	_ = config
}
```
