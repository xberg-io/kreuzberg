```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .useCache(true)
    .enableQualityProcessing(true)
    .build();
ExtractionResult result = Xberg.extractFile("document.pdf", config);
```
