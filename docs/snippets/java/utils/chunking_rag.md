```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(500)
        .maxOverlap(50)
        .embedding("balanced")
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("research_paper.pdf", config);

System.out.println("Content: " + result.getContent()
    .substring(0, Math.min(100, result.getContent().length())) + "...");
```
