//! Repository security baseline validation.

use std::fs;
use std::path::{Path, PathBuf};

use fancy_regex::Regex as FancyRegex;
use globset::GlobSet;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::agent_orchestration::common::{
    compile_globset, normalize_path, resolve_repo_relative_path, resolve_validation_repo_root,
};
use crate::error::ValidateSecurityBaselineCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::ValidationCheckStatus;

const DEFAULT_BASELINE_PATH: &str = ".github/governance/security-baseline.json";

/// Request payload for `validate-security-baseline`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSecurityBaselineRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit security baseline path.
    pub baseline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateSecurityBaselineRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-security-baseline`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSecurityBaselineResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// Number of repository files evaluated after exclusions.
    pub repository_files_evaluated: usize,
    /// Number of repository files selected for content scanning.
    pub files_scanned: usize,
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
#[serde(rename_all = "camelCase")]
struct SecurityBaseline {
    #[serde(default)]
    required_files: Vec<String>,
    #[serde(default)]
    required_directories: Vec<String>,
    #[serde(default)]
    scan_extensions: Vec<String>,
    #[serde(default)]
    excluded_path_globs: Vec<String>,
    #[serde(default)]
    forbidden_path_globs: Vec<String>,
    #[serde(default)]
    forbidden_content_patterns: Vec<ForbiddenContentPattern>,
    #[serde(default)]
    allowed_content_patterns: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ForbiddenContentPattern {
    #[serde(default)]
    id: String,
    #[serde(default)]
    pattern: String,
    #[serde(default)]
    severity: String,
}

#[derive(Debug, Clone)]
struct RepositoryFileEntry {
    full_path: PathBuf,
    relative_path: String,
    extension: String,
}

#[derive(Debug)]
struct CompiledContentRule {
    id: String,
    severity: ContentRuleSeverity,
    regex: FancyRegex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContentRuleSeverity {
    Failure,
    Warning,
}

/// Run the repository security baseline validation.
///
/// # Errors
///
/// Returns [`ValidateSecurityBaselineCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_security_baseline(
    request: &ValidateSecurityBaselineRequest,
) -> Result<ValidateSecurityBaselineResult, ValidateSecurityBaselineCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
        ValidateSecurityBaselineCommandError::ResolveWorkspaceRoot { source }
    })?;
    let baseline_path = resolve_repo_relative_path(
        &repo_root,
        request.baseline_path.as_deref(),
        DEFAULT_BASELINE_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut repository_files_evaluated = 0usize;
    let mut files_scanned = 0usize;

    let baseline = if baseline_path.is_file() {
        read_security_baseline(
            &baseline_path,
            request.warning_only,
            &mut warnings,
            &mut failures,
        )
    } else {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Security baseline file not found: {}",
                to_repo_relative_path(&repo_root, &baseline_path)
            ),
        );
        None
    };

    if let Some(baseline) = baseline {
        validate_required_paths(
            &repo_root,
            &baseline.required_files,
            &baseline.required_directories,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        let repository_files = collect_repository_files(
            &repo_root,
            &baseline.excluded_path_globs,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        repository_files_evaluated = repository_files.len();

        validate_forbidden_paths(
            &repository_files,
            &baseline.forbidden_path_globs,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        let scan_files = content_scan_files(&repository_files, &baseline.scan_extensions);
        files_scanned = scan_files.len();

        let allowed_patterns = compile_regex_list(
            &baseline.allowed_content_patterns,
            "allowedContentPatterns",
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        let forbidden_rules = compile_content_rules(
            &baseline.forbidden_content_patterns,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        if forbidden_rules.is_empty() {
            warnings.push("No forbiddenContentPatterns configured in security baseline.".to_string());
        } else {
            validate_forbidden_content(
                &scan_files,
                &forbidden_rules,
                &allowed_patterns,
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateSecurityBaselineResult {
        repo_root,
        warning_only: request.warning_only,
        baseline_path,
        repository_files_evaluated,
        files_scanned,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_security_baseline(
    baseline_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<SecurityBaseline> {
    let document = match fs::read_to_string(baseline_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Could not read security baseline {}: {error}",
                    baseline_path.display()
                ),
            );
            return None;
        }
    };

    match serde_json::from_str::<SecurityBaseline>(&document) {
        Ok(baseline) => Some(baseline),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Invalid JSON in security baseline file {}: {error}",
                    baseline_path.display()
                ),
            );
            None
        }
    }
}

fn validate_required_paths(
    repo_root: &Path,
    required_files: &[String],
    required_directories: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for required_file in required_files {
        if required_file.trim().is_empty() {
            continue;
        }

        if !repo_root.join(required_file).is_file() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Missing required file: {required_file}"),
            );
        }
    }

    for required_directory in required_directories {
        if required_directory.trim().is_empty() {
            continue;
        }

        if !repo_root.join(required_directory).is_dir() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Missing required directory: {required_directory}"),
            );
        }
    }
}

