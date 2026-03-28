//! Trim trailing blank lines and trailing whitespace from text files.

use anyhow::{anyhow, Context};
use nettoolskit_core::editorconfig::resolve_insert_final_newline_policy;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use crate::error::RuntimeTrimTrailingBlankLinesCommandError;

const BINARY_EXTENSIONS: &[&str] = &[
    ".dll", ".exe", ".pdb", ".png", ".jpg", ".jpeg", ".gif", ".ico", ".zip", ".7z", ".rar", ".pdf",
    ".mp4", ".mp3", ".wav", ".ogg", ".webp", ".bmp", ".ttf", ".otf", ".woff", ".woff2", ".snk",
    ".nupkg", ".sln",
];

const EXCLUDED_DIRS: &[&str] = &[
    "bin",
    "obj",
    ".git",
    "node_modules",
    ".vs",
    ".idea",
    ".build",
    ".deployment",
    "artifacts",
    "target",
];

/// Request payload for `trim-trailing-blank-lines`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeTrimTrailingBlankLinesRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional directory or single-file path to scan.
    pub path: Option<PathBuf>,
    /// Explicit file paths to trim.
    pub literal_paths: Vec<PathBuf>,
    /// Do not modify files; only report the files that would change.
    pub check_only: bool,
    /// Limit discovery to files currently reported by git status.
    pub git_changed_only: bool,
}

/// Runtime trim-trailing-blank-lines result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTrimTrailingBlankLinesStatus {
    /// Command completed and applied any required changes.
    Passed,
    /// Command completed in check-only mode.
    CheckOnly,
}

/// Result payload for `trim-trailing-blank-lines`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTrimTrailingBlankLinesResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Files discovered for processing.
    pub discovered_files: Vec<PathBuf>,
    /// Files that were changed.
    pub changed_files: Vec<PathBuf>,
    /// Final command status.
    pub status: RuntimeTrimTrailingBlankLinesStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Remove trailing blank lines and trailing whitespace from text files.
///
/// # Errors
///
/// Returns [`RuntimeTrimTrailingBlankLinesCommandError`] when repository root,
/// discovery, or file normalization fails.
pub fn invoke_trim_trailing_blank_lines(
    request: &RuntimeTrimTrailingBlankLinesRequest,
) -> Result<RuntimeTrimTrailingBlankLinesResult, RuntimeTrimTrailingBlankLinesCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeTrimTrailingBlankLinesCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| RuntimeTrimTrailingBlankLinesCommandError::ResolveWorkspaceRoot { source },
        )?;

    let discovered_files = discover_files(&repo_root, request)
        .map_err(|source| RuntimeTrimTrailingBlankLinesCommandError::DiscoverFiles { source })?;
    let mut changed_files = Vec::new();

    for file_path in &discovered_files {
        let changed =
            normalize_file(&repo_root, file_path, request.check_only).map_err(|source| {
                RuntimeTrimTrailingBlankLinesCommandError::NormalizeFiles { source }
            })?;
        if changed {
            changed_files.push(file_path.clone());
        }
    }

    let exit_code = if request.check_only && !changed_files.is_empty() {
        1
    } else {
        0
    };

    Ok(RuntimeTrimTrailingBlankLinesResult {
        repo_root,
        discovered_files,
        changed_files,
        status: if request.check_only {
            RuntimeTrimTrailingBlankLinesStatus::CheckOnly
        } else {
            RuntimeTrimTrailingBlankLinesStatus::Passed
        },
        exit_code,
    })
}

fn discover_files(
    repo_root: &Path,
    request: &RuntimeTrimTrailingBlankLinesRequest,
) -> anyhow::Result<Vec<PathBuf>> {
    if !request.literal_paths.is_empty() {
        return discover_explicit_paths(repo_root, &request.literal_paths);
    }

    if let Some(requested_path) = request.path.as_deref() {
        let target_path = resolve_full_path(repo_root, requested_path);
        if !target_path.exists() {
            return Err(anyhow!("target path not found: {}", target_path.display()));
        }

        return discover_from_target_path(repo_root, &target_path);
    }

    if request.git_changed_only {
        return discover_git_changed_files(repo_root);
    }

    discover_all_files(repo_root)
}

