```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths

fun main() {
    val config = ExtractionConfig.builder().build()
    try {
        val result = Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
        println(result.content())
    } catch (e: XbergRsException) {
        System.err.println("Extraction failed: ${e.message}")
        System.err.println("Error code: ${e.code}")
    } catch (e: Exception) {
        System.err.println("Unexpected error: ${e.message}")
    }
}
```