fn collect_repository_files(
    repo_root: &Path,
    excluded_path_globs: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<RepositoryFileEntry> {
    let excluded_globset = compile_globset(
        excluded_path_globs,
        "excludedPathGlobs",
        warning_only,
        warnings,
        failures,
    );

    let mut entries = WalkDir::new(repo_root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.path() == repo_root {
                return true;
            }

            let relative = to_repo_relative_path(repo_root, entry.path());
            !matches_globset(&relative, excluded_globset.as_ref())
        })
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let path = entry.into_path();
            RepositoryFileEntry {
                relative_path: to_repo_relative_path(repo_root, &path),
                extension: path
                    .extension()
                    .map(|extension| format!(".{}", extension.to_string_lossy().to_ascii_lowercase()))
                    .unwrap_or_default(),
                full_path: path,
            }
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    entries
}

fn validate_forbidden_paths(
    repository_files: &[RepositoryFileEntry],
    forbidden_path_globs: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let forbidden_globset = compile_globset(
        forbidden_path_globs,
        "forbiddenPathGlobs",
        warning_only,
        warnings,
        failures,
    );

    for entry in repository_files {
        if matches_globset(&entry.relative_path, forbidden_globset.as_ref()) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Forbidden sensitive file path found: {}", entry.relative_path),
            );
        }
    }
}

fn content_scan_files(
    repository_files: &[RepositoryFileEntry],
    scan_extensions: &[String],
) -> Vec<RepositoryFileEntry> {
    let normalized_extensions = scan_extensions
        .iter()
        .map(|extension| extension.trim())
        .filter(|extension| !extension.is_empty())
        .map(|extension| {
            if extension.starts_with('.') {
                extension.to_ascii_lowercase()
            } else {
                format!(".{}", extension.to_ascii_lowercase())
            }
        })
        .collect::<std::collections::BTreeSet<_>>();

    if normalized_extensions.is_empty() {
        return Vec::new();
    }

    repository_files
        .iter()
        .filter(|entry| normalized_extensions.contains(&entry.extension))
        .cloned()
        .collect()
}

fn compile_content_rules(
    pattern_objects: &[ForbiddenContentPattern],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<CompiledContentRule> {
    let mut rules = Vec::new();
    for pattern_object in pattern_objects {
        let rule_id = if pattern_object.id.trim().is_empty() {
            "unnamed-pattern".to_string()
        } else {
            pattern_object.id.trim().to_string()
        };
        if pattern_object.pattern.trim().is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Security baseline pattern has empty regex: {rule_id}"),
            );
            continue;
        }

        let severity = if pattern_object.severity.eq_ignore_ascii_case("warning") {
            ContentRuleSeverity::Warning
        } else {
            ContentRuleSeverity::Failure
        };

        match build_fancy_regex(&pattern_object.pattern) {
            Ok(regex) => rules.push(CompiledContentRule {
                id: rule_id,
                severity,
                regex,
            }),
            Err(error) => {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Invalid regex in security baseline pattern '{rule_id}': {error}"),
                );
            }
        }
    }

    rules
}

fn compile_regex_list(
    pattern_list: &[String],
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<FancyRegex> {
    let mut regexes = Vec::new();
    for pattern in pattern_list {
        if pattern.trim().is_empty() {
            continue;
        }

        match build_fancy_regex(pattern) {
            Ok(regex) => regexes.push(regex),
            Err(error) => {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Invalid regex in {label}: {error}"),
                );
            }
        }
    }

    regexes
}

fn validate_forbidden_content(
    file_entries: &[RepositoryFileEntry],
    rules: &[CompiledContentRule],
    allowed_patterns: &[FancyRegex],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for entry in file_entries {
        let content = match fs::read_to_string(&entry.full_path) {
            Ok(content) => content,
            Err(_) => {
                warnings.push(format!(
                    "Skipping unreadable file during content scan: {}",
                    entry.relative_path
                ));
                continue;
            }
        };

        for rule in rules {
            let Some(mat) = rule.regex.find(&content).ok().flatten() else {
                continue;
            };

            let matched_value = mat.as_str();
            if allowed_patterns
                .iter()
                .any(|regex| regex.is_match(matched_value).ok().unwrap_or(false))
            {
                continue;
            }

            let line_number = content[..mat.start()].chars().filter(|character| *character == '\n').count() + 1;
            let message = format!(
                "{}:{} matched '{}' -> {}",
                entry.relative_path,
                line_number,
                rule.id,
                match_preview(matched_value)
            );
            match rule.severity {
                ContentRuleSeverity::Warning => warnings.push(message),
                ContentRuleSeverity::Failure => push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    message,
                ),
            }
        }
    }
}

fn match_preview(value: &str) -> String {
    let single_line = value.replace(['\r', '\n'], " ").trim().to_string();
    if single_line.chars().count() <= 80 {
        single_line
    } else {
        let preview = single_line.chars().take(80).collect::<String>();
        format!("{preview}...")
    }
}

fn matches_globset(relative_path: &str, globset: Option<&GlobSet>) -> bool {
    globset
        .map(|globset| globset.is_match(normalize_path(relative_path)))
        .unwrap_or(false)
}

fn build_fancy_regex(pattern: &str) -> Result<FancyRegex, fancy_regex::Error> {
    FancyRegex::new(&format!("(?m){pattern}"))
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}