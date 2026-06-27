```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'document.pdf'), config: ExtractionConfig());

  print(result.results[0].content);

  for (final table in result.results[0].tables) {
    print('Table: $table');
  }

  final chunks = result.results[0].chunks;
  if (chunks != null) {
    for (final chunk in chunks) {
      print('Chunk: $chunk');
    }
  }
}
```
