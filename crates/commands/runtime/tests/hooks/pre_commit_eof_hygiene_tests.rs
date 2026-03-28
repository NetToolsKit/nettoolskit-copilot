//! Tests for runtime pre-commit EOF hygiene.

use nettoolskit_runtime::{
    invoke_pre_commit_eof_hygiene, RuntimePreCommitEofHygieneRequest,
    RuntimePreCommitEofHygieneStatus,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_git_repo(repo_root: &Path) {
    let output = Command::new("git")
        .arg("init")
        .arg(repo_root)
        .output()
        .expect("git init should execute");
    assert!(output.status.success(), "git init should succeed");
}

fn write_repo_baseline(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github/governance"))
        .expect("governance directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    write_file(
        &repo_root.join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n\n[*.{rs,toml,lock}]\ninsert_final_newline = true\n",
    );
    write_file(
        &repo_root.join(".github/governance/git-hook-eof-modes.json"),
        r#"{
  "schemaVersion": 1,
  "defaultMode": "manual",
  "defaultScope": "local-repo",
  "scopes": {
    "local-repo": { "description": "repo" },
    "global": { "description": "global" }
  },
  "modes": {
    "manual": { "description": "manual", "autoFixStagedFiles": false },
    "autofix": { "description": "autofix", "autoFixStagedFiles": true }
  }
}"#,
    );
}

fn local_settings_path(repo_root: &Path) -> PathBuf {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["rev-parse", "--git-path", "codex-hook-eof-settings.json"])
        .output()
        .expect("git rev-parse should execute");
    assert!(output.status.success(), "git rev-parse should succeed");

    let raw_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let path = PathBuf::from(raw_path);
    if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    }
}

fn write_git_hook_selection(repo_root: &Path, mode_name: &str) {
    let settings_path = local_settings_path(repo_root);
    write_file(
        &settings_path,
        &format!(
            r#"{{"schemaVersion":1,"selectedMode":"{mode_name}","selectedScope":"local-repo","updatedAt":"2026-03-27T00:00:00.0000000Z","catalogPath":"catalog.json"}}"#
        ),
    );
}

fn stage_file(repo_root: &Path, relative_path: &str) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["add", "--", relative_path])
        .output()
        .expect("git add should execute");
    assert!(output.status.success(), "git add should succeed");
}

fn read_staged_blob(repo_root: &Path, relative_path: &str) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["show", &format!(":{relative_path}")])
        .output()
        .expect("git show should execute");
    assert!(output.status.success(), "git show should succeed");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn new_repo() -> TempDir {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_git_repo(repo.path());
    write_repo_baseline(repo.path());
    repo
}

#[test]
fn test_invoke_pre_commit_eof_hygiene_skips_when_autofix_is_disabled() {
    let repo = new_repo();
    write_git_hook_selection(repo.path(), "manual");
    write_file(&repo.path().join("notes.md"), "alpha   \n\n");
    stage_file(repo.path(), "notes.md");

    let result = invoke_pre_commit_eof_hygiene(&RuntimePreCommitEofHygieneRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimePreCommitEofHygieneRequest::default()
    })
    .expect("pre-commit eof hygiene should execute");

    assert_eq!(result.status, RuntimePreCommitEofHygieneStatus::Skipped);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.mode_name, "manual");
    assert_eq!(result.mode_source, "local-settings");
    assert_eq!(result.skipped_reason.as_deref(), Some("autofix disabled"));
}

#[test]
fn test_invoke_pre_commit_eof_hygiene_skips_when_no_staged_files_exist() {
    let repo = new_repo();
    write_git_hook_selection(repo.path(), "autofix");

    let result = invoke_pre_commit_eof_hygiene(&RuntimePreCommitEofHygieneRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimePreCommitEofHygieneRequest::default()
    })
    .expect("pre-commit eof hygiene should execute");

    assert_eq!(result.status, RuntimePreCommitEofHygieneStatus::Skipped);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.mode_name, "autofix");
    assert_eq!(result.mode_source, "local-settings");
    assert_eq!(result.skipped_reason.as_deref(), Some("no staged files"));
}

#[test]
fn test_invoke_pre_commit_eof_hygiene_blocks_mixed_stage_files() {
    let repo = new_repo();
    write_git_hook_selection(repo.path(), "autofix");
    write_file(&repo.path().join("notes.md"), "alpha   \n\n");
    stage_file(repo.path(), "notes.md");
    write_file(&repo.path().join("notes.md"), "alpha   \n\nbeta\n");

    let result = invoke_pre_commit_eof_hygiene(&RuntimePreCommitEofHygieneRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimePreCommitEofHygieneRequest::default()
    })
    .expect("pre-commit eof hygiene should execute");

    assert_eq!(result.status, RuntimePreCommitEofHygieneStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.blocked_files, vec!["notes.md".to_string()]);
}

#[test]
fn test_invoke_pre_commit_eof_hygiene_trims_and_restages_files() {
    let repo = new_repo();
    write_git_hook_selection(repo.path(), "autofix");
    write_file(&repo.path().join("notes.md"), "alpha   \n\n\n");
    stage_file(repo.path(), "notes.md");

    let result = invoke_pre_commit_eof_hygiene(&RuntimePreCommitEofHygieneRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimePreCommitEofHygieneRequest::default()
    })
    .expect("pre-commit eof hygiene should execute");

    assert_eq!(result.status, RuntimePreCommitEofHygieneStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.trimmed_file_count, 1);
    assert_eq!(
        fs::read_to_string(repo.path().join("notes.md")).expect("file should be readable"),
        "alpha"
    );
    assert_eq!(read_staged_blob(repo.path(), "notes.md"), "alpha");
}

#[test]
fn test_invoke_pre_commit_eof_hygiene_keeps_rust_final_newline_when_editorconfig_requires_it() {
    let repo = new_repo();
    write_git_hook_selection(repo.path(), "autofix");
    write_file(&repo.path().join("src/lib.rs"), "pub fn sample() {}  \n\n");
    stage_file(repo.path(), "src/lib.rs");

    let result = invoke_pre_commit_eof_hygiene(&RuntimePreCommitEofHygieneRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimePreCommitEofHygieneRequest::default()
    })
    .expect("pre-commit eof hygiene should execute");

    assert_eq!(result.status, RuntimePreCommitEofHygieneStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.trimmed_file_count, 1);
    assert_eq!(
        fs::read_to_string(repo.path().join("src/lib.rs")).expect("file should be readable"),
        "pub fn sample() {}\n"
    );
    assert_eq!(read_staged_blob(repo.path(), "src/lib.rs"), "pub fn sample() {}\n");
}
