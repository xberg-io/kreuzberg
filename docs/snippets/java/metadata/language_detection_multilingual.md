```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.LanguageDetectionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .languageDetection(LanguageDetectionConfig.builder()
        .enabled(true)
        .minConfidence(0.8)
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("multilingual_document.pdf", config);

System.out.println("Detected languages: " + result.getDetectedLanguages());
```
