# Extracted from test-office.R:252

# test -------------------------------------------------------------------------
result <- run_fixture(
  "office_mdx_basic",
  "markdown/sample.mdx",
  NULL,
  requirements = character(0),
  notes = NULL,
  skip_if_missing = TRUE
)
