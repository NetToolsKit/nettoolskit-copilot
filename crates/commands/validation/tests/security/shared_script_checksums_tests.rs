//! Tests for security::shared_script_checksums module.

use nettoolskit_validation::{
    invoke_validate_shared_script_checksums, ValidateSharedScriptChecksumsRequest,
    ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::security_fixtures::{
    initialize_shared_checksums_repo, remove_repo_path, write_repo_file,
};

#[test]
fn test_invoke_validate_shared_script_checksums_passes_for_valid_manifest() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shared_checksums_repo(repo.path());

    let result = invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSharedScriptChecksumsRequest::default()
    })
    .expect("shared script checksum validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.manifest_entries, 2);
    assert_eq!(result.current_entries, 2);
}

#[test]
fn test_invoke_validate_shared_script_checksums_reports_missing_manifest_entry() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shared_checksums_repo(repo.path());
    write_repo_file(repo.path(), "scripts/common/c.ps1", "Write-Output 'c'\n");

    let result = invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSharedScriptChecksumsRequest::default()
    })
    .expect("shared script checksum validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message
            .contains("File exists but is missing in manifest: scripts/common/c.ps1")));
}

#[test]
fn test_invoke_validate_shared_script_checksums_reports_missing_source_file() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shared_checksums_repo(repo.path());
    remove_repo_path(repo.path(), "scripts/security/b.ps1");

    let result = invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSharedScriptChecksumsRequest::default()
    })
    .expect("shared script checksum validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(
        |message| message.contains("Manifest references missing file: scripts/security/b.ps1")
    ));
}

#[test]
fn test_invoke_validate_shared_script_checksums_reports_checksum_mismatch() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shared_checksums_repo(repo.path());
    write_repo_file(
        repo.path(),
        "scripts/common/a.ps1",
        "Write-Output 'changed'\n",
    );

    let result = invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        detailed_output: true,
        ..ValidateSharedScriptChecksumsRequest::default()
    })
    .expect("shared script checksum validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(
        result.hash_mismatches,
        vec!["scripts/common/a.ps1".to_string()]
    );
    assert_eq!(result.mismatch_details.len(), 1);
}

#[test]
fn test_invoke_validate_shared_script_checksums_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shared_checksums_repo(repo.path());
    remove_repo_path(repo.path(), "scripts/security");

    let result = invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateSharedScriptChecksumsRequest::default()
    })
    .expect("shared script checksum validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Included root folder not found: scripts/security")));
}