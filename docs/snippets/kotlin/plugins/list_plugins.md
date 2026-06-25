```kotlin title="Kotlin"
import dev.xberg.*
import dev.xberg.kt.Xberg

fun listAllPlugins() {
    val extractors: List<String> = Xberg.listDocumentExtractors()
    println("Registered extractors: $extractors")

    val processors: List<String> = Xberg.listPostProcessors()
    println("Registered post-processors: $processors")

    val backends: List<String> = Xberg.listOcrBackends()
    println("Registered OCR backends: $backends")

    val validators: List<String> = Xberg.listValidators()
    println("Registered validators: $validators")
}
```
