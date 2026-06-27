```ts title="TypeScript"
import {
  extract,
  registerValidator,
  unregisterValidator,
  ValidationError,
  type ExtractedDocument,
} from "@xberg-io/xberg";

class MinLengthValidator {
  name = "min_length_validator";
  priority = 10;

  validate(result: ExtractedDocument): void {
    if (result.content.length < 50) {
      throw new ValidationError(`Content too short: ${result.content.length}`);
    }
  }
}

registerValidator(new MinLengthValidator());

const output = await extract({
  kind: "uri",
  uri: "document.pdf",
});
console.log(`Validated content length: ${output.results[0].content.length}`);

unregisterValidator("min_length_validator");
```
