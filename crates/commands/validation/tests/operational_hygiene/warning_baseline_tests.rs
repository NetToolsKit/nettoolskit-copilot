//! Tests for operational_hygiene::warning_baseline module.

use nettoolskit_validation::{
    invoke_validate_warning_baseline, ValidateWarningBaselineRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::operational_hygiene_fixtures::{
    initialize_warning_baseline_repo, write_warning_analyzer_report,
};

#[test]
fn test_invoke_validate_warning_baseline_passes_when_counts_fit_thresholds() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_warning_baseline_repo(repo.path());
    let analyzer_report_path = repo.path().join(".temp/audit/analyzer-warning-report.json");
    write_warning_analyzer_report(
        &analyzer_report_path,
        &[("PSAvoidUsingWriteHost", "scripts/example.ps1")],
    );

    let result = invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        analyzer_report_path: Some(analyzer_report_path),
        warning_only: false,
        ..ValidateWarningBaselineRequest::default()
    })
    .expect("warning baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.total_warnings, 1);
    assert_eq!(
        result.warning_by_rule.get("PSAvoidUsingWriteHost"),
        Some(&1usize)
    );
    assert!(result.report_written);
    assert!(result.report_path.is_file());
}

#[test]
fn test_invoke_validate_warning_baseline_reports_threshold_overage() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_warning_baseline_repo(repo.path());
    let analyzer_report_path = repo.path().join(".temp/audit/analyzer-warning-report.json");
    write_warning_analyzer_report(
        &analyzer_report_path,
        &[
            ("PSAvoidUsingWriteHost", "scripts/example.ps1"),
            ("PSAvoidUsingWriteHost", "scripts/example.ps1"),
            ("PSAvoidUsingWriteHost", "scripts/example.ps1"),
            ("PSUseSingularNouns", "scripts/example.ps1"),
        ],
    );

    let result = invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        analyzer_report_path: Some(analyzer_report_path),
        warning_only: false,
        ..ValidateWarningBaselineRequest::default()
    })
    .expect("warning baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message.contains("Total analyzer warnings (4) exceed baseline maxTotalWarnings (3)")
    }));
    assert!(result.failures.iter().any(|message| {
        message.contains(
            "Analyzer warning count for 'PSAvoidUsingWriteHost' (3) exceeds threshold (2).",
        )
    }));
}

#[test]
fn test_invoke_validate_warning_baseline_warns_for_unknown_rules() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_warning_baseline_repo(repo.path());
    let analyzer_report_path = repo.path().join(".temp/audit/analyzer-warning-report.json");
    write_warning_analyzer_report(
        &analyzer_report_path,
        &[("UnknownRule", "scripts/example.ps1")],
    );

    let result = invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        analyzer_report_path: Some(analyzer_report_path),
        warning_only: false,
        ..ValidateWarningBaselineRequest::default()
    })
    .expect("warning baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Analyzer reported rule not present in baseline")));
}

#[test]
fn test_invoke_validate_warning_baseline_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_warning_baseline_repo(repo.path());
    let analyzer_report_path = repo.path().join(".temp/audit/analyzer-warning-report.json");
    write_warning_analyzer_report(
        &analyzer_report_path,
        &[
            ("PSAvoidUsingWriteHost", "scripts/example.ps1"),
            ("PSAvoidUsingWriteHost", "scripts/example.ps1"),
            ("PSAvoidUsingWriteHost", "scripts/example.ps1"),
            ("PSUseSingularNouns", "scripts/example.ps1"),
        ],
    );

    let result = invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
        repo_root: Some(repo.path().to_path_buf()),
        analyzer_report_path: Some(analyzer_report_path),
        warning_only: true,
        ..ValidateWarningBaselineRequest::default()
    })
    .expect("warning baseline validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result.warnings.iter().any(|message| {
        message.contains("Total analyzer warnings (4) exceed baseline maxTotalWarnings (3)")
    }));
}