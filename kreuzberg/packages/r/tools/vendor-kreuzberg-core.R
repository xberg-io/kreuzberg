#!/usr/bin/env Rscript
# Vendor kreuzberg core crates into R package.
# R-based fallback when Python is not available.
#
# This script:
# 1. Reads workspace.dependencies from root Cargo.toml
# 2. Copies core crates to packages/r/vendor/
# 3. Replaces workspace = true with explicit versions
# 4. Generates vendor/Cargo.toml with proper workspace setup

repo_root <- Sys.getenv("REPO_ROOT", unset = NA)
if (is.na(repo_root) || repo_root == "") {
  # Try to guess from script location
  script_dir <- getwd()
  repo_root <- normalizePath(file.path(script_dir, "..", ".."), mustWork = FALSE)
}

cat("=== Vendoring kreuzberg core crate (R fallback) ===\n")

# --- Helper: read Cargo.toml workspace version ---
root_cargo <- file.path(repo_root, "Cargo.toml")
if (!file.exists(root_cargo)) {
  stop("Root Cargo.toml not found at: ", root_cargo)
}

cargo_lines <- readLines(root_cargo, warn = FALSE)
cargo_text <- paste(cargo_lines, collapse = "\n")

# Extract workspace version
version_match <- regmatches(
  cargo_text,
  regexpr('\\[workspace\\.package\\][^\\[]*version\\s*=\\s*"([^"]+)"', cargo_text, perl = TRUE)
)
core_version <- sub('.*version\\s*=\\s*"([^"]+)".*', "\\1", version_match)
cat("Core version:", core_version, "\n")

# Extract workspace dependencies block
# We'll do a simpler approach: read everything between [workspace.dependencies] and next [
ws_dep_start <- grep("^\\[workspace\\.dependencies\\]", cargo_lines)
if (length(ws_dep_start) == 0) {
  stop("Could not find [workspace.dependencies] in root Cargo.toml")
}

# Find next section header
next_section <- grep("^\\[", cargo_lines[(ws_dep_start + 1):length(cargo_lines)])[1]
if (is.na(next_section)) {
  ws_dep_end <- length(cargo_lines)
} else {
  ws_dep_end <- ws_dep_start + next_section - 1
}

ws_dep_lines <- cargo_lines[(ws_dep_start + 1):ws_dep_end]
ws_dep_lines <- ws_dep_lines[nzchar(trimws(ws_dep_lines))]
ws_dep_text <- paste(ws_dep_lines, collapse = "\n")

# --- Setup paths ---
vendor_base <- file.path(repo_root, "packages", "r", "vendor")
dir.create(vendor_base, recursive = TRUE, showWarnings = FALSE)

# --- Clean existing vendor crates ---
crate_names <- c("kreuzberg", "kreuzberg-ffi", "kreuzberg-tesseract",
                  "kreuzberg-paddle-ocr", "kreuzberg-pdfium-render")
for (name in crate_names) {
  p <- file.path(vendor_base, name)
  if (dir.exists(p)) unlink(p, recursive = TRUE)
}
vendor_cargo <- file.path(vendor_base, "Cargo.toml")
if (file.exists(vendor_cargo)) file.remove(vendor_cargo)
cat("Cleaned vendor crate directories\n")

# --- Copy crates ---
crates_to_copy <- list(
  c("crates/kreuzberg", "kreuzberg"),
  c("crates/kreuzberg-ffi", "kreuzberg-ffi"),
  c("crates/kreuzberg-tesseract", "kreuzberg-tesseract"),
  c("crates/kreuzberg-paddle-ocr", "kreuzberg-paddle-ocr"),
  c("crates/kreuzberg-pdfium-render", "kreuzberg-pdfium-render")
)

copied_crates <- character(0)
for (pair in crates_to_copy) {
  src <- file.path(repo_root, pair[1])
  dest <- file.path(vendor_base, pair[2])
  if (dir.exists(src)) {
    tryCatch({
      # Use system cp -R for reliability with symlinks
      if (.Platform$OS.type == "windows") {
        # xcopy handles Windows paths better
        system2("xcopy", c(shQuote(src), shQuote(dest), "/E", "/I", "/Q", "/H"),
                stdout = FALSE, stderr = FALSE)
      } else {
        file.copy(src, vendor_base, recursive = TRUE)
      }
      copied_crates <- c(copied_crates, pair[2])
      cat("Copied", pair[2], "\n")
    }, error = function(e) {
      warning("Failed to copy ", pair[2], ": ", conditionMessage(e))
    })
  } else {
    cat("Warning: Source directory not found:", pair[1], "\n")
  }
}

# --- Clean build artifacts from vendor ---
for (crate_dir in copied_crates) {
  crate_path <- file.path(vendor_base, crate_dir)
  for (artifact in c(".fastembed_cache", "target")) {
    p <- file.path(crate_path, artifact)
    if (dir.exists(p)) unlink(p, recursive = TRUE)
  }
}
cat("Cleaned build artifacts\n")

