//go:build !kreuzberg_dev
// +build !kreuzberg_dev

//go:generate go run github.com/kreuzberg-dev/kreuzberg/packages/go/v4/cmd/install@latest

// Package kreuzberg provides a high-performance document intelligence library for Go.
//
// The go:generate directive above downloads the FFI library for your platform
// and generates the CGO flags needed to build. Run it once after installing:
//
//	go generate github.com/kreuzberg-dev/kreuzberg/packages/go/v4
//
// This eliminates the need to manually set CGO_CFLAGS and CGO_LDFLAGS environment variables.
package kreuzberg
