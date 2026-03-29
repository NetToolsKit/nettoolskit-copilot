//! PowerShell analyzer warning baseline validation.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ValidateWarningBaselineCommandError;
use crate::operational_hygiene::common::{
    current_timestamp_string, derive_status, push_required_finding, resolve_executable,
};
use crate::ValidationCheckStatus;

const DEFAULT_BASELINE_PATH: &str = ".github/governance/warning-baseline.json";
const DEFAULT_REPORT_PATH: &str = ".temp/audit/warning-baseline-report.json";

/// Request payload for `validate-warning-baseline`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateWarningBaselineRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit warning baseline path.
    pub baseline_path: Option<PathBuf>,
    /// Optional explicit analyzer warning report path for deterministic tests or offline reuse.
    pub analyzer_report_path: Option<PathBuf>,
    /// Optional explicit JSON report output path.
    pub report_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateWarningBaselineRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            analyzer_report_path: None,
            report_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-warning-baseline`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateWarningBaselineResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved warning baseline path.
    pub baseline_path: PathBuf,
    /// Resolved warning scan root.
    pub scan_root: PathBuf,
    /// Optional explicit analyzer report path.
    pub analyzer_report_path: Option<PathBuf>,
    /// Resolved JSON report output path.
    pub report_path: PathBuf,
    /// Whether the JSON report was written.
    pub report_written: bool,
    /// Total analyzer warnings detected.
    pub total_warnings: usize,
    /// Analyzer warning count by rule.
    pub warning_by_rule: BTreeMap<String, usize>,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Deserialize)]
struct WarningBaselineDocument {
    #[serde(rename = "maxTotalWarnings")]
    max_total_warnings: usize,
    #[serde(rename = "scanRoot")]
    scan_root: String,
    #[serde(default, rename = "maxWarningsByRule")]
    max_warnings_by_rule: BTreeMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct AnalyzerWarningRecord {
    #[serde(default, alias = "ruleName", rename = "RuleName")]
    rule_name: String,
}

/// Run the PowerShell analyzer warning baseline validation.
///
/// # Errors
///
/// Returns [`ValidateWarningBaselineCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_warning_baseline(
    request: &ValidateWarningBaselineRequest,
) -> Result<ValidateWarningBaselineResult, ValidateWarningBaselineCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateWarningBaselineCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateWarningBaselineCommandError::ResolveWorkspaceRoot { source })?;
    let baseline_path = match request.baseline_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_BASELINE_PATH),
    };
    let analyzer_report_path = request
        .analyzer_report_path
        .as_deref()
        .map(|path| resolve_full_path(&repo_root, path));
    let report_path = match request.report_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_REPORT_PATH),
    };

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    let baseline = read_warning_baseline(
        &baseline_path,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let scan_root = baseline
        .as_ref()
        .map(|baseline| resolve_full_path(&repo_root, Path::new(&baseline.scan_root)))
        .unwrap_or_else(|| repo_root.join("scripts"));

    if baseline.is_some() && !scan_root.is_dir() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "scanRoot not found in warning baseline: {}",
                scan_root
                    .strip_prefix(&repo_root)
                    .unwrap_or(&scan_root)
                    .display()
            ),
        );
    }

    let analyzer_warnings = if baseline.is_some() && scan_root.is_dir() {
        load_analyzer_warnings(
            &scan_root,
            analyzer_report_path.as_deref(),
            request.warning_only,
            &mut warnings,
            &mut failures,
        )
    } else {
        Vec::new()
    };

    let warning_by_rule = count_warnings_by_rule(&analyzer_warnings);
    let total_warnings = analyzer_warnings.len();

    if let Some(baseline) = baseline.as_ref() {
        if total_warnings > baseline.max_total_warnings {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Total analyzer warnings ({total_warnings}) exceed baseline maxTotalWarnings ({}).",
                    baseline.max_total_warnings
                ),
            );
        }

        for (rule_name, threshold) in &baseline.max_warnings_by_rule {
            let count = warning_by_rule.get(rule_name).copied().unwrap_or(0);
            if count > *threshold {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!(
                        "Analyzer warning count for '{rule_name}' ({count}) exceeds threshold ({threshold})."
                    ),
                );
            }
        }

        for (rule_name, count) in &warning_by_rule {
            if !baseline.max_warnings_by_rule.contains_key(rule_name) {
                warnings.push(format!(
                    "Analyzer reported rule not present in baseline thresholds: {rule_name} ({count})"
                ));
            }
        }
    }

    let report_written = if baseline.is_some() {
        write_warning_report(
            &report_path,
            &repo_root,
            &scan_root,
            total_warnings,
            baseline
                .as_ref()
                .map(|document| document.max_total_warnings)
                .unwrap_or_default(),
            &warning_by_rule,
            &mut warnings,
        )
    } else {
        false
    };

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateWarningBaselineResult {
        repo_root,
        warning_only: request.warning_only,
        baseline_path,
        scan_root,
        analyzer_report_path,
        report_path,
        report_written,
        total_warnings,
        warning_by_rule,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_warning_baseline(
    baseline_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<WarningBaselineDocument> {
    if !baseline_path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing warning baseline: {}", baseline_path.display()),
        );
        return None;
    }

    let document = match fs::read_to_string(baseline_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Could not read warning baseline: {} :: {error}",
                    baseline_path.display()
                ),
            );
            return None;
        }
    };

    match serde_json::from_str::<WarningBaselineDocument>(&document) {
        Ok(value) => Some(value),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Invalid JSON in warning baseline: {} :: {error}",
                    baseline_path.display()
                ),
            );
            None
        }
    }
}

