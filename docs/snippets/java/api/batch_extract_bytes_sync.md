```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.BatchBytesItem;
import dev.xberg.ExtractionConfig;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Arrays;

byte[] doc1 = Files.readAllBytes(Paths.get("doc1.pdf"));
byte[] doc2 = Files.readAllBytes(Paths.get("doc2.docx"));

List<BatchBytesItem> items = Arrays.asList(
    new BatchBytesItem(doc1, "application/pdf", null),
    new BatchBytesItem(doc2, "application/vnd.openxmlformats-officedocument.wordprocessingml.document", null)
);

ExtractionConfig config = ExtractionConfig.builder().build();
List<ExtractionResult> results = Xberg.batchExtractBytesSync(items, config);
System.out.println("Processed " + results.size() + " documents");
```
