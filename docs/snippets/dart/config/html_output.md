```dart title="Dart"
import 'package:xberg/xberg.dart';

Future<void> main() async {
  final config = ExtractionConfig(
    useCache: true,
    enableQualityProcessing: true,
    forceOcr: false,
    disableOcr: false,
    htmlOutput: const HtmlOutputConfig(
      theme: HtmlTheme.gitHub,
      classPrefix: 'kb-',
      embedCss: true,
    ),
    resultFormat: ResultFormat.unified,
    outputFormat: OutputFormat.html(),
    includeDocumentStructure: false,
    maxArchiveDepth: 3,
    useLayoutForMarkdown: false,
  );

  final result = await XbergBridge.extract(ExtractInput(kind: ExtractInputKind.uri, uri: 'document.pdf'), config: config);
  print(result.results[0].content);
}
```
