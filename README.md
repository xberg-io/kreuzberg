# Kreuzberg

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://github.com/xberg-io/alef">
    <img src="https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6" alt="Bindings">
  </a>
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/kreuzberg">
    <img src="https://img.shields.io/crates/v/kreuzberg?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/kreuzberg/">
    <img src="https://img.shields.io/pypi/v/kreuzberg?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/node">
    <img src="https://img.shields.io/npm/v/@kreuzberg/node?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/kreuzberg">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/kreuzberg?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/tree/main/packages/go/v5">
    <img src="https://img.shields.io/github/v/tag/xberg-io/kreuzberg?label=Go&color=007ec6&filter=v5*" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Kreuzberg/">
    <img src="https://img.shields.io/nuget/v/Kreuzberg?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/xberg-io/kreuzberg">
    <img src="https://img.shields.io/packagist/v/xberg-io/kreuzberg?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/kreuzberg">
    <img src="https://img.shields.io/gem/v/kreuzberg?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/kreuzberg">
    <img src="https://img.shields.io/hexpm/v/kreuzberg?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://xberg-io.r-universe.dev/kreuzberg">
    <img src="https://img.shields.io/badge/R-kreuzberg-007ec6" alt="R">
  </a>
  <a href="https://pub.dev/packages/kreuzberg">
    <img src="https://img.shields.io/pub/v/kreuzberg?label=Dart&color=007ec6" alt="Dart">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/kreuzberg-android">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/kreuzberg-android?label=Kotlin&color=007ec6" alt="Kotlin">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-SPM-007ec6" alt="Swift">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-package-007ec6" alt="Zig">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/pkgs/container/kreuzberg">
    <img src="https://img.shields.io/badge/Docker-ghcr.io-007ec6?logo=docker&logoColor=white" alt="Docker">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/pkgs/container/charts%2Fkreuzberg">
    <img src="https://img.shields.io/badge/Helm-ghcr.io-007ec6?logo=helm&logoColor=white" alt="Helm">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/xberg-io/kreuzberg/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-Elastic--2.0-007ec6" alt="License">
  </a>
  <a href="https://docs.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-kreuzberg-007ec6" alt="Documentation">
  </a>
  <a href="https://huggingface.co/Kreuzberg">
    <img src="https://img.shields.io/badge/Hugging%20Face-Kreuzberg-007ec6" alt="Hugging Face">
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
  <a href="https://docs.kreuzberg.dev/demo.html">
    <img height="22" src="https://img.shields.io/badge/Live%20Demo-Open-007ec6?logo=webassembly&logoColor=white" alt="Live Demo">
  </a>
  <a href="https://github.com/xberg-io/kreuzberg/stargazers">
    <img height="22" src="https://img.shields.io/github/stars/xberg-io/kreuzberg?style=social" alt="GitHub Stars">
  </a>
</div>

Extract text, metadata, transcripts, and code intelligence from 96 file formats and 306 programming languages at native speeds without needing a GPU.

## What and Why?

Kreuzberg is a document-intelligence framework with a Rust core and native bindings for 16 languages. It turns documents, images, audio, and source code into clean, structured text — extracting tables, metadata, transcripts, and code intelligence from 96 file formats and 306 programming languages.

Modern AI and RAG pipelines need fast, reliable extraction without a GPU or a stack of heavyweight dependencies. Kreuzberg delivers that from a single Rust core: SIMD-accelerated parsing, pure-Rust PDF, streaming for multi-GB files, and consistent output across every binding. Run it as a library, CLI, REST API, or MCP server.

OCR (Tesseract, PaddleOCR, EasyOCR, and VLM across 143 vision providers), Whisper audio/video transcription, chunking, language detection, embeddings, and structured LLM extraction are all built in.

### Features

