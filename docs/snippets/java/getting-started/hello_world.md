```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractInput;
import io.xberg.ExtractionConfig;
import java.io.IOException;

public class HelloWorld {
    public static void main(String[] args) throws IOException {
        ExtractionResult output = Xberg.extract(
            ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.pdf").build(),
            ExtractionConfig.builder().build()
        );
        ExtractedDocument result = output.results().get(0);
        System.out.println("Extracted content:");
        System.out.println(result.content().substring(0, Math.min(200, result.content().length())));
    }
}
```
