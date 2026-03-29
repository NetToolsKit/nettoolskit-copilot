//! Runtime pre-commit EOF hygiene.

use anyhow::{anyhow, Context};
use nettoolskit_core::editorconfig::resolve_insert_final_newline_policy;
use nettoolskit_core::path_utils::repository::resolve_repository_root;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::error::RuntimePreCommitEofHygieneCommandError;
use crate::hooks::eof_settings::resolve_effective_git_hook_eof_mode;

/// Request payload for `invoke-pre-commit-eof-hygiene`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimePreCommitEofHygieneRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit EOF catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit global settings path.
    pub global_settings_path: Option<PathBuf>,
}

/// Runtime pre-commit EOF result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimePreCommitEofHygieneStatus {
    /// Command passed and completed any required restaging.
    Passed,
    /// Command skipped because autofix was disabled or no staged files existed.
    Skipped,
    /// Command failed because mixed staged/unstaged files blocked restaging.
    Failed,
}

/// Result payload for `invoke-pre-commit-eof-hygiene`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePreCommitEofHygieneResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective EOF mode name.
    pub mode_name: String,
    /// Effective mode resolution source.
    pub mode_source: String,
    /// Whether the effective mode enables autofix.
    pub auto_fix_staged_files: bool,
    /// Number of staged files discovered.
    pub staged_file_count: usize,
    /// Number of files trimmed in-place.
    pub trimmed_file_count: usize,
    /// Repository-relative files blocked because they had staged and unstaged changes.
    pub blocked_files: Vec<String>,
    /// Optional skip reason.
    pub skipped_reason: Option<String>,
    /// Final command status.
    pub status: RuntimePreCommitEofHygieneStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Apply the runtime-managed pre-commit EOF hygiene flow.
///
/// # Errors
///
/// Returns [`RuntimePreCommitEofHygieneCommandError`] when repository or mode
/// resolution fails, when git state inspection fails, or when file trim/restage
/// work cannot complete.
pub fn invoke_pre_commit_eof_hygiene(
    request: &RuntimePreCommitEofHygieneRequest,
) -> Result<RuntimePreCommitEofHygieneResult, RuntimePreCommitEofHygieneCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimePreCommitEofHygieneCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| RuntimePreCommitEofHygieneCommandError::ResolveWorkspaceRoot { source },
        )?;
    let effective_mode = resolve_effective_git_hook_eof_mode(
        &repo_root,
        request.catalog_path.as_deref(),
        request.global_settings_path.as_deref(),
    )
    .map_err(|source| RuntimePreCommitEofHygieneCommandError::ResolveMode { source })?;

    if !effective_mode.auto_fix_staged_files {
        return Ok(RuntimePreCommitEofHygieneResult {
            repo_root,
            mode_name: effective_mode.name,
            mode_source: effective_mode.source,
            auto_fix_staged_files: false,
            staged_file_count: 0,
            trimmed_file_count: 0,
            blocked_files: Vec::new(),
            skipped_reason: Some("autofix disabled".to_string()),
            status: RuntimePreCommitEofHygieneStatus::Skipped,
            exit_code: 0,
        });
    }

    let staged_files = enumerate_staged_files(&repo_root)
        .map_err(|source| RuntimePreCommitEofHygieneCommandError::InspectGitState { source })?;
    if staged_files.is_empty() {
        return Ok(RuntimePreCommitEofHygieneResult {
            repo_root,
            mode_name: effective_mode.name,
            mode_source: effective_mode.source,
            auto_fix_staged_files: true,
            staged_file_count: 0,
            trimmed_file_count: 0,
            blocked_files: Vec::new(),
            skipped_reason: Some("no staged files".to_string()),
            status: RuntimePreCommitEofHygieneStatus::Skipped,
            exit_code: 0,
        });
    }

    let blocked_files = find_mixed_stage_files(&repo_root, &staged_files)
        .map_err(|source| RuntimePreCommitEofHygieneCommandError::InspectGitState { source })?;
    if !blocked_files.is_empty() {
        return Ok(RuntimePreCommitEofHygieneResult {
            repo_root,
            mode_name: effective_mode.name,
            mode_source: effective_mode.source,
            auto_fix_staged_files: true,
            staged_file_count: staged_files.len(),
            trimmed_file_count: 0,
            blocked_files,
            skipped_reason: None,
            status: RuntimePreCommitEofHygieneStatus::Failed,
            exit_code: 1,
        });
    }

    let trimmed_file_count = normalize_staged_files(&repo_root, &staged_files)
        .map_err(|source| RuntimePreCommitEofHygieneCommandError::NormalizeFiles { source })?;
    restage_files(&repo_root, &staged_files)
        .map_err(|source| RuntimePreCommitEofHygieneCommandError::RestageFiles { source })?;

    Ok(RuntimePreCommitEofHygieneResult {
        repo_root,
        mode_name: effective_mode.name,
        mode_source: effective_mode.source,
        auto_fix_staged_files: true,
        staged_file_count: staged_files.len(),
        trimmed_file_count,
        blocked_files: Vec::new(),
        skipped_reason: None,
        status: RuntimePreCommitEofHygieneStatus::Passed,
        exit_code: 0,
    })
}

