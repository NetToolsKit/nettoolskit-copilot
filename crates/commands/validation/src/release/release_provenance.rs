//! Release provenance validation.

use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use crate::agent_orchestration::common::read_required_json_document;
use crate::error::ValidateReleaseProvenanceCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::release::common::{
    collect_changelog_matches, current_utc_date, invoke_git_command, parse_iso_date,
    resolve_release_path, resolve_release_repo_root, to_repo_relative_path, ChangelogEntry,
};
use crate::ValidationCheckStatus;

const DEFAULT_BASELINE_PATH: &str = ".github/governance/release-provenance.baseline.json";
const DEFAULT_AUDIT_REPORT_PATH: &str = ".temp/audit-report.json";

/// Request payload for `validate-release-provenance`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReleaseProvenanceRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional baseline path override.
    pub baseline_path: Option<PathBuf>,
    /// Optional audit report path override.
    pub audit_report_path: Option<PathBuf>,
    /// Force audit-report validation even if the baseline does not require it.
    pub require_audit_report: bool,
    /// Convert required findings into warnings.
    pub warning_only: bool,
}

impl Default for ValidateReleaseProvenanceRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            audit_report_path: None,
            require_audit_report: false,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-release-provenance`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReleaseProvenanceResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// Resolved audit report path.
    pub audit_report_path: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Effective require-audit-report mode.
    pub require_audit_report: bool,
    /// Number of required checks declared by the baseline.
    pub checks_declared: usize,
    /// Number of validation checks found in `validate-all`.
    pub checks_found_in_validate_all: usize,
    /// Number of required evidence files declared by the baseline.
    pub evidence_files: usize,
    /// Latest changelog version when one could be parsed.
    pub latest_version: Option<String>,
    /// Current git branch when git metadata could be resolved.
    pub current_branch: Option<String>,
    /// Current HEAD commit when git metadata could be resolved.
    pub head_commit: Option<String>,
    /// Whether git is available in the environment.
    pub git_available: bool,
    /// Whether the worktree is dirty when git metadata could be resolved.
    pub is_dirty: Option<bool>,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct ReleaseProvenanceBaseline {
    version: i64,
    release_branch: String,
    require_clean_worktree: bool,
    warn_on_dirty_worktree: bool,
    require_audit_report: bool,
    warn_on_missing_optional_audit_report: bool,
    warn_on_audit_commit_mismatch: bool,
    changelog_path: String,
    validate_all_path: String,
    required_validation_checks: Vec<String>,
    required_evidence_files: Vec<String>,
}

/// Run the release provenance validation.
///
/// # Errors
///
/// Returns [`ValidateReleaseProvenanceCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_release_provenance(
    request: &ValidateReleaseProvenanceRequest,
) -> Result<ValidateReleaseProvenanceResult, ValidateReleaseProvenanceCommandError> {
    let repo_root = resolve_release_repo_root(request.repo_root.as_deref())
        .map_err(|source| ValidateReleaseProvenanceCommandError::ResolveWorkspaceRoot { source })?;
    let baseline_path = resolve_release_path(
        &repo_root,
        request.baseline_path.as_deref(),
        DEFAULT_BASELINE_PATH,
    );
    let audit_report_path = resolve_release_path(
        &repo_root,
        request.audit_report_path.as_deref(),
        DEFAULT_AUDIT_REPORT_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let baseline = read_required_json_document::<ReleaseProvenanceBaseline>(
        &baseline_path,
        "release provenance baseline",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let Some(baseline) = baseline else {
        let status = derive_status(&warnings, &failures);
        let exit_code = if failures.is_empty() { 0 } else { 1 };
        return Ok(ValidateReleaseProvenanceResult {
            repo_root,
            baseline_path,
            audit_report_path,
            warning_only: request.warning_only,
            require_audit_report: request.require_audit_report,
            checks_declared: 0,
            checks_found_in_validate_all: 0,
            evidence_files: 0,
            latest_version: None,
            current_branch: None,
            head_commit: None,
            git_available: false,
            is_dirty: None,
            warnings,
            failures,
            status,
            exit_code,
        });
    };

    if baseline.version < 1 {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "release-provenance baseline version must be >= 1.".to_string(),
        );
    }

    if baseline.changelog_path.trim().is_empty() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "release-provenance baseline must define changelogPath.".to_string(),
        );
    }
    if baseline.validate_all_path.trim().is_empty() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "release-provenance baseline must define validateAllPath.".to_string(),
        );
    }
    if baseline.required_validation_checks.is_empty() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "release-provenance baseline must define at least one requiredValidationCheck."
                .to_string(),
        );
    }
    if baseline.required_evidence_files.is_empty() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "release-provenance baseline must define at least one requiredEvidenceFile."
                .to_string(),
        );
    }

    let changelog_path = resolve_release_path(&repo_root, None, &baseline.changelog_path);
    let validate_all_path = resolve_release_path(&repo_root, None, &baseline.validate_all_path);
    let effective_require_audit_report =
        baseline.require_audit_report || request.require_audit_report;

    let latest_entry = validate_latest_changelog_entry(
        &repo_root,
        &changelog_path,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let defined_checks = collect_validate_all_check_names(
        &repo_root,
        &validate_all_path,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    validate_check_coverage(
        &baseline.required_validation_checks,
        &defined_checks,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    validate_evidence_files(
        &repo_root,
        &baseline.required_evidence_files,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let mut git_available = false;
    let mut current_branch = None;
    let mut head_commit = None;
    let mut is_dirty = None;
    if std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_err()
    {
        warnings.push("Git command not found; skipping git provenance checks.".to_string());
    } else {
        git_available = true;

        let branch_result = invoke_git_command(&repo_root, &["rev-parse", "--abbrev-ref", "HEAD"]);
        let branch = branch_result.output_lines.first().cloned();
        if branch_result.exit_code != 0 || branch.as_deref().unwrap_or_default().is_empty() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                "Could not resolve current git branch.".to_string(),
            );
        } else {
            current_branch = branch;
        }

        if let Some(branch) = current_branch.as_deref() {
            if !baseline.release_branch.trim().is_empty() && branch != baseline.release_branch {
                warnings.push(format!(
                    "Current branch '{branch}' differs from releaseBranch '{}'.",
                    baseline.release_branch
                ));
            }
        }

        let head_result = invoke_git_command(&repo_root, &["rev-parse", "HEAD"]);
        let head = head_result.output_lines.first().cloned();
        if head_result.exit_code != 0 || head.as_deref().unwrap_or_default().is_empty() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                "Could not resolve HEAD commit hash.".to_string(),
            );
        } else {
            head_commit = head;
        }

        let status_result = invoke_git_command(&repo_root, &["status", "--porcelain"]);
        if status_result.exit_code == 0 {
            let dirty = !status_result.output_lines.is_empty();
            is_dirty = Some(dirty);
            if dirty && baseline.require_clean_worktree {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    "Worktree is dirty and release-provenance baseline requires clean state."
                        .to_string(),
                );
            } else if dirty && baseline.warn_on_dirty_worktree {
                warnings.push(
                    "Worktree is dirty; provenance checks usually run cleaner in committed state."
                        .to_string(),
                );
            }
        }

        validate_git_evidence_traceability(
            &repo_root,
            &baseline.required_evidence_files,
            is_dirty.unwrap_or(false),
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    validate_audit_report_contract(
        &audit_report_path,
        &AuditReportContractOptions {
            is_required: effective_require_audit_report,
            head_commit: head_commit.as_deref(),
            warn_on_missing_optional_report: baseline.warn_on_missing_optional_audit_report,
            warn_on_commit_mismatch: baseline.warn_on_audit_commit_mismatch,
            warning_only: request.warning_only,
        },
        &mut warnings,
        &mut failures,
    );

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateReleaseProvenanceResult {
        repo_root,
        baseline_path,
        audit_report_path,
        warning_only: request.warning_only,
        require_audit_report: effective_require_audit_report,
        checks_declared: baseline.required_validation_checks.len(),
        checks_found_in_validate_all: defined_checks.len(),
        evidence_files: baseline.required_evidence_files.len(),
        latest_version: latest_entry.map(|entry| entry.version),
        current_branch,
        head_commit,
        git_available,
        is_dirty,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn validate_latest_changelog_entry(
    repo_root: &Path,
    changelog_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<ChangelogEntry> {
    if !changelog_path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Changelog file not found: {}",
                to_repo_relative_path(repo_root, changelog_path)
            ),
        );
        return None;
    }

    let content = match fs::read_to_string(changelog_path) {
        Ok(content) => content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Failed to read changelog: {error}"),
            );
            return None;
        }
    };

    let Some((version, date_token)) = collect_changelog_matches(&content).into_iter().next() else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "CHANGELOG has no entries matching [X.Y.Z] - YYYY-MM-DD.".to_string(),
        );
        return None;
    };

    let Some(date) = parse_iso_date(&date_token) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Invalid latest changelog date: {date_token}"),
        );
        return None;
    };

    if let Some(today) = current_utc_date() {
        if date > today {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Latest changelog date is in the future: {date_token}"),
            );
        }
    }

    Some(ChangelogEntry {
        version,
        date_token,
        date,
    })
}

