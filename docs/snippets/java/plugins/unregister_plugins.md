```java title="Java"
import dev.xberg.Xberg;

try {
    // Unregister specific plugins
    Xberg.unregisterPostProcessor("word-count");
    Xberg.unregisterValidator("min-length");
} catch (XbergException e) {
    System.err.println("Failed to unregister: " + e.getMessage());
}
```
