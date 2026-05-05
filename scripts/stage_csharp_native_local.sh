#!/usr/bin/env bash
# Stage libkreuzberg_ffi into packages/csharp/Kreuzberg/runtimes/<rid>/native/
# so dotnet test can locate it via runtime asset resolution.
#
# Auto-detects host RID. Idempotent.

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

case "$(uname -s)" in
Darwin)
  ext=dylib
  case "$(uname -m)" in
  arm64 | aarch64) rid=osx-arm64 ;;
  *) rid=osx-x64 ;;
  esac
  ;;
Linux)
  ext=so
  case "$(uname -m)" in
  aarch64 | arm64) rid=linux-arm64 ;;
  *) rid=linux-x64 ;;
  esac
  ;;
MINGW* | MSYS* | CYGWIN*)
  ext=dll
  rid=win-x64
  ;;
*)
  echo "Unsupported platform: $(uname -s)" >&2
  exit 1
  ;;
esac

src="target/release/libkreuzberg_ffi.${ext}"
if [ "$ext" = "dll" ]; then
  src="target/release/kreuzberg_ffi.${ext}"
fi

if [ ! -f "$src" ]; then
  echo "ERROR: $src not found. Run: cargo build --release -p kreuzberg-ffi" >&2
  exit 1
fi

dst_dir="packages/csharp/Kreuzberg/runtimes/${rid}/native"
mkdir -p "$dst_dir"
cp -f "$src" "$dst_dir/"

echo "Staged $(basename "$src") -> $dst_dir/"
