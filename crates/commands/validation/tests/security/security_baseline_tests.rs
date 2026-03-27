//! Tests for security::security_baseline module.

use nettoolskit_validation::{
    invoke_validate_security_baseline, ValidateSecurityBaselineRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::security_fixtures::{
    initialize_security_repo, remove_repo_path, write_repo_file,
};

#[test]
fn test_invoke_validate_security_baseline_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_repo(repo.path());

    let result = invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSecurityBaselineRequest::default()
    })
    .expect("security baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.files_scanned, 4);
}

#[test]
fn test_invoke_validate_security_baseline_reports_missing_required_file() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_repo(repo.path());
    remove_repo_path(repo.path(), "CODEOWNERS");

    let result = invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSecurityBaselineRequest::default()
    })
    .expect("security baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing required file: CODEOWNERS")));
}

#[test]
fn test_invoke_validate_security_baseline_reports_forbidden_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_repo(repo.path());
    write_repo_file(repo.path(), "secrets/local.key", "private");

    let result = invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSecurityBaselineRequest::default()
    })
    .expect("security baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Forbidden sensitive file path found: secrets/local.key")));
}

#[test]
fn test_invoke_validate_security_baseline_emits_warning_for_warning_rule() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_repo(repo.path());
    write_repo_file(repo.path(), "docs/notes.md", "password = \"supersecret1\"\n");

    let result = invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSecurityBaselineRequest::default()
    })
    .expect("security baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("matched 'hardcoded-password-assignment'")));
}

#[test]
fn test_invoke_validate_security_baseline_honors_allowed_content_patterns() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_repo(repo.path());
    write_repo_file(repo.path(), "docs/notes.md", "password = \"example-password\"\n");

    let result = invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSecurityBaselineRequest::default()
    })
    .expect("security baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
}