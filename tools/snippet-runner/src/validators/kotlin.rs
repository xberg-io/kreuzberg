use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use crate::validators::{SnippetValidator, run_command};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct KotlinValidator {
    repo_root: PathBuf,
}

impl KotlinValidator {
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

    /// Wrap a Kotlin fragment in `fun main()` if it's not already a complete file.
    ///
    /// Heuristic:
    /// - If the snippet contains `fun main(`, a `class ` or `object ` declaration,
    ///   it is treated as a complete program and returned as-is (after dedent).
    /// - Otherwise, top-level `import` lines are hoisted to the top of the file
    ///   and the remaining body is wrapped. If the body uses the `suspend`
    ///   keyword, the wrap uses `runBlocking { ... }` and adds a
    ///   `kotlinx.coroutines` import.
    fn wrap_if_fragment(code: &str) -> String {
        let code = Self::dedent(code);
        let trimmed = code.trim();

        // Already a complete program — leave as-is.
        if trimmed.contains("fun main(") || trimmed.contains("class ") || trimmed.contains("object ") {
            return code;
        }

        // Separate top-level imports from body.
        let mut imports = Vec::new();
        let mut body = Vec::new();
        for line in code.lines() {
            let t = line.trim();
            if t.starts_with("import ") {
                imports.push(line.to_string());
            } else {
                body.push(line.to_string());
            }
        }

        let body_str = body.join("\n");
        let body_trimmed = body_str.trim();

        // Detect suspending calls — if present, wrap with runBlocking.
        let needs_runblocking = body_trimmed.contains("suspend") || body_trimmed.contains(".await");

        if needs_runblocking {
            let has_coroutines_import = imports.iter().any(|l| l.contains("kotlinx.coroutines"));
            if !has_coroutines_import {
                imports.insert(0, "import kotlinx.coroutines.runBlocking".to_string());
            }
        }

        let imports_str = if imports.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", imports.join("\n"))
        };

        if needs_runblocking {
            format!("{imports_str}fun main() {{\n    runBlocking {{\n{body_str}\n    }}\n}}")
        } else {
            format!("{imports_str}fun main() {{\n{body_str}\n}}")
        }
    }

    /// Find the pre-built kreuzberg Kotlin JAR at `packages/kotlin/build/libs/kreuzberg-*.jar`.
    /// Returns `None` when the file is not present (i.e. `gradle compileKotlin` was never run).
    fn find_kreuzberg_jar(&self) -> Option<PathBuf> {
        let libs = self
            .repo_root
            .join("packages")
            .join("kotlin")
            .join("build")
            .join("libs");
        let entries = std::fs::read_dir(&libs).ok()?;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("kreuzberg-") && name_str.ends_with(".jar") {
                return Some(entry.path());
            }
        }
        None
    }

    /// Collect transitive-dependency JARs from the local Gradle cache.
    ///
    /// `~/.gradle/caches/modules-2/files-2.1/` stores JARs by
    /// `group/artifact/version/hash/artifact-version.jar`. We look for exactly
    /// the coordinates that `packages/kotlin/build.gradle.kts` declares as `api`
    /// dependencies so snippets that reference these types compile without errors.
    ///
    /// Returns an empty Vec when the Gradle cache is not found — compilation will
    /// still succeed for snippets that only use types from the kreuzberg JAR itself.
    fn collect_dep_jars() -> Vec<PathBuf> {
        let home = dirs_home();
        let cache = home.join(".gradle").join("caches").join("modules-2").join("files-2.1");
        if !cache.exists() {
            return Vec::new();
        }

        // Coordinates from packages/kotlin/build.gradle.kts api dependencies.
        let coords: &[(&str, &str, &str)] = &[
            ("net.java.dev.jna", "jna", "5.18.1"),
            ("com.fasterxml.jackson.core", "jackson-core", "2.18.2"),
            ("com.fasterxml.jackson.core", "jackson-annotations", "2.18.2"),
            ("com.fasterxml.jackson.core", "jackson-databind", "2.18.2"),
            ("com.fasterxml.jackson.datatype", "jackson-datatype-jdk8", "2.18.2"),
            ("org.jspecify", "jspecify", "1.0.0"),
        ];

        let mut jars = Vec::new();
        for (group, artifact, version) in coords {
            let artifact_dir = cache.join(group).join(artifact).join(version);
            if let Ok(entries) = std::fs::read_dir(&artifact_dir) {
                for entry in entries.flatten() {
                    // Each version dir contains hash subdirs; recurse one level.
                    if entry.path().is_dir() {
                        let jar_name = format!("{artifact}-{version}.jar");
                        let candidate = entry.path().join(&jar_name);
                        if candidate.exists() {
                            jars.push(candidate);
                            break; // only need one hash variant
                        }
                    }
                }
            }
        }

        // Also add the latest kotlinx-coroutines-core-jvm we can find — needed
        // for snippets that call runBlocking or use coroutines directly.
        let coroutines_dir = cache.join("org.jetbrains.kotlinx").join("kotlinx-coroutines-core-jvm");
        if let Ok(mut versions) = std::fs::read_dir(&coroutines_dir) {
            // Pick lexicographically-greatest version.
            let mut best: Option<PathBuf> = None;
            for ver_entry in versions.by_ref().flatten() {
                if let Ok(hash_entries) = std::fs::read_dir(ver_entry.path()) {
                    for hash_entry in hash_entries.flatten() {
                        let jar = hash_entry
                            .path()
                            .read_dir()
                            .ok()
                            .and_then(|mut d| d.next())
                            .and_then(|e| e.ok())
                            .map(|e| e.path())
                            .filter(|p| p.extension().is_some_and(|x| x == "jar"));
                        if let Some(j) = jar
                            && best.as_ref().is_none_or(|b| j > *b)
                        {
                            best = Some(j);
                        }
                    }
                }
            }
            if let Some(p) = best {
                jars.push(p);
            }
        }

        jars
    }

    /// Build the `-classpath` string: kreuzberg JAR + transitive dep JARs.
    fn build_classpath(&self) -> Option<String> {
        let kreuzberg_jar = self.find_kreuzberg_jar()?;
        let mut entries = vec![kreuzberg_jar.to_string_lossy().into_owned()];
        for dep in Self::collect_dep_jars() {
            entries.push(dep.to_string_lossy().into_owned());
        }
        Some(entries.join(":"))
    }
}

