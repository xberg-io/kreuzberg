```swift title="Swift"
import Xberg

let names = [
    "custom-json-extractor",
    "word_count",
    "cloud-ocr",
    "min_length_validator",
]

try Xberg.unregisterDocumentExtractor(names[0])
try Xberg.unregisterPostProcessor(names[1])
try Xberg.unregisterOcrBackend(names[2])
try Xberg.unregisterValidator(names[3])

print("Plugins unregistered")
```
