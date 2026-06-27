```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractInput;
import io.xberg.ExtractionConfig;
import io.xberg.PostProcessor;
import io.xberg.XbergException;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class WordCountExample {
    public static void main(String[] args) {
        PostProcessor wordCount = result -> {
            long count = result.content().split("\\s+").length;
            Map<String, Object> metadata = new HashMap<>(result.metadata());
            metadata.put("word_count", count);
            return result;
        };
        try {
            Xberg.registerPostProcessor("word-count", wordCount, 50);
            ExtractionResult output = Xberg.extract(
                ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.pdf").build(),
                ExtractionConfig.builder().build()
            );
            ExtractedDocument result = output.results().get(0);
            System.out.println("Word count: " + result.metadata().get("word_count"));
        } catch (IOException | XbergException e) {
            e.printStackTrace();
        }
    }
}
```
