```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;
import java.io.IOException;

public class ExtractWithOCR {
    public static void main(String[] args) throws IOException {
        OcrConfig ocrConfig = OcrConfig.builder()
            .backend("tesseract")
            .language("eng")
            .build();

        ExtractionConfig config = ExtractionConfig.builder()
            .ocr(ocrConfig)
            .build();

        ExtractionResult result = Xberg.extractFile("scanned.pdf", config);

        System.out.println("Extracted text from scanned document:");
        System.out.println(result.getContent());
        System.out.println("Used OCR backend: tesseract");
    }
}
```
