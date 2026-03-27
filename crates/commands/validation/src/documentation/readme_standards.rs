//! README standards validation.

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use serde::Deserialize;

use crate::{error::ValidateReadmeStandardsCommandError, ValidationCheckStatus};

const DEFAULT_BASELINE_PATH: &str = ".github/governance/readme-standards.baseline.json";

/// Request payload for `validate-readme-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReadmeStandardsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    pub baseline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateReadmeStandardsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-readme-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReadmeStandardsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// README files validated successfully enough to inspect.
    pub files_checked: usize,
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
struct ReadmeStandardsBaseline {
    global: Option<ReadmeGlobalRules>,
    #[serde(default)]
    files: Vec<ReadmeFileRule>,
}

#[derive(Debug, Deserialize, Default)]
struct ReadmeGlobalRules {
    #[serde(default, rename = "requireFeaturesCheckmarks")]
    require_features_checkmarks: bool,
    #[serde(default, rename = "requireCodeFences")]
    require_code_fences: bool,
    #[serde(default, rename = "requireTocLinks")]
    require_toc_links: bool,
    #[serde(default, rename = "requireHorizontalSeparators")]
    require_horizontal_separators: bool,
}

#[derive(Debug, Deserialize)]
struct ReadmeFileRule {
    path: String,
    #[serde(default, rename = "requiredSections")]
    required_sections: Vec<String>,
    #[serde(default, rename = "allowIntroductionPreamble")]
    allow_introduction_preamble: bool,
}

/// Run the README standards validation.
///
/// # Errors
///
/// Returns [`ValidateReadmeStandardsCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_readme_standards(
    request: &ValidateReadmeStandardsRequest,
) -> Result<ValidateReadmeStandardsResult, ValidateReadmeStandardsCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateReadmeStandardsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateReadmeStandardsCommandError::ResolveWorkspaceRoot { source })?;
    let baseline_path = match request.baseline_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_BASELINE_PATH),
    };

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut files_checked = 0usize;

    if !baseline_path.is_file() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Baseline file not found: {}",
                to_repo_relative_path(&repo_root, &baseline_path)
            ),
        );

        return Ok(build_result(
            repo_root,
            request.warning_only,
            baseline_path,
            files_checked,
            warnings,
            failures,
        ));
    }

    let document = match fs::read_to_string(&baseline_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Could not read README standards baseline {}: {error}",
                    to_repo_relative_path(&repo_root, &baseline_path)
                ),
            );
            return Ok(build_result(
                repo_root,
                request.warning_only,
                baseline_path,
                files_checked,
                warnings,
                failures,
            ));
        }
    };

    let baseline = match serde_json::from_str::<ReadmeStandardsBaseline>(&document) {
        Ok(baseline) => baseline,
        Err(error) => {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Invalid JSON in baseline file {}: {error}",
                    to_repo_relative_path(&repo_root, &baseline_path)
                ),
            );
            return Ok(build_result(
                repo_root,
                request.warning_only,
                baseline_path,
                files_checked,
                warnings,
                failures,
            ));
        }
    };

    let global_rules = if let Some(global) = baseline.global {
        global
    } else {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Baseline missing 'global' section: {}",
                to_repo_relative_path(&repo_root, &baseline_path)
            ),
        );
        ReadmeGlobalRules::default()
    };

    if baseline.files.is_empty() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Baseline must define at least one file entry: {}",
                to_repo_relative_path(&repo_root, &baseline_path)
            ),
        );
    }

    for file_rule in baseline.files {
        if file_rule.path.trim().is_empty() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Baseline has file entry with empty path: {}",
                    to_repo_relative_path(&repo_root, &baseline_path)
                ),
            );
            continue;
        }

        let resolved_file_path = repo_root.join(&file_rule.path);
        if !resolved_file_path.is_file() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("README file not found: {}", file_rule.path),
            );
            continue;
        }

        let content = match fs::read_to_string(&resolved_file_path) {
            Ok(content) => content,
            Err(error) => {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!("Could not read README file {}: {error}", file_rule.path),
                );
                continue;
            }
        };

        files_checked += 1;
        let heading_keys = heading_key_set(&content);
        test_required_sections(
            &file_rule.path,
            &heading_keys,
            &file_rule.required_sections,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        test_introduction_preamble(
            &file_rule.path,
            &content,
            file_rule.allow_introduction_preamble,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        test_global_formatting_rules(
            &file_rule.path,
            &content,
            &heading_keys,
            &global_rules,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    Ok(build_result(
        repo_root,
        request.warning_only,
        baseline_path,
        files_checked,
        warnings,
        failures,
    ))
}

fn build_result(
    repo_root: PathBuf,
    warning_only: bool,
    baseline_path: PathBuf,
    files_checked: usize,
    warnings: Vec<String>,
    failures: Vec<String>,
) -> ValidateReadmeStandardsResult {
    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    ValidateReadmeStandardsResult {
        repo_root,
        warning_only,
        baseline_path,
        files_checked,
        warnings,
        failures,
        status,
        exit_code,
    }
}

fn test_required_sections(
    relative_path: &str,
    heading_keys: &BTreeSet<String>,
    required_sections: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for required_section in required_sections {
        let alternatives = required_section
            .split('|')
            .map(heading_to_key)
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        if alternatives.is_empty() {
            warnings.push(format!(
                "Invalid requiredSections entry in baseline for {relative_path}: '{required_section}'"
            ));
            continue;
        }

        if !alternatives
            .iter()
            .any(|candidate| heading_keys.contains(candidate))
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Missing required section '{required_section}' in {relative_path}"),
            );
        }
    }
}

