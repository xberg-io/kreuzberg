```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  print('Hello from xberg!');
  final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'document.pdf'), config: ExtractionConfig());
  print(result.results[0].content);
}
```
