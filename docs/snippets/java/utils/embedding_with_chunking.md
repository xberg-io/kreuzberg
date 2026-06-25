```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(1024)
        .maxOverlap(100)
        .embedding("balanced")
        .build())
    .build();
```
