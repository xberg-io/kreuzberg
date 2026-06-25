```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import java.nio.file.Files;
import java.nio.file.Paths;

byte[] data = Files.readAllBytes(Paths.get("document.pdf"));
ExtractionConfig config = ExtractionConfig.builder().build();
ExtractionResult result = Xberg.extractBytes(data, "application/pdf", config);

System.out.println(result.content());
System.out.println(result.mimeType());
```