fn collect_validate_all_check_names(
    repo_root: &Path,
    validate_all_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<String> {
    if !validate_all_path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "validate-all script not found: {}",
                to_repo_relative_path(repo_root, validate_all_path)
            ),
        );
        return Vec::new();
    }

    let content = match fs::read_to_string(validate_all_path) {
        Ok(content) => content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Failed to read validate-all script: {error}"),
            );
            return Vec::new();
        }
    };

    let regex = Regex::new(r"name\s*=\s*'(?P<name>[^']+)'")
        .expect("release provenance validate-all regex should compile");
    let mut check_names = regex
        .captures_iter(&content)
        .filter_map(|captures| captures.name("name").map(|name| name.as_str().to_string()))
        .collect::<Vec<_>>();
    check_names.sort();
    check_names.dedup();
    check_names
}

fn validate_check_coverage(
    required_checks: &[String],
    defined_checks: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for required_check in required_checks {
        if !defined_checks
            .iter()
            .any(|defined| defined == required_check)
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Required check missing from validate-all: {required_check}"),
            );
        }
    }
}

fn validate_evidence_files(
    repo_root: &Path,
    evidence_files: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for evidence_file in evidence_files {
        let evidence_path = repo_root.join(evidence_file);
        if !evidence_path.is_file() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Required evidence file not found: {evidence_file}"),
            );
        }
    }
}

