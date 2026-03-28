//! Tests for runtime install profile catalog helpers.

use nettoolskit_core::runtime_install_profiles::{
    read_runtime_install_profile_catalog, resolve_runtime_install_profile,
    runtime_install_profile_catalog_path,
};
use std::fs;
use tempfile::TempDir;

fn write_runtime_install_profile_catalog(repo_root: &std::path::Path) {
    let governance_dir = repo_root.join(".github/governance");
    fs::create_dir_all(&governance_dir).expect("governance directory should be created");
    fs::write(
        governance_dir.join("runtime-install-profiles.json"),
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}},"all":{"description":"all profile","install":{"bootstrap":true,"globalVscodeSettings":true,"globalVscodeSnippets":true,"localGitHooks":true,"globalGitAliases":true,"healthcheck":true},"runtime":{"github":true,"codex":true,"claude":true}}}}"#,
    )
    .expect("catalog should be written");
}

#[test]
fn test_runtime_install_profile_catalog_path_points_to_governance_file() {
    let repo = TempDir::new().expect("temporary repository should be created");

    let path = runtime_install_profile_catalog_path(repo.path());

    assert_eq!(
        path,
        repo.path()
            .join(".github/governance/runtime-install-profiles.json")
    );
}

#[test]
fn test_read_runtime_install_profile_catalog_reads_default_and_names() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_runtime_install_profile_catalog(repo.path());

    let catalog_info =
        read_runtime_install_profile_catalog(repo.path()).expect("catalog should be readable");

    assert_eq!(catalog_info.default_profile, "none");
    assert_eq!(catalog_info.profile_names, vec!["all", "none"]);
}

#[test]
fn test_resolve_runtime_install_profile_uses_fallback_and_exposes_toggles() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_runtime_install_profile_catalog(repo.path());

    let profile = resolve_runtime_install_profile(repo.path(), None, Some("all"))
        .expect("profile should resolve");

    assert_eq!(profile.name, "all");
    assert!(profile.install_bootstrap);
    assert!(profile.install_global_vscode_settings);
    assert!(profile.enable_github_runtime);
    assert!(profile.enable_codex_runtime);
    assert!(profile.enable_claude_runtime);
}

#[test]
fn test_resolve_runtime_install_profile_rejects_unknown_profile() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_runtime_install_profile_catalog(repo.path());

    let error = resolve_runtime_install_profile(repo.path(), Some("missing"), None)
        .expect_err("unknown profile should fail");

    assert!(error
        .to_string()
        .contains("unknown runtime profile 'missing'"));
}