fn test_introduction_preamble(
    relative_path: &str,
    content: &str,
    allow_introduction_preamble: bool,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if allow_introduction_preamble {
        return;
    }

    let first_content_line = content.lines().map(str::trim).find(|line| !line.is_empty());

    let Some(first_content_line) = first_content_line else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("README file is empty: {relative_path}"),
        );
        return;
    };

    if !first_content_line.starts_with("# ") {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("README must start with heading when preamble is disabled: {relative_path}"),
        );
    }
}

fn test_global_formatting_rules(
    relative_path: &str,
    content: &str,
    heading_keys: &BTreeSet<String>,
    global_rules: &ReadmeGlobalRules,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if global_rules.require_code_fences && content.matches("```").count() < 2 {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("README must include at least one fenced code block: {relative_path}"),
        );
    }

    if global_rules.require_horizontal_separators
        && content.lines().filter(|line| line.trim() == "---").count() < 1
    {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("README must include at least one horizontal separator (---): {relative_path}"),
        );
    }

    if global_rules.require_features_checkmarks
        && heading_keys.contains(&heading_to_key("Features"))
        && section_body(content, &["Features"])
            .is_none_or(|body| !body.lines().any(is_checkmark_bullet))
    {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Features section must include checkmark bullet items: {relative_path}"),
        );
    }

    if global_rules.require_toc_links {
        let toc_body = section_body(content, &["Table of Contents", "Contents"]);
        if toc_body.is_none() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("README must include Table of Contents/Contents section: {relative_path}"),
            );
        } else if !toc_body
            .as_deref()
            .unwrap_or_default()
            .lines()
            .any(is_markdown_anchor_link)
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Table of Contents must include markdown anchor links: {relative_path}"),
            );
        }
    }
}

fn heading_key_set(content: &str) -> BTreeSet<String> {
    content
        .lines()
        .filter_map(parse_heading_title)
        .map(heading_to_key)
        .filter(|value| !value.is_empty())
        .collect()
}

fn parse_heading_title(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let hash_count = trimmed
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&hash_count) {
        return None;
    }

    let rest = trimmed[hash_count..].trim_start();
    if rest.is_empty() || rest == trimmed {
        return None;
    }

    Some(rest.trim_end_matches('#').trim())
}

fn heading_to_key(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn section_body(content: &str, heading_options: &[&str]) -> Option<String> {
    let normalized_options = heading_options
        .iter()
        .map(|heading| heading_to_key(heading))
        .collect::<Vec<_>>();
    let lines = content.lines().collect::<Vec<_>>();

    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("## ") else {
            continue;
        };
        let heading = heading_to_key(rest.trim_end_matches('#').trim());
        if !normalized_options
            .iter()
            .any(|candidate| candidate == &heading)
        {
            continue;
        }

        let mut end_index = lines.len();
        for (candidate_index, candidate_line) in lines.iter().enumerate().skip(index + 1) {
            if candidate_line.trim().starts_with("## ") {
                end_index = candidate_index;
                break;
            }
        }
        return Some(lines[index + 1..end_index].join("\n"));
    }

    None
}

fn is_checkmark_bullet(line: &str) -> bool {
    let trimmed = line.trim_start();
    let Some(rest) = trimmed
        .strip_prefix('-')
        .or_else(|| trimmed.strip_prefix('*'))
    else {
        return false;
    };

    rest.trim_start().starts_with('✅')
}

fn is_markdown_anchor_link(line: &str) -> bool {
    let trimmed = line.trim_start();
    let Some(rest) = trimmed
        .strip_prefix('-')
        .or_else(|| trimmed.strip_prefix('*'))
    else {
        return false;
    };

    let remainder = rest.trim_start();
    remainder.starts_with('[') && remainder.contains("](#")
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