//! Tests for audit ledger validation.

use nettoolskit_validation::{
    invoke_validate_audit_ledger, ValidateAuditLedgerRequest, ValidationCheckStatus,
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
fn test_invoke_validate_audit_ledger_passes_when_ledger_is_missing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());

    let result = invoke_validate_audit_ledger(&ValidateAuditLedgerRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAuditLedgerRequest::default()
    })
    .expect("audit ledger validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.entries_checked, 0);
}

#[test]
fn test_invoke_validate_audit_ledger_detects_hash_mismatch() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_file(
        &repo.path().join(".temp/audit/validation-ledger.jsonl"),
        r#"{"schemaVersion":1,"payloadJson":"{}","payloadHash":"bad","prevHash":"0000000000000000000000000000000000000000000000000000000000000000","entryHash":"bad"}"#,
    );

    let result = invoke_validate_audit_ledger(&ValidateAuditLedgerRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAuditLedgerRequest::default()
    })
    .expect("audit ledger validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.entries_checked, 1);
    assert!(!result.failures.is_empty());
}

#[test]
fn test_invoke_validate_audit_ledger_converts_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_file(
        &repo.path().join(".temp/audit/validation-ledger.jsonl"),
        "not-json",
    );

    let result = invoke_validate_audit_ledger(&ValidateAuditLedgerRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateAuditLedgerRequest::default()
    })
    .expect("audit ledger validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(!result.warnings.is_empty());
}