fn discover_explicit_paths(
    repo_root: &Path,
    requested_paths: &[PathBuf],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for requested_path in requested_paths {
        let resolved = resolve_full_path(repo_root, requested_path);
        if !resolved.exists() {
            return Err(anyhow!("explicit path not found: {}", resolved.display()));
        }

        files.extend(discover_from_target_path(repo_root, &resolved)?);
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn discover_from_target_path(repo_root: &Path, target_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if target_path.is_file() {
        return Ok(if should_process_file(repo_root, target_path) {
            vec![target_path.to_path_buf()]
        } else {
            Vec::new()
        });
    }

    discover_files_in_directory(repo_root, target_path)
}

fn discover_all_files(repo_root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    discover_files_in_directory(repo_root, repo_root)
}

fn discover_files_in_directory(root: &Path, directory: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut discovered = Vec::new();
    let mut stack = vec![directory.to_path_buf()];

    while let Some(current) = stack.pop() {
        let entries = fs::read_dir(&current)
            .with_context(|| format!("failed to enumerate '{}'", current.display()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if should_skip_path(&path) {
                    continue;
                }

                stack.push(path);
                continue;
            }

            if should_process_file(root, &path) {
                discovered.push(path);
            }
        }
    }

    discovered.sort();
    discovered.dedup();
    Ok(discovered)
}

fn discover_git_changed_files(repo_root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .output()
        .with_context(|| "failed to start git status".to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let message = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("git exited with {}", output.status)
        };
        return Err(anyhow!(message));
    }

    let raw_output = String::from_utf8_lossy(&output.stdout);
    let entries = raw_output
        .split('\0')
        .filter(|entry| !entry.is_empty())
        .collect::<Vec<_>>();
    let mut selected = Vec::new();
    let mut index = 0usize;

    while index < entries.len() {
        let entry = entries[index];
        if entry.len() < 4 {
            index += 1;
            continue;
        }

        let status_code = &entry[..2];
        let mut relative_path = &entry[3..];

        if status_code.as_bytes().contains(&b'D') {
            index += 1;
            continue;
        }

        if status_code
            .as_bytes()
            .iter()
            .any(|code| matches!(code, b'R' | b'C'))
        {
            if index + 1 >= entries.len() {
                break;
            }

            index += 1;
            relative_path = entries[index];
        }

        let path = repo_root.join(relative_path);
        if path.is_file() && should_process_file(repo_root, &path) {
            selected.push(path);
        }

        index += 1;
    }

    selected.sort();
    selected.dedup();
    Ok(selected)
}

fn should_process_file(root: &Path, path: &Path) -> bool {
    if should_skip_path(path) {
        return false;
    }

    let relative_path = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    if should_skip_relative_path(&relative_path) {
        return false;
    }

    if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
        let extension = format!(".{}", extension.to_ascii_lowercase());
        if BINARY_EXTENSIONS.contains(&extension.as_str()) {
            return false;
        }
    }

    true
}

fn should_skip_relative_path(relative_path: &str) -> bool {
    relative_path.split('/').any(|segment| {
        EXCLUDED_DIRS
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(segment))
    })
}

fn should_skip_path(path: &Path) -> bool {
    path.components().any(|component| match component {
        Component::Normal(value) => {
            let value = value.to_string_lossy();
            EXCLUDED_DIRS
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(&value))
        }
        _ => false,
    })
}

fn normalize_file(repo_root: &Path, file_path: &Path, check_only: bool) -> anyhow::Result<bool> {
    let bytes =
        fs::read(file_path).with_context(|| format!("failed to read '{}'", file_path.display()))?;
    if bytes.contains(&0) {
        return Ok(false);
    }

    let Ok(text) = String::from_utf8(bytes) else {
        return Ok(false);
    };

    let preferred_newline = detect_preferred_newline(&text);
    let keep_final_newline =
        resolve_insert_final_newline_policy(repo_root, file_path)?.unwrap_or(false);
    let normalized = normalize_text(&text, preferred_newline, keep_final_newline);
    if normalized == text {
        return Ok(false);
    }

    if !check_only {
        fs::write(file_path, normalized)
            .with_context(|| format!("failed to write '{}'", file_path.display()))?;
    }
    Ok(true)
}

fn normalize_text(text: &str, preferred_newline: &str, keep_final_newline: bool) -> String {
    let trimmed = trim_terminal_whitespace(text);
    if keep_final_newline && !trimmed.is_empty() {
        format!("{trimmed}{preferred_newline}")
    } else {
        trimmed
    }
}

fn trim_terminal_whitespace(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut end = bytes.len();

    loop {
        while end > 0 && matches!(bytes[end - 1], b' ' | b'\t') {
            end -= 1;
        }

        if end == 0 {
            break;
        }

        match bytes[end - 1] {
            b'\n' => {
                end -= 1;
                if end > 0 && bytes[end - 1] == b'\r' {
                    end -= 1;
                }
            }
            b'\r' => {
                end -= 1;
            }
            _ => break,
        }
    }

    text[..end].to_string()
}

fn detect_preferred_newline(text: &str) -> &'static str {
    if text.contains("\r\n") {
        "\r\n"
    } else if text.contains('\n') {
        "\n"
    } else if text.contains('\r') {
        "\r"
    } else {
        "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_preferred_newline, normalize_text, trim_terminal_whitespace};

    #[test]
    fn test_trim_terminal_whitespace_removes_trailing_blank_lines_and_spaces() {
        assert_eq!(trim_terminal_whitespace("alpha   \n\n"), "alpha");
        assert_eq!(trim_terminal_whitespace("alpha\r\n\r\n"), "alpha");
        assert_eq!(trim_terminal_whitespace("alpha  "), "alpha");
    }

    #[test]
    fn test_normalize_text_applies_final_newline_only_when_requested() {
        assert_eq!(normalize_text("alpha   \n\n", "\n", false), "alpha");
        assert_eq!(normalize_text("alpha   \n\n", "\n", true), "alpha\n");
    }

    #[test]
    fn test_detect_preferred_newline_prefers_existing_line_endings() {
        assert_eq!(detect_preferred_newline("alpha\r\nbeta"), "\r\n");
        assert_eq!(detect_preferred_newline("alpha\nbeta"), "\n");
        assert_eq!(detect_preferred_newline("alpha\rbeta"), "\r");
        assert_eq!(detect_preferred_newline("alpha"), "\n");
    }
}
