```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.XbergException;
import io.xberg.*;
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
            ExtractionResult output = Xberg.extract(
                ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.pdf").build(),
                config
            );
            ExtractedDocument result = output.results().get(0);
            if (!result.detectedLanguages().isEmpty()) {
                System.out.println("Languages: " + result.detectedLanguages());
            }
        } catch (IOException | XbergException e) {
            System.err.println("Extraction failed: " + e.getMessage());
        }
    }
}
```
