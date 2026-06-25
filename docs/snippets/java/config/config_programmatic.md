```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ChunkingConfig;
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;
import dev.xberg.TesseractConfig;

public final class ProgrammaticConfigExample {
    public static void main(String[] args) throws Exception {
        ExtractionConfig config = ExtractionConfig.builder()
            .ocr(OcrConfig.builder()
                .backend("tesseract")
                .language("eng+deu")
                .tesseractConfig(TesseractConfig.builder()
                    .psm(6)
                    .build())
                .build())
            .chunking(ChunkingConfig.builder()
                .maxChars(1000)
                .maxOverlap(200)
                .build())
            .useCache(true)
            .enableQualityProcessing(true)
            .build();

        ExtractionResult result = Xberg.extractFile("document.pdf", config);
        System.out.printf("Content length: %d%n", result.getContent().length());
    }
}
```
