<!-- snippet:skip reason="DocumentExtractor trait has no createDocumentExtractorDartImpl factory; custom extractors must be implemented in Rust." -->
```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  // Custom document extractors cannot be registered from Dart. While
  // registerDocumentExtractor exists in the XbergBridge API, there is
  // no createDocumentExtractorDartImpl factory to construct a Dart-based
  // extractor implementation.
  //
  // Built-in extractors are registered automatically when the library
  // initializes. Custom extractors must be written in Rust and linked into
  // a Rust shim crate before the Dart host process loads the dynamic library.
}
```
