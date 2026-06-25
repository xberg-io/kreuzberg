```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.PostProcessor;
import dev.xberg.XbergException;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class WordCountExample {
    public static void main(String[] args) {
        PostProcessor wordCount = result -> {
            long count = result.getContent().split("\\s+").length;

            Map<String, Object> metadata = new HashMap<>(result.getMetadata());
            metadata.put("word_count", count);

            return result;
        };

        try {
            Xberg.registerPostProcessor("word-count", wordCount, 50);

            ExtractionResult result = Xberg.extractFile("document.pdf");
            System.out.println("Word count: " + result.getMetadata().get("word_count"));
        } catch (IOException | XbergException e) {
            e.printStackTrace();
        }
    }
}
```