fn validate_git_evidence_traceability(
    repo_root: &Path,
    evidence_files: &[String],
    allow_pending_changes: bool,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for evidence_file in evidence_files {
        let tracked_result = invoke_git_command(
            repo_root,
            &["ls-files", "--error-unmatch", "--", evidence_file],
        );
        if tracked_result.exit_code != 0 {
            let message = if allow_pending_changes {
                format!(
                    "Evidence file is not tracked by git yet (pending changes): {evidence_file}"
                )
            } else {
                format!("Evidence file is not tracked by git: {evidence_file}")
            };
            if allow_pending_changes {
                warnings.push(message);
            } else {
                push_required_finding(warning_only, warnings, failures, message);
            }
            continue;
        }

        let history_result = invoke_git_command(
            repo_root,
            &["log", "-1", "--format=%H", "--", evidence_file],
        );
        let last_commit = history_result.output_lines.first().cloned();
        if history_result.exit_code != 0 || last_commit.as_deref().unwrap_or_default().is_empty() {
            let message = if allow_pending_changes {
                format!(
                    "No committed history for evidence file yet (pending changes): {evidence_file}"
                )
            } else {
                format!("No git history found for evidence file: {evidence_file}")
            };
            if allow_pending_changes {
                warnings.push(message);
            } else {
                push_required_finding(warning_only, warnings, failures, message);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
struct AuditReportContractOptions<'a> {
    is_required: bool,
    head_commit: Option<&'a str>,
    warn_on_missing_optional_report: bool,
    warn_on_commit_mismatch: bool,
    warning_only: bool,
}

fn validate_audit_report_contract(
    audit_report_path: &Path,
    options: &AuditReportContractOptions<'_>,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if !audit_report_path.is_file() {
        if options.is_required {
            push_required_finding(
                options.warning_only,
                warnings,
                failures,
                format!(
                    "Required audit report not found: {}",
                    audit_report_path.display()
                ),
            );
        } else if options.warn_on_missing_optional_report {
            warnings.push(format!(
                "Audit report not found (optional): {}",
                audit_report_path.display()
            ));
        }
        return;
    }

    let document = match fs::read_to_string(audit_report_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                options.warning_only,
                warnings,
                failures,
                format!("Invalid JSON in audit report: {error}"),
            );
            return;
        }
    };

    let audit_report = match serde_json::from_str::<Value>(&document) {
        Ok(audit_report) => audit_report,
        Err(error) => {
            push_required_finding(
                options.warning_only,
                warnings,
                failures,
                format!("Invalid JSON in audit report: {error}"),
            );
            return;
        }
    };

    let overall_status = audit_report
        .get("summary")
        .and_then(|summary| summary.get("overallStatus"))
        .and_then(Value::as_str);
    match overall_status {
        None | Some("") => push_required_finding(
            options.warning_only,
            warnings,
            failures,
            "Audit report summary.overallStatus is missing.".to_string(),
        ),
        Some("passed") => {}
        Some(status) => push_required_finding(
            options.warning_only,
            warnings,
            failures,
            format!("Audit report overallStatus must be 'passed' but found '{status}'."),
        ),
    }

    if options.warn_on_commit_mismatch {
        if let (Some(head_commit), Some(audit_commit)) = (
            options.head_commit,
            audit_report
                .get("git")
                .and_then(|git| git.get("commit"))
                .and_then(Value::as_str),
        ) {
            if !audit_commit.trim().is_empty() && audit_commit != head_commit {
                warnings.push(format!(
                    "Audit report commit differs from HEAD (audit={audit_commit}, head={head_commit})."
                ));
            }
        }
    }
}
