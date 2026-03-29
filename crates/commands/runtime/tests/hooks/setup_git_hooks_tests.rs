//! Tests for runtime Git hook setup.

use nettoolskit_runtime::{invoke_setup_git_hooks, RuntimeSetupGitHooksRequest};
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

fn initialize_hook_support_tree(repo_root: &Path) {
    write_file(&repo_root.join(".githooks/pre-commit"), "#!/usr/bin/env sh");
    write_file(
        &repo_root.join(".githooks/post-commit"),
        "#!/usr/bin/env sh",
    );
    write_file(&repo_root.join(".githooks/post-merge"), "#!/usr/bin/env sh");
    write_file(
        &repo_root.join(".githooks/post-checkout"),
        "#!/usr/bin/env sh",
    );
    write_file(
        &repo_root.join("scripts/git-hooks/invoke-pre-commit-eof-hygiene.ps1"),
        "Write-Output 'hook'",
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
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn read_global_hooks_path(repo_root: &Path, git_config_global_path: &Path) -> Option<String> {
    let output = Command::new("git")
        .env("GIT_CONFIG_GLOBAL", git_config_global_path)
        .arg("-C")
        .arg(repo_root)
        .args(["config", "--global", "--get", "core.hooksPath"])
        .output()
        .expect("git config should execute");

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn write_local_eof_settings(repo_root: &Path, selected_mode: &str) {
    write_file(
        &local_settings_path(repo_root),
        &format!(
            r#"{{"schemaVersion":1,"selectedMode":"{selected_mode}","selectedScope":"local-repo","updatedAt":"2026-03-27T00:00:00.0000000Z","catalogPath":"catalog.json"}}"#
        ),
    );
}

fn new_repo() -> TempDir {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_git_repo(repo.path());
    initialize_hook_catalog(repo.path());
    initialize_hook_support_tree(repo.path());
    repo
}

#[test]
fn test_invoke_setup_git_hooks_local_install_sets_hooks_path_and_local_eof_settings() {
    // Arrange
    let repo = new_repo();

    // Act
    let result = invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        eof_hygiene_mode: Some("autofix".to_string()),
        ..RuntimeSetupGitHooksRequest::default()
    })
    .expect("setup git hooks should execute");

    // Assert
    assert_eq!(
        result.repo_root,
        fs::canonicalize(repo.path()).expect("repo root should canonicalize")
    );
    assert_eq!(result.scope_name, "local-repo");
    assert_eq!(result.mode_name.as_deref(), Some("autofix"));
    assert!(!result.uninstall);
    assert!(result.selection_persisted);
    assert_eq!(result.hooks_path.as_deref(), Some(".githooks"));
    assert_eq!(result.git_config_global_path, None);
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
    invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        eof_hygiene_mode: Some("autofix".to_string()),
        ..RuntimeSetupGitHooksRequest::default()
    })
    .expect("setup git hooks install should execute");

    // Act
    let result = invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        uninstall: true,
        ..RuntimeSetupGitHooksRequest::default()
    })
    .expect("setup git hooks uninstall should execute");

    // Assert
    assert_eq!(result.scope_name, "local-repo");
    assert!(result.uninstall);
    assert_eq!(result.mode_name, None);
    assert!(!result.selection_persisted);
    assert_eq!(result.hooks_path, None);
    assert_eq!(result.git_config_global_path, None);
    assert_eq!(read_local_hooks_path(repo.path()), None);
    assert!(!local_settings_path(repo.path()).exists());
}

