dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([kreuzberg],
  [whether to enable the kreuzberg extension],
  [AS_HELP_STRING([--enable-kreuzberg],
    [Enable kreuzberg extension support])],
  [yes])

if test "$PHP_KREUZBERG_ENABLED" = "yes"; then
  dnl Register the extension directory so phpize creates modules/ and sets up build rules.
  PHP_NEW_EXTENSION(kreuzberg, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library and copy it to modules/.
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/kreuzberg-php/Cargo.toml"; then
      (cd crates/kreuzberg-php && cargo build --release) || exit 1

      dnl Detect output filename based on platform
      if test -f "crates/kreuzberg-php/target/release/libkreuzberg_php.dylib"; then
        cargo_lib="crates/kreuzberg-php/target/release/libkreuzberg_php.dylib"
      elif test -f "crates/kreuzberg-php/target/release/libkreuzberg_php.so"; then
        cargo_lib="crates/kreuzberg-php/target/release/libkreuzberg_php.so"
      else
        echo "ERROR: cargo build succeeded but .so/.dylib not found in crates/kreuzberg-php/target/release" >&2
        exit 1
      fi

      mkdir -p modules
      cp "$cargo_lib" "modules/kreuzberg.so" || exit 1
    else
      echo "ERROR: crates/kreuzberg-php/Cargo.toml not found" >&2
      exit 1
    fi
  ], [])
fi
