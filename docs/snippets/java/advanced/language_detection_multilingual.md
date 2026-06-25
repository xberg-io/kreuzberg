```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.LanguageDetectionConfig;
import java.math.BigDecimal;
import java.util.List;

ExtractionConfig config = ExtractionConfig.builder()
    .languageDetection(LanguageDetectionConfig.builder()
        .enabled(true)
        .minConfidence(new BigDecimal("0.8"))
        .detectMultiple(true)
        .build())
    .build();

try {
    ExtractionResult result = Xberg.extractFile("multilingual_document.pdf", config);

    List<String> languages = result.getDetectedLanguages() != null
        ? result.getDetectedLanguages()
        : List.of();

    if (!languages.isEmpty()) {
        System.out.println("Detected " + languages.size() + " language(s): " + String.join(", ", languages));
    } else {
        System.out.println("No languages detected");
    }

    System.out.println("Total content: " + result.getContent().length() + " characters");
    System.out.println("MIME type: " + result.getMimeType());
} catch (Exception ex) {
    System.err.println("Processing failed: " + ex.getMessage());
}
```
