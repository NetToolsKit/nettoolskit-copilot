//! Tests for operational_hygiene::runtime_script_tests module.

use nettoolskit_validation::{
    invoke_validate_runtime_script_tests, ValidateRuntimeScriptTestsRequest,
    ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::operational_hygiene_fixtures::{
    initialize_runtime_script_tests_repo, write_runtime_test_script,
};

#[test]
fn test_invoke_validate_runtime_script_tests_passes_when_all_tests_succeed() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_script_tests_repo(repo.path());
    write_runtime_test_script(
        repo.path(),
        "pass.tests.ps1",
        "param([string]$RepoRoot)\nexit 0\n",
    );
    write_runtime_test_script(
        repo.path(),
        "pass-two.tests.ps1",
        "param([string]$RepoRoot)\nexit 0\n",
    );

    let result = invoke_validate_runtime_script_tests(&ValidateRuntimeScriptTestsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateRuntimeScriptTestsRequest::default()
    })
    .expect("runtime script tests should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.test_scripts_checked, 2);
    assert_eq!(result.passed_tests, 2);
    assert_eq!(result.failed_tests, 0);
}

#[test]
fn test_invoke_validate_runtime_script_tests_reports_failing_scripts() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_script_tests_repo(repo.path());
    write_runtime_test_script(
        repo.path(),
        "pass.tests.ps1",
        "param([string]$RepoRoot)\nexit 0\n",
    );
    write_runtime_test_script(
        repo.path(),
        "fail.tests.ps1",
        "param([string]$RepoRoot)\nWrite-Error 'boom'\nexit 7\n",
    );

    let result = invoke_validate_runtime_script_tests(&ValidateRuntimeScriptTestsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateRuntimeScriptTestsRequest::default()
    })
    .expect("runtime script tests should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.test_scripts_checked, 2);
    assert_eq!(result.passed_tests, 1);
    assert_eq!(result.failed_tests, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Runtime test failed: fail.tests.ps1 (exit code 7)")));
}

#[test]
fn test_invoke_validate_runtime_script_tests_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_script_tests_repo(repo.path());
    write_runtime_test_script(
        repo.path(),
        "fail.tests.ps1",
        "param([string]$RepoRoot)\nWrite-Error 'boom'\nexit 7\n",
    );

    let result = invoke_validate_runtime_script_tests(&ValidateRuntimeScriptTestsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateRuntimeScriptTestsRequest::default()
    })
    .expect("runtime script tests should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Runtime test failed: fail.tests.ps1 (exit code 7)")));
}

#[test]
fn test_invoke_validate_runtime_script_tests_reports_missing_test_root() {
    let repo = TempDir::new().expect("temporary repository should be created");
    std::fs::create_dir_all(repo.path().join(".github"))
        .expect("github directory should be created");
    std::fs::create_dir_all(repo.path().join(".codex"))
        .expect("codex directory should be created");

    let result = invoke_validate_runtime_script_tests(&ValidateRuntimeScriptTestsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateRuntimeScriptTestsRequest::default()
    })
    .expect("runtime script tests should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Runtime test path not found")));
}