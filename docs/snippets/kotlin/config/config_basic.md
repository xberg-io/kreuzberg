```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths
import java.util.Optional

fun main() {
    val config = ExtractionConfig.builder()
        .withUseCache(true)
        .withEnableQualityProcessing(true)
        .build()

    val result = Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
    println(result.content())
}
```