| Feature | Description |
| ------- | ----------- |
| **96 file formats** | PDF, Office, images, HTML/XML, email, archives, and academic formats across 8 categories |
| **306 languages** | Code intelligence — functions, classes, imports, symbols, docstrings — via tree-sitter |
| **Polyglot** | Native bindings for Rust, Python, Node.js, WebAssembly, Ruby, Go, Java, Kotlin, C#, PHP, Elixir, R, Dart, Swift, Zig, and C |
| **OCR** | Tesseract (incl. WASM), PaddleOCR, EasyOCR, and VLM OCR across 143 vision providers — extensible via plugins |
| **Transcription** | Whisper ONNX transcripts for MP3, M4A, WAV, WebM, and MP4 audio tracks |
| **LLM intelligence** | Structured JSON extraction, embeddings, and VLM OCR through [liter-llm](https://github.com/xberg-io/liter-llm), including local engines |
| **Deployment** | Use as a library, CLI tool, REST API server, or MCP server |
| **High performance** | Rust core with pure-Rust PDF, SIMD optimizations, full parallelism, and streaming for multi-GB files |
| **Token-efficient output** | TOON wire format uses ~30–50% fewer tokens than JSON for LLM/RAG pipelines |
| **Extensible** | Plugin system for custom OCR backends, validators, post-processors, extractors, and renderers |

### Supported Formats

96 file formats across 8 categories — Office documents, images (OCR-enabled), web and structured data, email, archives, academic, and audio/video — plus code intelligence for 306 programming languages. See the [format reference](https://docs.kreuzberg.dev/reference/formats/) for the complete list.

<div align="center">
  <a href="https://github.com/xberg-io/kreuzberg/stargazers">
    <img src="docs/assets/star.gif" alt="Star Kreuzberg on GitHub" width="640">
  </a>
</div>

<p align="center"><strong>⭐ Star this repo to show your support — it helps others discover Kreuzberg.</strong></p>

## Quick Start

### Language Packages

<details open>
<summary><strong>Python</strong></summary>

```sh
pip install kreuzberg
```

```sh
uv add kreuzberg
```

See [Python README](https://github.com/xberg-io/kreuzberg/tree/main/packages/python) for full documentation.

</details>

<details>
<summary><strong>Node.js</strong></summary>

```sh
npm install @kreuzberg/node
```

See [Node.js README](https://github.com/xberg-io/kreuzberg/tree/main/crates/kreuzberg-node) for full documentation.

</details>

<details>
<summary><strong>Rust</strong></summary>

```sh
cargo add kreuzberg
```

See [Rust README](https://github.com/xberg-io/kreuzberg/tree/main/crates/kreuzberg) for full documentation.

</details>

<details>
<summary><strong>Go</strong></summary>

```sh
go get github.com/xberg-io/kreuzberg/v5
```

See [Go README](https://github.com/xberg-io/kreuzberg/tree/main/packages/go/v5) for full documentation.

</details>

<details>
<summary><strong>Java</strong></summary>

Available on Maven Central as `dev.kreuzberg:kreuzberg`. See [Java README](https://github.com/xberg-io/kreuzberg/tree/main/packages/java) for the dependency snippet and current version.

</details>

<details>
<summary><strong>C#</strong></summary>

```sh
dotnet add package Kreuzberg
```

See [C# README](https://github.com/xberg-io/kreuzberg/tree/main/packages/csharp) for full documentation.

</details>

<details>
<summary><strong>Ruby</strong></summary>

```sh
gem install kreuzberg
```

See [Ruby README](https://github.com/xberg-io/kreuzberg/tree/main/packages/ruby) for full documentation.

</details>

<details>
<summary><strong>PHP</strong></summary>

```sh
composer require xberg-io/kreuzberg
```

See [PHP README](https://github.com/xberg-io/kreuzberg/tree/main/packages/php) for full documentation.

</details>

<details>
<summary><strong>Elixir</strong></summary>

Add `{:kreuzberg, "~> 5.0"}` to your `mix.exs` dependencies. See [Elixir README](https://github.com/xberg-io/kreuzberg/tree/main/packages/elixir) for full documentation.

</details>

<details>
<summary><strong>WebAssembly</strong></summary>

```sh
npm install @kreuzberg/wasm
```

See [WebAssembly README](https://github.com/xberg-io/kreuzberg/tree/main/crates/kreuzberg-wasm) for full documentation.

</details>

<details>
<summary><strong>R</strong></summary>

Install from r-universe. See [R README](https://github.com/xberg-io/kreuzberg/tree/main/packages/r) for full documentation.

</details>

<details>
<summary><strong>Kotlin (Android)</strong></summary>

Available on Maven Central as `dev.kreuzberg:kreuzberg-android`. See [Kotlin README](https://github.com/xberg-io/kreuzberg/tree/main/packages/kotlin-android) for the dependency snippet and current version.

</details>

<details>
<summary><strong>Swift</strong></summary>

Add via Swift Package Manager. See [Swift README](https://github.com/xberg-io/kreuzberg/tree/main/packages/swift) for full documentation.

</details>

<details>
<summary><strong>Dart / Flutter</strong></summary>

```sh
dart pub add kreuzberg
```

See [Dart README](https://github.com/xberg-io/kreuzberg/tree/main/packages/dart) for full documentation.

</details>

<details>
<summary><strong>Zig</strong></summary>

Add via `zig fetch`. See [Zig README](https://github.com/xberg-io/kreuzberg/tree/main/packages/zig) for full documentation.

</details>

<details>
<summary><strong>C/C++ (FFI)</strong></summary>

Build from source as part of this workspace. See [C (FFI) README](https://github.com/xberg-io/kreuzberg/tree/main/crates/kreuzberg-ffi) for full documentation.

</details>

<details>
<summary><strong>CLI</strong></summary>

```sh
brew install xberg-io/tap/kreuzberg
```

See [CLI usage](https://docs.kreuzberg.dev/cli/usage/) for full documentation.

</details>

<details>
<summary><strong>Docker</strong></summary>

```sh
docker pull ghcr.io/xberg-io/kreuzberg:latest
```

See [Docker guide](https://docs.kreuzberg.dev/guides/docker/) for API, CLI, and MCP server modes.

</details>

<details>
<summary><strong>MCP Server</strong></summary>

Run Kreuzberg as a [Model Context Protocol](https://modelcontextprotocol.io/) server. The prebuilt
binaries (Homebrew, `install.sh`, Docker) include it; from source, enable the `mcp` feature.

```sh
# Prebuilt (Homebrew / install.sh / Docker) — MCP is included
brew install xberg-io/tap/kreuzberg
kreuzberg mcp                                   # stdio (default)

# From source — enable the mcp feature
cargo install kreuzberg-cli --features mcp
kreuzberg mcp

# HTTP transport instead of stdio
kreuzberg mcp --transport http --host 127.0.0.1 --port 8001
```

Add it to an MCP client (Claude Desktop `claude_desktop_config.json`, Cursor `.cursor/mcp.json`):

```json
{
  "mcpServers": {
    "kreuzberg": { "command": "kreuzberg", "args": ["mcp"] }
  }
}
```

See the [MCP integration guide](https://docs.kreuzberg.dev/guides/mcp-integration/) for tools,
resources, prompts, HTTP transport, and configuration.

</details>

### AI Coding Assistants

Install the Kreuzberg plugin from the [`xberg-io/plugins`](https://github.com/xberg-io/plugins) marketplace. It ships the Kreuzberg agent skills (extraction APIs, OCR backends, configuration, language conventions) and works with every major coding agent — expand your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add xberg-io/plugins
/plugin install kreuzberg@kreuzberg
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/xberg-io/plugins
```

Then search for `kreuzberg` and select **Install Plugin**.

</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/xberg-io/plugins`, then select **kreuzberg**.

</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/xberg-io/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/xberg-io/plugins
droid plugin install kreuzberg@kreuzberg
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/xberg-io/plugins
copilot plugin install kreuzberg@kreuzberg
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@kreuzberg/opencode-kreuzberg"]
}
```

</details>

## Documentation

Full guides, API references for every binding, and the complete format and configuration reference live at **[kreuzberg.dev](https://kreuzberg.dev/)**. Try it in the browser with the [live demo](https://docs.kreuzberg.dev/demo.html).

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## Part of Kreuzberg.dev

- [Kreuzberg Cloud](https://github.com/xberg-io/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/xberg-io/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/xberg-io/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/xberg-io/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/xberg-io/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

## License

Elastic License 2.0 (ELv2) — see [LICENSE](LICENSE) for details. See [the Elastic License](https://www.elastic.co/licensing/elastic-license) for the full text.
