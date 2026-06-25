```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths
import java.util.Optional

fun main() {
    val ocr = OcrConfig.builder()
        .withBackend("tesseract")
        .withLanguage("eng")
        .build()

    val config = ExtractionConfig.builder()
        .withOcr(Optional.of(ocr))
        .withForceOcr(true)
        .build()

    val result = Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
    println(result.content())
}
```
