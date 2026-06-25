```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.TokenReductionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .tokenReduction(TokenReductionConfig.builder()
        .mode("moderate")
        .preserveImportantWords(true)
        .build())
    .build();
```
