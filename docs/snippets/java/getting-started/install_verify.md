```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractInput;
import io.xberg.ExtractionConfig;
import java.io.IOException;

public class InstallVerify {
    public static void main(String[] args) throws IOException {
        System.out.println("Xberg FFI bindings loaded successfully");
        ExtractionResult output = Xberg.extract(
            ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("sample.pdf").build(),
            ExtractionConfig.builder().build()
        );
        ExtractedDocument result = output.results().get(0);
        System.out.println("Installation verified!");
        System.out.println("Extracted " + result.content().length() + " characters");
    }
}
```
