#' @keywords internal
"_PACKAGE"

#' @useDynLib kreuzberg, .registration = TRUE
NULL

# Null-coalescing operator for R < 4.4 compatibility.
# In R >= 4.4 this is available in base, but we need to support R >= 4.2.
`%||%` <- function(x, y) {
  if (is.null(x)) y else x
}
