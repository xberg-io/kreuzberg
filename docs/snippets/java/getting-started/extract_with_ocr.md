```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractInput;
import io.xberg.OcrConfig;
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
        ExtractionResult output = Xberg.extract(
            ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("scanned.pdf").build(),
            config
        );
        ExtractedDocument result = output.results().get(0);
        System.out.println("Extracted text from scanned document:");
        System.out.println(result.content());
        System.out.println("Used OCR backend: tesseract");
    }
}
```
