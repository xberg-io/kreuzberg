```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import java.nio.file.Paths;

ExtractionConfig config = ExtractionConfig.builder().build();
ExtractionResult result = Xberg.extractFile(Paths.get("document.pdf"), config);

System.out.println(result.content());
System.out.println(result.mimeType());
```
