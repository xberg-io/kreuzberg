```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  final ocr = OcrConfig(
    enabled: true,
    backend: 'tesseract',
    language: 'eng',
    autoRotate: false,
  );

  final config = ExtractionConfig(
    useCache: true,
    enableQualityProcessing: true,
    forceOcr: true,
    disableOcr: false,
    resultFormat: ResultFormat.unified,
    outputFormat: OutputFormat.plain(),
    includeDocumentStructure: false,
    maxArchiveDepth: 3,
    useLayoutForMarkdown: false,    ocr: ocr,
  );

  final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'scanned.pdf'), config: config);
  print(result.results[0].content);
  print('Detected languages: ${result.detectedLanguages}');
}
```
