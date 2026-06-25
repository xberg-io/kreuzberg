```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;
import dev.xberg.TokenReductionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .tokenReduction(TokenReductionConfig.builder()
        .mode("moderate")
        .preserveImportantWords(true)
        .build())
    .build();

ExtractionResult result = Xberg.extractFile("verbose_document.pdf", config);

Object originalTokens = result.getMetadata().get("original_token_count");
Object reducedTokens = result.getMetadata().get("token_count");
Object reductionRatio = result.getMetadata().get("token_reduction_ratio");

System.out.println("Reduced from " + originalTokens + " to " + reducedTokens + " tokens");
System.out.println("Reduction: " + ((Number)reductionRatio).doubleValue() * 100 + "%");
```
