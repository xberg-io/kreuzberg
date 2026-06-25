```kotlin title="Kotlin"
import dev.xberg.*
import java.nio.file.Paths

fun main() {
    val config = ExtractionConfig.builder().build()
    val items = listOf(
        BatchFileItem(Paths.get("doc1.pdf"), null),
        BatchFileItem(Paths.get("doc2.docx"), null),
        BatchFileItem(Paths.get("report.pdf"), null),
    )
    val results = Xberg.batchExtractFilesSync(items, config)

    results.forEachIndexed { index, result ->
        println("File $index: ${result.content().length} chars")
    }
}
```
