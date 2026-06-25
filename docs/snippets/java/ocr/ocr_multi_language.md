```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .ocr(OcrConfig.builder()
        .backend("tesseract")
        .language("eng+deu+fra")
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("multilingual.pdf", config);
System.out.println(result.getContent());
```
