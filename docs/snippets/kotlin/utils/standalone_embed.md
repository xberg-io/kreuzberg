```kotlin title="Kotlin"
import dev.xberg.*
import java.util.Optional

fun main() {
    val config = EmbeddingConfig.builder()
        .withModel(EmbeddingModelType.Preset("balanced"))
        .withNormalize(true)
        .build()

    val texts = listOf("Hello, world!", "Xberg is fast")
    val embeddings = Xberg.embedTexts(texts, config)

    println("Texts embedded: ${embeddings.size}")
    println("Dimensions: ${embeddings[0].size}")
}
```
