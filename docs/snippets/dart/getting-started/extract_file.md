```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  final result = await XbergBridge.extractFile('document.pdf', null);

  print(result.content);
  print('MIME type: ${result.mimeType}');
  print('Tables: ${result.tables.length}');
}
```
