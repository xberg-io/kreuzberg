```kotlin title="Kotlin"
import dev.xberg.*
import java.util.Optional

fun main() {
    val process = ProcessBuilder("xberg", "mcp")
        .inheritIO()
        .start()
    process.waitFor()
}
```
