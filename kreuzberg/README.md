# Kreuzberg

This directory contains the Kreuzberg library source code.

See the [root README](../README.md) for documentation, installation guides, and project overview.

## Structure

```
crates/          # Rust crates (core library, CLI, FFI, language bindings)
packages/        # Language-specific packages (Python, Go, Java, C#, Ruby, PHP, Elixir, R)
e2e/             # End-to-end tests across all languages
tools/           # Development tools (benchmark harness, e2e generator, snippet runner)
scripts/         # Build, publish, and CI scripts
docker/          # Dockerfiles (core, full, CLI, musl variants)
fixtures/        # Test fixtures (JSON contract tests)
skills/          # MCP server skills
```
