//! Tests for operational_hygiene::shell_hooks module.

use nettoolskit_validation::{
    invoke_validate_shell_hooks, ValidateShellHooksRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::operational_hygiene_fixtures::{
    initialize_shell_hooks_repo, write_fake_shell_command, write_fake_shellcheck_command,
    write_hook_file,
};

#[test]
fn test_invoke_validate_shell_hooks_passes_for_valid_hooks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shell_hooks_repo(repo.path());
    write_all_required_hooks(repo.path(), "#!/bin/sh\necho ok\n");
    let shell_path = write_fake_shell_command(repo.path());

    let result = invoke_validate_shell_hooks(&ValidateShellHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        shell_path: Some(shell_path),
        warning_only: false,
        ..ValidateShellHooksRequest::default()
    })
    .expect("shell hooks validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.hook_files_checked, 4);
}

#[test]
fn test_invoke_validate_shell_hooks_reports_missing_hooks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shell_hooks_repo(repo.path());
    write_hook_file(repo.path(), "pre-commit", "#!/bin/sh\necho ok\n");
    let shell_path = write_fake_shell_command(repo.path());

    let result = invoke_validate_shell_hooks(&ValidateShellHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        shell_path: Some(shell_path),
        warning_only: false,
        ..ValidateShellHooksRequest::default()
    })
    .expect("shell hooks validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Hook file not found: .githooks/post-commit")));
}

#[test]
fn test_invoke_validate_shell_hooks_reports_semantic_guard_failures() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shell_hooks_repo(repo.path());
    write_all_required_hooks(repo.path(), "#!/bin/sh\necho ok\n");
    write_hook_file(
        repo.path(),
        "pre-commit",
        "#!/bin/sh\npwsh -File scripts/runtime/healthcheck.ps1 -WarningOnly true\n",
    );
    let shell_path = write_fake_shell_command(repo.path());

    let result = invoke_validate_shell_hooks(&ValidateShellHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        shell_path: Some(shell_path),
        warning_only: false,
        ..ValidateShellHooksRequest::default()
    })
    .expect("shell hooks validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message
            .contains("Hook uses unsupported boolean argument form for PowerShell bool parameters")
    }));
}

#[test]
fn test_invoke_validate_shell_hooks_emits_shellcheck_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shell_hooks_repo(repo.path());
    write_all_required_hooks(repo.path(), "#!/bin/sh\necho shellcheck-warn\n");
    let shell_path = write_fake_shell_command(repo.path());
    let shellcheck_path = write_fake_shellcheck_command(repo.path());

    let result = invoke_validate_shell_hooks(&ValidateShellHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        shell_path: Some(shell_path),
        shellcheck_path: Some(shellcheck_path),
        enable_shellcheck: true,
        warning_only: false,
        ..ValidateShellHooksRequest::default()
    })
    .expect("shell hooks validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("shellcheck: hook warning")));
}

#[test]
fn test_invoke_validate_shell_hooks_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shell_hooks_repo(repo.path());
    write_all_required_hooks(repo.path(), "#!/bin/sh\nsyntax-error\n");
    let shell_path = write_fake_shell_command(repo.path());

    let result = invoke_validate_shell_hooks(&ValidateShellHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        shell_path: Some(shell_path),
        warning_only: true,
        ..ValidateShellHooksRequest::default()
    })
    .expect("shell hooks validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Shell syntax check failed")));
}

fn write_all_required_hooks(repo_root: &std::path::Path, contents: &str) {
    for hook_name in ["pre-commit", "post-commit", "post-merge", "post-checkout"] {
        write_hook_file(repo_root, hook_name, contents);
    }
}
