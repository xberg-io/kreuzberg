```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths
import java.util.Optional

fun main() {
    val config = ExtractionConfig.builder()
        .withEnableQualityProcessing(true)
        .withUseCache(true)
        .build()

    val result = Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
    println("Quality score: ${result.qualityScore()}")
    println("Warnings: ${result.processingWarnings()?.size ?: 0}")
}
```
