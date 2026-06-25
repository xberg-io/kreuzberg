```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Files
import java.nio.file.Paths

fun extractText(bytes: ByteArray, mimeType: String): String {
    val config = ExtractionConfig.builder().build()
    val result = Xberg.extractBytesSync(bytes, mimeType, config)
    return result.content()
}

fun main() {
    val bytes = try {
        Files.readAllBytes(Paths.get("document.pdf"))
    } catch (e: Exception) {
        ByteArray(0)
    }

    try {
        val text = extractText(bytes, "application/pdf")
        println("Extracted ${text.length} chars")
    } catch (e: XbergRsException) {
        System.err.println("Extraction error (code=${e.code}): ${e.message}")
    } catch (e: Exception) {
        System.err.println("Unexpected error: ${e.message}")
    }
}
```
