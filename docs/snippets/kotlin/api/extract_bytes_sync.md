```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Files
import java.nio.file.Paths

fun main() {
    val content = Files.readAllBytes(Paths.get("document.pdf"))
    val config = ExtractionConfig.builder().build()
    val result = Xberg.extractBytesSync(content, "application/pdf", config)

    println(result.content())
    println("Tables: ${result.tables()?.size ?: 0}")
}
```
