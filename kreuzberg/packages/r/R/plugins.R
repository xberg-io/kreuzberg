# --- OCR Backend Plugins ---

#' Register a custom OCR backend
#'
#' @param name Character string naming the backend.
#' @param callback Callback object for OCR processing.
#' @return Invisible NULL on success.
#' @export
register_ocr_backend <- function(name, callback) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(register_ocr_backend_native(name, callback))
}

#' Unregister a custom OCR backend
#'
#' @param name Character string naming the backend to remove.
#' @return Invisible NULL on success.
#' @export
unregister_ocr_backend <- function(name) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(unregister_ocr_backend_native(name))
}

#' List registered OCR backends
#'
#' @return Character vector of registered backend names.
#' @export
list_ocr_backends <- function() {
  check_native_result(list_ocr_backends_native())
}

#' Clear all registered OCR backends
#'
#' @return Invisible NULL on success.
#' @export
clear_ocr_backends <- function() {
  check_native_result(clear_ocr_backends_native())
}

# --- Post-Processor Plugins ---

#' Register a custom post-processor
#'
#' @param name Character string naming the post-processor.
#' @param callback Callback object for post-processing.
#' @return Invisible NULL on success.
#' @export
register_post_processor <- function(name, callback) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(register_post_processor_native(name, callback))
}

#' Unregister a custom post-processor
#'
#' @param name Character string naming the post-processor to remove.
#' @return Invisible NULL on success.
#' @export
unregister_post_processor <- function(name) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(unregister_post_processor_native(name))
}

#' List registered post-processors
#'
#' @return Character vector of registered post-processor names.
#' @export
list_post_processors <- function() {
  check_native_result(list_post_processors_native())
}

#' Clear all registered post-processors
#'
#' @return Invisible NULL on success.
#' @export
clear_post_processors <- function() {
  check_native_result(clear_post_processors_native())
}

# --- Validator Plugins ---

#' Register a custom validator
#'
#' @param name Character string naming the validator.
#' @param callback Callback object for validation.
#' @return Invisible NULL on success.
#' @export
register_validator <- function(name, callback) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(register_validator_native(name, callback))
}

#' Unregister a custom validator
#'
#' @param name Character string naming the validator to remove.
#' @return Invisible NULL on success.
#' @export
unregister_validator <- function(name) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(unregister_validator_native(name))
}

#' List registered validators
#'
#' @return Character vector of registered validator names.
#' @export
list_validators <- function() {
  check_native_result(list_validators_native())
}

#' Clear all registered validators
#'
#' @return Invisible NULL on success.
#' @export
clear_validators <- function() {
  check_native_result(clear_validators_native())
}

# --- Document Extractor Plugins ---

#' List registered document extractors
#'
#' @return Character vector of registered extractor names.
#' @export
list_document_extractors <- function() {
  check_native_result(list_document_extractors_native())
}

#' Unregister a custom document extractor
#'
#' @param name Character string naming the extractor to remove.
#' @return Invisible NULL on success.
#' @export
unregister_document_extractor <- function(name) {
  stopifnot(is.character(name), length(name) == 1L)
  check_native_result(unregister_document_extractor_native(name))
}

#' Clear all registered document extractors
#'
#' @return Invisible NULL on success.
#' @export
clear_document_extractors <- function() {
  check_native_result(clear_document_extractors_native())
}
