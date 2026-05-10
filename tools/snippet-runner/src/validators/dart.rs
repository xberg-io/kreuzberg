use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use crate::validators::{SnippetValidator, run_command};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct DartValidator {
    repo_root: PathBuf,
}

impl DartValidator {
    pub fn new(repo_root: PathBuf) -> Self {
        Self { repo_root }
    }

    /// Dedent code that has uniform leading whitespace (from markdown indentation).
    fn dedent(code: &str) -> String {
        let min_indent = code
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);

        if min_indent == 0 {
            return code.to_string();
        }

        code.lines()
            .map(|l| {
                if l.trim().is_empty() {
                    ""
                } else if l.len() > min_indent {
                    &l[min_indent..]
                } else {
                    l.trim()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Wrap a snippet as a complete Dart program if it appears to be a fragment.
    ///
    /// A snippet is considered complete if it contains a `main` function or any
    /// import directive. Otherwise it is wrapped in `Future<void> main() async { ... }`.
    fn wrap_if_fragment(code: &str) -> String {
        // Dedent first to handle indented markdown snippets
        let code = Self::dedent(code);
        let trimmed = code.trim();

        // Already a complete program — has a main() function
        let has_main = trimmed.contains("void main(")
            || trimmed.contains("Future<void> main(")
            || trimmed.contains("Future main(")
            || trimmed.contains("dynamic main(");

        // Has import directives — treat as complete file
        let has_import = code.lines().any(|l| {
            let t = l.trim();
            t.starts_with("import 'dart:")
                || t.starts_with("import \"dart:")
                || t.starts_with("import 'package:")
                || t.starts_with("import \"package:")
        });

        if has_main || has_import {
            return code;
        }

        format!("Future<void> main() async {{\n{code}\n}}")
    }

    /// Write the pubspec.yaml for the temp project.
    ///
    /// If the kreuzberg Dart package exists at `repo_root/packages/dart`, a path
    /// dependency is declared so `import 'package:kreuzberg/...'` resolves.
    /// Falls back to a stub pubspec with no package dependencies when the path
    /// does not exist (e.g. the tool is run outside the kreuzberg repo).
    fn write_pubspec(&self, dir: &std::path::Path) -> std::io::Result<()> {
        let dart_pkg = self.repo_root.join("packages").join("dart");
        let pubspec = if dart_pkg.exists() {
            format!(
                "name: snippet_check\nenvironment:\n  sdk: '>=3.0.0 <4.0.0'\ndependencies:\n  kreuzberg:\n    path: {}\n",
                dart_pkg.display()
            )
        } else {
            "name: snippet_check\nenvironment:\n  sdk: '>=3.0.0 <4.0.0'\n".to_string()
        };
        std::fs::write(dir.join("pubspec.yaml"), pubspec)
    }
}

impl SnippetValidator for DartValidator {
    fn language(&self) -> Language {
        Language::Dart
    }

    fn is_available(&self) -> bool {
        which::which("dart").is_ok()
    }

    fn validate(
        &self,
        snippet: &Snippet,
        level: ValidationLevel,
        timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)> {
        let dir = TempDir::new()?;

        self.write_pubspec(dir.path())?;

        let bin_dir = dir.path().join("bin");
        std::fs::create_dir_all(&bin_dir)?;

        let code = Self::wrap_if_fragment(&snippet.code);
        let main_path = bin_dir.join("main.dart");
        let mut file = std::fs::File::create(&main_path)?;
        file.write_all(code.as_bytes())?;

        // Run `dart pub get` to resolve path dep before analyzing.
        // We run this unconditionally — for snippets without kreuzberg imports it is
        // a fast no-op (no network traffic when the dep is a local path).
        let dart_pkg = self.repo_root.join("packages").join("dart");
        if dart_pkg.exists() {
            let mut pub_cmd = std::process::Command::new("dart");
            pub_cmd.args(["pub", "get"]).current_dir(dir.path());
            let (ok, pub_out) = run_command(&mut pub_cmd, timeout_secs)?;
            if !ok {
                return Ok((SnippetStatus::Fail, Some(format!("dart pub get failed:\n{pub_out}"))));
            }
        }

        let mut cmd = match level {
            ValidationLevel::Syntax | ValidationLevel::Compile => {
                let mut c = std::process::Command::new("dart");
                c.args(["analyze", "--fatal-infos", "--fatal-warnings"])
                    .arg(&main_path)
                    .current_dir(dir.path());
                c
            }
            ValidationLevel::Run => {
                let mut c = std::process::Command::new("dart");
                c.args(["run", "bin/main.dart"]).current_dir(dir.path());
                c
            }
        };

        let (success, output) = run_command(&mut cmd, timeout_secs)?;

        if success {
            Ok((SnippetStatus::Pass, None))
        } else {
            Ok((SnippetStatus::Fail, Some(output)))
        }
    }

    fn max_level(&self) -> ValidationLevel {
        ValidationLevel::Run
    }

    fn is_dependency_error(&self, output: &str) -> bool {
        let error_lines: Vec<&str> = output
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty()
                    && (t.contains("error")
                        || t.contains("warning")
                        || t.contains("info")
                        || t.contains("Error")
                        || t.contains("Warning")
                        || t.contains("URI")
                        || t.contains("Undefined")
                        || t.contains("isn't defined")
                        || t.contains("isn't a")
                        || t.contains("not found")
                        || t.contains("Couldn't resolve"))
            })
            .collect();

        if error_lines.is_empty() {
            return false;
        }

        error_lines.iter().all(|line| {
            line.contains("URI doesn't exist")
                || line.contains("Target of URI doesn't exist")
                || line.contains("Undefined name")
                || line.contains("Undefined class")
                || line.contains("Undefined identifier")
                || line.contains("undefined_identifier")
                || line.contains("isn't defined")
                || line.contains("isn't a class")
                || line.contains("isn't a type")
                || line.contains("not found")
                || line.contains("Couldn't resolve")
                || line.contains("uri_does_not_exist")
                || line.contains("unused_import")
                || line.contains("unused_local_variable")
                || line.contains("unused_element")
                || line.contains("depend_on_referenced_packages")
                || line.contains("avoid_print")
                || line.contains("issue found")
                || line.contains("issues found")
                || line.contains("Analyzing")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validator() -> DartValidator {
        DartValidator::new(PathBuf::new())
    }

    #[test]
    fn test_dedent_removes_uniform_indentation() {
        let input = "    line1\n    line2\n    line3";
        let output = DartValidator::dedent(input);
        assert_eq!(output, "line1\nline2\nline3");
    }

    #[test]
    fn test_dedent_preserves_relative_indentation() {
        let input = "  outer\n    inner\n  outer";
        let output = DartValidator::dedent(input);
        assert_eq!(output, "outer\n  inner\nouter");
    }

    #[test]
    fn test_dedent_no_indent_returns_unchanged() {
        let input = "line1\nline2\nline3";
        let output = DartValidator::dedent(input);
        assert_eq!(output, "line1\nline2\nline3");
    }

    #[test]
    fn test_dedent_handles_empty_lines() {
        let input = "    line1\n\n    line2";
        let output = DartValidator::dedent(input);
        assert_eq!(output, "line1\n\nline2");
    }

    #[test]
    fn test_wrap_if_fragment_complete_with_void_main() {
        let input = "void main() {\n  print('hello');\n}";
        let output = DartValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_if_fragment_complete_with_future_main() {
        let input = "Future<void> main() async {\n  await foo();\n}";
        let output = DartValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_if_fragment_complete_with_dart_import() {
        let input = "import 'dart:io';\n\nvoid foo() {}";
        let output = DartValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_if_fragment_complete_with_package_import() {
        let input = "import 'package:kreuzberg/kreuzberg.dart';\n\nfinal x = 1;";
        let output = DartValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_if_fragment_wraps_statement() {
        let input = "print('hello');";
        let output = DartValidator::wrap_if_fragment(input);
        assert_eq!(output, "Future<void> main() async {\nprint('hello');\n}");
    }

    #[test]
    fn test_wrap_if_fragment_dedents_then_wraps() {
        let input = "    final x = 42;\n    print(x);";
        let output = DartValidator::wrap_if_fragment(input);
        assert_eq!(output, "Future<void> main() async {\nfinal x = 42;\nprint(x);\n}");
    }

    #[test]
    fn test_pubspec_with_valid_repo_root_includes_path_dep() {
        let dir = tempfile::tempdir().unwrap();
        // Simulate a repo_root that has packages/dart/
        let packages_dart = dir.path().join("packages").join("dart");
        std::fs::create_dir_all(&packages_dart).unwrap();
        let validator = DartValidator::new(dir.path().to_path_buf());
        let output_dir = tempfile::tempdir().unwrap();
        validator.write_pubspec(output_dir.path()).unwrap();
        let content = std::fs::read_to_string(output_dir.path().join("pubspec.yaml")).unwrap();
        assert!(content.contains("kreuzberg:"), "pubspec should declare kreuzberg dep");
        assert!(content.contains("path:"), "pubspec should use path dep");
        assert!(
            content.contains(&packages_dart.display().to_string()),
            "path dep should point to packages/dart"
        );
    }

    #[test]
    fn test_pubspec_without_repo_root_is_minimal() {
        let validator = DartValidator::new(PathBuf::from("/nonexistent/path"));
        let output_dir = tempfile::tempdir().unwrap();
        validator.write_pubspec(output_dir.path()).unwrap();
        let content = std::fs::read_to_string(output_dir.path().join("pubspec.yaml")).unwrap();
        assert!(!content.contains("path:"), "fallback pubspec should not have path dep");
        assert!(content.contains("snippet_check"), "pubspec should have project name");
    }

    #[test]
    fn test_is_dependency_error_uri_does_not_exist() {
        let v = validator();
        let output = "  error - main.dart:1:8 - Target of URI doesn't exist: 'package:kreuzberg/kreuzberg.dart'. - uri_does_not_exist\n1 issue found.";
        assert!(v.is_dependency_error(output));
    }

    #[test]
    fn test_is_dependency_error_isnt_a_class() {
        let v = validator();
        let output = "  error - main.dart:3:16 - The name 'ChunkingConfig' isn't a class. - creation_with_non_type\n1 issue found.";
        assert!(v.is_dependency_error(output));
    }

    #[test]
    fn test_is_dependency_error_mixed_dep_errors_all_pass() {
        let v = validator();
        // Multiple dependency-style errors should all pass
        let output = "  error - main.dart:1:8 - Target of URI doesn't exist: 'package:kreuzberg/kreuzberg.dart'. - uri_does_not_exist\n  error - main.dart:3:16 - The name 'ChunkingConfig' isn't a class. - creation_with_non_type\n  error - main.dart:4:20 - Undefined name 'ResultFormat'. - undefined_identifier\n10 issues found.";
        assert!(v.is_dependency_error(output));
    }

    #[test]
    fn test_is_dependency_error_real_syntax_error_returns_false() {
        let v = validator();
        let output = "  error - main.dart:5:3 - Expected to find ';'. - expected_token\n1 issue found.";
        assert!(!v.is_dependency_error(output));
    }

    #[test]
    fn test_language_is_dart() {
        assert_eq!(validator().language(), Language::Dart);
    }
}
