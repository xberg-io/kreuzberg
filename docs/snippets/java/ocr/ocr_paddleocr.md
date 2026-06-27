```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.XbergException;
import io.xberg.ExtractionConfig;
import io.xberg.ExtractInput;
import io.xberg.OcrConfig;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        try {
            ExtractionConfig config = ExtractionConfig.builder()
                .ocr(OcrConfig.builder()
                    .backend("paddle-ocr")
                    .language("en")
                    // .paddleOcrConfig(PaddleOcrConfig.builder().modelTier("server").build()) // for max accuracy
                    .build())
                .build();
            ExtractionResult output = Xberg.extract(
                ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("scanned.pdf").build(),
                config
            );
            ExtractedDocument result = output.results().get(0);
            System.out.println(result.content());
        } catch (IOException | XbergException e) {
            System.err.println("Extraction failed: " + e.getMessage());
        }
    }
}
```
