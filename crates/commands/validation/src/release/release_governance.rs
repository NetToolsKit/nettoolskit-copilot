//! Release governance validation.

use std::fs;
use std::path::PathBuf;

use regex::Regex;
use serde::Deserialize;

use crate::error::ValidateReleaseGovernanceCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::release::common::{
    collect_changelog_matches, parse_iso_date, resolve_release_path, resolve_release_repo_root,
    to_repo_relative_path, ChangelogEntry,
};
use crate::ValidationCheckStatus;

const DEFAULT_CHANGELOG_PATH: &str = "CHANGELOG.md";
const DEFAULT_CODEOWNERS_PATH: &str = "CODEOWNERS";
const DEFAULT_GOVERNANCE_DOC_PATH: &str = ".github/governance/release-governance.md";
const DEFAULT_BRANCH_PROTECTION_BASELINE_PATH: &str =
    ".github/governance/branch-protection.baseline.json";

/// Request payload for `validate-release-governance`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReleaseGovernanceRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional changelog path override.
    pub changelog_path: Option<PathBuf>,
    /// Optional CODEOWNERS path override.
    pub codeowners_path: Option<PathBuf>,
    /// Optional governance document path override.
    pub governance_doc_path: Option<PathBuf>,
    /// Optional branch protection baseline path override.
    pub branch_protection_baseline_path: Option<PathBuf>,
    /// Convert required findings into warnings.
    pub warning_only: bool,
}

impl Default for ValidateReleaseGovernanceRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            changelog_path: None,
            codeowners_path: None,
            governance_doc_path: None,
            branch_protection_baseline_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-release-governance`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReleaseGovernanceResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved changelog path.
    pub changelog_path: PathBuf,
    /// Resolved CODEOWNERS path.
    pub codeowners_path: PathBuf,
    /// Resolved governance doc path.
    pub governance_doc_path: PathBuf,
    /// Resolved branch protection baseline path.
    pub branch_protection_baseline_path: PathBuf,
    /// Latest changelog version when one could be parsed.
    pub latest_version: Option<String>,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct BranchProtectionBaseline {
    repository: String,
    branch: String,
    protection: Option<BranchProtectionSettings>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct BranchProtectionSettings {
    required_status_checks: Option<RequiredStatusChecks>,
    enforce_admins: bool,
    required_pull_request_reviews: Option<RequiredPullRequestReviews>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RequiredStatusChecks {
    strict: bool,
    contexts: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RequiredPullRequestReviews {
    require_code_owner_reviews: bool,
    required_approving_review_count: i64,
}

/// Run the release governance validation.
///
/// # Errors
///
/// Returns [`ValidateReleaseGovernanceCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_release_governance(
    request: &ValidateReleaseGovernanceRequest,
) -> Result<ValidateReleaseGovernanceResult, ValidateReleaseGovernanceCommandError> {
    let repo_root = resolve_release_repo_root(request.repo_root.as_deref()).map_err(|source| {
        ValidateReleaseGovernanceCommandError::ResolveWorkspaceRoot { source }
    })?;
    let changelog_path =
        resolve_release_path(&repo_root, request.changelog_path.as_deref(), DEFAULT_CHANGELOG_PATH);
    let codeowners_path = resolve_release_path(
        &repo_root,
        request.codeowners_path.as_deref(),
        DEFAULT_CODEOWNERS_PATH,
    );
    let governance_doc_path = resolve_release_path(
        &repo_root,
        request.governance_doc_path.as_deref(),
        DEFAULT_GOVERNANCE_DOC_PATH,
    );
    let branch_protection_baseline_path = resolve_release_path(
        &repo_root,
        request.branch_protection_baseline_path.as_deref(),
        DEFAULT_BRANCH_PROTECTION_BASELINE_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    for required_path in [
        &changelog_path,
        &codeowners_path,
        &governance_doc_path,
        &branch_protection_baseline_path,
    ] {
        if !required_path.is_file() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Required file not found: {}",
                    to_repo_relative_path(&repo_root, required_path)
                ),
            );
        }
    }

    let latest_version = if changelog_path.is_file() {
        validate_changelog(
            &repo_root,
            &changelog_path,
            request.warning_only,
            &mut warnings,
            &mut failures,
        )
    } else {
        None
    };

    if codeowners_path.is_file() {
        validate_codeowners(
            &codeowners_path,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    if governance_doc_path.is_file() {
        validate_governance_document(
            &governance_doc_path,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    if branch_protection_baseline_path.is_file() {
        validate_branch_protection_baseline(
            &branch_protection_baseline_path,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateReleaseGovernanceResult {
        repo_root,
        warning_only: request.warning_only,
        changelog_path,
        codeowners_path,
        governance_doc_path,
        branch_protection_baseline_path,
        latest_version,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn validate_changelog(
    repo_root: &std::path::Path,
    changelog_path: &std::path::Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<String> {
    let content = match fs::read_to_string(changelog_path) {
        Ok(content) => content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Failed to read changelog '{}': {error}",
                    to_repo_relative_path(repo_root, changelog_path)
                ),
            );
            return None;
        }
    };

    let raw_entries = collect_changelog_matches(&content);
    if raw_entries.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "CHANGELOG does not contain entries in format [X.Y.Z] - YYYY-MM-DD.".to_string(),
        );
        return None;
    }

    let mut entries = Vec::new();
    for (version, date_token) in raw_entries {
        let Some(date) = parse_iso_date(&date_token) else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid changelog date '{date_token}' for version {version}."),
            );
            continue;
        };

        entries.push(ChangelogEntry {
            version,
            date_token,
            date,
        });
    }

    for index in 1..entries.len() {
        if entries[index].date > entries[index - 1].date {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "CHANGELOG date order invalid: {} appears after newer entry {}.",
                    entries[index].date_token, entries[index - 1].date_token
                ),
            );
            break;
        }
    }

    entries.first().map(|entry| entry.version.clone())
}

