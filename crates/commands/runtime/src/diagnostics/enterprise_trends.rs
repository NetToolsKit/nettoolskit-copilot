//! Enterprise validation and vulnerability trend export.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_git_root_or_current_path};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::RuntimeExportEnterpriseTrendsCommandError;

/// Request payload for `export-enterprise-trends`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExportEnterpriseTrendsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Validation ledger path.
    pub ledger_path: Option<PathBuf>,
    /// Latest validate-all report path.
    pub validation_report_path: Option<PathBuf>,
    /// Latest vulnerability summary path.
    pub vulnerability_summary_path: Option<PathBuf>,
    /// Output JSON dashboard path.
    pub output_path: Option<PathBuf>,
    /// Output Markdown summary path.
    pub summary_path: Option<PathBuf>,
    /// Maximum number of ledger entries included in history.
    pub max_entries: usize,
    /// Preserve warning-only compatibility semantics.
    pub warning_only: bool,
}

impl Default for RuntimeExportEnterpriseTrendsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            ledger_path: None,
            validation_report_path: None,
            vulnerability_summary_path: None,
            output_path: None,
            summary_path: None,
            max_entries: 30,
            warning_only: true,
        }
    }
}

/// Result payload for `export-enterprise-trends`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExportEnterpriseTrendsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved JSON output path.
    pub output_path: PathBuf,
    /// Resolved Markdown summary path.
    pub summary_path: PathBuf,
    /// Number of trend history entries included in the output.
    pub history_entries: usize,
    /// Non-blocking warnings preserved in the output.
    pub warnings: Vec<String>,
    /// Persisted JSON payload.
    pub dashboard_json: String,
    /// Persisted Markdown payload.
    pub summary_markdown: String,
}

/// Export enterprise trend artifacts from validation and vulnerability evidence.
///
/// # Errors
///
/// Returns [`RuntimeExportEnterpriseTrendsCommandError`] when workspace
/// resolution or output persistence fails.
pub fn invoke_export_enterprise_trends(
    request: &RuntimeExportEnterpriseTrendsRequest,
) -> Result<RuntimeExportEnterpriseTrendsResult, RuntimeExportEnterpriseTrendsCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeExportEnterpriseTrendsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_git_root_or_current_path(request.repo_root.as_deref(), &current_dir)
        .map_err(|source| RuntimeExportEnterpriseTrendsCommandError::ResolveWorkspaceRoot {
            source,
        })?;
    let ledger_path =
        resolve_path(&repo_root, request.ledger_path.as_deref(), ".temp/audit/validation-ledger.jsonl");
    let validation_report_path = resolve_path(
        &repo_root,
        request.validation_report_path.as_deref(),
        ".temp/audit/validate-all.latest.json",
    );
    let vulnerability_summary_path = resolve_path(
        &repo_root,
        request.vulnerability_summary_path.as_deref(),
        ".temp/vulnerability-audit/prebuild-security-gate-summary.json",
    );
    let output_path = resolve_path(
        &repo_root,
        request.output_path.as_deref(),
        ".temp/audit/enterprise-trends.latest.json",
    );
    let summary_path = resolve_path(
        &repo_root,
        request.summary_path.as_deref(),
        ".temp/audit/enterprise-trends.latest.md",
    );

    initialize_output_parent(&output_path)?;
    initialize_output_parent(&summary_path)?;

    let mut warnings = Vec::new();
    let validation_report =
        read_json_file_safe(&validation_report_path, "validate-all report", &mut warnings);
    let vulnerability_summary = read_json_file_safe(
        &vulnerability_summary_path,
        "vulnerability summary",
        &mut warnings,
    );
    let ledger_records = read_ledger_records(&ledger_path, &mut warnings);
    let history = build_validation_history(&ledger_records, request.max_entries);
    let validation_summary = build_validation_summary(validation_report.as_ref());
    let vulnerability_snapshot = build_vulnerability_snapshot(vulnerability_summary.as_ref());

    let total_checks = validation_summary
        .get("totalChecks")
        .and_then(Value::as_u64)
        .unwrap_or(0) as f64;
    let warnings_total = validation_summary
        .get("warnings")
        .and_then(Value::as_u64)
        .unwrap_or(0) as f64;
    let failures_total = validation_summary
        .get("failed")
        .and_then(Value::as_u64)
        .unwrap_or(0) as f64;
    let warning_rate = if total_checks > 0.0 {
        ((warnings_total * 100.0) / total_checks * 100.0).round() / 100.0
    } else {
        0.0
    };
    let failure_rate = if total_checks > 0.0 {
        ((failures_total * 100.0) / total_checks * 100.0).round() / 100.0
    } else {
        0.0
    };
    let average_duration_last_n = if history.is_empty() {
        0.0
    } else {
        let duration_total = history
            .iter()
            .map(|entry| {
                entry.get("totalDurationMs")
                    .and_then(Value::as_u64)
                    .unwrap_or(0) as f64
            })
            .sum::<f64>();
        ((duration_total / history.len() as f64) * 100.0).round() / 100.0
    };

    let dashboard = json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string()?,
        "repoRoot": repo_root.display().to_string(),
        "inputs": {
            "ledgerPath": display_repo_relative_path(&repo_root, &ledger_path),
            "validationReportPath": display_repo_relative_path(&repo_root, &validation_report_path),
            "vulnerabilitySummaryPath": display_repo_relative_path(&repo_root, &vulnerability_summary_path),
        },
        "current": {
            "validation": validation_summary,
            "vulnerability": vulnerability_snapshot,
        },
        "trends": {
            "validationHistory": history,
        },
        "kpis": {
            "validationWarningRatePercent": warning_rate,
            "validationFailureRatePercent": failure_rate,
            "averageDurationMsLastN": average_duration_last_n,
            "historyEntries": history.len(),
        },
        "warnings": warnings,
    });
    let dashboard_json = serde_json::to_string_pretty(&dashboard).map_err(|source| {
        RuntimeExportEnterpriseTrendsCommandError::WriteOutput {
            source: source.into(),
        }
    })?;
    let summary_markdown = render_summary_markdown(&dashboard);

    fs::write(&output_path, &dashboard_json)
        .with_context(|| format!("failed to write '{}'", output_path.display()))
        .map_err(|source| RuntimeExportEnterpriseTrendsCommandError::WriteOutput { source })?;
    fs::write(&summary_path, &summary_markdown)
        .with_context(|| format!("failed to write '{}'", summary_path.display()))
        .map_err(|source| RuntimeExportEnterpriseTrendsCommandError::WriteOutput { source })?;

    Ok(RuntimeExportEnterpriseTrendsResult {
        repo_root,
        output_path,
        summary_path,
        history_entries: dashboard
            .get("kpis")
            .and_then(|kpis| kpis.get("historyEntries"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize,
        warnings: dashboard
            .get("warnings")
            .and_then(Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        dashboard_json,
        summary_markdown,
    })
}

fn resolve_path(repo_root: &Path, path: Option<&Path>, default_relative: &str) -> PathBuf {
    match path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(default_relative),
    }
}

fn initialize_output_parent(
    path: &Path,
) -> Result<(), RuntimeExportEnterpriseTrendsCommandError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))
            .map_err(|source| RuntimeExportEnterpriseTrendsCommandError::PrepareArtifacts {
                source,
            })?;
    }

    Ok(())
}

