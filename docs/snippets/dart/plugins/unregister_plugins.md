```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  // Custom-plugin construction (createXxxDartImpl) is unreachable from Dart
  // due to opaque BoxFn closure types in the flutter_rust_bridge binding,
  // so this snippet exercises the lifecycle against the *built-in* renderer
  // registry (markdown / html / djot / plain).

  var renderers = await XbergBridge.listRenderers();
  print('Renderers before unregister: $renderers');

  // Unregister a single renderer by name.
  await XbergBridge.unregisterRenderer('plain');
  renderers = await XbergBridge.listRenderers();
  print('Renderers after unregister: $renderers');

  // Bulk-clear all renderers (including remaining built-ins).
  await XbergBridge.clearRenderers();
  renderers = await XbergBridge.listRenderers();
  print('Renderers after clear: $renderers');
}
```
