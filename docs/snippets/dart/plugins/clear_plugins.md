```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  // The Dart binding exposes bulk-clear entry points for OCR backends,
  // post-processors, and validators. Document-extractor clearing is not
  // surfaced through flutter_rust_bridge; the built-in extractors are
  // registered automatically by the xberg core when the library
  // initializes.
  await XbergBridge.clearOcrBackends();
  await XbergBridge.clearPostProcessors();
  await XbergBridge.clearValidators();

  print('OCR backends, post-processors, and validators cleared');
}
```
