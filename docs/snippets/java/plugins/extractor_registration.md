```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.XbergException;
import java.io.IOException;

public class CustomExtractorExample {
    public static void main(String[] args) {
        try {
            ExtractionResult result = Xberg.extractFile("document.json");
            System.out.println("Extracted content length: " + result.getContent().length());
        } catch (IOException | XbergException e) {
            e.printStackTrace();
        }
    }
}
```
