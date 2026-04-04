# Extracted from test-contract.R:17

# test -------------------------------------------------------------------------
result <- run_fixture_with_method(
  "api_batch_bytes_async",
  "pdf/fake_memo.pdf",
  NULL,
  "batch_async",
  "bytes",
  requirements = character(0),
  notes = NULL,
  skip_if_missing = TRUE
)
