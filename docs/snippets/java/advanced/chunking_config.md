```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.ChunkingConfig;
import dev.xberg.EmbeddingConfig;
import dev.xberg.EmbeddingModelType;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(1000)
        .maxOverlap(200)
        .embedding(EmbeddingConfig.builder()
            .model(EmbeddingModelType.preset("all-minilm-l6-v2"))
            .normalize(true)
            .batchSize(32)
            .build())
        .build())
    .build();
```
