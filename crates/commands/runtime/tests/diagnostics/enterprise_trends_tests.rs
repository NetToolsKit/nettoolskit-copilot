//! Tests for enterprise-trends export commands.

use nettoolskit_runtime::{
    invoke_export_enterprise_trends, RuntimeExportEnterpriseTrendsRequest,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

#[test]
fn test_invoke_export_enterprise_trends_writes_json_and_markdown_outputs() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".temp/audit/validation-ledger.jsonl"),
        "{\"generatedAt\":\"2026-03-29T01:00:00Z\",\"profile\":\"release\",\"warningOnly\":true,\"payloadJson\":\"{\\\"summary\\\":{\\\"totalChecks\\\":4,\\\"passed\\\":3,\\\"warnings\\\":1,\\\"failed\\\":0},\\\"checks\\\":[{\\\"durationMs\\\":5},{\\\"durationMs\\\":7}]}\"}\n",
    );
    write_file(
        &repo.path().join(".temp/audit/validate-all.latest.json"),
        r#"{"profile":"release","summary":{"totalChecks":4,"passed":3,"warnings":1,"failed":0,"suiteWarnings":0},"performance":{"totalDurationMs":12,"averageCheckDurationMs":6.0}}"#,
    );
    write_file(
        &repo
            .path()
            .join(".temp/vulnerability-audit/prebuild-security-gate-summary.json"),
        r#"{"overallStatus":"passed","warningOnly":true,"auditRuns":[{"status":"PASS"},{"status":"FAIL"}],"failures":[],"warnings":["warn"]}"#,
    );

    let result = invoke_export_enterprise_trends(&RuntimeExportEnterpriseTrendsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimeExportEnterpriseTrendsRequest::default()
    })
    .expect("enterprise trends export should execute");

    assert!(result.output_path.is_file());
    assert!(result.summary_path.is_file());
    assert_eq!(result.history_entries, 1);
    assert!(result.dashboard_json.contains("\"validationHistory\""));
    assert!(result.summary_markdown.contains("# Enterprise Trends Snapshot"));
}

#[test]
fn test_invoke_export_enterprise_trends_records_missing_inputs_as_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());

    let result = invoke_export_enterprise_trends(&RuntimeExportEnterpriseTrendsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimeExportEnterpriseTrendsRequest::default()
    })
    .expect("enterprise trends export should execute");

    assert!(result.output_path.is_file());
    assert!(result.summary_path.is_file());
    assert!(!result.warnings.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.contains("Missing validate-all report")));
}
