```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;

ExtractionConfig config = Xberg.discoverExtractionConfig();
ExtractionResult result = Xberg.extractFile("document.pdf", config);
```
