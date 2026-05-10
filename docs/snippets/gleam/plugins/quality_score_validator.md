<!-- snippet:skip reason="Gleam targets Erlang/JS and inherits the BEAM actor-model boundary; Rustler-style trait callbacks are not feasible from a pure Gleam process. Custom plugins must be implemented in Rust." -->

```gleam title="Gleam"
import kreuzberg

// Note: the Gleam binding does not expose a Gleam-implementable
// `Validator` trait. A quality-score validator that inspects
// `Metadata.quality_score` and rejects results below a threshold must be
// implemented in Rust against `kreuzberg::plugins::Validator` and
// registered from the host Rust binary that loads the kreuzberg NIF.
//
// `kreuzberg.register_validator` exists but only accepts an Erlang PID
// whose owning GenServer answers `{:trait_call, ...}` messages — that
// callback module is wired on the Elixir/Rustler side per the kreuzberg
// Gleam module docs, not from Gleam directly.
pub fn main() {
  Nil
}
```