# --- Replace workspace = true in each crate's Cargo.toml ---
replace_workspace_refs <- function(toml_path, version) {
  if (!file.exists(toml_path)) return()
  content <- readLines(toml_path, warn = FALSE)
  text <- paste(content, collapse = "\n")

  # Replace version.workspace = true
  text <- gsub("^version\\.workspace = true$",
               paste0('version = "', version, '"'),
               text, perl = TRUE)
  # Replace edition.workspace = true
  text <- gsub("^edition\\.workspace = true$",
               'edition = "2024"', text, perl = TRUE)
  # Replace rust-version.workspace = true
  text <- gsub("^rust-version\\.workspace = true$",
               'rust-version = "1.91"', text, perl = TRUE)
  # Replace authors.workspace = true
  text <- gsub("^authors\\.workspace = true$",
               'authors = ["Na\'aman Hirschfeld <nhirschfeld@gmail.com>"]',
               text, perl = TRUE)
  # Replace license.workspace = true
  text <- gsub("^license\\.workspace = true$",
               'license = "MIT"', text, perl = TRUE)

  # Replace simple `dep = { workspace = true }` with the workspace dep line
  # For each workspace dependency, replace the workspace reference
  for (dep_line in ws_dep_lines) {
    dep_name <- sub("^([a-zA-Z0-9_-]+)\\s*=.*", "\\1", trimws(dep_line))
    dep_spec <- sub("^[a-zA-Z0-9_-]+\\s*=\\s*", "", trimws(dep_line))

    # Simple: dep = { workspace = true }
    pattern_simple <- paste0("^", gsub("-", "\\\\-", dep_name),
                             "\\s*=\\s*\\{\\s*workspace\\s*=\\s*true\\s*\\}$")
    text <- gsub(pattern_simple, trimws(dep_line), text, perl = TRUE)

    # With extra fields: dep = { workspace = true, optional = true }
    pattern_extra <- paste0("^", gsub("-", "\\\\-", dep_name),
                            "\\s*=\\s*\\{\\s*workspace\\s*=\\s*true\\s*,\\s*(.+?)\\s*\\}$")
    if (grepl(pattern_extra, text, perl = TRUE)) {
      # Extract the extra fields and merge with the workspace spec
      text <- gsub(pattern_extra,
                   paste0(dep_name, " = { ", sub("^\\{\\s*(.+)\\s*\\}$", "\\1",
                          sub(paste0("^", gsub("-", "\\\\-", dep_name), "\\s*=\\s*"), "", trimws(dep_line))),
                          ", \\1 }"),
                   text, perl = TRUE)
    }
  }

  writeLines(strsplit(text, "\n")[[1]], toml_path)
}

for (crate_dir in copied_crates) {
  crate_toml <- file.path(vendor_base, crate_dir, "Cargo.toml")
  replace_workspace_refs(crate_toml, core_version)
  cat("Updated", paste0(crate_dir, "/Cargo.toml"), "\n")
}

# --- Update path dependencies in kreuzberg crate ---
if ("kreuzberg" %in% copied_crates) {
  k_toml <- file.path(vendor_base, "kreuzberg", "Cargo.toml")
  if (file.exists(k_toml)) {
    text <- paste(readLines(k_toml, warn = FALSE), collapse = "\n")
    if ("kreuzberg-tesseract" %in% copied_crates) {
      text <- gsub(
        'kreuzberg-tesseract = \\{ version = "[^"]*", optional = true \\}',
        'kreuzberg-tesseract = { path = "../kreuzberg-tesseract", optional = true }',
        text, perl = TRUE)
    }
    if ("kreuzberg-paddle-ocr" %in% copied_crates) {
      text <- gsub(
        'kreuzberg-paddle-ocr = \\{ version = "[^"]*", optional = true \\}',
        'kreuzberg-paddle-ocr = { path = "../kreuzberg-paddle-ocr", optional = true }',
        text, perl = TRUE)
    }
    if ("kreuzberg-pdfium-render" %in% copied_crates) {
      text <- gsub(
        'pdfium-render = \\{ package = "kreuzberg-pdfium-render",(?:\\s*path = "[^"]*",)?(?:\\s*version = "[^"]*")',
        'pdfium-render = { package = "kreuzberg-pdfium-render", path = "../kreuzberg-pdfium-render"',
        text, perl = TRUE)
    }
    writeLines(strsplit(text, "\n")[[1]], k_toml)
  }
}

# --- Generate vendor/Cargo.toml ---
members_str <- paste0('"', copied_crates[copied_crates %in%
  c("kreuzberg", "kreuzberg-ffi", "kreuzberg-tesseract",
    "kreuzberg-paddle-ocr", "kreuzberg-pdfium-render")], '"',
  collapse = ", ")

vendor_toml_content <- paste0(
  '[workspace]\nmembers = [', members_str, ']\n\n',
  '[workspace.package]\n',
  'version = "', core_version, '"\n',
  'edition = "2024"\n',
  'rust-version = "1.91"\n',
  'authors = ["Na\'aman Hirschfeld <nhirschfeld@gmail.com>"]\n',
  'license = "MIT"\n',
  'repository = "https://github.com/kreuzberg-dev/kreuzberg"\n',
  'homepage = "https://kreuzberg.dev"\n\n',
  '[workspace.dependencies]\n',
  ws_dep_text, '\n'
)

writeLines(vendor_toml_content, file.path(vendor_base, "Cargo.toml"))
cat("Generated vendor/Cargo.toml\n")

# --- Update R package Cargo.toml ---
r_toml <- file.path(repo_root, "packages", "r", "src", "rust", "Cargo.toml")
if (file.exists(r_toml)) {
  text <- paste(readLines(r_toml, warn = FALSE), collapse = "\n")
  text <- gsub('path = "../../../../crates/kreuzberg"',
               'path = "../../vendor/kreuzberg"', text, fixed = TRUE)
  text <- gsub('path = "../../../../crates/kreuzberg-ffi"',
               'path = "../../vendor/kreuzberg-ffi"', text, fixed = TRUE)
  writeLines(strsplit(text, "\n")[[1]], r_toml)
  cat("Updated R package Cargo.toml to use vendored crates\n")
}

cat("\nVendoring complete (core version:", core_version, ")\n")
cat("  - path '../../vendor/kreuzberg' for kreuzberg crate\n")
cat("  - path '../../vendor/kreuzberg-ffi' for kreuzberg-ffi crate\n")
