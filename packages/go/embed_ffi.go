//go:build ignore
// +build ignore

package xberg

import "embed"

// This file ensures that FFI header files and library artifacts are included
// when this module is vendored. The //go:embed directive tells Go to include
// the include/ and .lib/ directories when running `go mod vendor`.
//
// This file itself is not compiled (//go:build ignore), but its directives are
// processed by Go's module system to ensure all necessary files are present in
// vendored installations.

//go:embed include/*
//go:embed .lib/*
var _ embed.FS
