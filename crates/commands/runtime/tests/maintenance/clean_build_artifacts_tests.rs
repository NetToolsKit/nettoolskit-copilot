//! Tests for runtime clean-build-artifacts commands.

use nettoolskit_runtime::{
    invoke_clean_build_artifacts, RuntimeCleanBuildArtifactsRequest,
    RuntimeCleanBuildArtifactsStatus,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn initialize_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

fn seed_artifacts(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".build/cache")).expect(".build should be created");
    fs::create_dir_all(repo_root.join(".deployment/releases"))
        .expect(".deployment should be created");
    fs::create_dir_all(repo_root.join("src/bin")).expect("bin directory should be created");
    fs::create_dir_all(repo_root.join("src/obj")).expect("obj directory should be created");
    fs::write(repo_root.join("src/lib.rs"), "pub fn sample() {}\n")
        .expect("source file should be written");
}

#[test]
fn test_invoke_clean_build_artifacts_removes_artifacts_when_forced() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    seed_artifacts(repo.path());

    let result = invoke_clean_build_artifacts(&RuntimeCleanBuildArtifactsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        force: true,
        ..RuntimeCleanBuildArtifactsRequest::default()
    })
    .expect("clean build artifacts should execute");

    assert_eq!(result.status, RuntimeCleanBuildArtifactsStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert!(result
        .discovered_directories
        .iter()
        .any(|path| path.ends_with(".build")));
    assert!(result
        .removed_directories
        .iter()
        .any(|path| path.ends_with(".deployment")));
    assert!(!repo.path().join(".build").exists());
    assert!(!repo.path().join(".deployment").exists());
    assert!(!repo.path().join("src/bin").exists());
    assert!(!repo.path().join("src/obj").exists());
    assert!(repo.path().join("src/lib.rs").exists());
}

#[test]
fn test_invoke_clean_build_artifacts_reports_confirmation_required_without_force() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    seed_artifacts(repo.path());

    let result = invoke_clean_build_artifacts(&RuntimeCleanBuildArtifactsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimeCleanBuildArtifactsRequest::default()
    })
    .expect("clean build artifacts should execute");

    assert_eq!(
        result.status,
        RuntimeCleanBuildArtifactsStatus::ConfirmationRequired
    );
    assert_eq!(result.exit_code, 1);
    assert!(repo.path().join(".build").exists());
    assert!(repo.path().join(".deployment").exists());
}

#[test]
fn test_invoke_clean_build_artifacts_keeps_artifacts_on_dry_run() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    seed_artifacts(repo.path());

    let result = invoke_clean_build_artifacts(&RuntimeCleanBuildArtifactsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        dry_run: true,
        ..RuntimeCleanBuildArtifactsRequest::default()
    })
    .expect("clean build artifacts should execute");

    assert_eq!(result.status, RuntimeCleanBuildArtifactsStatus::DryRun);
    assert_eq!(result.exit_code, 0);
    assert!(repo.path().join(".build").exists());
    assert!(repo.path().join(".deployment").exists());
    assert!(repo.path().join("src/bin").exists());
}
