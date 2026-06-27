```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'document.pdf'), config: ExtractionConfig());

  final pages = result.metadata.pages;
  if (pages == null) {
    print('No page structure available');
    return;
  }

  final boundaries = pages.boundaries;
  if (boundaries == null || boundaries.isEmpty) {
    print('No page boundaries available');
    return;
  }

  final content = result.results[0].content;
  for (final boundary in boundaries.take(3)) {
    final start = boundary.byteStart.toInt();
    final end = boundary.byteEnd.toInt();
    final pageText = content.substring(start, end);
    final previewEnd = pageText.length < 100 ? pageText.length : 100;

    print('Page ${boundary.pageNumber}:');
    print('  Byte range: $start-$end');
    print('  Preview: ${pageText.substring(0, previewEnd)}...');
  }
}
```
