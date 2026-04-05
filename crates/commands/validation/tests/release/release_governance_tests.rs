//! Tests for release governance validation.

use nettoolskit_validation::{
    invoke_validate_release_governance, ValidateReleaseGovernanceRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::release_fixtures::{
    initialize_release_governance_repo, remove_governance_file, write_governance_file,
    write_repo_file,
};

#[test]
fn test_invoke_validate_release_governance_passes_for_valid_repo() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_governance_repo(repo.path());

    let result = invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseGovernanceRequest::default()
    })
    .expect("release governance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.latest_version.as_deref(), Some("2.0.0"));
    assert!(result.failures.is_empty());
}

#[test]
fn test_invoke_validate_release_governance_reports_missing_codeowners_rule() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_governance_repo(repo.path());
    write_repo_file(
        repo.path(),
        "CODEOWNERS",
        "* @example\n.github/ @example\n.githooks/ @example\n",
    );

    let result = invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseGovernanceRequest::default()
    })
    .expect("release governance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(
        |failure| failure.contains("CODEOWNERS missing required ownership rule for 'scripts/'")
    ));
}

#[test]
fn test_invoke_validate_release_governance_reports_invalid_changelog_order() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_governance_repo(repo.path());
    write_repo_file(
        repo.path(),
        "CHANGELOG.md",
        "[2.0.0] - 2026-02-10\n[1.9.0] - 2026-03-20\n",
    );

    let result = invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseGovernanceRequest::default()
    })
    .expect("release governance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|failure| failure.contains("CHANGELOG date order invalid")));
}

#[test]
fn test_invoke_validate_release_governance_surfaces_branch_protection_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_governance_repo(repo.path());
    write_governance_file(
        repo.path(),
        "branch-protection.baseline.json",
        r#"{
  "repository": "example/repo",
  "branch": "main",
  "protection": {
    "required_status_checks": {
      "strict": false,
      "contexts": ["Validate Instructions Runtime and Policies"]
    },
    "enforce_admins": false,
    "required_pull_request_reviews": {
      "require_code_owner_reviews": true,
      "required_approving_review_count": 1
    }
  }
}"#,
    );

    let result = invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReleaseGovernanceRequest::default()
    })
    .expect("release governance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.contains("strict=false")));
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.contains("enforce_admins=false")));
}

#[test]
fn test_invoke_validate_release_governance_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_governance_repo(repo.path());
    remove_governance_file(repo.path(), "release-governance.md");

    let result = invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateReleaseGovernanceRequest::default()
    })
    .expect("release governance should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.contains("Required file not found")));
}