/// Return the user's home directory as a PathBuf.
fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/"))
}

impl SnippetValidator for KotlinValidator {
    fn language(&self) -> Language {
        Language::Kotlin
    }

    fn is_available(&self) -> bool {
        which::which("kotlinc").is_ok()
    }

    fn validate(
        &self,
        snippet: &Snippet,
        level: ValidationLevel,
        timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)> {
        let dir = TempDir::new()?;
        let code = Self::wrap_if_fragment(&snippet.code);

        let src_path = dir.path().join("Snippet.kt");
        let mut file = std::fs::File::create(&src_path)?;
        file.write_all(code.as_bytes())?;

        let out_dir = dir.path().join("out");
        std::fs::create_dir_all(&out_dir)?;

        let classpath = self.build_classpath();

        let mut cmd = match level {
            ValidationLevel::Syntax | ValidationLevel::Compile | ValidationLevel::Run => {
                // Running JVM bytecode is heavy; max_level caps at Compile so we
                // only ever hit this branch for Syntax/Compile in practice. Both
                // use the same kotlinc invocation — produces class files but we
                // don't execute them.
                let mut c = std::process::Command::new("kotlinc");
                c.arg("-nowarn").arg("-d").arg(&out_dir);
                if let Some(ref cp) = classpath {
                    c.arg("-classpath").arg(cp);
                }
                c.arg(&src_path).current_dir(dir.path());
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
        // Running JVM bytecode is heavy — cap at Compile.
        ValidationLevel::Compile
    }

    fn is_dependency_error(&self, output: &str) -> bool {
        let error_lines: Vec<&str> = output
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && (t.contains("error:") || t.contains("error "))
            })
            .collect();

        if error_lines.is_empty() {
            return false;
        }

        error_lines.iter().all(|line| {
            line.contains("unresolved reference")
                || line.contains("cannot access")
                || line.contains("cannot find symbol")
                || line.contains("unresolved import")
                || line.contains("error: package")
                || (line.contains("package ") && line.contains("is missing"))
                || line.contains("kotlinx.coroutines")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validator() -> KotlinValidator {
        KotlinValidator::new(PathBuf::new())
    }

    #[test]
    fn test_dedent_strips_uniform_indentation() {
        let input = "    val x = 1\n    val y = 2";
        let output = KotlinValidator::dedent(input);
        assert_eq!(output, "val x = 1\nval y = 2");
    }

    #[test]
    fn test_dedent_no_indent_is_noop() {
        let input = "val x = 1\nval y = 2";
        let output = KotlinValidator::dedent(input);
        assert_eq!(output, "val x = 1\nval y = 2");
    }

    #[test]
    fn test_wrap_preserves_complete_program_with_main() {
        let input = "fun main() {\n    println(\"hi\")\n}";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_preserves_complete_program_with_class() {
        let input = "class Foo {\n    fun bar() = 1\n}";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_preserves_complete_program_with_object() {
        let input = "object Foo {\n    fun bar() = 1\n}";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_wrap_fragment_in_main() {
        let input = "println(\"hi\")";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert!(output.contains("fun main()"));
        assert!(output.contains("println(\"hi\")"));
        assert!(!output.contains("runBlocking"));
    }

    #[test]
    fn test_wrap_hoists_imports() {
        let input = "import kotlin.math.PI\nval x = PI";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert!(output.starts_with("import kotlin.math.PI"));
        assert!(output.contains("fun main()"));
        assert!(output.contains("val x = PI"));
    }

    #[test]
    fn test_wrap_uses_runblocking_for_suspend() {
        let input = "suspend fun fetch() = 1\nfetch()";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert!(output.contains("import kotlinx.coroutines.runBlocking"));
        assert!(output.contains("runBlocking"));
    }

    #[test]
    fn test_wrap_uses_runblocking_for_await() {
        let input = "deferred.await()";
        let output = KotlinValidator::wrap_if_fragment(input);
        assert!(output.contains("import kotlinx.coroutines.runBlocking"));
        assert!(output.contains("runBlocking"));
    }

    #[test]
    fn test_wrap_does_not_double_import_coroutines() {
        let input = "import kotlinx.coroutines.runBlocking\nsuspend fun fetch() = 1\nfetch()";
        let output = KotlinValidator::wrap_if_fragment(input);
        let count = output.matches("import kotlinx.coroutines").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_language_is_kotlin() {
        assert_eq!(validator().language(), Language::Kotlin);
    }

    #[test]
    fn test_max_level_is_compile() {
        assert_eq!(validator().max_level(), ValidationLevel::Compile);
    }

    #[test]
    fn test_is_dependency_error_detects_unresolved_reference() {
        let output = "Snippet.kt:3:5: error: unresolved reference: foo";
        assert!(validator().is_dependency_error(output));
    }

    #[test]
    fn test_is_dependency_error_detects_cannot_access() {
        let output = "Snippet.kt:3:5: error: cannot access 'Foo'";
        assert!(validator().is_dependency_error(output));
    }

    #[test]
    fn test_is_dependency_error_returns_false_on_empty() {
        assert!(!validator().is_dependency_error(""));
    }

    #[test]
    fn test_is_dependency_error_returns_false_on_real_syntax_error() {
        let output = "Snippet.kt:3:5: error: expecting ')'";
        assert!(!validator().is_dependency_error(output));
    }

    #[test]
    fn test_find_kreuzberg_jar_returns_none_for_empty_root() {
        let v = KotlinValidator::new(PathBuf::from("/nonexistent/path"));
        assert!(v.find_kreuzberg_jar().is_none());
    }

    #[test]
    fn test_find_kreuzberg_jar_finds_jar_when_present() {
        let dir = tempfile::tempdir().unwrap();
        let libs = dir.path().join("packages").join("kotlin").join("build").join("libs");
        std::fs::create_dir_all(&libs).unwrap();
        std::fs::write(libs.join("kreuzberg-1.0.0.jar"), b"").unwrap();
        let v = KotlinValidator::new(dir.path().to_path_buf());
        assert!(v.find_kreuzberg_jar().is_some());
    }

    #[test]
    fn test_build_classpath_returns_none_when_no_jar() {
        let v = KotlinValidator::new(PathBuf::from("/nonexistent"));
        assert!(v.build_classpath().is_none());
    }

    #[test]
    fn test_build_classpath_includes_kreuzberg_jar() {
        let dir = tempfile::tempdir().unwrap();
        let libs = dir.path().join("packages").join("kotlin").join("build").join("libs");
        std::fs::create_dir_all(&libs).unwrap();
        std::fs::write(libs.join("kreuzberg-5.0.0.jar"), b"").unwrap();
        let v = KotlinValidator::new(dir.path().to_path_buf());
        let cp = v.build_classpath().unwrap();
        assert!(
            cp.contains("kreuzberg-5.0.0.jar"),
            "classpath should include kreuzberg JAR"
        );
    }
}
