//! Tests for release provenance validation.

use nettoolskit_validation::{
    invoke_validate_release_provenance, ValidateReleaseProvenanceRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::release_fixtures::{
    initialize_release_provenance_repo, initialize_git_repository, write_audit_report,
    write_repo_file,
};

#[test]
fn test_invoke_validate_release_provenance_passes_for_valid_repo() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo(repo.path());
    initialize_git_repository(repo.path());

    let result = invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseProvenanceRequest::default()
    })
    .expect("release provenance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.latest_version.as_deref(), Some("2.0.0"));
    assert_eq!(result.current_branch.as_deref(), Some("main"));
    assert!(result.failures.is_empty());
}

#[test]
fn test_invoke_validate_release_provenance_reports_missing_required_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo(repo.path());
    write_repo_file(
        repo.path(),
        "scripts/validation/validate-all.ps1",
        "$definitions = @(\n    @{ name = 'validate-release-governance' }\n)\n",
    );
    initialize_git_repository(repo.path());

    let result = invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseProvenanceRequest::default()
    })
    .expect("release provenance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|failure| failure.contains("Required check missing from validate-all"))
    );
}

#[test]
fn test_invoke_validate_release_provenance_fails_when_required_audit_report_is_missing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo(repo.path());
    initialize_git_repository(repo.path());

    let result = invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        require_audit_report: true,
        warning_only: false,
        ..ValidateReleaseProvenanceRequest::default()
    })
    .expect("release provenance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|failure| failure.contains("Required audit report not found"))
    );
}

#[test]
fn test_invoke_validate_release_provenance_reports_dirty_worktree_when_required_clean() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo(repo.path());
    write_repo_file(
        repo.path(),
        ".github/governance/release-provenance.baseline.json",
        r#"{
  "version": 1,
  "releaseBranch": "main",
  "requireCleanWorktree": true,
  "warnOnDirtyWorktree": true,
  "requireAuditReport": false,
  "warnOnMissingOptionalAuditReport": false,
  "warnOnAuditCommitMismatch": false,
  "changelogPath": "CHANGELOG.md",
  "validateAllPath": "scripts/validation/validate-all.ps1",
  "requiredValidationChecks": [
    "validate-release-governance",
    "validate-release-provenance"
  ],
  "requiredEvidenceFiles": [
    "CHANGELOG.md",
    "CODEOWNERS",
    ".github/governance/release-governance.md",
    ".github/governance/release-provenance.baseline.json"
  ]
}"#,
    );
    initialize_git_repository(repo.path());
    write_repo_file(repo.path(), "DIRTY.md", "dirty\n");

    let result = invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseProvenanceRequest::default()
    })
    .expect("release provenance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|failure| failure.contains("requires clean state"))
    );
}

#[test]
fn test_invoke_validate_release_provenance_warns_for_audit_commit_mismatch() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo(repo.path());
    let head_commit = initialize_git_repository(repo.path());
    write_audit_report(repo.path(), ".temp/audit-report.json", "ffffffff", "passed");

    let result = invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseProvenanceRequest::default()
    })
    .expect("release provenance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_ne!(head_commit, "ffffffff");
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("Audit report commit differs from HEAD"))
    );
}