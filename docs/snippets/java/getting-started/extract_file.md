```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import java.io.IOException;

public class ExtractFile {
    public static void main(String[] args) throws IOException {
        ExtractionConfig config = ExtractionConfig.builder()
            .useCache(true)
            .enableQualityProcessing(true)
            .build();

        ExtractionResult result = Xberg.extractFile("contract.pdf", config);

        System.out.println("Extracted " + result.getContent().length() + " characters");
        System.out.println("Quality score: " + result.getQualityScore());
        System.out.println("Processing time: " + result.getMetadata().get("processing_time") + "ms");
    }
}
```
