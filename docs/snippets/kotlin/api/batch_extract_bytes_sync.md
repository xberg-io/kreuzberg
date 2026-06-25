```kotlin title="Kotlin"
import dev.xberg.*

fun main() {
    val config = ExtractionConfig.builder().build()
    val items = listOf(
        BatchBytesItem("Hello, world!".toByteArray(), "text/plain", null),
        BatchBytesItem("# Heading\n\nParagraph text.".toByteArray(), "text/markdown", null),
    )
    val results = Xberg.batchExtractBytesSync(items, config)

    results.forEachIndexed { index, result ->
        println("Item $index: ${result.content().length} chars")
    }
}
```
