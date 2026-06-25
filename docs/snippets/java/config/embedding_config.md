```java title="Java"
import dev.xberg.ChunkingConfig;
import dev.xberg.EmbeddingConfig;
import dev.xberg.EmbeddingModelType;
import dev.xberg.ExtractionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(1000)
        .embedding(EmbeddingConfig.builder()
            .model(EmbeddingModelType.builder()
                .type("preset")
                .name("all-mpnet-base-v2")
                .build())
            .batchSize(16)
            .normalize(true)
            .showDownloadProgress(true)
            .build())
        .build())
    .build();
```
