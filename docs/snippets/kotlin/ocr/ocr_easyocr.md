```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths
import java.util.Optional

fun main() {
    val ocr = OcrConfig.builder()
        .withBackend("easyocr")
        .withLanguage("en")
        .build()

    val config = ExtractionConfig.builder()
        .withOcr(Optional.of(ocr))
        .build()

    val result = Xberg.extractFileSync(Paths.get("document.pdf"), null, config)
    println("Extracted text: ${result.content()}")
}
```
