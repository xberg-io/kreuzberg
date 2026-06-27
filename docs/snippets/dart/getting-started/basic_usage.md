```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  // Default ExtractionConfig — flutter_rust_bridge surfaces every call
  // as a Future, so even non-async-flavored entrypoints must be awaited.
  final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'document.pdf'), config: ExtractionConfig());

  print(result.results[0].content);
  print('MIME type: ${result.results[0].mimeType}');
}
```
