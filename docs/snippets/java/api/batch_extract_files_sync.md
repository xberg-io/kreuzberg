```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.BatchFileItem;
import dev.xberg.ExtractionConfig;
import java.nio.file.Paths;
import java.util.List;
import java.util.Arrays;

List<BatchFileItem> items = Arrays.asList(
    new BatchFileItem(Paths.get("doc1.pdf"), null),
    new BatchFileItem(Paths.get("doc2.docx"), null),
    new BatchFileItem(Paths.get("doc3.pptx"), null)
);

ExtractionConfig config = ExtractionConfig.builder().build();
List<ExtractionResult> results = Xberg.batchExtractFilesSync(items, config);

for (ExtractionResult result : results) {
    System.out.println("Content length: " + result.content().length());
}
```
