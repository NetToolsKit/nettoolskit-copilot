//! Architecture boundary baseline validation.

use std::fs;
use std::path::{Path, PathBuf};

use globset::GlobBuilder;
use regex::Regex;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::agent_orchestration::common::{
    resolve_repo_relative_path, resolve_validation_repo_root,
};
use crate::error::ValidateArchitectureBoundariesCommandError;
use crate::operational_hygiene::common::derive_status;
use crate::ValidationCheckStatus;

const DEFAULT_BASELINE_PATH: &str = ".github/governance/architecture-boundaries.baseline.json";

/// Request payload for `validate-architecture-boundaries`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateArchitectureBoundariesRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    pub baseline_path: Option<PathBuf>,
}

impl Default for ValidateArchitectureBoundariesRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
        }
    }
}

/// Result payload for `validate-architecture-boundaries`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateArchitectureBoundariesResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// Number of rules evaluated.
    pub rules_checked: usize,
    /// Number of file-rule evaluations performed.
    pub file_checks: usize,
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
struct ArchitectureBoundariesBaseline {
    #[serde(default)]
    rules: Vec<BoundaryRule>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BoundaryRule {
    #[serde(default)]
    id: String,
    #[serde(default)]
    files: Vec<String>,
    #[serde(default)]
    required_patterns: Vec<String>,
    #[serde(default)]
    forbidden_patterns: Vec<String>,
    #[serde(default)]
    allowed_patterns: Vec<String>,
    #[serde(default)]
    severity: String,
}

#[derive(Debug, Clone)]
struct RepositoryFile {
    absolute_path: PathBuf,
    relative_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleSeverity {
    Failure,
    Warning,
}

impl RuleSeverity {
    fn from_text(value: &str) -> Self {
        if value.trim().eq_ignore_ascii_case("warning") {
            Self::Warning
        } else {
            Self::Failure
        }
    }
}

/// Run the architecture boundary validation sweep.
///
/// # Errors
///
/// Returns [`ValidateArchitectureBoundariesCommandError`] when the repository
/// root cannot be resolved.
pub fn invoke_validate_architecture_boundaries(
    request: &ValidateArchitectureBoundariesRequest,
) -> Result<ValidateArchitectureBoundariesResult, ValidateArchitectureBoundariesCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
        ValidateArchitectureBoundariesCommandError::ResolveWorkspaceRoot { source }
    })?;
    let baseline_path = resolve_repo_relative_path(
        &repo_root,
        request.baseline_path.as_deref(),
        DEFAULT_BASELINE_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut file_checks = 0usize;

    let baseline = if !baseline_path.is_file() {
        failures.push(format!(
            "Baseline file not found: {}",
            request
                .baseline_path
                .as_ref()
                .map_or_else(|| DEFAULT_BASELINE_PATH.to_string(), |path| {
                    path.to_string_lossy().to_string()
                })
        ));
        None
    } else {
        read_baseline(&baseline_path, &mut failures)
    };

    let mut rules_checked = 0usize;
    if let Some(baseline) = baseline {
        if baseline.rules.is_empty() {
            failures.push(format!(
                "Baseline must include at least one rule: {}",
                to_repo_relative_path(&repo_root, &baseline_path)
            ));
        } else {
            rules_checked = baseline.rules.len();
            let repository_files = collect_repository_files(&repo_root);
            for rule in &baseline.rules {
                validate_rule(
                    &repo_root,
                    rule,
                    &repository_files,
                    &mut file_checks,
                    &mut warnings,
                    &mut failures,
                );
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateArchitectureBoundariesResult {
        repo_root,
        baseline_path,
        rules_checked,
        file_checks,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_baseline(
    baseline_path: &Path,
    failures: &mut Vec<String>,
) -> Option<ArchitectureBoundariesBaseline> {
    let label = baseline_path.to_string_lossy();
    let content = match fs::read_to_string(baseline_path) {
        Ok(content) => content,
        Err(error) => {
            failures.push(format!("Invalid JSON in baseline file {label}: {error}"));
            return None;
        }
    };

    match serde_json::from_str::<ArchitectureBoundariesBaseline>(&content) {
        Ok(document) => Some(document),
        Err(error) => {
            failures.push(format!("Invalid JSON in baseline file {label}: {error}"));
            None
        }
    }
}

fn collect_repository_files(repo_root: &Path) -> Vec<RepositoryFile> {
    let mut files = WalkDir::new(repo_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| !entry.path().components().any(|component| component.as_os_str() == ".git"))
        .map(|entry| {
            let absolute_path = entry.into_path();
            let relative_path = to_repo_relative_path(repo_root, &absolute_path);
            RepositoryFile {
                absolute_path,
                relative_path,
            }
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    files
}

fn validate_rule(
    repo_root: &Path,
    rule: &BoundaryRule,
    repository_files: &[RepositoryFile],
    file_checks: &mut usize,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let rule_id = if rule.id.trim().is_empty() {
        "unnamed-rule".to_string()
    } else {
        rule.id.trim().to_string()
    };
    let severity = RuleSeverity::from_text(&rule.severity);
    let matched_files = resolve_rule_files(repo_root, repository_files, &rule_id, &rule.files, warnings);
    if matched_files.is_empty() {
        warnings.push(format!("Boundary rule '{rule_id}' has no files to evaluate."));
        return;
    }

    let required_patterns = compile_patterns(&rule.required_patterns, &rule_id, "required", failures);
    let forbidden_patterns = compile_patterns(&rule.forbidden_patterns, &rule_id, "forbidden", failures);
    let allowed_patterns = compile_patterns(&rule.allowed_patterns, &rule_id, "allowed", failures);

    for file in matched_files {
        *file_checks += 1;
        let content = match fs::read_to_string(&file.absolute_path) {
            Ok(content) => content,
            Err(error) => {
                failures.push(format!(
                    "Boundary rule '{rule_id}' could not read file {}: {error}",
                    file.relative_path
                ));
                continue;
            }
        };

        for pattern in &required_patterns {
            if !pattern.is_match(&content) {
                push_rule_issue(
                    severity,
                    warnings,
                    failures,
                    format!(
                        "Boundary rule '{rule_id}' missing required pattern in {}: {}",
                        file.relative_path,
                        pattern.as_str()
                    ),
                );
            }
        }

        for pattern in &forbidden_patterns {
            for matched in pattern.find_iter(&content) {
                let allowed = allowed_patterns
                    .iter()
                    .any(|allow_pattern| allow_pattern.is_match(matched.as_str()));
                if allowed {
                    continue;
                }
                let line = line_number_from_index(&content, matched.start());
                push_rule_issue(
                    severity,
                    warnings,
                    failures,
                    format!(
                        "Boundary rule '{rule_id}' forbidden pattern in {}:{} :: {}",
                        file.relative_path,
                        line,
                        pattern.as_str()
                    ),
                );
            }
        }
    }
}

fn resolve_rule_files(
    repo_root: &Path,
    repository_files: &[RepositoryFile],
    rule_id: &str,
    patterns: &[String],
    warnings: &mut Vec<String>,
) -> Vec<RepositoryFile> {
    let mut matched = Vec::new();
    for pattern in patterns.iter().filter(|pattern| !pattern.trim().is_empty()) {
        let normalized = pattern.replace('\\', "/");
        if contains_wildcard(&normalized) {
            let Ok(glob) = GlobBuilder::new(&normalized).case_insensitive(true).build() else {
                warnings.push(format!(
                    "Boundary rule '{rule_id}' has invalid wildcard pattern: {pattern}"
                ));
                continue;
            };
            let matcher = glob.compile_matcher();
            let mut matched_any = false;
            for file in repository_files {
                if matcher.is_match(&file.relative_path) {
                    matched.push(file.clone());
                    matched_any = true;
                }
            }
            if !matched_any {
                warnings.push(format!(
                    "Boundary rule '{rule_id}' pattern matched no files: {pattern}"
                ));
            }
            continue;
        }

        let absolute_path = resolve_repo_relative_path(repo_root, Some(Path::new(&normalized)), &normalized);
        if !absolute_path.is_file() {
            warnings.push(format!(
                "Boundary rule '{rule_id}' references missing file: {pattern}"
            ));
            continue;
        }

        matched.push(RepositoryFile {
            relative_path: to_repo_relative_path(repo_root, &absolute_path),
            absolute_path,
        });
    }

    matched.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    matched.dedup_by(|left, right| left.relative_path == right.relative_path);
    matched
}

fn compile_patterns(
    patterns: &[String],
    rule_id: &str,
    label: &str,
    failures: &mut Vec<String>,
) -> Vec<Regex> {
    let mut compiled = Vec::new();
    for pattern in patterns.iter().filter(|pattern| !pattern.trim().is_empty()) {
        match Regex::new(pattern) {
            Ok(regex) => compiled.push(regex),
            Err(error) => failures.push(format!(
                "Boundary rule '{rule_id}' has invalid {label} pattern '{pattern}': {error}"
            )),
        }
    }
    compiled
}

fn contains_wildcard(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn line_number_from_index(content: &str, index: usize) -> usize {
    content[..index.min(content.len())]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1
}

fn push_rule_issue(
    severity: RuleSeverity,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
    message: String,
) {
    match severity {
        RuleSeverity::Failure => failures.push(message),
        RuleSeverity::Warning => warnings.push(message),
    }
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}