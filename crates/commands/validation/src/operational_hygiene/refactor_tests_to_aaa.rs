//! Refactor Rust test files toward the AAA comment pattern.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::error::RefactorTestsToAaaCommandError;

/// Request payload for `refactor-tests-to-aaa`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefactorTestsToAaaRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Rust test file to rewrite.
    pub test_file: PathBuf,
    /// Report changes without writing the file.
    pub dry_run: bool,
}

/// Result status for `refactor-tests-to-aaa`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefactorTestsToAaaStatus {
    /// Command completed and wrote the file when needed.
    Passed,
    /// Command only reported the changes that would be applied.
    DryRun,
}

/// Result payload for `refactor-tests-to-aaa`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefactorTestsToAaaResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved test file path.
    pub test_file: PathBuf,
    /// Whether the file content changed or would change.
    pub changed: bool,
    /// Decorative separator comments removed from the file.
    pub removed_separator_comments: usize,
    /// AAA sections inserted into test bodies.
    pub inserted_arrange_markers: usize,
    /// AAA sections inserted into test bodies.
    pub inserted_act_markers: usize,
    /// AAA sections inserted into test bodies.
    pub inserted_assert_markers: usize,
    /// Final command status.
    pub status: RefactorTestsToAaaStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Rewrite one Rust test file toward the AAA pattern.
///
/// # Errors
///
/// Returns [`RefactorTestsToAaaCommandError`] when the repository root or test
/// file path cannot be resolved or when file I/O fails.
pub fn invoke_refactor_tests_to_aaa(
    request: &RefactorTestsToAaaRequest,
) -> Result<RefactorTestsToAaaResult, RefactorTestsToAaaCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RefactorTestsToAaaCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| RefactorTestsToAaaCommandError::ResolveWorkspaceRoot { source })?;
    let test_file = resolve_full_path(&repo_root, &request.test_file);
    if !test_file.is_file()
        || test_file
            .extension()
            .and_then(|value| value.to_str())
            .is_none_or(|value| !value.eq_ignore_ascii_case("rs"))
    {
        return Err(RefactorTestsToAaaCommandError::ResolveTestFilePath {
            test_file: test_file.display().to_string(),
        });
    }

    let original = fs::read_to_string(&test_file)
        .with_context(|| format!("failed to read '{}'", test_file.display()))
        .map_err(|source| RefactorTestsToAaaCommandError::ReadTestFile { source })?;
    let rewrite = rewrite_test_file(&original);

    if rewrite.changed && !request.dry_run {
        fs::write(&test_file, rewrite.content)
            .with_context(|| format!("failed to write '{}'", test_file.display()))
            .map_err(|source| RefactorTestsToAaaCommandError::WriteTestFile { source })?;
    }

    Ok(RefactorTestsToAaaResult {
        repo_root,
        test_file,
        changed: rewrite.changed,
        removed_separator_comments: rewrite.removed_separator_comments,
        inserted_arrange_markers: rewrite.inserted_arrange_markers,
        inserted_act_markers: rewrite.inserted_act_markers,
        inserted_assert_markers: rewrite.inserted_assert_markers,
        status: if request.dry_run {
            RefactorTestsToAaaStatus::DryRun
        } else {
            RefactorTestsToAaaStatus::Passed
        },
        exit_code: 0,
    })
}

struct RewriteResult {
    content: String,
    changed: bool,
    removed_separator_comments: usize,
    inserted_arrange_markers: usize,
    inserted_act_markers: usize,
    inserted_assert_markers: usize,
}

fn rewrite_test_file(content: &str) -> RewriteResult {
    let newline = detect_preferred_newline(content);
    let trailing_newline = content.ends_with('\n') || content.ends_with('\r');
    let (without_separators, removed_separator_comments) = strip_separator_comments(content);
    let lines = without_separators
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();

    let mut rewritten = Vec::new();
    let mut inserted_arrange_markers = 0usize;
    let mut inserted_act_markers = 0usize;
    let mut inserted_assert_markers = 0usize;
    let mut index = 0usize;

    while index < lines.len() {
        if !is_test_attribute(&lines[index]) {
            rewritten.push(lines[index].clone());
            index += 1;
            continue;
        }

        let Some((header_end, block_end)) = locate_test_block(&lines, index) else {
            rewritten.push(lines[index].clone());
            index += 1;
            continue;
        };

        rewritten.extend(lines[index..=header_end].iter().cloned());
        let body_lines = &lines[(header_end + 1)..block_end];
        let transformed = transform_test_body(body_lines);
        inserted_arrange_markers += transformed.inserted_arrange_markers;
        inserted_act_markers += transformed.inserted_act_markers;
        inserted_assert_markers += transformed.inserted_assert_markers;
        rewritten.extend(transformed.lines);
        rewritten.push(lines[block_end].clone());
        index = block_end + 1;
    }

    let mut final_content = rewritten.join(newline);
    if trailing_newline {
        final_content.push_str(newline);
    }

    let changed = final_content != content;
    RewriteResult {
        content: final_content,
        changed,
        removed_separator_comments,
        inserted_arrange_markers,
        inserted_act_markers,
        inserted_assert_markers,
    }
}

fn strip_separator_comments(content: &str) -> (String, usize) {
    let separator_pattern =
        Regex::new(r"(?m)^\s*//\s*={3,}.*(?:\r?\n)?").expect("separator regex should compile");
    let removed_separator_comments = separator_pattern.find_iter(content).count();
    let without_separators = separator_pattern.replace_all(content, "");
    (without_separators.into_owned(), removed_separator_comments)
}

