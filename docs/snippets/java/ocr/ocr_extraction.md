```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.XbergException;
import dev.xberg.ExtractionConfig;
import dev.xberg.OcrConfig;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        try {
            ExtractionConfig config = ExtractionConfig.builder()
                .ocr(OcrConfig.builder()
                    .backend("tesseract")
                    .language("eng")
                    .build())
                .build();

            ExtractionResult result = Xberg.extractFile("scanned.pdf", config);
            System.out.println(result.getContent());
        } catch (IOException | XbergException e) {
            System.err.println("Extraction failed: " + e.getMessage());
        }
    }
}
```
