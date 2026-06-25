```go title="Go"
package main

import (
	"fmt"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	preset := "balanced"
	normalize := true
	config := xberg.EmbeddingConfig{
		Model: xberg.EmbeddingModelType{
			Type: "preset",
			Name: &preset,
		},
		Normalize: &normalize,
	}

	// Synchronous
	embeddings, err := xberg.EmbedTexts([]string{"Hello, world!", "Xberg is fast"}, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(len(embeddings))    // 2
	fmt.Println(len(embeddings[0])) // 768

	// Asynchronous
	embeddings, err = xberg.EmbedTextsAsync([]string{"Hello, world!"}, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(len(embeddings[0])) // 768
}
```
