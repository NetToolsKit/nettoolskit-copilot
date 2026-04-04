//! C# test naming validation.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::resolve_repository_root;
use regex::Regex;
use walkdir::WalkDir;

use crate::error::ValidateTestNamingCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::ValidationCheckStatus;

const DEFAULT_REQUIRED_UNDERSCORES: usize = 3;
const TEST_PROJECT_SUFFIX: &str = "Tests.csproj";

/// Request payload for `validate-test-naming`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateTestNamingRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional project name, path, or substring filters.
    pub projects: Option<Vec<String>>,
    /// Minimum number of underscores required in each test method name.
    pub required_underscores: usize,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateTestNamingRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            projects: None,
            required_underscores: DEFAULT_REQUIRED_UNDERSCORES,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-test-naming`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateTestNamingResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Required underscore count enforced by the validation.
    pub required_underscores: usize,
    /// Number of test projects discovered and selected.
    pub test_projects_checked: usize,
    /// Number of source files inspected for test methods.
    pub test_files_checked: usize,
    /// Number of test methods inspected.
    pub test_methods_checked: usize,
    /// Number of underscore violations found.
    pub violations_found: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Clone)]
struct TestProject {
    name: String,
    path: PathBuf,
    directory: PathBuf,
}

/// Run the test naming validation.
///
/// # Errors
///
/// Returns [`ValidateTestNamingCommandError`] when the repository root cannot be resolved
/// or when the requested underscore threshold is invalid.
pub fn invoke_validate_test_naming(
    request: &ValidateTestNamingRequest,
) -> Result<ValidateTestNamingResult, ValidateTestNamingCommandError> {
    if request.required_underscores < 1 {
        return Err(ValidateTestNamingCommandError::InvalidRequiredUnderscores {
            required_underscores: request.required_underscores,
        });
    }

    let current_dir = env::current_dir().map_err(|source| {
        ValidateTestNamingCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateTestNamingCommandError::ResolveWorkspaceRoot { source })?;

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let discovered_projects = discover_test_projects(&repo_root);
    let selected_projects =
        select_test_projects(&discovered_projects, request, &mut warnings, &mut failures);

    let mut test_files_checked = 0usize;
    let mut test_methods_checked = 0usize;
    let mut violations_found = 0usize;

    for project in &selected_projects {
        for test_file in discover_test_files(&project.directory) {
            test_files_checked += 1;
            let Ok(method_names) = extract_test_methods(&test_file) else {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!(
                        "Could not read test file: {}",
                        relative_display_path(&repo_root, &test_file)
                    ),
                );
                continue;
            };

            for method_name in method_names {
                test_methods_checked += 1;
                let underscore_count = method_name.matches('_').count();
                if underscore_count < request.required_underscores {
                    violations_found += 1;
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        format!(
                            "Test method name violates underscore convention: {} :: {} (underscores: {underscore_count}, required: {})",
                            relative_display_path(&repo_root, &test_file),
                            method_name,
                            request.required_underscores
                        ),
                    );
                }
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateTestNamingResult {
        repo_root,
        warning_only: request.warning_only,
        required_underscores: request.required_underscores,
        test_projects_checked: selected_projects.len(),
        test_files_checked,
        test_methods_checked,
        violations_found,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn select_test_projects(
    discovered_projects: &[TestProject],
    request: &ValidateTestNamingRequest,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<TestProject> {
    let requested = request
        .projects
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|project| project.trim())
        .filter(|project| !project.is_empty())
        .map(|project| project.to_string())
        .collect::<Vec<_>>();

    if requested.is_empty() {
        if discovered_projects.is_empty() {
            push_required_finding(
                request.warning_only,
                warnings,
                failures,
                "No test projects (*.Tests.csproj) were found.".to_string(),
            );
        }
        return discovered_projects.to_vec();
    }

    let selected = discovered_projects
        .iter()
        .filter(|project| {
            requested.iter().any(|needle| {
                contains_case_insensitive(&project.name, needle)
                    || contains_case_insensitive(&project.path.to_string_lossy(), needle)
                    || contains_case_insensitive(&project.directory.to_string_lossy(), needle)
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    for needle in requested {
        if !selected.iter().any(|project| {
            contains_case_insensitive(&project.name, &needle)
                || contains_case_insensitive(&project.path.to_string_lossy(), &needle)
                || contains_case_insensitive(&project.directory.to_string_lossy(), &needle)
        }) {
            push_required_finding(
                request.warning_only,
                warnings,
                failures,
                format!(
                    "Requested project '{needle}' was not found among discovered test projects."
                ),
            );
        }
    }

    if selected.is_empty() {
        push_required_finding(
            request.warning_only,
            warnings,
            failures,
            "No test projects selected for validation.".to_string(),
        );
    }

    selected
}

fn discover_test_projects(repo_root: &Path) -> Vec<TestProject> {
    let mut projects = WalkDir::new(repo_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("csproj"))
        .filter(|path| !path_contains_ignored_directory(path))
        .filter(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .is_some_and(has_test_project_suffix)
        })
        .map(|path| TestProject {
            name: path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("unknown")
                .to_string(),
            directory: path.parent().unwrap_or(repo_root).to_path_buf(),
            path,
        })
        .collect::<Vec<_>>();

    projects.sort_by(|left, right| left.path.cmp(&right.path));
    projects
}

fn discover_test_files(project_directory: &Path) -> Vec<PathBuf> {
    let mut files = WalkDir::new(project_directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("cs"))
        .filter(|path| !path_contains_ignored_directory(path))
        .filter(|path| is_test_source_file(path))
        .collect::<Vec<_>>();

    files.sort();
    files
}

fn extract_test_methods(file_path: &Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|error| format!("could not read {}: {error}", file_path.display()))?;
    let attribute_pattern = Regex::new(
        r"(?i)\[(Fact|Theory|Test|TestMethod|DataTestMethod|TestCase|TestCaseSource|SkippableFact|SkippableTheory|Property|Combinatorial|Sequential)\b",
    )
    .expect("test attribute regex should compile");
    let method_pattern = Regex::new(
        r"(?ms)((?:\s*\[[^\]]+\]\s*)+)\s*public\s+(?:async\s+)?(?:Task(?:<[^>]+>)?|ValueTask(?:<[^>]+>)?|void)\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\s*\(",
    )
    .expect("test method regex should compile");

    Ok(method_pattern
        .captures_iter(&content)
        .filter(|capture| attribute_pattern.is_match(&capture[1]))
        .map(|capture| capture["name"].to_string())
        .collect::<Vec<_>>())
}

fn is_test_source_file(path: &Path) -> bool {
    let normalized = path
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    normalized.contains("/tests/") || file_name.ends_with("tests.cs")
}

fn path_contains_ignored_directory(path: &Path) -> bool {
    path.components().any(|component| {
        component.as_os_str().to_str().is_some_and(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "bin" | "obj" | "artifacts"
            )
        })
    })
}

fn contains_case_insensitive(text: &str, needle: &str) -> bool {
    text.to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn has_test_project_suffix(name: &str) -> bool {
    name.to_ascii_lowercase()
        .ends_with(&TEST_PROJECT_SUFFIX.to_ascii_lowercase())
}

fn relative_display_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}