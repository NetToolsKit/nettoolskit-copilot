//! Tests for policy::compatibility_lifecycle_policy module.

use nettoolskit_validation::{
    invoke_validate_compatibility_lifecycle_policy,
    ValidateCompatibilityLifecyclePolicyRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::policy_fixtures::{
    initialize_compatibility_lifecycle_repo, remove_repo_path, write_compatibility_file,
};

#[test]
fn test_invoke_validate_compatibility_lifecycle_policy_passes_for_valid_table() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo(repo.path());

    let result = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect("compatibility lifecycle validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.reference_date.as_deref(), Some("2025-01-15"));
    assert_eq!(result.rows_checked, 1);
}

#[test]
fn test_invoke_validate_compatibility_lifecycle_policy_accepts_case_insensitive_month_names() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo(repo.path());
    write_compatibility_file(
        repo.path(),
        "COMPATIBILITY.md",
        "january 15, 2025",
        &[
            "| 1.2 | january 1, 2024 | february 1, 2025 | march 1, 2025 | march 2, 2025 | Active |",
        ],
    );

    let result = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect("compatibility lifecycle validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.reference_date.as_deref(), Some("2025-01-15"));
}

#[test]
fn test_invoke_validate_compatibility_lifecycle_policy_reports_invalid_eol_date() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo(repo.path());
    write_compatibility_file(
        repo.path(),
        "COMPATIBILITY.md",
        "January 15, 2025",
        &[
            "| 1.2 | January 1, 2024 | February 1, 2025 | March 1, 2025 | March 3, 2025 | Active |",
        ],
    );

    let result = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect("compatibility lifecycle validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("EOL date must be Maintenance date + 1 day.")));
}

#[test]
fn test_invoke_validate_compatibility_lifecycle_policy_reports_status_mismatch() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo(repo.path());
    write_compatibility_file(
        repo.path(),
        "COMPATIBILITY.md",
        "April 10, 2025",
        &[
            "| 1.2 | January 1, 2024 | February 1, 2025 | March 1, 2025 | March 2, 2025 | Active |",
        ],
    );

    let result = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect("compatibility lifecycle validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message.contains("Status 'Active' does not match reference date (2025-04-10). Expected 'Unsupported'.")
    }));
}

#[test]
fn test_invoke_validate_compatibility_lifecycle_policy_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo(repo.path());
    write_compatibility_file(
        repo.path(),
        "COMPATIBILITY.md",
        "January 15, 2025",
        &["| 0.9 | N/A | N/A | N/A | N/A | Maintenance |"],
    );

    let result = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: true,
            detailed_output: true,
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect("compatibility lifecycle validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Status must be Unsupported when dates are N/A.")));
    assert!(result
        .details
        .iter()
        .any(|message| message.contains("legacy N/A row")));
}

#[test]
fn test_invoke_validate_compatibility_lifecycle_policy_keeps_missing_file_as_failure() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo(repo.path());
    remove_repo_path(repo.path(), "COMPATIBILITY.md");

    let result = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: true,
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect("compatibility lifecycle validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.warnings.is_empty());
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Compatibility file not found")));
}