fn validate_codeowners(
    codeowners_path: &std::path::Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let content = match fs::read_to_string(codeowners_path) {
        Ok(content) => content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Failed to read CODEOWNERS: {error}"),
            );
            return;
        }
    };

    let active_lines = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    if active_lines.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "CODEOWNERS has no active rules.".to_string(),
        );
        return;
    }

    let catch_all_regex =
        Regex::new(r"^\*\s+@").expect("release governance CODEOWNERS regex should compile");
    if !active_lines.iter().any(|line| catch_all_regex.is_match(line)) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "CODEOWNERS must define a catch-all owner rule: \"* @owner\".".to_string(),
        );
    }

    for required_path in [".github/", ".githooks/", "scripts/"] {
        let path_regex = Regex::new(&format!(r"^{}\s+@", regex::escape(required_path)))
            .expect("release governance path regex should compile");
        if !active_lines.iter().any(|line| path_regex.is_match(line)) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("CODEOWNERS missing required ownership rule for '{required_path}'."),
            );
        }
    }
}

fn validate_governance_document(
    governance_doc_path: &std::path::Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let content = match fs::read_to_string(governance_doc_path) {
        Ok(content) => content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Failed to read release governance document: {error}"),
            );
            return;
        }
    };

    for section_pattern in [
        r"(?m)^## Scope",
        r"(?m)^## Branch Protection",
        r"(?m)^## CODEOWNERS",
        r"(?m)^## Release Checklist",
        r"(?m)^## Rollback",
    ] {
        let section_regex =
            Regex::new(section_pattern).expect("release governance section regex should compile");
        if !section_regex.is_match(&content) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Release governance doc missing section matching '{}'.",
                    section_pattern.trim_start_matches("(?m)")
                ),
            );
        }
    }
}

fn validate_branch_protection_baseline(
    baseline_path: &std::path::Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let content = match fs::read_to_string(baseline_path) {
        Ok(content) => content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid JSON in branch protection baseline: {error}"),
            );
            return;
        }
    };

    let baseline = match serde_json::from_str::<BranchProtectionBaseline>(&content) {
        Ok(baseline) => baseline,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid JSON in branch protection baseline: {error}"),
            );
            return;
        }
    };

    let repository_regex =
        Regex::new(r"^[^/]+/[^/]+$").expect("release governance repository regex should compile");
    if baseline.repository.trim().is_empty() || !repository_regex.is_match(&baseline.repository) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "branch-protection.baseline.json must include repository in owner/name format."
                .to_string(),
        );
    }

    if baseline.branch.trim().is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "branch-protection.baseline.json must include a target branch.".to_string(),
        );
    }

    let Some(protection) = baseline.protection else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "branch-protection.baseline.json must include a protection object.".to_string(),
        );
        return;
    };

    let Some(status_checks) = protection.required_status_checks else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Branch protection baseline must define required_status_checks.".to_string(),
        );
        return;
    };

    if status_checks.contexts.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Branch protection baseline must define at least one required status check context."
                .to_string(),
        );
    }

    if !status_checks.strict {
        warnings.push(
            "Branch protection baseline has strict=false (recommended strict=true).".to_string(),
        );
    }

    if !protection.enforce_admins {
        warnings
            .push("Branch protection baseline has enforce_admins=false (recommended true).".to_string());
    }

    let Some(reviews) = protection.required_pull_request_reviews else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Branch protection baseline must define required_pull_request_reviews.".to_string(),
        );
        return;
    };

    if !reviews.require_code_owner_reviews {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Branch protection baseline must set require_code_owner_reviews=true.".to_string(),
        );
    }

    if reviews.required_approving_review_count < 1 {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Branch protection baseline must require at least 1 approving review.".to_string(),
        );
    }
}