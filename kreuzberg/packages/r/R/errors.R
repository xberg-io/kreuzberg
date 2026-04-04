#' Kreuzberg error condition
#'
#' @param message Error message.
#' @param class Additional error class (e.g., "ValidationError", "ParsingError").
#' @param call The call that triggered the error.
#' @return A condition object.
#' @keywords internal
kreuzberg_error <- function(message, class = NULL, call = NULL) {
  structure(
    class = c(class, "kreuzberg_error", "error", "condition"),
    list(message = message, call = call)
  )
}

#' Check if a native result is an extendr error condition and raise it
#'
#' When the extendr `result_condition` feature is enabled, Rust errors are
#' returned as R objects with class "extendr_error" rather than being raised.
#' This function detects those conditions and re-raises them as proper R errors.
#'
#' Typed error prefixes (e.g., `[ValidationError]`) from the Rust side are
#' parsed to create specific R condition classes, enabling targeted error
#' handling with `tryCatch()`.
#'
#' @param result The result from a native function call.
#' @return The result if not an error; otherwise stops with a typed condition.
#' @keywords internal
check_native_result <- function(result) {
  if (inherits(result, "extendr_error")) {
    msg <- if (!is.null(result$value)) {
      as.character(result$value)
    } else {
      result$message
    }
    # Parse typed error prefix from Rust kreuzberg_error()
    m <- regmatches(msg, regexpr("^\\[([A-Za-z]+)\\]", msg))
    if (length(m) > 0 && nchar(m) > 0) {
      error_class <- sub("^\\[|\\]$", "", m)
      clean_msg <- trimws(sub("^\\[[A-Za-z]+\\]\\s*", "", msg))
      cond <- kreuzberg_error(clean_msg, class = error_class)
      stop(cond)
    }
    stop(kreuzberg_error(msg))
  }
  result
}
