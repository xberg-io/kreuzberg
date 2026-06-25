```elixir title="Elixir"
# List all registered document extractors
{:ok, extractors} = Xberg.list_document_extractors()
IO.inspect(extractors, label: "Document extractors")

# List all registered OCR backends
{:ok, backends} = Xberg.list_ocr_backends()
IO.inspect(backends, label: "OCR backends")

# List all registered post-processors
{:ok, processors} = Xberg.list_post_processors()
IO.inspect(processors, label: "Post-processors")

# List all registered validators
{:ok, validators} = Xberg.list_validators()
IO.inspect(validators, label: "Validators")
```