fn read_json_file_safe(path: &Path, label: &str, warnings: &mut Vec<String>) -> Option<Value> {
    if !path.is_file() {
        warnings.push(format!("Missing {label}: {}", path.display()));
        return None;
    }

    let document = match fs::read_to_string(path) {
        Ok(document) => document,
        Err(error) => {
            warnings.push(format!("Invalid JSON in {label}: {error}"));
            return None;
        }
    };
    match serde_json::from_str::<Value>(&document) {
        Ok(document) => Some(document),
        Err(error) => {
            warnings.push(format!("Invalid JSON in {label}: {error}"));
            None
        }
    }
}

fn read_ledger_records(path: &Path, warnings: &mut Vec<String>) -> Vec<Value> {
    if !path.is_file() {
        warnings.push(format!("Validation ledger not found: {}", path.display()));
        return Vec::new();
    }

    let Ok(document) = fs::read_to_string(path) else {
        warnings.push(format!("Validation ledger not readable: {}", path.display()));
        return Vec::new();
    };

    let mut records = Vec::new();
    for line in document.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let entry: Value = match serde_json::from_str(line) {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(format!("Skipped invalid ledger line: {error}"));
                continue;
            }
        };
        let payload = entry
            .get("payloadJson")
            .and_then(Value::as_str)
            .and_then(|payload| serde_json::from_str::<Value>(payload).ok())
            .unwrap_or(Value::Null);
        records.push(json!({
            "generatedAt": entry.get("generatedAt").and_then(Value::as_str).unwrap_or_default(),
            "profile": entry.get("profile").and_then(Value::as_str).unwrap_or_default(),
            "warningOnly": entry.get("warningOnly").and_then(Value::as_bool).unwrap_or(true),
            "payload": payload,
        }));
    }

    records
}

fn build_validation_history(records: &[Value], max_entries: usize) -> Vec<Value> {
    if max_entries == 0 {
        return Vec::new();
    }

    let start = records.len().saturating_sub(max_entries);
    records[start..]
        .iter()
        .map(|entry| {
            let payload = entry.get("payload").unwrap_or(&Value::Null);
            let summary = payload.get("summary").unwrap_or(&Value::Null);
            let total_duration_ms = payload
                .get("checks")
                .and_then(Value::as_array)
                .map(|checks| {
                    checks
                        .iter()
                        .map(|check| check.get("durationMs").and_then(Value::as_u64).unwrap_or(0))
                        .sum::<u64>()
                })
                .unwrap_or(0);

            json!({
                "generatedAt": entry.get("generatedAt").and_then(Value::as_str).unwrap_or_default(),
                "profile": entry.get("profile").and_then(Value::as_str).unwrap_or_default(),
                "warningOnly": entry.get("warningOnly").and_then(Value::as_bool).unwrap_or(true),
                "totalChecks": summary.get("totalChecks").and_then(Value::as_u64).unwrap_or(0),
                "passed": summary.get("passed").and_then(Value::as_u64).unwrap_or(0),
                "warnings": summary.get("warnings").and_then(Value::as_u64).unwrap_or(0),
                "failed": summary.get("failed").and_then(Value::as_u64).unwrap_or(0),
                "totalDurationMs": total_duration_ms,
            })
        })
        .collect()
}

