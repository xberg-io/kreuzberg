```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import java.nio.file.Paths;

ExtractionConfig config = ExtractionConfig.builder().build();
ExtractionResult result = Xberg.extractFileSync(Paths.get("document.pdf"), config);

System.out.println(result.content());
System.out.println("Tables: " + (result.tables() != null ? result.tables().size() : 0));
System.out.println("Metadata: " + result.metadata());
```
