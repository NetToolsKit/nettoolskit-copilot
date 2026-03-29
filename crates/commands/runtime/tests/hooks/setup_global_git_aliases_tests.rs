//! Tests for runtime global Git alias setup.

use nettoolskit_runtime::{invoke_setup_global_git_aliases, RuntimeSetupGlobalGitAliasesRequest};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn runtime_binary_file_name() -> &'static str {
    if cfg!(windows) {
        "ntk.exe"
    } else {
        "ntk"
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_runtime_alias_repo(repo_root: &Path, target_codex_path: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(target_codex_path.join("bin"))
        .expect("codex runtime bin directory should be created");
    write_file(
        &target_codex_path
            .join("bin")
            .join(runtime_binary_file_name()),
        "binary",
    );
}

fn read_global_alias(config_path: &Path, alias_name: &str) -> Option<String> {
    let output = Command::new("git")
        .env("GIT_CONFIG_GLOBAL", config_path)
        .args([
            "config",
            "--global",
            "--get",
            &format!("alias.{alias_name}"),
        ])
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

#[test]
fn test_invoke_setup_global_git_aliases_installs_trim_alias_into_isolated_config() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let target_codex = repo.path().join(".runtime/codex");
    let git_config = repo.path().join("isolated-git-config");
    initialize_runtime_alias_repo(repo.path(), &target_codex);

    let result = invoke_setup_global_git_aliases(&RuntimeSetupGlobalGitAliasesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(target_codex.clone()),
        git_config_global_path: Some(git_config.clone()),
        ..RuntimeSetupGlobalGitAliasesRequest::default()
    })
    .expect("setup global git aliases should execute");

    assert_eq!(
        result.repo_root,
        fs::canonicalize(repo.path()).expect("repo root should canonicalize")
    );
    assert_eq!(result.target_codex_path, target_codex);
    assert!(!result.uninstall);
    assert_eq!(result.git_config_global_path, Some(git_config.clone()));
    assert!(result.removed_aliases.is_empty());
    assert_eq!(result.configured_aliases.len(), 1);

    let alias_value = result
        .configured_aliases
        .get("trim-eof")
        .expect("trim-eof alias should be configured");
    assert!(alias_value.contains(runtime_binary_file_name()));
    assert!(alias_value.contains("runtime trim-trailing-blank-lines"));
    assert!(alias_value.contains("--repo-root"));
    assert!(alias_value.contains("--git-changed-only"));

    assert_eq!(
        read_global_alias(&git_config, "trim-eof"),
        Some(alias_value.clone())
    );
}

#[test]
fn test_invoke_setup_global_git_aliases_uninstalls_trim_alias_from_isolated_config() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let target_codex = repo.path().join(".runtime/codex");
    let git_config = repo.path().join("isolated-git-config");
    initialize_runtime_alias_repo(repo.path(), &target_codex);

    invoke_setup_global_git_aliases(&RuntimeSetupGlobalGitAliasesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(target_codex.clone()),
        git_config_global_path: Some(git_config.clone()),
        ..RuntimeSetupGlobalGitAliasesRequest::default()
    })
    .expect("initial alias install should execute");

    let result = invoke_setup_global_git_aliases(&RuntimeSetupGlobalGitAliasesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(target_codex),
        uninstall: true,
        git_config_global_path: Some(git_config.clone()),
    })
    .expect("alias uninstall should execute");

    assert!(result.uninstall);
    assert!(result.configured_aliases.is_empty());
    assert_eq!(result.removed_aliases, vec!["trim-eof".to_string()]);
    assert_eq!(read_global_alias(&git_config, "trim-eof"), None);
}
