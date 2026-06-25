```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.Validator;
import dev.xberg.ValidationException;
import dev.xberg.XbergException;
import java.io.IOException;

public class MinLengthValidatorExample {
    public static void main(String[] args) {
        int minLength = 100;

        Validator minLengthValidator = result -> {
            if (result.getContent().length() < minLength) {
                throw new ValidationException(
                    "Content too short: " + result.getContent().length() +
                    " < " + minLength
                );
            }
        };

        try {
            Xberg.registerValidator("min-length", minLengthValidator, 100);

            ExtractionResult result = Xberg.extractFile("document.pdf");
            System.out.println("Validation passed!");
        } catch (ValidationException e) {
            System.err.println("Validation failed: " + e.getMessage());
        } catch (IOException | XbergException e) {
            e.printStackTrace();
        }
    }
}
```
