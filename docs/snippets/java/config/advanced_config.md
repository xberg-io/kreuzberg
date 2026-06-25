```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.XbergException;
import dev.xberg.*;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        try {
            ExtractionConfig config = ExtractionConfig.builder()
                .ocr(OcrConfig.builder()
                    .backend("tesseract")
                    .language("eng+deu")
                    .build())
                .chunking(ChunkingConfig.builder()
                    .maxChars(1000)
                    .maxOverlap(100)
                    .build())
                .tokenReduction(TokenReductionConfig.builder()
                    .mode("moderate")
                    .preserveImportantWords(true)
                    .build())
                .languageDetection(LanguageDetectionConfig.builder()
                    .enabled(true)
                    .build())
                .useCache(true)
                .enableQualityProcessing(true)
                .build();

            ExtractionResult result = Xberg.extractFile("document.pdf", config);

            if (!result.getDetectedLanguages().isEmpty()) {
                System.out.println("Languages: " + result.getDetectedLanguages());
            }
        } catch (IOException | XbergException e) {
            System.err.println("Extraction failed: " + e.getMessage());
        }
    }
}
```
