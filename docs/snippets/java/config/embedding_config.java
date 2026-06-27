import io.xberg.EmbeddingConfig;
import io.xberg.EmbeddingModelType;
import io.xberg.ChunkingConfig;
import io.xberg.ExtractionConfig;

public class EmbeddingConfigExample {
    public static void main(String[] args) {
        // Example 1: Preset model (recommended)
        // Fast, balanced, or quality preset configurations optimized for common use cases.
        EmbeddingConfig embeddingConfig = EmbeddingConfig.builder()
            .model(EmbeddingModelType.preset("balanced"))
            .batchSize(32)
            .normalize(true)
            .showDownloadProgress(true)
            .cacheDir("~/.cache/xberg/embeddings")
            .build();

        // Available presets:
        // - "fast" (384 dims): Quick prototyping, development, resource-constrained
        // - "balanced" (768 dims): Production, general-purpose RAG, English documents
        // - "quality" (1024 dims): Complex documents, maximum accuracy
        // - "multilingual" (768 dims): International documents, 100+ languages


        // Example 2: Custom ONNX model (requires embeddings feature)
        // Direct access to specific ONNX embedding models from HuggingFace with custom dimensions.
        embeddingConfig = EmbeddingConfig.builder()
            .model(EmbeddingModelType.custom("BAAI/bge-small-en-v1.5", 384))
            .batchSize(32)
            .normalize(true)
            .showDownloadProgress(true)
            .cacheDir(null)  // Uses default: .xberg/embeddings/
            .build();

        // Popular ONNX-compatible models:
        // - "BAAI/bge-small-en-v1.5" (384 dims): Fast, efficient
        // - "BAAI/bge-base-en-v1.5" (768 dims): Balanced quality/speed
        // - "BAAI/bge-large-en-v1.5" (1024 dims): High quality, slower
        // - "sentence-transformers/paraphrase-multilingual-mpnet-base-v2" (768 dims): Multilingual support


        // Example 3: Alternative Custom ONNX Model
        // For advanced users wanting different ONNX embedding models.
        embeddingConfig = EmbeddingConfig.builder()
            .model(EmbeddingModelType.custom("sentence-transformers/all-mpnet-base-v2", 768))
            .batchSize(16)  // Larger model requires smaller batch size
            .normalize(true)
            .showDownloadProgress(true)
            .cacheDir("/var/cache/embeddings")
            .build();


        // Integration with ChunkingConfig
        // Add embeddings to your chunking configuration:
        ChunkingConfig chunkingConfig = ChunkingConfig.builder()
            .maxChars(1024)
            .maxOverlap(100)
            .preset("balanced")
            .embedding(EmbeddingConfig.builder()
                .model(EmbeddingModelType.preset("balanced"))
                .batchSize(32)
                .normalize(true)
                .build())
            .build();

        ExtractionConfig extractionConfig = ExtractionConfig.builder()
            .chunking(chunkingConfig)
            .build();
    }
}

// Key parameter explanations:
//
// batchSize: Number of texts to embed at once (32-128 typical)
//   - Larger batches are faster but use more memory
//   - Smaller batches for resource-constrained environments
//
// normalize: Whether to normalize vectors (L2 norm)
//   - true (recommended): Enables cosine similarity in vector DBs
//   - false: Raw embedding values
//
// cacheDir: Where to store downloaded models
//   - null: Uses .xberg/embeddings/ in current directory
//   - String path: Custom directory for model storage
//
// showDownloadProgress: Display download progress bar
//   - Useful for monitoring large model downloads
