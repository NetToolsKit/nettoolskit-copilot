//! Tests for PowerShell standards validation.

use nettoolskit_validation::{
    invoke_validate_powershell_standards, ValidatePowerShellStandardsRequest,
    ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::standards_fixtures::{
    initialize_powershell_standards_repo, write_powershell_script,
};

#[test]
fn test_invoke_validate_powershell_standards_passes_for_valid_scripts() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_powershell_standards_repo(repo.path());
    let result = invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        skip_script_analyzer: true,
        warning_only: false,
        ..ValidatePowerShellStandardsRequest::default()
    })
    .expect("powershell standards should execute");
    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.files_checked, 1);
}

#[test]
fn test_invoke_validate_powershell_standards_reports_missing_help_block() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_powershell_standards_repo(repo.path());
    write_powershell_script(
        repo.path(),
        "scripts/runtime/install.ps1",
        "param([string] $RepoRoot)\n$ErrorActionPreference = 'Stop'\n",
    );

    let result = invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        skip_script_analyzer: true,
        warning_only: false,
        ..ValidatePowerShellStandardsRequest::default()
    })
    .expect("powershell standards should execute");
    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|failure| failure.contains("Missing comment-based help block"))
    );
}

#[test]
fn test_invoke_validate_powershell_standards_escalates_style_findings_in_strict_mode() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_powershell_standards_repo(repo.path());
    write_powershell_script(
        repo.path(),
        "scripts/runtime/install.ps1",
        r#"<#
.SYNOPSIS
Installs runtime assets.

.DESCRIPTION
Ensures runtime assets are present.

.PARAMETER RepoRoot
Optional repository root.

.EXAMPLE
pwsh -File scripts/runtime/install.ps1

.NOTES
Version: 1.0
#>

param(
    [string] $RepoRoot
)

function CustomThing {
    param()

    return 'ok'
}
"#,
    );

    let result = invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        strict: true,
        skip_script_analyzer: true,
        warning_only: false,
        ..ValidatePowerShellStandardsRequest::default()
    })
    .expect("powershell standards should execute");
    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|failure| failure.contains("Verb-Noun format"))
    );
}

#[test]
fn test_invoke_validate_powershell_standards_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_powershell_standards_repo(repo.path());
    write_powershell_script(
        repo.path(),
        "scripts/runtime/install.ps1",
        "$ErrorActionPreference = 'Stop'\n",
    );

    let result = invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        skip_script_analyzer: true,
        warning_only: true,
        ..ValidatePowerShellStandardsRequest::default()
    })
    .expect("powershell standards should execute");
    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("Missing comment-based help block"))
    );
}