fn load_analyzer_warnings(
    scan_root: &Path,
    analyzer_report_path: Option<&Path>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<AnalyzerWarningRecord> {
    if let Some(path) = analyzer_report_path {
        return read_analyzer_warning_report(path, warning_only, warnings, failures);
    }

    let Some(powershell_path) = resolve_executable(None, &["pwsh", "powershell"], &[]) else {
        warnings.push("PowerShell runtime not found; warning baseline check skipped.".to_string());
        return Vec::new();
    };

    let temp_script_path = env::temp_dir().join(format!(
        "nettoolskit-validate-warning-baseline-{}.ps1",
        std::process::id()
    ));
    let script_body = r#"
param(
    [string] $ScanRoot
)

$command = Get-Command Invoke-ScriptAnalyzer -ErrorAction SilentlyContinue
if ($null -eq $command) {
    exit 3
}

try {
    $results = @(Invoke-ScriptAnalyzer -Path $ScanRoot -Recurse -Severity Warning | Select-Object RuleName, ScriptPath)
    if ($results.Count -eq 0) {
        Write-Output '[]'
        exit 0
    }

    $results | ConvertTo-Json -Depth 8 -Compress
    exit 0
}
catch {
    Write-Error $_.Exception.Message
    exit 4
}
"#;
    if let Err(error) = fs::write(&temp_script_path, script_body) {
        warnings.push(format!(
            "Could not create temporary ScriptAnalyzer bridge: {error}"
        ));
        return Vec::new();
    }

    let output = Command::new(&powershell_path)
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&temp_script_path)
        .arg("-ScanRoot")
        .arg(scan_root)
        .output();
    let _ = fs::remove_file(&temp_script_path);

    let Ok(output) = output else {
        warnings.push(
            "PSScriptAnalyzer execution failed: could not launch PowerShell runtime.".to_string(),
        );
        return Vec::new();
    };

    match output.status.code().unwrap_or(1) {
        0 => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout.is_empty() {
                return Vec::new();
            }

            let value = match serde_json::from_str::<Value>(&stdout) {
                Ok(value) => value,
                Err(error) => {
                    warnings.push(format!(
                        "PSScriptAnalyzer returned invalid JSON output: {error}"
                    ));
                    return Vec::new();
                }
            };
            normalize_warning_records(value, warnings)
        }
        3 => {
            warnings
                .push("PSScriptAnalyzer not found; warning baseline check skipped.".to_string());
            Vec::new()
        }
        _ => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let detail = if !stderr.is_empty() {
                stderr
            } else if !stdout.is_empty() {
                stdout
            } else {
                "unknown analyzer error".to_string()
            };
            warnings.push(format!("PSScriptAnalyzer execution failed: {detail}"));
            Vec::new()
        }
    }
}

fn read_analyzer_warning_report(
    report_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<AnalyzerWarningRecord> {
    if !report_path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Analyzer warning report not found: {}",
                report_path.display()
            ),
        );
        return Vec::new();
    }

    let document = match fs::read_to_string(report_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Could not read analyzer warning report: {} :: {error}",
                    report_path.display()
                ),
            );
            return Vec::new();
        }
    };

    let value = match serde_json::from_str::<Value>(&document) {
        Ok(value) => value,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Invalid JSON in analyzer warning report: {} :: {error}",
                    report_path.display()
                ),
            );
            return Vec::new();
        }
    };

    normalize_warning_records(value, warnings)
}

fn normalize_warning_records(
    value: Value,
    warnings: &mut Vec<String>,
) -> Vec<AnalyzerWarningRecord> {
    let records = match value {
        Value::Array(items) => items,
        Value::Object(_) => vec![value],
        _ => {
            warnings.push("Analyzer warning report must be a JSON object or array.".to_string());
            return Vec::new();
        }
    };

    records
        .into_iter()
        .filter_map(
            |record| match serde_json::from_value::<AnalyzerWarningRecord>(record) {
                Ok(record) if !record.rule_name.trim().is_empty() => Some(record),
                Ok(_) => {
                    warnings.push(
                        "Analyzer warning report contained an entry without RuleName.".to_string(),
                    );
                    None
                }
                Err(error) => {
                    warnings.push(format!(
                        "Analyzer warning report entry could not be parsed: {error}"
                    ));
                    None
                }
            },
        )
        .collect()
}

fn count_warnings_by_rule(records: &[AnalyzerWarningRecord]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for record in records {
        let counter = counts.entry(record.rule_name.clone()).or_insert(0);
        *counter += 1;
    }
    counts
}

fn write_warning_report(
    report_path: &Path,
    repo_root: &Path,
    scan_root: &Path,
    total_warnings: usize,
    max_total_warnings: usize,
    warning_by_rule: &BTreeMap<String, usize>,
    warnings: &mut Vec<String>,
) -> bool {
    if let Some(parent) = report_path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            warnings.push(format!(
                "Could not create warning baseline report directory: {} :: {error}",
                parent.display()
            ));
            return false;
        }
    }

    let rule_breakdown = warning_by_rule
        .iter()
        .map(|(rule, count)| {
            json!({
                "rule": rule,
                "count": count,
            })
        })
        .collect::<Vec<_>>();
    let payload = serde_json::to_string_pretty(&json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string(),
        "scanRoot": scan_root
            .strip_prefix(repo_root)
            .unwrap_or(scan_root)
            .display()
            .to_string()
            .replace('\\', "/"),
        "totalWarnings": total_warnings,
        "maxTotalWarnings": max_total_warnings,
        "warningByRule": rule_breakdown,
    }));

    match payload.and_then(|payload| fs::write(report_path, payload).map_err(serde_json::Error::io))
    {
        Ok(_) => true,
        Err(error) => {
            warnings.push(format!(
                "Could not write warning baseline report: {} :: {error}",
                report_path.display()
            ));
            false
        }
    }
}
