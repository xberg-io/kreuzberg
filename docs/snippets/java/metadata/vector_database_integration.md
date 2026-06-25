```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(512)
        .maxOverlap(50)
        .embedding("balanced")
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("document.pdf", config);

System.out.println("Extracted content: " + result.getContent().length() + " characters");
```
