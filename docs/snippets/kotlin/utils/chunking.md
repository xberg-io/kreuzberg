```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths
import java.util.Optional

fun main() {
    val chunking = ChunkingConfig.builder()
        .withMaxCharacters(1500L)
        .withOverlap(200L)
        .build()

    val config = ExtractionConfig.builder()
        .withChunking(Optional.of(chunking))
        .build()

    val result = Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
    println("Chunks: ${result.chunks()?.size ?: 0}")
}
```
