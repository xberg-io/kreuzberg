```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  try {
    final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'document.pdf'), config: ExtractionConfig());
    print(result.results[0].content);
  } on Exception catch (e) {
    // flutter_rust_bridge converts every XbergError variant
    // (Io / UnsupportedFormat / Parsing / MissingDependency, ...)
    // into a Dart exception whose message preserves the original context.
    print('Extraction failed: $e');
  }
}
```
