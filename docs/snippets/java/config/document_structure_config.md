```java title="Document Structure Config (Java)"
import dev.xberg.Xberg;
import dev.xberg.ExtractionConfig;
import dev.xberg.ExtractionResult;

ExtractionConfig config = ExtractionConfig.builder()
    .includeDocumentStructure(true)
    .build();

ExtractionResult result = Xberg.extractFileSync("document.pdf", config);

if (result.getDocumentStructure().isPresent()) {
    var document = result.getDocumentStructure().get();
    for (var node : document.nodes()) {
        System.out.println("[" + node.content().nodeType() + "]");
    }
}
```
