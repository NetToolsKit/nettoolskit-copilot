//! Shared template standards validation.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::Regex;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::{error::ValidateTemplateStandardsCommandError, ValidationCheckStatus};

const CANONICAL_BASELINE_PATH: &str =
    "definitions/providers/github/governance/template-standards.baseline.json";
const LEGACY_BASELINE_PATH: &str = ".github/governance/template-standards.baseline.json";
const CANONICAL_TEMPLATE_DIRECTORY: &str = "definitions/templates";
const LEGACY_TEMPLATE_DIRECTORY: &str = ".github/templates";

/// Request payload for `validate-template-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateTemplateStandardsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    pub baseline_path: Option<PathBuf>,
    /// Optional explicit template directory path.
    pub template_directory: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateTemplateStandardsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            template_directory: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-template-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateTemplateStandardsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// Resolved template directory.
    pub template_directory: PathBuf,
    /// Number of template files checked.
    pub templates_checked: usize,
    /// Number of template rules checked.
    pub rules_checked: usize,
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
struct TemplateStandardsBaseline {
    #[serde(default, rename = "requiredFiles")]
    required_files: Vec<String>,
    #[serde(default, rename = "templateRules")]
    template_rules: Vec<TemplateRule>,
}

#[derive(Debug, Deserialize)]
struct TemplateRule {
    path: String,
    #[serde(default, rename = "requiredPatterns")]
    required_patterns: Vec<String>,
    #[serde(default, rename = "forbiddenPatterns")]
    forbidden_patterns: Vec<String>,
    #[serde(default, rename = "requiredPathReferences")]
    required_path_references: Vec<String>,
}

/// Run the template standards validation.
///
/// # Errors
///
/// Returns [`ValidateTemplateStandardsCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_template_standards(
    request: &ValidateTemplateStandardsRequest,
) -> Result<ValidateTemplateStandardsResult, ValidateTemplateStandardsCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateTemplateStandardsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateTemplateStandardsCommandError::ResolveWorkspaceRoot { source })?;
    let baseline_path = match request.baseline_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => resolve_default_repo_path(
            &repo_root,
            &[CANONICAL_BASELINE_PATH, LEGACY_BASELINE_PATH],
            CANONICAL_BASELINE_PATH,
        ),
    };
    let template_directory = match request.template_directory.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => resolve_default_repo_path(
            &repo_root,
            &[CANONICAL_TEMPLATE_DIRECTORY, LEGACY_TEMPLATE_DIRECTORY],
            CANONICAL_TEMPLATE_DIRECTORY,
        ),
    };

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut templates_checked = 0usize;
    let mut rules_checked = 0usize;

    if !template_directory.is_dir() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Template directory not found: {}",
                to_repo_relative_path(&repo_root, &template_directory)
            ),
        );
    }

    let baseline = if baseline_path.is_file() {
        read_baseline_document(
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
                "Template baseline file not found: {}",
                to_repo_relative_path(&repo_root, &baseline_path)
            ),
        );
        None
    };

    if let Some(baseline) = baseline {
        for required_file in &baseline.required_files {
            let required_path = repo_root.join(required_file);
            if !required_path.is_file() {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!("Required template not found: {required_file}"),
                );
            }
        }

        if template_directory.is_dir() {
            let mut template_files = WalkDir::new(&template_directory)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| entry.into_path())
                .collect::<Vec<_>>();
            template_files.sort();

            templates_checked = template_files.len();
            for template_path in &template_files {
                let display_path = to_repo_relative_path(&repo_root, template_path);
                let content = match fs::read_to_string(template_path) {
                    Ok(content) => content,
                    Err(error) => {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!("Could not read template file {display_path}: {error}"),
                        );
                        continue;
                    }
                };

                if content.trim().is_empty() {
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        format!("Template file is empty: {display_path}"),
                    );
                }

                for (index, line) in content.lines().enumerate() {
                    if line.ends_with(' ') || line.ends_with('\t') {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!(
                                "Template contains trailing whitespace: {display_path}:{}",
                                index + 1
                            ),
                        );
                    }
                }
            }
        }

        rules_checked = baseline.template_rules.len();
        for rule in &baseline.template_rules {
            if rule.path.trim().is_empty() {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    "Template rule entry contains empty path.".to_string(),
                );
                continue;
            }

            let rule_path = repo_root.join(&rule.path);
            if !rule_path.is_file() {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!("Template rule path not found: {}", rule.path),
                );
                continue;
            }

            let display_path = to_repo_relative_path(&repo_root, &rule_path);
            let content = match fs::read_to_string(&rule_path) {
                Ok(content) => content,
                Err(error) => {
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        format!("Could not read template rule file {display_path}: {error}"),
                    );
                    continue;
                }
            };

            for required_pattern in &rule.required_patterns {
                match Regex::new(required_pattern) {
                    Ok(regex) => {
                        if !regex.is_match(&content) {
                            push_required_finding(
                                request.warning_only,
                                &mut warnings,
                                &mut failures,
                                format!(
                                    "Template missing required pattern '{required_pattern}': {display_path}"
                                ),
                            );
                        }
                    }
                    Err(error) => {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!(
                                "Invalid required pattern '{required_pattern}' in template baseline: {error}"
                            ),
                        );
                    }
                }
            }

            for forbidden_pattern in &rule.forbidden_patterns {
                match Regex::new(forbidden_pattern) {
                    Ok(regex) => {
                        if regex.is_match(&content) {
                            push_required_finding(
                                request.warning_only,
                                &mut warnings,
                                &mut failures,
                                format!(
                                    "Template contains forbidden pattern '{forbidden_pattern}': {display_path}"
                                ),
                            );
                        }
                    }
                    Err(error) => {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!(
                                "Invalid forbidden pattern '{forbidden_pattern}' in template baseline: {error}"
                            ),
                        );
                    }
                }
            }

            for path_reference in &rule.required_path_references {
                let reference_path = repo_root.join(path_reference);
                if !reference_path.exists() {
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        format!(
                            "Template references missing path '{path_reference}': {display_path}"
                        ),
                    );
                }
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateTemplateStandardsResult {
        repo_root,
        warning_only: request.warning_only,
        baseline_path,
        template_directory,
        templates_checked,
        rules_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn resolve_default_repo_path(repo_root: &Path, candidates: &[&str], default_path: &str) -> PathBuf {
    for candidate in candidates {
        let resolved = repo_root.join(candidate);
        if resolved.exists() {
            return resolved;
        }
    }

    repo_root.join(default_path)
}

fn read_baseline_document(
    path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<TemplateStandardsBaseline> {
    let document = fs::read_to_string(path).ok()?;
    match serde_json::from_str::<TemplateStandardsBaseline>(&document) {
        Ok(document) => Some(document),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Invalid JSON in template baseline {}: {error}",
                    path.display()
                ),
            );
            None
        }
    }
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn push_required_finding(
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
    message: String,
) {
    if warning_only {
        warnings.push(message);
    } else {
        failures.push(message);
    }
}

fn derive_status(warnings: &[String], failures: &[String]) -> ValidationCheckStatus {
    if !failures.is_empty() {
        ValidationCheckStatus::Failed
    } else if !warnings.is_empty() {
        ValidationCheckStatus::Warning
    } else {
        ValidationCheckStatus::Passed
    }
}