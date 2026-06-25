```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  final extractors = await XbergBridge.listDocumentExtractors();
  print('Registered extractors: $extractors');

  final processors = await XbergBridge.listPostProcessors();
  print('Registered processors: $processors');

  final backends = await XbergBridge.listOcrBackends();
  print('Registered OCR backends: $backends');

  final validators = await XbergBridge.listValidators();
  print('Registered validators: $validators');
}
```
