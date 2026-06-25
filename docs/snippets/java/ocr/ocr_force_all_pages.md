```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .ocr(OcrConfig.builder()
        .backend("tesseract")
        .build())
    .forceOcr(true)
    .build();

ExtractionResult result = Xberg.extractFile("document.pdf", config);
System.out.println(result.getContent());
```
