//go:build windows

package kreuzberg

/*
// Kreuzberg FFI - CGO Configuration (Windows)
//
// Windows-specific CGO configuration that avoids using ${SRCDIR} in CFLAGS.
// On Windows, ${SRCDIR} expands to paths like "C:\Users\..." where the colon
// causes CGO to fail with "malformed #cgo argument" errors.
//
// Library linking is configured via:
//   - CI: CGO_CFLAGS and CGO_LDFLAGS environment variables set by setup-go-cgo-env action
//   - Development: Use -tags kreuzberg_dev for monorepo builds
//   - Production: Run go generate to download FFI and generate cgo_flags.go
//
// IMPORTANT: On Windows, you must set CGO_CFLAGS environment variable to include
// the path to internal/ffi directory, e.g.:
//   set CGO_CFLAGS=-I/c/path/to/packages/go/v4/internal/ffi
// The path should use forward slashes and MSYS2-style drive letters (/c/ instead of C:/)

#include "internal/ffi/kreuzberg.h"
#include <stdlib.h>
#include <stdint.h>
*/
import "C"
