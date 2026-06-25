```java
import dev.xberg.Xberg;
import dev.xberg.EmbeddingConfig;

// Embed with default config
float[][] embeddings = Xberg.embed(List.of("Hello world", "How are you?"), null);

// Embed with specific preset
EmbeddingConfig config = EmbeddingConfig.withPreset("fast");
float[][] fastEmbeddings = Xberg.embed(List.of("Hello world"), config);

// Async variant
CompletableFuture<float[][]> future = Xberg.embedAsync(texts, null);
```
