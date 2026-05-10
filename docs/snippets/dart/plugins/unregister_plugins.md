```dart title="Dart"
import 'package:kreuzberg/kreuzberg.dart';

Future<void> main() async {
  // Custom-plugin construction (createXxxDartImpl) is unreachable from Dart
  // due to opaque BoxFn closure types in the flutter_rust_bridge binding,
  // so this snippet exercises the lifecycle against the *built-in* renderer
  // registry (markdown / html / djot / plain).

  var renderers = await KreuzbergBridge.listRenderers();
  print('Renderers before unregister: $renderers');

  // Unregister a single renderer by name.
  await KreuzbergBridge.unregisterRenderer('plain');
  renderers = await KreuzbergBridge.listRenderers();
  print('Renderers after unregister: $renderers');

  // Bulk-clear all renderers (including remaining built-ins).
  await KreuzbergBridge.clearRenderers();
  renderers = await KreuzbergBridge.listRenderers();
  print('Renderers after clear: $renderers');
}
```
