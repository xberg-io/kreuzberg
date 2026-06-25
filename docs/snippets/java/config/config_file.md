```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import java.nio.file.Path;

public final class ConfigFileExample {
    public static void main(String[] args) throws Exception {
        ExtractionConfig config = Xberg.loadExtractionConfigFromFile(Path.of("xberg.toml"));
        ExtractionResult result = Xberg.extractFile(Path.of("document.pdf"), config);
        System.out.printf("Detected MIME: %s%n", result.getMimeType());
    }
}
```
