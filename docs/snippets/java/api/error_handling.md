```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.XbergRsException;
import java.nio.file.Paths;

try {
    ExtractionConfig config = ExtractionConfig.builder().build();
    ExtractionResult result = Xberg.extractFileSync(Paths.get("missing.pdf"), config);
    System.out.println(result.content());
} catch (XbergRsException e) {
    System.err.println("Extraction failed: " + e.getMessage());
    System.err.println("Error code: " + e.getCode());
}
```
