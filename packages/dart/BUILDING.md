# Building kreuzberg Dart bindings

## Prerequisites

Install the flutter_rust_bridge codegen tool (one-time setup):

```sh
cargo install flutter_rust_bridge_codegen
```

## Build steps

1. Build the Rust binding crate:

   ```sh
   cargo build -p kreuzberg-dart
   ```

2. Run the FRB codegen to generate Dart bridge files:

   ```sh
   flutter_rust_bridge_codegen generate
   ```

   Alternatively, use alef which runs this step automatically via the configured
   post-build hook:

   ```sh
   alef build --lang=dart
   ```

3. Fetch Dart dependencies and run the test suite:

   ```sh
   dart pub get
   dart test
   ```
