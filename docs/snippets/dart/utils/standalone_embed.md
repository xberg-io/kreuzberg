```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  const config = EmbeddingConfig(
    model: EmbeddingModelType.preset(name: 'balanced'),
    normalize: true,
    batchSize: 32,
    showDownloadProgress: false,
  );

  final texts = <String>['Hello, world!', 'Xberg is fast'];
  final embeddings = await XbergBridge.embedTexts(texts, config);

  print('Vectors: ${embeddings.length}');
  print('Dimensions: ${embeddings.first.length}');
}
```