fn enumerate_staged_files(repo_root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let output = run_git(
        repo_root,
        &[
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=ACMR",
            "-z",
        ],
    )?;
    let raw = String::from_utf8_lossy(&output.stdout);
    let mut staged_files = raw
        .split('\0')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| repo_root.join(entry))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    staged_files.sort();
    staged_files.dedup();
    Ok(staged_files)
}

fn find_mixed_stage_files(
    repo_root: &Path,
    staged_files: &[PathBuf],
) -> anyhow::Result<Vec<String>> {
    let mut blocked_files = Vec::new();
    for staged_file in staged_files {
        let relative_path = to_repo_relative_path(repo_root, staged_file);
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .arg("diff")
            .arg("--quiet")
            .arg("--")
            .arg(&relative_path)
            .output()
            .with_context(|| format!("failed to inspect unstaged changes for '{relative_path}'"))?;

        match output.status.code() {
            Some(0) => {}
            Some(1) => blocked_files.push(relative_path),
            _ => {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                return Err(anyhow!(if stderr.is_empty() {
                    format!("could not inspect unstaged changes for '{relative_path}'")
                } else {
                    stderr
                }));
            }
        }
    }

    Ok(blocked_files)
}

fn normalize_staged_files(repo_root: &Path, staged_files: &[PathBuf]) -> anyhow::Result<usize> {
    let mut trimmed_file_count = 0usize;
    for staged_file in staged_files {
        if normalize_text_file(repo_root, staged_file)? {
            trimmed_file_count += 1;
        }
    }

    Ok(trimmed_file_count)
}

fn normalize_text_file(repo_root: &Path, path: &Path) -> anyhow::Result<bool> {
    let bytes = fs::read(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    if bytes.contains(&0) {
        return Ok(false);
    }

    let Ok(document) = String::from_utf8(bytes) else {
        return Ok(false);
    };
    let keep_final_newline = resolve_insert_final_newline_policy(repo_root, path)?.unwrap_or(false);
    let normalized = normalize_document(&document, keep_final_newline);
    if normalized == document {
        return Ok(false);
    }

    fs::write(path, normalized).with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(true)
}

fn normalize_document(document: &str, keep_final_newline: bool) -> String {
    let mut lines = document
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .split('\n')
        .map(|line| line.trim_end_matches([' ', '\t']).to_string())
        .collect::<Vec<_>>();

    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }

    let normalized = lines.join("\n");
    if keep_final_newline && !normalized.is_empty() {
        format!("{normalized}\n")
    } else {
        normalized
    }
}

fn restage_files(repo_root: &Path, staged_files: &[PathBuf]) -> anyhow::Result<()> {
    for staged_file in staged_files {
        let relative_path = to_repo_relative_path(repo_root, staged_file);
        run_git(repo_root, &["add", "--", &relative_path]).with_context(|| {
            format!("failed to restage '{relative_path}' after EOF normalization")
        })?;
    }

    Ok(())
}

fn run_git(repo_root: &Path, arguments: &[&str]) -> anyhow::Result<Output> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .output()
        .with_context(|| format!("failed to start git with args {:?}", arguments))?;
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

    Ok(output)
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::normalize_document;

    #[test]
    fn test_normalize_document_trims_whitespace_and_trailing_blank_lines() {
        assert_eq!(
            normalize_document("hello  \nworld\t\n\n\n", false),
            "hello\nworld"
        );
        assert_eq!(normalize_document("single line\n", false), "single line");
        assert_eq!(normalize_document("", false), "");
    }

    #[test]
    fn test_normalize_document_keeps_final_newline_when_requested() {
        assert_eq!(
            normalize_document("pub fn sample() {}\n\n", true),
            "pub fn sample() {}\n"
        );
    }
}
