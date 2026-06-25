// Example 1: Preset model (recommended)
// Fast, balanced, or quality preset configurations optimized for common use cases.
let embedding_config = EmbeddingConfig {
    model: EmbeddingModelType::Preset {
        name: "balanced".to_string(),
    },
    batch_size: 32,
    normalize: true,
    show_download_progress: true,
    cache_dir: Some(std::path::PathBuf::from("~/.cache/xberg/embeddings")),
    acceleration: None,
};

// Available presets:
// - "fast" (384 dims): Quick prototyping, development, resource-constrained
// - "balanced" (768 dims): Production, general-purpose RAG, English documents
// - "quality" (1024 dims): Complex documents, maximum accuracy
// - "multilingual" (768 dims): International documents, 100+ languages

// Example 2: Custom ONNX model (requires embeddings feature)
// Direct access to specific ONNX embedding models from HuggingFace with custom dimensions.
let embedding_config = EmbeddingConfig {
    model: EmbeddingModelType::Custom {
        model_id: "BAAI/bge-small-en-v1.5".to_string(),
        dimensions: 384,
    },
    batch_size: 32,
    normalize: true,
    show_download_progress: true,
    cache_dir: None,  // Uses default: .xberg/embeddings/
    acceleration: None,
};

// Popular ONNX-compatible models:
// - "BAAI/bge-small-en-v1.5" (384 dims): Fast, efficient
// - "BAAI/bge-base-en-v1.5" (768 dims): Balanced quality/speed
// - "BAAI/bge-large-en-v1.5" (1024 dims): High quality, slower
// - "sentence-transformers/paraphrase-multilingual-mpnet-base-v2" (768 dims): Multilingual support

// Example 3: Alternative Custom ONNX Model
// For advanced users wanting different ONNX embedding models.
let embedding_config = EmbeddingConfig {
    model: EmbeddingModelType::Custom {
        model_id: "sentence-transformers/all-mpnet-base-v2".to_string(),
        dimensions: 768,
    },
    batch_size: 16,  // Larger model requires smaller batch size
    normalize: true,
    show_download_progress: true,
    cache_dir: Some(std::path::PathBuf::from("/var/cache/embeddings")),
    acceleration: None,
};

// Integration with ChunkingConfig
// Add embeddings to your chunking configuration:
use xberg::{ChunkingConfig, ExtractionConfig};

let chunking_with_embeddings = ChunkingConfig {
    max_characters: 1024,
    overlap: 100,
    preset: Some("balanced".to_string()),
    embedding: Some(EmbeddingConfig::default()),  // Uses balanced preset
};

let extraction_config = ExtractionConfig {
    chunking: Some(chunking_with_embeddings),
    ..Default::default()
};

// Key parameter explanations:
//
// batch_size: Number of texts to embed at once (32-128 typical)
//   - Larger batches are faster but use more memory
//   - Smaller batches for resource-constrained environments
//
// normalize: Whether to normalize vectors (L2 norm)
//   - true (recommended): Enables cosine similarity in vector DBs
//   - false: Raw embedding values
//
// cache_dir: Where to store downloaded models
//   - None: Uses .xberg/embeddings/ in current directory
//   - Some(path): Custom directory for model storage
//
// show_download_progress: Display download progress bar
//   - Useful for monitoring large model downloads
