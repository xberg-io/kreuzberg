```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.LanguageDetectionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .languageDetection(LanguageDetectionConfig.builder()
        .enabled(true)
        .minConfidence(0.8)
        .build())
    .build();
```