#[test]
fn test_invoke_setup_git_hooks_global_scope_configures_managed_hooks_path() {
    // Arrange
    let repo = new_repo();
    let global_settings_path = repo
        .path()
        .join("isolated-global/git-hook-eof-settings.json");
    let global_hooks_path = repo.path().join("isolated-global/hooks");
    let git_config_global_path = repo.path().join("isolated-global/.gitconfig");
    write_local_eof_settings(repo.path(), "manual");

    let output = Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(["config", "--local", "core.hooksPath", ".githooks"])
        .output()
        .expect("git config should execute");
    assert!(output.status.success(), "git config should succeed");

    // Act
    let result = invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        eof_hygiene_mode: Some("autofix".to_string()),
        eof_hygiene_scope: Some("global".to_string()),
        global_settings_path: Some(global_settings_path.clone()),
        git_hooks_path: Some(global_hooks_path.clone()),
        git_config_global_path: Some(git_config_global_path.clone()),
        ..RuntimeSetupGitHooksRequest::default()
    })
    .expect("setup git hooks should execute");

    // Assert
    assert_eq!(result.scope_name, "global");
    assert_eq!(result.mode_name.as_deref(), Some("autofix"));
    assert!(result.selection_persisted);
    assert_eq!(
        result.hooks_path.as_deref().map(PathBuf::from),
        Some(global_hooks_path.clone())
    );
    assert_eq!(
        result.git_config_global_path,
        Some(git_config_global_path.clone())
    );
    assert!(global_settings_path.is_file());
    assert_eq!(read_local_hooks_path(repo.path()), None);
    assert!(!local_settings_path(repo.path()).exists());
    assert_eq!(
        read_global_hooks_path(repo.path(), &git_config_global_path).map(PathBuf::from),
        Some(global_hooks_path.clone())
    );
    assert!(global_hooks_path.join("pre-commit").is_file());
    let pre_commit_hook = fs::read_to_string(global_hooks_path.join("pre-commit"))
        .expect("managed global pre-commit hook should be readable");
    assert!(pre_commit_hook.contains("CODEX_GIT_HOOK_EOF_CATALOG_PATH"));
    assert!(!pre_commit_hook.contains("CODEX_GIT_HOOK_EOF_TRIM_SCRIPT_PATH"));

    let settings =
        fs::read_to_string(global_settings_path).expect("global settings should be readable");
    assert!(settings.contains(r#""selectedMode":"autofix""#));
    assert!(settings.contains(r#""selectedScope":"global""#));
}

#[test]
fn test_invoke_setup_git_hooks_global_uninstall_removes_managed_hook_path() {
    // Arrange
    let repo = new_repo();
    let global_settings_path = repo
        .path()
        .join("isolated-global/git-hook-eof-settings.json");
    let global_hooks_path = repo.path().join("isolated-global/hooks");
    let git_config_global_path = repo.path().join("isolated-global/.gitconfig");
    invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        eof_hygiene_mode: Some("autofix".to_string()),
        eof_hygiene_scope: Some("global".to_string()),
        global_settings_path: Some(global_settings_path.clone()),
        git_hooks_path: Some(global_hooks_path.clone()),
        git_config_global_path: Some(git_config_global_path.clone()),
        ..RuntimeSetupGitHooksRequest::default()
    })
    .expect("setup git hooks global install should execute");

    // Act
    let result = invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        eof_hygiene_scope: Some("global".to_string()),
        uninstall: true,
        global_settings_path: Some(global_settings_path.clone()),
        git_hooks_path: Some(global_hooks_path.clone()),
        git_config_global_path: Some(git_config_global_path.clone()),
        ..RuntimeSetupGitHooksRequest::default()
    })
    .expect("setup git hooks global uninstall should execute");

    // Assert
    assert_eq!(result.scope_name, "global");
    assert!(result.uninstall);
    assert_eq!(result.hooks_path, None);
    assert_eq!(
        result.git_config_global_path,
        Some(git_config_global_path.clone())
    );
    assert!(!global_settings_path.exists());
    assert!(!global_hooks_path.exists());
    assert_eq!(read_local_hooks_path(repo.path()), None);
    assert_eq!(
        read_global_hooks_path(repo.path(), &git_config_global_path),
        None
    );
}
