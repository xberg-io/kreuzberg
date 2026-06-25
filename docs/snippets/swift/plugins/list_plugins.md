```swift title="Swift"
import Xberg

let extractors = try Xberg.listDocumentExtractors()
let renderers = try Xberg.listRenderers()
let processors = try Xberg.listPostProcessors()
let ocrBackends = try Xberg.listOcrBackends()
let validators = try Xberg.listValidators()
let embeddingBackends = try Xberg.listEmbeddingBackends()

print("Extractors: \(extractors)")
print("Renderers: \(renderers)")
print("Processors: \(processors)")
print("OCR backends: \(ocrBackends)")
print("Validators: \(validators)")
print("Embedding backends: \(embeddingBackends)")
```
