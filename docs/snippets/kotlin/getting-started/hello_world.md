```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths

fun main() {
    println("Hello from Xberg!")
    val config = ExtractionConfig.builder().build()
    val result = dev.xberg.Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
    println(result.content())
}
```
