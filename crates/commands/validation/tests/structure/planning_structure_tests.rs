//! Tests for planning structure validation.

use nettoolskit_validation::{
    invoke_validate_planning_structure, ValidatePlanningStructureRequest, ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo_root(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

#[test]
fn test_invoke_validate_planning_structure_passes_for_versioned_workspace() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_file(&repo.path().join("planning/README.md"), "# planning");
    write_file(&repo.path().join("planning/specs/README.md"), "# specs");

    let result = invoke_validate_planning_structure(&ValidatePlanningStructureRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("planning structure validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert!(result.missing_required_files.is_empty());
    assert!(result.missing_required_directories.is_empty());
}

#[test]
fn test_invoke_validate_planning_structure_reports_missing_required_artifacts() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());

    let result = invoke_validate_planning_structure(&ValidatePlanningStructureRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("planning structure validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(!result.failures.is_empty());
}

#[test]
fn test_invoke_validate_planning_structure_converts_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    fs::create_dir_all(repo.path().join(".temp/planning"))
        .expect("legacy planning directory should be created");

    let result = invoke_validate_planning_structure(&ValidatePlanningStructureRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
    })
    .expect("planning structure validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.legacy_temp_planning_detected);
    assert!(!result.warnings.is_empty());
}