package main

import (
	"xberg"
)

func main() {
	// Example 1: Preset model (recommended)
	// Fast, balanced, or quality preset configurations optimized for common use cases.
	embeddingConfig := xberg.EmbeddingConfig{
		Model: xberg.EmbeddingModelType{
			Type: "preset",
			Name: "balanced",
		},
		BatchSize:            32,
		Normalize:            true,
		ShowDownloadProgress: true,
		CacheDir:             "~/.cache/xberg/embeddings",
	}

	// Available presets:
	// - "fast" (384 dims): Quick prototyping, development, resource-constrained
	// - "balanced" (768 dims): Production, general-purpose RAG, English documents
	// - "quality" (1024 dims): Complex documents, maximum accuracy
	// - "multilingual" (768 dims): International documents, 100+ languages

	// Example 2: Custom ONNX model (requires embeddings feature)
	// Direct access to specific ONNX embedding models from HuggingFace with custom dimensions.
	embeddingConfig = xberg.EmbeddingConfig{
		Model: xberg.EmbeddingModelType{
			Type:       "custom",
			ModelID:    "BAAI/bge-small-en-v1.5",
			Dimensions: 384,
		},
		BatchSize:            32,
		Normalize:            true,
		ShowDownloadProgress: true,
		CacheDir:             "",  // Uses default: .xberg/embeddings/
	}

	// Popular ONNX-compatible models:
	// - "BAAI/bge-small-en-v1.5" (384 dims): Fast, efficient
	// - "BAAI/bge-base-en-v1.5" (768 dims): Balanced quality/speed
	// - "BAAI/bge-large-en-v1.5" (1024 dims): High quality, slower
	// - "sentence-transformers/paraphrase-multilingual-mpnet-base-v2" (768 dims): Multilingual support

	// Example 3: Alternative Custom ONNX Model
	// For advanced users wanting different ONNX embedding models.
	embeddingConfig = xberg.EmbeddingConfig{
		Model: xberg.EmbeddingModelType{
			Type:       "custom",
			ModelID:    "sentence-transformers/all-mpnet-base-v2",
			Dimensions: 768,
		},
		BatchSize:            16,  // Larger model requires smaller batch size
		Normalize:            true,
		ShowDownloadProgress: true,
		CacheDir:             "/var/cache/embeddings",
	}

	// Integration with ChunkingConfig
	// Add embeddings to your chunking configuration:
	chunkingConfig := xberg.ChunkingConfig{
		MaxChars:  1024,
		MaxOverlap: 100,
		Preset:     "balanced",
		Embedding: &xberg.EmbeddingConfig{
			Model: xberg.EmbeddingModelType{
				Type: "preset",
				Name: "balanced",
			},
			BatchSize: 32,
			Normalize: true,
		},
	}

	extractionConfig := xberg.ExtractionConfig{
		Chunking: &chunkingConfig,
	}

	_ = embeddingConfig
	_ = extractionConfig
}

// Key parameter explanations:
//
// BatchSize: Number of texts to embed at once (32-128 typical)
//   - Larger batches are faster but use more memory
//   - Smaller batches for resource-constrained environments
//
// Normalize: Whether to normalize vectors (L2 norm)
//   - true (recommended): Enables cosine similarity in vector DBs
//   - false: Raw embedding values
//
// CacheDir: Where to store downloaded models
//   - Empty string: Uses .xberg/embeddings/ in current directory
//   - Non-empty: Custom directory for model storage
//
// ShowDownloadProgress: Display download progress bar
//   - Useful for monitoring large model downloads
