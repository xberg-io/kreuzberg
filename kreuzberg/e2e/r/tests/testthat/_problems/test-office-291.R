# Extracted from test-office.R:291

# test -------------------------------------------------------------------------
result <- run_fixture(
  "office_mdx_using_mdx",
  "markdown/mdx_using_mdx.mdx",
  NULL,
  requirements = character(0),
  notes = NULL,
  skip_if_missing = TRUE
)
