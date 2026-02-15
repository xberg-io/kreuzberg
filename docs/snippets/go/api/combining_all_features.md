```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v4"
)

func main() {
	maxChars := 512
	maxOverlap := 50
	minConfidence := 0.8
	config := &kreuzberg.ExtractionConfig{
		EnableQualityProcessing: true,

		LanguageDetection: &kreuzberg.LanguageDetectionConfig{
			Enabled:        true,
			MinConfidence:  &minConfidence,
			DetectMultiple: true,
		},

		TokenReduction: &kreuzberg.TokenReductionConfig{
			Mode:             "moderate",
			PreserveMarkdown: true,
		},

		Chunking: &kreuzberg.ChunkingConfig{
			MaxChars:   &maxChars,
			MaxOverlap: &maxOverlap,
			Embedding: &kreuzberg.EmbeddingConfig{
				Model:     "balanced",
				Normalize: true,
			},
		},

		Keywords: &kreuzberg.KeywordConfig{
			Algorithm:   "YAKE",
			MaxKeywords: 10,
		},
	}

	result, err := kreuzberg.ExtractFileSync("document.pdf", config)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	if result.QualityScore != nil {
		fmt.Printf("Quality: %.2f\n", *result.QualityScore)
	}
	fmt.Printf("Languages: %v\n", result.DetectedLanguages)
	fmt.Printf("Keywords: %v\n", result.ExtractedKeywords)
	if result.Chunks != nil && len(result.Chunks) > 0 && result.Chunks[0].Embedding != nil {
		fmt.Printf("Chunks: %d with %d dimensions\n", len(result.Chunks), len(result.Chunks[0].Embedding))
	}
}
```
