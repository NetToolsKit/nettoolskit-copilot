//! Tests for runtime execution context helpers.

use nettoolskit_core::runtime_execution::{
    resolve_runtime_execution_context, resolved_runtime_target_arguments,
    runtime_target_arguments,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn normalize_path(path: &std::path::Path) -> String {
    path.to_string_lossy()
        .replace("\\\\?\\", "")
        .replace('\\', "/")
}

fn write_runtime_install_profile_catalog(repo_root: &std::path::Path) {
    let governance_dir = repo_root.join(".github/governance");
    fs::create_dir_all(&governance_dir).expect("governance directory should be created");
    fs::write(
        governance_dir.join("runtime-install-profiles.json"),
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}},"github":{"description":"github profile","install":{"bootstrap":true,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":true},"runtime":{"github":true,"codex":false,"claude":false}}}}"#,
    )
    .expect("catalog should be written");
}

#[test]
fn test_resolve_runtime_execution_context_builds_sources_and_targets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo.path().join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo.path().join("scripts")).expect("scripts directory should be created");
    write_runtime_install_profile_catalog(repo.path());

    let context = resolve_runtime_execution_context(
        Some(repo.path()),
        Some("github"),
        None,
        Some(Path::new(".runtime/github")),
        Some(Path::new(".runtime/codex")),
        Some(Path::new(".runtime/agents-skills")),
        Some(Path::new(".runtime/copilot-skills")),
        Some(Path::new(".runtime/claude")),
        repo.path(),
    )
    .expect("execution context should resolve");

    assert_eq!(context.resolved_repo_root, repo.path().canonicalize().expect("repo should canonicalize"));
    assert_eq!(context.runtime_profile.name, "github");
    assert!(context.runtime_profile.enable_github_runtime);
    assert_eq!(
        normalize_path(&context.targets.github_runtime_root),
        normalize_path(&repo.path().join(".runtime/github"))
    );
    assert_eq!(
        normalize_path(&context.sources.github_root),
        normalize_path(&repo.path().join(".github"))
    );
    assert_eq!(
        normalize_path(&context.sources.codex_root),
        normalize_path(&repo.path().join(".codex"))
    );
    assert_eq!(
        normalize_path(&context.sources.common_scripts_root),
        normalize_path(&repo.path().join("scripts/common"))
    );
}

#[test]
fn test_runtime_target_arguments_include_optional_fields() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo.path().join(".codex")).expect("codex directory should be created");
    write_runtime_install_profile_catalog(repo.path());

    let context = resolve_runtime_execution_context(
        Some(repo.path()),
        Some("github"),
        None,
        Some(Path::new(".runtime/github")),
        Some(Path::new(".runtime/codex")),
        Some(Path::new(".runtime/agents-skills")),
        Some(Path::new(".runtime/copilot-skills")),
        Some(Path::new(".runtime/claude")),
        repo.path(),
    )
    .expect("execution context should resolve");

    let arguments = runtime_target_arguments(&context, true, true);

    assert_eq!(arguments.repo_root, Some(repo.path().canonicalize().expect("repo should canonicalize")));
    assert_eq!(arguments.runtime_profile.as_deref(), Some("github"));
    assert_eq!(
        normalize_path(&arguments.target_codex_path),
        normalize_path(&repo.path().join(".runtime/codex"))
    );
}

#[test]
fn test_resolved_runtime_target_arguments_resolve_relative_paths_against_repo_root() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo.path().join(".codex")).expect("codex directory should be created");
    write_runtime_install_profile_catalog(repo.path());

    let context = resolve_runtime_execution_context(
        Some(repo.path()),
        Some("github"),
        None,
        Some(Path::new(".runtime/github")),
        Some(Path::new(".runtime/codex")),
        Some(Path::new(".runtime/agents-skills")),
        Some(Path::new(".runtime/copilot-skills")),
        Some(Path::new(".runtime/claude")),
        repo.path(),
    )
    .expect("execution context should resolve");

    let arguments = resolved_runtime_target_arguments(
        &context,
        &repo.path().join("subdir"),
        true,
        true,
    );

    assert_eq!(arguments.repo_root, Some(repo.path().join("subdir")));
    assert_eq!(
        normalize_path(&arguments.target_github_path),
        normalize_path(&repo.path().join(".runtime/github"))
    );
    assert_eq!(arguments.runtime_profile.as_deref(), Some("github"));
}