```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.LanguageDetectionConfig;
import java.math.BigDecimal;

ExtractionConfig config = ExtractionConfig.builder()
    .languageDetection(LanguageDetectionConfig.builder()
        .enabled(true)
        .minConfidence(new BigDecimal("0.8"))
        .detectMultiple(false)
        .build())
    .build();
```
