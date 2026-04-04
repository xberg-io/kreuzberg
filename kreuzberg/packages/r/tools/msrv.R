# Minimum Supported Rust Version check
msrv <- "1.91.0"

check_rust_version <- function() {
  tryCatch({
    version_output <- system2("rustc", "--version", stdout = TRUE)
    version_string <- sub("^rustc (\\S+).*$", "\\1", version_output)
    if (utils::compareVersion(version_string, msrv) < 0) {
      stop(sprintf("Rust version %s is required, but %s was found. Please update Rust.", msrv, version_string))
    }
    message(sprintf("Rust version %s >= %s (OK)", version_string, msrv))
  }, error = function(e) {
    stop("Rust compiler (rustc) not found. Please install Rust: https://rustup.rs")
  })
}

check_rust_version()
