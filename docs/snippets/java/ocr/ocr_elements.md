```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.XbergException;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractInput;
import io.xberg.OcrConfig;
import io.xberg.types.OcrElement;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        try {
            ExtractionConfig config = ExtractionConfig.builder()
                .ocr(OcrConfig.builder()
                    .backend("paddle-ocr")
                    .language("en")
                    .build())
                .build();
            ExtractionResult output = Xberg.extract(
                ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("scanned.pdf").build(),
                config
            );
            ExtractedDocument result = output.results().get(0);
            if (result.ocrElements() != null) {
                for (OcrElement element : result.ocrElements()) {
                    System.out.printf("Text: %s%n", element.text());
                    System.out.printf("Confidence: %.2f%n", element.confidence().recognition());
                    System.out.printf("Geometry: %s%n", element.geometry());
                    if (element.rotation() != null) {
                        System.out.printf("Rotation: %.1f°%n", element.rotation().angle());
                    }
                    System.out.println();
                }
            }
        } catch (IOException | XbergException e) {
            System.err.println("Extraction failed: " + e.getMessage());
        }
    }
}
```
