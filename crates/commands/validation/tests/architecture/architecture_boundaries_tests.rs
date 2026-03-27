//! Tests for architecture::architecture_boundaries module.

use nettoolskit_validation::{
    invoke_validate_architecture_boundaries, ValidateArchitectureBoundariesRequest,
    ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::architecture_fixtures::{initialize_architecture_boundaries_repo, write_file};

#[test]
fn test_invoke_validate_architecture_boundaries_passes_for_valid_baseline() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_architecture_boundaries_repo(repo.path());

    let result = invoke_validate_architecture_boundaries(
        &ValidateArchitectureBoundariesRequest {
            repo_root: Some(repo.path().to_path_buf()),
            ..ValidateArchitectureBoundariesRequest::default()
        },
    )
    .expect("architecture boundaries validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.rules_checked, 1);
    assert_eq!(result.file_checks, 1);
}

#[test]
fn test_invoke_validate_architecture_boundaries_reports_missing_required_pattern() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_architecture_boundaries_repo(repo.path());
    write_file(&repo.path().join("src/sample.rs"), "pub struct OtherBoundary;\n");

    let result = invoke_validate_architecture_boundaries(
        &ValidateArchitectureBoundariesRequest {
            repo_root: Some(repo.path().to_path_buf()),
            ..ValidateArchitectureBoundariesRequest::default()
        },
    )
    .expect("architecture boundaries validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("missing required pattern")));
}

#[test]
fn test_invoke_validate_architecture_boundaries_warns_for_warning_severity_rule() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_architecture_boundaries_repo(repo.path());
    write_file(
        &repo.path().join(".github/governance/architecture-boundaries.baseline.json"),
        r#"{
  "rules": [
    {
      "id": "warning-boundary",
      "files": ["src/sample.rs"],
      "forbiddenPatterns": ["SampleBoundary"],
      "allowedPatterns": [],
      "severity": "warning"
    }
  ]
}"#,
    );

    let result = invoke_validate_architecture_boundaries(
        &ValidateArchitectureBoundariesRequest {
            repo_root: Some(repo.path().to_path_buf()),
            ..ValidateArchitectureBoundariesRequest::default()
        },
    )
    .expect("architecture boundaries validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("forbidden pattern")));
}

#[test]
fn test_invoke_validate_architecture_boundaries_warns_for_unmatched_pattern() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_architecture_boundaries_repo(repo.path());
    write_file(
        &repo.path().join(".github/governance/architecture-boundaries.baseline.json"),
        r#"{
  "rules": [
    {
      "id": "missing-file-rule",
      "files": ["src/missing/*.rs"],
      "requiredPatterns": ["SampleBoundary"],
      "severity": "failure"
    }
  ]
}"#,
    );

    let result = invoke_validate_architecture_boundaries(
        &ValidateArchitectureBoundariesRequest {
            repo_root: Some(repo.path().to_path_buf()),
            ..ValidateArchitectureBoundariesRequest::default()
        },
    )
    .expect("architecture boundaries validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("pattern matched no files")));
}