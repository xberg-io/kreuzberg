```java title="Java"
import io.xberg.ExtractionConfig;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .maxChars(1000)
        .maxOverlap(200)
        .build())
    .build();
```
```java title="Java - Markdown with Heading Context"
import io.xberg.Xberg;
import io.xberg.ExtractInput;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .chunkerType("markdown")
        .maxChars(500)
        .maxOverlap(50)
        .sizingTokenizer("Xenova/gpt-4o")
        .build())
    .build();
ExtractionResult output = Xberg.extract(
    ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.md").build(),
    config
);
ExtractedDocument result = output.results().get(0);
result.chunks().forEach(chunk -> {
    var headingContext = chunk.metadata().headingContext();
    if (headingContext.isPresent()) {
        System.out.println("Headings:");
        headingContext.get().headings().forEach(heading ->
            System.out.println("  Level " + heading.level() + ": " + heading.text())
        );
    }
});
```

```java title="Java - Prepend Heading Context"
import io.xberg.Xberg;
import io.xberg.ExtractInput;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ChunkingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .chunking(ChunkingConfig.builder()
        .prependHeadingContext(true)
        .build())
    .build();
ExtractionResult output = Xberg.extract(
    ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.md").build(),
    config
);
ExtractedDocument result = output.results().get(0);
// Each chunk's content is prefixed with its heading breadcrumb
result.chunks().forEach(chunk ->
    System.out.println(chunk.content().substring(0, Math.min(100, chunk.content().length())))
);
```
