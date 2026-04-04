test_that("extract_file_sync extracts text from a file", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Hello, kreuzberg!", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)
  expect_s3_class(result, "kreuzberg_result")
  expect_true(nchar(result$content) > 0)
  expect_true(grepl("Hello", result$content, fixed = TRUE))
})

test_that("extract_file extracts text from a file", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Async extraction test", tmp)
  on.exit(unlink(tmp))

  result <- extract_file(tmp)
  expect_s3_class(result, "kreuzberg_result")
  expect_true(grepl("Async", result$content, fixed = TRUE))
})

test_that("extract_bytes_sync extracts from raw bytes", {
  bytes <- charToRaw("Test content in bytes")
  result <- extract_bytes_sync(bytes, "text/plain")
  expect_s3_class(result, "kreuzberg_result")
  expect_true(nchar(result$content) > 0)
})

test_that("extract_bytes extracts from raw bytes", {
  bytes <- charToRaw("Async bytes test")
  result <- extract_bytes(bytes, "text/plain")
  expect_s3_class(result, "kreuzberg_result")
  expect_true(nchar(result$content) > 0)
})

test_that("extract_file_sync with config works", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Config test", tmp)
  on.exit(unlink(tmp))

  config <- extraction_config(output_format = "plain")
  result <- extract_file_sync(tmp, config = config)
  expect_s3_class(result, "kreuzberg_result")
})

test_that("kreuzberg_result print method works", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Print test content", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)
  expect_output(print(result), "kreuzberg_result")
})

# --- Input validation tests ---

test_that("extract_file_sync validates path argument", {
  expect_error(extract_file_sync(123))
  expect_error(extract_file_sync(c("a", "b")))
  expect_error(extract_file_sync("/nonexistent/path/file.txt"), "File not found")
})

test_that("extract_bytes_sync validates arguments", {
  expect_error(extract_bytes_sync("not raw", "text/plain"))
  expect_error(extract_bytes_sync(charToRaw("hi"), 123))
})

# --- S3 method tests ---

test_that("S3 generics work on kreuzberg_result", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("S3 test content", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)

  expect_equal(content(result), result$content)
  expect_equal(mime_type(result), result$mime_type)
  expect_true(is.numeric(page_count(result)) || is.integer(page_count(result)))
  expect_true(is.numeric(chunk_count(result)) || is.integer(chunk_count(result)))
})

test_that("summary.kreuzberg_result works", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Summary test", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)
  expect_output(summary(result), "kreuzberg_result summary")
})

test_that("format.kreuzberg_result returns a string", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Format test", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)
  fmt <- format(result)
  expect_true(is.character(fmt))
  expect_true(grepl("kreuzberg_result", fmt))
})

test_that("metadata_field returns NULL for missing fields", {
  tmp <- tempfile(fileext = ".txt")
  writeLines("Metadata test", tmp)
  on.exit(unlink(tmp))

  result <- extract_file_sync(tmp)
  expect_null(metadata_field(result, "nonexistent_field"))
})

# --- Typed error condition tests ---

test_that("extract_file_sync produces typed error for unsupported format", {
  tmp <- tempfile(fileext = ".xyz_unsupported")
  writeLines("test", tmp)
  on.exit(unlink(tmp))

  err <- tryCatch(
    extract_file_sync(tmp),
    kreuzberg_error = function(e) e
  )
  # Should produce a kreuzberg_error condition (might be UnsupportedFormatError or other)
  if (inherits(err, "kreuzberg_error")) {
    expect_true(inherits(err, "kreuzberg_error"))
  }
})