fn build_validation_summary(report: Option<&Value>) -> Value {
    let summary = report.and_then(|document| document.get("summary"));
    let performance = report.and_then(|document| document.get("performance"));
    json!({
        "profile": report.and_then(|document| document.get("profile")).and_then(Value::as_str).unwrap_or("unknown"),
        "totalChecks": summary.and_then(|value| value.get("totalChecks")).and_then(Value::as_u64).unwrap_or(0),
        "passed": summary.and_then(|value| value.get("passed")).and_then(Value::as_u64).unwrap_or(0),
        "warnings": summary.and_then(|value| value.get("warnings")).and_then(Value::as_u64).unwrap_or(0),
        "failed": summary.and_then(|value| value.get("failed")).and_then(Value::as_u64).unwrap_or(0),
        "suiteWarnings": summary.and_then(|value| value.get("suiteWarnings")).and_then(Value::as_u64).unwrap_or(0),
        "totalDurationMs": performance.and_then(|value| value.get("totalDurationMs")).and_then(Value::as_u64).unwrap_or(0),
        "averageCheckDurationMs": performance.and_then(|value| value.get("averageCheckDurationMs")).and_then(Value::as_f64).unwrap_or(0.0),
    })
}

fn build_vulnerability_snapshot(summary: Option<&Value>) -> Value {
    let audit_runs = summary
        .and_then(|document| document.get("auditRuns"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let failed_audits = audit_runs
        .iter()
        .filter(|run| run.get("status").and_then(Value::as_str) == Some("FAIL"))
        .count();

    json!({
        "available": summary.is_some(),
        "overallStatus": summary.and_then(|document| document.get("overallStatus")).and_then(Value::as_str).unwrap_or("unknown"),
        "warningOnly": summary.and_then(|document| document.get("warningOnly")).and_then(Value::as_bool),
        "auditRuns": audit_runs.len(),
        "failedAudits": failed_audits,
        "failures": summary.and_then(|document| document.get("failures")).and_then(Value::as_array).map(Vec::len).unwrap_or(0),
        "warnings": summary.and_then(|document| document.get("warnings")).and_then(Value::as_array).map(Vec::len).unwrap_or(0),
    })
}

fn render_summary_markdown(dashboard: &Value) -> String {
    let validation = dashboard
        .get("current")
        .and_then(|current| current.get("validation"))
        .unwrap_or(&Value::Null);
    let vulnerability = dashboard
        .get("current")
        .and_then(|current| current.get("vulnerability"))
        .unwrap_or(&Value::Null);
    let kpis = dashboard.get("kpis").unwrap_or(&Value::Null);

    [
        "# Enterprise Trends Snapshot".to_string(),
        String::new(),
        format!(
            "- Generated At: {}",
            dashboard
                .get("generatedAt")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ),
        format!(
            "- Profile: {}",
            validation
                .get("profile")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
        ),
        String::new(),
        "## Validation".to_string(),
        format!(
            "- Total Checks: {}",
            validation
                .get("totalChecks")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
        format!(
            "- Passed: {}",
            validation
                .get("passed")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
        format!(
            "- Warnings: {}",
            validation
                .get("warnings")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
        format!(
            "- Failed: {}",
            validation
                .get("failed")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
        String::new(),
        "## Vulnerability Snapshot".to_string(),
        format!(
            "- Available: {}",
            vulnerability
                .get("available")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        ),
        format!(
            "- Overall Status: {}",
            vulnerability
                .get("overallStatus")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
        ),
        format!(
            "- Audit Runs: {}",
            vulnerability
                .get("auditRuns")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
        String::new(),
        "## KPIs".to_string(),
        format!(
            "- Validation Warning Rate (%): {}",
            kpis.get("validationWarningRatePercent")
                .and_then(Value::as_f64)
                .unwrap_or(0.0)
        ),
        format!(
            "- Validation Failure Rate (%): {}",
            kpis.get("validationFailureRatePercent")
                .and_then(Value::as_f64)
                .unwrap_or(0.0)
        ),
        format!(
            "- Average Duration Last N (ms): {}",
            kpis.get("averageDurationMsLastN")
                .and_then(Value::as_f64)
                .unwrap_or(0.0)
        ),
        format!(
            "- Trend Entries Considered: {}",
            kpis.get("historyEntries")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
    ]
    .join("\n")
}

fn display_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn current_timestamp_string() -> Result<String, RuntimeExportEnterpriseTrendsCommandError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to compute current timestamp")
        .map_err(|source| RuntimeExportEnterpriseTrendsCommandError::PrepareArtifacts {
            source,
        })?;
    Ok(duration.as_secs().to_string())
}
