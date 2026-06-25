```swift title="Swift"
import Xberg

// Wrap a custom embedder (e.g., CoreML, ONNX, API-based).
// The Swift class must implement the EmbeddingBackend protocol.
final class MyEmbedder: EmbeddingBackend {
    private let modelUrl: URL

    init(modelUrl: URL) {
        self.modelUrl = modelUrl
    }

    // Plugin trait hooks
    func name() -> String {
        "my-embedder"
    }

    func version() -> String {
        "1.0.0"
    }

    func initialize() -> String {  // Returns JSON-encoded Result
        do {
            // Warm-up logic here
            return "{\"ok\": null}"
        } catch {
            return "{\"err\": \"Failed to initialize: \(error)\"}"
        }
    }

    func shutdown() -> String {  // Returns JSON-encoded Result
        "{\"ok\": null}"
    }

    // EmbeddingBackend hooks
    func dimensions() -> UInt {
        // Fixed dimensionality for this backend
        768
    }

    func embed(texts: [String]) -> String {  // Returns JSON-encoded Vec<Vec<f32>>
        do {
            // Embed texts using your backend (e.g., CoreML inference)
            let embeddings: [[Float]] = texts.map { _ in
                Array(repeating: 0.5, count: 768)  // Placeholder
            }
            let data = try JSONEncoder().encode(embeddings)
            let json = String(data: data, encoding: .utf8) ?? "[]"
            return "{\"ok\": \(json)}"
        } catch {
            return "{\"err\": \"Embedding failed: \(error)\"}"
        }
    }
}

// Register once at startup
let embedder = MyEmbedder(modelUrl: URL(fileURLWithPath: "/path/to/model"))
try Xberg.registerEmbeddingBackend(embedder)

print("Embedding backend 'my-embedder' registered")
// The registered backend can now be referenced by name in EmbeddingConfig
// via the plugin selection mechanism once alef supports it
```
