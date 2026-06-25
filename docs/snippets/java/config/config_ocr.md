```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;
import dev.xberg.TesseractConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .ocr(OcrConfig.builder()
        .backend("tesseract")
        .language("eng+fra")
        .tesseractConfig(TesseractConfig.builder()
            .psm(3)
            .build())
        .build())
    .build();
```
