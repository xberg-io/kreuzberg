```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractInput;
import io.xberg.LanguageDetectionConfig;
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
    ExtractionResult output = Xberg.extract(
        ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("multilingual_document.pdf").build(),
        config
    );
    ExtractedDocument result = output.results().get(0);
    List<String> languages = result.detectedLanguages() != null
        ? result.detectedLanguages()
        : List.of();
    if (!languages.isEmpty()) {
        System.out.println("Detected " + languages.size() + " language(s): " + String.join(", ", languages));
    } else {
        System.out.println("No languages detected");
    }
    System.out.println("Total content: " + result.content().length() + " characters");
    System.out.println("MIME type: " + result.mimeType());
} catch (Exception ex) {
    System.err.println("Processing failed: " + ex.getMessage());
}
```
