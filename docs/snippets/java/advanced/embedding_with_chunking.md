```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.ChunkingConfig;
import dev.xberg.EmbeddingConfig;
import dev.xberg.EmbeddingModelType;
import java.util.List;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(512)
        .maxOverlap(50)
        .embedding(EmbeddingConfig.builder()
            .model(EmbeddingModelType.preset("balanced"))
            .normalize(true)
            .batchSize(32)
            .showDownloadProgress(false)
            .build())
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("document.pdf", config);

List<Object> chunks = result.getChunks() != null ? result.getChunks() : List.of();
for (int index = 0; index < chunks.size(); index++) {
    Object chunk = chunks.get(index);
    String chunkId = "doc_chunk_" + index;
    System.out.println("Chunk " + chunkId + ": " + chunk.toString().substring(0, Math.min(50, chunk.toString().length())));

    if (chunk instanceof java.util.Map) {
        Object embedding = ((java.util.Map<String, Object>) chunk).get("embedding");
        if (embedding != null) {
            System.out.println("  Embedding dimensions: " + ((float[]) embedding).length);
        }
    }
}
```
