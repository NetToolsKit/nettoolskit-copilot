//! .NET template standards validation.

use std::fs;
use std::path::PathBuf;

use regex::Regex;

use crate::agent_orchestration::common::{
    resolve_repo_relative_path, resolve_validation_repo_root,
};
use crate::error::ValidateDotnetStandardsCommandError;
use crate::operational_hygiene::common::derive_status;
use crate::ValidationCheckStatus;

const DEFAULT_TEMPLATE_DIRECTORY: &str = ".github/templates";
const REQUIRED_TEMPLATE_RULES: &[(&str, &[&str])] = &[
    (
        ".github/templates/dotnet-class-template.cs",
        &[
            r"public\s+class\s+\[ClassName\]",
            r"namespace\s+\[Namespace\]",
        ],
    ),
    (
        ".github/templates/dotnet-interface-template.cs",
        &[
            r"public\s+interface\s+\[InterfaceName\]",
            r"namespace\s+\[Namespace\]",
        ],
    ),
    (
        ".github/templates/dotnet-unit-test-template.cs",
        &[r"\[TEST_CLASS\]", r"(\[Fact\]|\[Test\]|\[Theory\])"],
    ),
    (
        ".github/templates/dotnet-integration-test-template.cs",
        &[r"IMediator", r"\[Test\]"],
    ),
];

/// Request payload for `validate-dotnet-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateDotnetStandardsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit template directory.
    pub template_directory: Option<PathBuf>,
}

impl Default for ValidateDotnetStandardsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            template_directory: None,
        }
    }
}

/// Result payload for `validate-dotnet-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateDotnetStandardsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved template directory.
    pub template_directory: PathBuf,
    /// Number of `.cs` templates checked.
    pub templates_checked: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the .NET template standards validation.
///
/// # Errors
///
/// Returns [`ValidateDotnetStandardsCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_dotnet_standards(
    request: &ValidateDotnetStandardsRequest,
) -> Result<ValidateDotnetStandardsResult, ValidateDotnetStandardsCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref())
        .map_err(|source| ValidateDotnetStandardsCommandError::ResolveWorkspaceRoot { source })?;
    let template_directory = resolve_repo_relative_path(
        &repo_root,
        request.template_directory.as_deref(),
        DEFAULT_TEMPLATE_DIRECTORY,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut templates_checked = 0usize;

    if !template_directory.is_dir() {
        failures.push(format!(
            "Template directory not found: {}",
            request.template_directory.as_ref().map_or_else(
                || DEFAULT_TEMPLATE_DIRECTORY.to_string(),
                |path| { path.to_string_lossy().to_string() }
            )
        ));
    } else {
        validate_required_templates(&repo_root, &mut warnings, &mut failures);

        let mut template_files = fs::read_dir(&template_directory)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().is_some_and(|extension| extension == "cs"))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        template_files.sort();
        templates_checked = template_files.len();

        for template_path in template_files {
            let relative_path = template_path
                .strip_prefix(&repo_root)
                .map_or_else(
                    |_| template_path.display().to_string(),
                    |path| path.display().to_string(),
                )
                .replace('\\', "/");
            let document = match fs::read_to_string(&template_path) {
                Ok(document) => document,
                Err(error) => {
                    failures.push(format!(
                        "Could not read .NET template {}: {error}",
                        relative_path
                    ));
                    continue;
                }
            };

            validate_template_conventions(&relative_path, &document, &mut warnings, &mut failures);
            validate_template_whitespace(&relative_path, &document, &mut failures);
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateDotnetStandardsResult {
        repo_root,
        template_directory,
        templates_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn validate_required_templates(
    repo_root: &std::path::Path,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for (relative_path, required_patterns) in REQUIRED_TEMPLATE_RULES {
        let template_path = repo_root.join(relative_path);
        if !template_path.is_file() {
            failures.push(format!("Required .NET template not found: {relative_path}"));
            continue;
        }

        let document = match fs::read_to_string(&template_path) {
            Ok(document) => document,
            Err(error) => {
                failures.push(format!(
                    "Could not read .NET template {}: {error}",
                    relative_path
                ));
                continue;
            }
        };

        for required_pattern in *required_patterns {
            let regex =
                Regex::new(required_pattern).expect("required template regex should compile");
            if !regex.is_match(&document) {
                failures.push(format!(
                    "Template missing required pattern '{required_pattern}': {relative_path}"
                ));
            }
        }

        validate_template_conventions(relative_path, &document, warnings, failures);
        validate_template_whitespace(relative_path, &document, failures);
    }
}

fn validate_template_conventions(
    relative_path: &str,
    document: &str,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if !document.contains("[Namespace]") {
        failures.push(format!(
            "Template missing [Namespace] placeholder: {relative_path}"
        ));
    }

    if !document.contains("<summary>") {
        warnings.push(format!(
            "Template missing XML <summary> section: {relative_path}"
        ));
    }
}

fn validate_template_whitespace(relative_path: &str, document: &str, failures: &mut Vec<String>) {
    for (index, raw_line) in document.split('\n').enumerate() {
        let line = raw_line.trim_end_matches('\r');
        let line_number = index + 1;

        if line.contains('\t') {
            failures.push(format!(
                "Template contains tab character: {relative_path}:{line_number}"
            ));
        }

        if line.trim_end_matches([' ', '\t']).len() != line.len() {
            failures.push(format!(
                "Template contains trailing whitespace: {relative_path}:{line_number}"
            ));
        }
    }
}