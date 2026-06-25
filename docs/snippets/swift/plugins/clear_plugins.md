```swift title="Swift"
import Xberg

// Clear all registered plugins in each registry
try Xberg.clearDocumentExtractors()
try Xberg.clearRenderers()
try Xberg.clearOcrBackends()
try Xberg.clearPostProcessors()
try Xberg.clearValidators()
try Xberg.clearEmbeddingBackends()

print("All plugins cleared")
```
