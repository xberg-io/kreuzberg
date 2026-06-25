```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import java.io.IOException;
import java.util.Map;

public class ReadContent {
    public static void main(String[] args) throws IOException {
        ExtractionResult result = Xberg.extractFile("document.pdf");

        String content = result.getContent();
        var tables = result.getTables();
        var images = result.getImages();
        Map<String, Object> metadata = result.getMetadata();

        System.out.println("Content: " + content.length() + " characters");
        System.out.println("Tables: " + tables.size());
        System.out.println("Images: " + images.size());
        if (metadata != null) {
            System.out.println("Metadata keys: " + metadata.keySet());
        }
    }
}
```
