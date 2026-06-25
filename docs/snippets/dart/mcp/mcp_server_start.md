<!-- snippet:syntax-only -->

```dart title="Dart"
import 'dart:io';

Future<void> main() async {
  final process = await Process.start(
    'xberg',
    <String>['mcp'],
    mode: ProcessStartMode.inheritStdio,
  );
  final exitCode = await process.exitCode;
  exit(exitCode);
}
```
