//! Tests for the planned runtime Git hook setup contract.
//!
//! These fixtures document the expected observable side effects for the future
//! native `invoke_setup_git_hooks` command surface.

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

fn initialize_hook_catalog(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github/governance"))
        .expect("governance directory should be created");
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

fn read_local_hooks_path(repo_root: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["config", "--local", "--get", "core.hooksPath"])
        .output()
        .expect("git config should execute");

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}

fn write_local_eof_settings(repo_root: &Path, selected_mode: &str) {
    write_file(
        &local_settings_path(repo_root),
        &format!(
            r#"{{"schemaVersion":1,"selectedMode":"{selected_mode}","selectedScope":"local-repo","updatedAt":"2026-03-27T00:00:00.0000000Z","catalogPath":"catalog.json"}}"#
        ),
    );
}

fn write_global_eof_settings(global_settings_path: &Path, selected_mode: &str) {
    write_file(
        global_settings_path,
        &format!(
            r#"{{"schemaVersion":1,"selectedMode":"{selected_mode}","selectedScope":"global","updatedAt":"2026-03-27T00:00:00.0000000Z","catalogPath":"catalog.json"}}"#
        ),
    );
}

fn new_repo() -> TempDir {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_git_repo(repo.path());
    initialize_hook_catalog(repo.path());
    repo
}

#[test]
fn test_invoke_setup_git_hooks_local_install_sets_hooks_path_and_local_eof_settings() {
    // Arrange
    let repo = new_repo();

    // Act
    let output = Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(["config", "--local", "core.hooksPath", ".githooks"])
        .output()
        .expect("git config should execute");
    assert!(output.status.success(), "git config should succeed");

    write_local_eof_settings(repo.path(), "autofix");

    // Assert
    assert_eq!(
        read_local_hooks_path(repo.path()),
        Some(".githooks".to_string())
    );
    assert!(local_settings_path(repo.path()).is_file());

    let settings = fs::read_to_string(local_settings_path(repo.path()))
        .expect("local settings should be readable");
    assert!(settings.contains(r#""selectedMode":"autofix""#));
    assert!(settings.contains(r#""selectedScope":"local-repo""#));
}

#[test]
fn test_invoke_setup_git_hooks_local_uninstall_removes_hooks_path_and_local_settings() {
    // Arrange
    let repo = new_repo();

    let output = Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(["config", "--local", "core.hooksPath", ".githooks"])
        .output()
        .expect("git config should execute");
    assert!(output.status.success(), "git config should succeed");
    write_local_eof_settings(repo.path(), "autofix");

    // Act
    let output = Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(["config", "--local", "--unset", "core.hooksPath"])
        .output()
        .expect("git config should execute");
    assert!(output.status.success(), "git config unset should succeed");
    fs::remove_file(local_settings_path(repo.path())).expect("local settings should be removed");

    // Assert
    assert_eq!(read_local_hooks_path(repo.path()), None);
    assert!(!local_settings_path(repo.path()).exists());
}

#[test]
fn test_invoke_setup_git_hooks_global_scope_writes_global_settings_without_local_hooks_path() {
    // Arrange
    let repo = new_repo();
    let global_settings_path = repo.path().join("isolated-global/git-hook-eof-settings.json");

    // Act
    write_global_eof_settings(&global_settings_path, "autofix");

    // Assert
    assert!(global_settings_path.is_file());
    assert_eq!(read_local_hooks_path(repo.path()), None);

    let settings = fs::read_to_string(global_settings_path)
        .expect("global settings should be readable");
    assert!(settings.contains(r#""selectedMode":"autofix""#));
    assert!(settings.contains(r#""selectedScope":"global""#));
}

#[test]
fn test_invoke_setup_git_hooks_global_uninstall_removes_global_settings_without_local_hooks_path() {
    // Arrange
    let repo = new_repo();
    let global_settings_path = repo.path().join("isolated-global/git-hook-eof-settings.json");
    write_global_eof_settings(&global_settings_path, "autofix");

    // Act
    fs::remove_file(&global_settings_path).expect("global settings should be removed");

    // Assert
    assert!(!global_settings_path.exists());
    assert_eq!(read_local_hooks_path(repo.path()), None);
}