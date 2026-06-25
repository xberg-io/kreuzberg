```java title="Java"
import dev.xberg.ChunkingConfig;
import dev.xberg.EmbeddingConfig;
import dev.xberg.EmbeddingModelType;
import dev.xberg.ExtractionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(1500)
        .maxOverlap(200)
        .embedding(EmbeddingConfig.builder()
            .model(EmbeddingModelType.builder()
                .type("preset")
                .name("text-embedding-all-minilm-l6-v2")
                .build())
            .build())
        .build())
    .build();
```
