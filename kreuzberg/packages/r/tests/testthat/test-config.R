test_that("extraction_config creates valid config", {
  config <- extraction_config()
  expect_true(is.list(config))

  config <- extraction_config(force_ocr = TRUE)
  expect_true(config$force_ocr)

  config <- extraction_config(
    ocr = ocr_config(backend = "tesseract", language = "deu"),
    chunking = chunking_config(max_characters = 500L)
  )
  expect_equal(config$ocr$backend, "tesseract")
  expect_equal(config$ocr$language, "deu")
  expect_equal(config$chunking$max_characters, 500L)
})

test_that("ocr_config creates valid config", {
  config <- ocr_config()
  expect_equal(config$backend, "tesseract")
  expect_equal(config$language, "eng")

  config <- ocr_config(backend = "paddle-ocr", language = "chi_sim", dpi = 300L)
  expect_equal(config$backend, "paddle-ocr")
  expect_equal(config$dpi, 300L)
})

test_that("chunking_config creates valid config", {
  config <- chunking_config()
  expect_equal(config$max_characters, 1000L)
  expect_equal(config$overlap, 200L)

  config <- chunking_config(max_characters = 2000L, overlap = 100L)
  expect_equal(config$max_characters, 2000L)
  expect_equal(config$overlap, 100L)
})

test_that("extraction_config serializes to JSON", {
  config <- extraction_config(force_ocr = TRUE, output_format = "markdown")
  json <- jsonlite::toJSON(config, auto_unbox = TRUE)
  parsed <- jsonlite::fromJSON(json)
  expect_true(parsed$force_ocr)
  expect_equal(parsed$output_format, "markdown")
})

test_that("config builders accept extra arguments", {
  config <- extraction_config(custom_field = "value")
  expect_equal(config$custom_field, "value")

  config <- ocr_config(custom_option = TRUE)
  expect_true(config$custom_option)

  config <- chunking_config(strategy = "semantic")
  expect_equal(config$strategy, "semantic")
})

# --- Input validation tests ---

test_that("ocr_config validates dpi is positive", {
  expect_error(ocr_config(dpi = -100), "dpi must be a positive")
  expect_error(ocr_config(dpi = 0), "dpi must be a positive")
})

test_that("chunking_config validates max_characters is positive", {
  expect_error(chunking_config(max_characters = -1), "max_characters must be a positive")
  expect_error(chunking_config(max_characters = 0), "max_characters must be a positive")
})

test_that("chunking_config validates overlap is non-negative", {
  expect_error(chunking_config(overlap = -1), "overlap must be non-negative")
})

test_that("ocr_config validates backend is character", {
  expect_error(ocr_config(backend = 123))
})

test_that("from_file validates path argument", {
  expect_error(from_file(123))
  expect_error(from_file(c("a", "b")))
})
