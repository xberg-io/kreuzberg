```kotlin title="Kotlin"
import dev.xberg.*
import dev.xberg.kt.Xberg
import kotlinx.coroutines.runBlocking
import java.nio.file.Paths

fun main() = runBlocking {
    val config = ExtractionConfig.builder().build()
    val result = Xberg.extractFile(Paths.get("document.pdf"), null, config)

    println(result.content())
    println("MIME type: ${result.mimeType()}")
    println("Tables: ${result.tables()?.size ?: 0}")
}
```
