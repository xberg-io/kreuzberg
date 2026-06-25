```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.KeywordConfig;
import dev.xberg.KeywordAlgorithm;

ExtractionConfig config = ExtractionConfig.builder()
    .keywords(KeywordConfig.builder()
        .algorithm(KeywordAlgorithm.YAKE)
        .maxKeywords(10)
        .minScore(0.3)
        .ngramRange(1, 3)
        .language("en")
        .build())
    .build();
```
