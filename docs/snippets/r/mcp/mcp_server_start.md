```r title="R"
# The xberg R bindings do not embed an MCP server: MCP is provided by the
# xberg CLI (Rust binary). Spawn it from the same R session that uses the
# xberg package for in-process extraction.
status <- system2("xberg", args = "mcp", stdout = "", stderr = "")
if (status != 0L) {
  stop(sprintf("MCP server exited with status %d", status))
}
```

<!-- snippet:syntax-only --> The R bindings expose extraction primitives only; MCP transport requires the standalone xberg CLI.