fn is_test_attribute(line: &str) -> bool {
    matches!(line.trim(), "#[test]" | "#[tokio::test]")
}

fn locate_test_block(lines: &[String], start_index: usize) -> Option<(usize, usize)> {
    let mut header_end = start_index;
    let mut saw_function = false;

    while header_end < lines.len() {
        let line = &lines[header_end];
        let trimmed = line.trim();
        if trimmed.contains("fn ") {
            saw_function = true;
        }
        if saw_function && trimmed.contains('{') {
            break;
        }
        header_end += 1;
    }

    if header_end >= lines.len() {
        return None;
    }

    let mut depth = 0i32;
    for line in &lines[start_index..=header_end] {
        depth += brace_delta(line);
    }

    let mut block_end = header_end + 1;
    while block_end < lines.len() {
        depth += brace_delta(&lines[block_end]);
        if depth == 0 {
            return Some((header_end, block_end));
        }
        block_end += 1;
    }

    None
}

fn brace_delta(line: &str) -> i32 {
    let opens = line.matches('{').count() as i32;
    let closes = line.matches('}').count() as i32;
    opens - closes
}

struct BodyTransformResult {
    lines: Vec<String>,
    inserted_arrange_markers: usize,
    inserted_act_markers: usize,
    inserted_assert_markers: usize,
}

fn transform_test_body(body_lines: &[String]) -> BodyTransformResult {
    if has_full_aaa_markers(body_lines) {
        return BodyTransformResult {
            lines: body_lines.to_vec(),
            inserted_arrange_markers: 0,
            inserted_act_markers: 0,
            inserted_assert_markers: 0,
        };
    }

    let mut transformed = Vec::new();
    let mut has_arrange = false;
    let mut has_act = false;
    let mut has_assert = false;
    let mut inserted_arrange_markers = 0usize;
    let mut inserted_act_markers = 0usize;
    let mut inserted_assert_markers = 0usize;

    for line in body_lines {
        let trimmed = line.trim();

        if !has_arrange && !trimmed.is_empty() && !trimmed.starts_with("//") {
            transformed.push("    // Arrange".to_string());
            inserted_arrange_markers += 1;
            if trimmed.starts_with("let _lock") {
                transformed.push("    // (using env lock for thread safety)".to_string());
            }
            has_arrange = true;
        }

        if !has_act && is_act_line(trimmed) {
            transformed.push(String::new());
            transformed.push("    // Act".to_string());
            inserted_act_markers += 1;
            has_act = true;
        }

        if !has_assert && is_assert_line(trimmed) {
            transformed.push(String::new());
            transformed.push("    // Assert".to_string());
            inserted_assert_markers += 1;
            has_assert = true;
        }

        transformed.push(line.clone());
    }

    BodyTransformResult {
        lines: transformed,
        inserted_arrange_markers,
        inserted_act_markers,
        inserted_assert_markers,
    }
}

fn has_full_aaa_markers(lines: &[String]) -> bool {
    let has_arrange = lines
        .iter()
        .any(|line| line.trim_start().starts_with("// Arrange"));
    let has_act = lines
        .iter()
        .any(|line| line.trim_start().starts_with("// Act"));
    let has_assert = lines
        .iter()
        .any(|line| line.trim_start().starts_with("// Assert"));
    has_arrange && has_act && has_assert
}

fn is_act_line(line: &str) -> bool {
    if line.contains("_lock") {
        return false;
    }

    Regex::new(r"^let\s+\w+\s*=\s*(Features::|Config::|[A-Z])")
        .expect("act regex should compile")
        .is_match(line)
}

fn is_assert_line(line: &str) -> bool {
    line.starts_with("assert") || line.starts_with("#[cfg")
}

fn detect_preferred_newline(content: &str) -> &'static str {
    if content.contains("\r\n") {
        "\r\n"
    } else if content.contains('\n') {
        "\n"
    } else if content.contains('\r') {
        "\r"
    } else {
        "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::rewrite_test_file;

    #[test]
    fn test_rewrite_test_file_removes_separators_and_inserts_aaa_markers() {
        let rewrite = rewrite_test_file(
            r#"#[test]
fn sample_test() {
    // =======
    let _lock = env_lock();
    let feature = Features::load();
    assert_eq!(feature, 1);
}
"#,
        );

        assert!(rewrite.changed);
        assert_eq!(rewrite.removed_separator_comments, 1);
        assert_eq!(rewrite.inserted_arrange_markers, 1);
        assert_eq!(rewrite.inserted_act_markers, 1);
        assert_eq!(rewrite.inserted_assert_markers, 1);
        assert!(rewrite.content.contains("// Arrange"));
        assert!(rewrite.content.contains("// Act"));
        assert!(rewrite.content.contains("// Assert"));
    }

    #[test]
    fn test_rewrite_test_file_preserves_existing_aaa_body() {
        let original = r#"#[test]
fn sample_test() {
    // Arrange
    let feature = Features::load();

    // Act
    let actual = feature;

    // Assert
    assert_eq!(actual, 1);
}
"#;
        let rewrite = rewrite_test_file(original);

        assert!(!rewrite.changed);
        assert_eq!(rewrite.content, original);
    }
}
