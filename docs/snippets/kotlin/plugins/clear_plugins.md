```kotlin title="Kotlin"
import dev.xberg.*
import dev.xberg.kt.Xberg

fun clearAllPlugins() {
    // Note: there is no Xberg.clearDocumentExtractors() — extractor
    // registration is not exposed through the Kotlin/Java plugin bridge.
    Xberg.clearPostProcessors()
    Xberg.clearOcrBackends()
    Xberg.clearValidators()

    println("All post-processors, OCR backends, and validators cleared")
}
```
