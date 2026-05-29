# kreuzberg

High-performance document intelligence library

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  kreuzberg: ^5.0.0-rc.3
```

Then run:

```sh
dart pub get
```

## Building

From the repository root:

```sh
cargo build -p kreuzberg-dart
flutter_rust_bridge_codegen generate
dart pub get
dart analyze
dart test
```

## License

Elastic-2.0
