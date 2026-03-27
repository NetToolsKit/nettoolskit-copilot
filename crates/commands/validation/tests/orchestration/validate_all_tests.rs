//! Tests for validation `validate-all` orchestration.

use nettoolskit_validation::{
    invoke_validate_all, ValidateAllRequest, ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_validation_profile_catalog(repo_root: &std::path::Path, check_order: &[&str]) {
    let checks = check_order
        .iter()
        .map(|check| format!("\"{check}\""))
        .collect::<Vec<_>>()
        .join(",");
    write_file(
        &repo_root.join(".github/governance/validation-profiles.json"),
        &format!(
            "{{\"version\":1,\"defaultProfile\":\"test\",\"profiles\":[{{\"id\":\"test\",\"warningOnly\":false,\"checkOrder\":[{checks}]}}]}}"
        ),
    );
}

fn initialize_repo_layout(repo_root: &std::path::Path, check_order: &[&str]) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join("scripts/validation"))
        .expect("validation directory should be created");
    write_validation_profile_catalog(repo_root, check_order);
}

#[test]
fn test_invoke_validate_all_runs_selected_profile_and_writes_report_and_ledger() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-policy"]);
    write_file(
        &repo.path().join("scripts/validation/validate-policy.ps1"),
        "param([string]$RepoRoot)\nexit 0",
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.profile_id, "test");
    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert!(result.output_path.is_file());
    assert!(result
        .ledger_path
        .as_ref()
        .expect("ledger path should exist")
        .is_file());
    assert!(result.report_json.contains("\"profile\": \"test\""));
}

#[test]
fn test_invoke_validate_all_converts_missing_script_to_warning_when_warning_only_enabled() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-policy"]);

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.warning_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.checks[0]
        .error
        .as_deref()
        .expect("missing script error should be present")
        .contains("script not found"));
}

#[test]
fn test_invoke_validate_all_archives_broken_ledger_and_enforces_failure() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-policy"]);
    write_file(
        &repo.path().join("scripts/validation/validate-policy.ps1"),
        "param([string]$RepoRoot)\nexit 1",
    );
    write_file(
        &repo.path().join(".temp/audit/validation-ledger.jsonl"),
        "not-json",
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.failed_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.archived_broken_ledger_path.is_some());
    assert!(result
        .ledger_path
        .as_ref()
        .expect("ledger path should exist")
        .is_file());
}