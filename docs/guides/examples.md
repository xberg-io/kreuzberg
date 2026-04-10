# Examples

Kreuzberg ships runnable end-to-end test suites under `e2e/` that double as reference implementations for embedding the library in each supported language. They cover the same extraction surface (PDF, DOCX, images, archives, embeddings, structured extraction) and run against the published bindings, so they show what real client code looks like — including setup, error handling, and config plumbing.

To run any of them, clone the repository and run the corresponding `task` target. Each suite has both a generation step (which regenerates skeletons from the canonical Rust definitions) and a test step that executes against installed bindings.

## Native bindings

| Language | Directory | Run with |
|----------|-----------|----------|
| Python | `e2e/python/` | `task python:e2e:test` |
| TypeScript / Node.js | `e2e/typescript/` | `task node:e2e:test` |
| Rust | `e2e/rust/` | `task rust:e2e:test` |
| Go | `e2e/go/` | `task go:e2e:test` |
| Java | `e2e/java/` | `task java:e2e:test` |
| .NET | `e2e/csharp/` | `task csharp:e2e:test` |
| Ruby | `e2e/ruby/` | `task ruby:e2e:test` |
| PHP | `e2e/php/` | `task php:e2e:test` |
| Elixir | `e2e/elixir/` | `task elixir:e2e:test` |
| R | `e2e/r/` | `task r:e2e:test` |
| C | `e2e/c/` | `task c:e2e:test` |

Each directory contains per-feature spec files (for example `pdf.spec.ts`, `email.spec.ts`, `structured.spec.ts`, `embeddings.spec.ts`) that you can read top-to-bottom to see how the binding is invoked, what it returns, and how options like `force_ocr`, chunking, and `ContentFilterConfig` are wired through.

For language-specific setup requirements (toolchain versions, dependencies), see the [Contributing guide](../contributing.md#prerequisites).

## WebAssembly

The `e2e/wasm/` directory holds vitest specs that exercise the kreuzberg WebAssembly module from Node and Deno. They're the closest thing to a runnable browser/edge example, since the same module loads identically in any WASM host.

| Surface | Directory | Run with |
|---------|-----------|----------|
| Node WASM bindings | `e2e/wasm/` | `task wasm:e2e:workers:test` |
| Deno WASM bindings | `e2e/wasm-deno/` | `task wasm:e2e:deno:test` |

The spec files (`pdf.spec.ts`, `office.spec.ts`, `structured.spec.ts`, `embeddings.spec.ts`, etc.) are the canonical references for embedding kreuzberg in browser, Cloudflare Workers, and other WASM-only environments. See the [WebAssembly API reference](../reference/api-wasm.md) for the JavaScript surface they call into, and the [TypeScript API reference](../reference/api-typescript.md) for the equivalent native binding.

## Running everything

To regenerate and run every language suite at once:

```bash title="Terminal"
task e2e:generate:all
task e2e:test:all
```

These targets are also what `ci.yaml` runs in the contributor pipeline, so a passing local run is a good signal that your change is mergeable.
