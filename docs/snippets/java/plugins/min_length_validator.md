```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractInput;
import io.xberg.ExtractionConfig;
import io.xberg.Validator;
import io.xberg.ValidationException;
import io.xberg.XbergException;
import java.io.IOException;

public class MinLengthValidatorExample {
    public static void main(String[] args) {
        int minLength = 100;
        Validator minLengthValidator = result -> {
            if (result.content().length() < minLength) {
                throw new ValidationException(
                    "Content too short: " + result.content().length() +
                    " < " + minLength
                );
            }
        };
        try {
            Xberg.registerValidator("min-length", minLengthValidator, 100);
            ExtractionResult output = Xberg.extract(
                ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.pdf").build(),
                ExtractionConfig.builder().build()
            );
            ExtractedDocument result = output.results().get(0);
            System.out.println("Validation passed!");
        } catch (ValidationException e) {
            System.err.println("Validation failed: " + e.getMessage());
        } catch (IOException | XbergException e) {
            e.printStackTrace();
        }
    }
}
```
