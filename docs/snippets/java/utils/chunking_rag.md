```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractInput;
import io.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(500)
        .maxOverlap(50)
        .embedding("balanced")
        .build())
    .build();
ExtractionResult output = Xberg.extract(
    ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("research_paper.pdf").build(),
    config
);
ExtractedDocument result = output.results().get(0);
System.out.println("Content: " + result.content()
    .substring(0, Math.min(100, result.content().length())) + "...");
```
