```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;
import dev.xberg.ImagePreprocessingConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .ocr(OcrConfig.builder()
        .backend("tesseract")
        .build())
    .imagePreprocessing(ImagePreprocessingConfig.builder()
        .targetDpi(300)
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("scanned.pdf", config);
```
