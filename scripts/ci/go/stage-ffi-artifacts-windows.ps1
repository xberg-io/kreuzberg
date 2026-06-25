#!/usr/bin/env pwsh
# Usage: stage-ffi-artifacts-windows.ps1 [StagingDir]
# Example: stage-ffi-artifacts-windows.ps1 "artifact-staging/xberg-ffi"
#
# Stages FFI artifacts (Windows MinGW) for packaging into distribution tarball.
# Copies compiled DLLs, import libraries, headers, and pkg-config files.

param(
    [string]$StagingDir = "artifact-staging/xberg-ffi"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$TargetDir = "target\x86_64-pc-windows-gnu\release"

Write-Host "=== Staging FFI artifacts to $StagingDir ==="

# Stage static library (.a) - required for Go static linking
$StaticLib = "$TargetDir\libxberg_ffi.a"
if (Test-Path $StaticLib) {
    $StaticLibSize = (Get-Item $StaticLib).Length / 1MB
    Copy-Item $StaticLib "$StagingDir\lib\"
    Write-Host "✓ Staged static library: libxberg_ffi.a ($([math]::Round($StaticLibSize, 1))MB)"
} else {
    Write-Error "ERROR: Static library not found: $StaticLib"
    exit 1
}

# Stage dynamic library (.dll) - optional for runtime linking
if (Test-Path "$TargetDir\xberg_ffi.dll") {
    Copy-Item "$TargetDir\xberg_ffi.dll" "$StagingDir\lib\"
    Write-Host "✓ Staged FFI library: xberg_ffi.dll"
}

# Copy import libraries (for dynamic linking)
$ImportLibs = @(Get-ChildItem "$TargetDir\*.dll.a" -ErrorAction SilentlyContinue)
if ($ImportLibs.Count -gt 0) {
    Copy-Item "$TargetDir\*.dll.a" "$StagingDir\lib\"
    Write-Host "✓ Staged import libraries: $($ImportLibs.Count) files"
}

# Copy PDFium (optional, bundled during build)
if (Test-Path "$TargetDir\pdfium.dll") {
    Copy-Item "$TargetDir\pdfium.dll" "$StagingDir\lib\"
    Write-Host "✓ Staged PDFium library"
}

# Copy header
Copy-Item "crates\xberg-ffi\include\xberg.h" "$StagingDir\include\"

# Generate pkg-config file inline (the .pc is gitignored because it carries the version).
$cargoToml = Get-Content "crates\xberg-ffi\Cargo.toml"
$ffiVersion = ($cargoToml | Select-String '^version').Line.Split('"')[1]
@"
prefix=/usr/local
exec_prefix=`${prefix}
libdir=`${exec_prefix}/lib
includedir=`${prefix}/include

Name: xberg-ffi
Description: C FFI bindings for Xberg document intelligence library
Version: $ffiVersion
URL: https://xberg.io
Libs: -L`${libdir} -lxberg_ffi
Cflags: -I`${includedir}
"@ | Set-Content "$StagingDir\share\pkgconfig\xberg-ffi.pc"

Write-Host "✓ FFI artifacts staged successfully"
