//! Tests for policy::repository_policy module.

use nettoolskit_validation::{invoke_validate_policy, ValidatePolicyRequest, ValidationCheckStatus};
use tempfile::TempDir;

use crate::support::policy_fixtures::{
    initialize_policy_repo, remove_repo_path, write_policy_file,
};

#[test]
fn test_invoke_validate_policy_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_policy_repo(repo.path());

    let result = invoke_validate_policy(&ValidatePolicyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidatePolicyRequest::default()
    })
    .expect("policy validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.policies_checked, 1);
}

#[test]
fn test_invoke_validate_policy_reports_missing_required_file() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_policy_repo(repo.path());
    remove_repo_path(repo.path(), "scripts/runtime/install.ps1");

    let result = invoke_validate_policy(&ValidatePolicyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidatePolicyRequest::default()
    })
    .expect("policy validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing required file 'scripts/runtime/install.ps1'")));
}

#[test]
fn test_invoke_validate_policy_warns_for_unknown_key() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_policy_repo(repo.path());
    write_policy_file(
        repo.path(),
        "baseline.policy.json",
        r#"{
  "id": "repository-baseline",
  "requiredFiles": ["README.md", "scripts/runtime/install.ps1"],
  "requiredDirectories": [".github/policies", ".githooks"],
  "requiredGitHooks": ["pre-commit", "post-commit"],
  "extraKey": true
}"#,
    );

    let result = invoke_validate_policy(&ValidatePolicyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidatePolicyRequest::default()
    })
    .expect("policy validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("unknown key 'extraKey'")));
}

#[test]
fn test_invoke_validate_policy_reports_invalid_json() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_policy_repo(repo.path());
    write_policy_file(repo.path(), "broken.policy.json", "{ invalid");

    let result = invoke_validate_policy(&ValidatePolicyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidatePolicyRequest::default()
    })
    .expect("policy validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Invalid JSON in policy file")));
}