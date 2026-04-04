# Hand-written binding-specific edge case tests for PDF rendering.
# Happy-path render tests are auto-generated from fixtures in e2e/.
# These tests cover error handling, validation, and lifecycle patterns
# that vary per language and can't be generated uniformly.

test_that("rendering functions are available", {
  expect_true(exists("render_pdf_page"))
  expect_true(exists("render_pdf_pages_iter"))
  expect_true(is.function(render_pdf_page))
  expect_true(is.function(render_pdf_pages_iter))
})

test_that("render_pdf_page errors on nonexistent file", {
  expect_error(render_pdf_page("/nonexistent/path/to/document.pdf", 0L), "not found|No such file")
})

test_that("render_pdf_page errors on out-of-bounds page index", {
  repo_root <- normalizePath(file.path(getwd(), "..", "..", "..", ".."), mustWork = FALSE)
  pdf_path <- file.path(repo_root, "test_documents", "pdf", "tiny.pdf")

  if (!file.exists(pdf_path)) {
    skip(paste("Test PDF not found at", pdf_path))
  }

  expect_error(render_pdf_page(pdf_path, 9999L))
})

test_that("render_pdf_page errors on negative page index", {
  repo_root <- normalizePath(file.path(getwd(), "..", "..", "..", ".."), mustWork = FALSE)
  pdf_path <- file.path(repo_root, "test_documents", "pdf", "tiny.pdf")

  if (!file.exists(pdf_path)) {
    skip(paste("Test PDF not found at", pdf_path))
  }

  expect_error(render_pdf_page(pdf_path, -1L), "negative")
})

test_that("render_pdf_pages_iter errors on nonexistent file", {
  expect_error(
    render_pdf_pages_iter("/nonexistent/path/to/document.pdf", callback = function(i, d) {}),
    "not found|No such file"
  )
})

test_that("render_pdf_page errors on empty path", {
  expect_error(render_pdf_page("", 0L))
})

test_that("render_pdf_pages_iter handles cleanup without consuming", {
  repo_root <- normalizePath(file.path(getwd(), "..", "..", "..", ".."), mustWork = FALSE)
  pdf_path <- file.path(repo_root, "test_documents", "pdf", "tiny.pdf")

  if (!file.exists(pdf_path)) {
    skip(paste("Test PDF not found at", pdf_path))
  }

  # Create iterator via callback, return FALSE to stop immediately
  called <- FALSE
  render_pdf_pages_iter(pdf_path, callback = function(i, d) {
    called <<- TRUE
    return(FALSE)
  })
  expect_true(called, "callback should have been called at least once")
})

test_that("render_pdf_pages_iter supports early termination with valid PNG", {
  repo_root <- normalizePath(file.path(getwd(), "..", "..", "..", ".."), mustWork = FALSE)
  pdf_path <- file.path(repo_root, "test_documents", "pdf", "tiny.pdf")

  if (!file.exists(pdf_path)) {
    skip(paste("Test PDF not found at", pdf_path))
  }

  first_png <- NULL
  render_pdf_pages_iter(pdf_path, callback = function(i, d) {
    first_png <<- d
    return(FALSE) # Stop after first page
  })
  expect_true(!is.null(first_png), "should have received first page")
  expect_true(length(first_png) > 8, "PNG data should be longer than 8 bytes")
  # PNG magic bytes: 89 50 4E 47
  expect_equal(first_png[1:4], as.raw(c(0x89, 0x50, 0x4e, 0x47)))
})
