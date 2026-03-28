//! Tests for repository path helpers.

use nettoolskit_core::path_utils::repository::{
    convert_to_relative_repo_path, parent_directory_path, resolve_explicit_or_git_root,
    resolve_repository_root, resolve_solution_or_layout_root, resolve_workspace_root,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_resolve_repository_root_detects_layout_from_script_root() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo.path().join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo.path().join("scripts/runtime"))
        .expect("script directory should be created");

    let resolved = resolve_repository_root(
        None,
        Some(&repo.path().join("scripts/runtime")),
        repo.path(),
    )
    .expect("repository root should be detected");

    assert_eq!(
        resolved,
        repo.path()
            .canonicalize()
            .expect("repo should canonicalize")
    );
}

#[test]
fn test_resolve_workspace_root_prefers_explicit_directory() {
    let workspace = TempDir::new().expect("temporary workspace should be created");

    let resolved = resolve_workspace_root(Some(workspace.path()), None)
        .expect("workspace root should resolve");

    assert_eq!(
        resolved,
        workspace
            .path()
            .canonicalize()
            .expect("workspace should canonicalize")
    );
}

#[test]
fn test_resolve_solution_or_layout_root_detects_src_and_github_layout() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo.path().join("src")).expect("src directory should be created");
    fs::create_dir_all(repo.path().join("tools/deep")).expect("nested directory should be created");

    let resolved = resolve_solution_or_layout_root(None, &repo.path().join("tools/deep"))
        .expect("layout root should be detected");

    assert_eq!(
        resolved,
        repo.path()
            .canonicalize()
            .expect("repo should canonicalize")
    );
}

#[test]
fn test_resolve_explicit_or_git_root_returns_explicit_path_when_provided() {
    let repo = TempDir::new().expect("temporary repository should be created");

    let resolved = resolve_explicit_or_git_root(Some(repo.path()), Path::new("."))
        .expect("explicit path should be accepted");

    assert_eq!(
        resolved,
        repo.path()
            .canonicalize()
            .expect("repo should canonicalize")
    );
}

#[test]
fn test_convert_to_relative_repo_path_normalizes_separators() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let nested_path = repo.path().join("planning/active/plan.md");
    fs::create_dir_all(nested_path.parent().expect("parent path should exist"))
        .expect("planning directory should be created");
    fs::write(&nested_path, "content").expect("plan file should be written");

    let relative_path = convert_to_relative_repo_path(repo.path(), &nested_path)
        .expect("path should become relative");

    assert_eq!(relative_path, "planning/active/plan.md");
}

#[test]
fn test_parent_directory_path_returns_parent_when_available() {
    let parent = parent_directory_path(Path::new("planning/active/plan.md"))
        .expect("parent path should exist");

    assert_eq!(parent, Path::new("planning/active"));
}
