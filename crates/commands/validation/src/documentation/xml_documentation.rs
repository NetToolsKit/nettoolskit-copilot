//! XML documentation validation for C# source files.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{
    resolve_full_path, resolve_solution_or_layout_root,
};
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

use crate::{error::ValidateXmlDocumentationCommandError, ValidationCheckStatus};

const DEFAULT_SOURCE_FOLDER: &str = "src";
const DEFAULT_OUTPUT_PATH: &str = "docs/missing-documentation.json";

/// Request payload for `validate-xml-documentation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateXmlDocumentationRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional single project path or folder.
    pub project_path: Option<PathBuf>,
    /// Optional explicit project paths or folders.
    pub projects: Vec<PathBuf>,
    /// Optional source folder used during auto-discovery.
    pub source_folder: Option<PathBuf>,
    /// Include test projects during auto-discovery.
    pub include_tests: bool,
    /// Export missing entries to JSON.
    pub export_missing: bool,
    /// Optional explicit export path.
    pub output_path: Option<PathBuf>,
    /// Keep parity with the legacy script surface.
    pub group_by_project: bool,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateXmlDocumentationRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            project_path: None,
            projects: Vec::new(),
            source_folder: None,
            include_tests: false,
            export_missing: false,
            output_path: None,
            group_by_project: false,
            warning_only: true,
        }
    }
}

/// One undocumented type finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissingXmlDocumentationEntry {
    /// Logical project name.
    pub project: String,
    /// Source file name.
    pub file: String,
    /// Repository-relative path.
    pub path: String,
    /// Absolute source file path.
    pub full_path: String,
    /// Declared type kind.
    pub type_kind: String,
    /// Declared type name.
    pub type_name: String,
    /// Optional namespace.
    pub namespace: Option<String>,
}

/// Result payload for `validate-xml-documentation`.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidateXmlDocumentationResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved source folder used for discovery.
    pub source_folder: PathBuf,
    /// Effective export path when export is enabled.
    pub output_path: Option<PathBuf>,
    /// Number of projects inspected.
    pub projects_checked: usize,
    /// Number of source files considered.
    pub total_files: usize,
    /// Number of source files with XML documentation.
    pub documented_files: usize,
    /// Number of source files missing XML documentation.
    pub missing_files: usize,
    /// Number of source files skipped because no supported type declaration was found.
    pub skipped_files: usize,
    /// Coverage percentage over analyzable files.
    pub coverage_percent: f64,
    /// Detailed missing entries.
    pub missing_entries: Vec<MissingXmlDocumentationEntry>,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectTarget {
    name: String,
    directory: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectSelectionMode {
    ExplicitList,
    SinglePath,
    AutoDiscovery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileAnalysisOutcome {
    documented: bool,
    skipped: bool,
    missing_entry: Option<MissingXmlDocumentationEntry>,
}

/// Run the XML documentation validation command.
///
/// # Errors
///
/// Returns [`ValidateXmlDocumentationCommandError`] when the workspace root
/// cannot be resolved.
pub fn invoke_validate_xml_documentation(
    request: &ValidateXmlDocumentationRequest,
) -> Result<ValidateXmlDocumentationResult, ValidateXmlDocumentationCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateXmlDocumentationCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_solution_or_layout_root(request.repo_root.as_deref(), &current_dir)
        .map_err(|source| ValidateXmlDocumentationCommandError::ResolveWorkspaceRoot { source })?;
    let source_folder = request
        .source_folder
        .as_deref()
        .map(|path| resolve_full_path(&repo_root, path))
        .unwrap_or_else(|| repo_root.join(DEFAULT_SOURCE_FOLDER));
    let output_path = request
        .output_path
        .as_deref()
        .map(|path| resolve_full_path(&repo_root, path));

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let project_targets = resolve_project_targets(
        &repo_root,
        &source_folder,
        request,
        &mut warnings,
        &mut failures,
    );

    let mut total_files = 0usize;
    let mut documented_files = 0usize;
    let mut missing_entries = Vec::new();
    let mut skipped_files = 0usize;

    for project in &project_targets {
        for file_path in collect_source_files(&project.directory) {
            total_files += 1;
            let analysis = analyze_source_file(&repo_root, &project.name, &file_path);
            if analysis.documented {
                documented_files += 1;
            }
            if analysis.skipped {
                skipped_files += 1;
            }
            if let Some(entry) = analysis.missing_entry {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!(
                        "Missing XML <summary> for {} {} in {}",
                        entry.type_kind, entry.type_name, entry.path
                    ),
                );
                missing_entries.push(entry);
            }
        }
    }

    let missing_files = missing_entries.len();
    let analyzable_files = total_files.saturating_sub(skipped_files);
    let coverage_percent = if analyzable_files > 0 {
        ((documented_files as f64) / (analyzable_files as f64) * 100.0 * 100.0).round() / 100.0
    } else {
        0.0
    };

    if request.export_missing && !missing_entries.is_empty() {
        let resolved_output_path = output_path
            .clone()
            .unwrap_or_else(|| repo_root.join(DEFAULT_OUTPUT_PATH));
        if let Err(error) = write_missing_entries_report(
            &resolved_output_path,
            total_files,
            documented_files,
            missing_files,
            skipped_files,
            coverage_percent,
            &missing_entries,
        ) {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Could not export XML documentation findings to {}: {error}",
                    to_repo_relative_path(&repo_root, &resolved_output_path)
                ),
            );
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateXmlDocumentationResult {
        repo_root,
        warning_only: request.warning_only,
        source_folder,
        output_path,
        projects_checked: project_targets.len(),
        total_files,
        documented_files,
        missing_files,
        skipped_files,
        coverage_percent,
        missing_entries,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn resolve_project_targets(
    repo_root: &Path,
    source_folder: &Path,
    request: &ValidateXmlDocumentationRequest,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<ProjectTarget> {
    let (mode, explicit_inputs) = if !request.projects.is_empty() {
        (ProjectSelectionMode::ExplicitList, request.projects.clone())
    } else if let Some(project_path) = &request.project_path {
        (ProjectSelectionMode::SinglePath, vec![project_path.clone()])
    } else {
        (
            ProjectSelectionMode::AutoDiscovery,
            vec![source_folder.to_path_buf()],
        )
    };

    let mut targets = Vec::new();
    match mode {
        ProjectSelectionMode::ExplicitList => {
            for project_input in explicit_inputs {
                match resolve_project_input(repo_root, &project_input) {
                    Some(target) => targets.push(target),
                    None => push_required_finding(
                        request.warning_only,
                        warnings,
                        failures,
                        format!(
                            "Project path not found or does not contain a .csproj: {}",
                            project_input.display()
                        ),
                    ),
                }
            }
        }
        ProjectSelectionMode::SinglePath => {
            let project_input = &explicit_inputs[0];
            if let Some(target) = resolve_project_input(repo_root, project_input) {
                targets.push(target);
            } else {
                let directory = resolve_full_path(repo_root, project_input);
                let discovered = discover_projects_in_folder(&directory, request.include_tests);
                if discovered.is_empty() {
                    push_required_finding(
                        request.warning_only,
                        warnings,
                        failures,
                        format!(
                            "No .csproj files found under {}",
                            to_repo_relative_path(repo_root, &directory)
                        ),
                    );
                } else {
                    targets.extend(discovered);
                }
            }
        }
        ProjectSelectionMode::AutoDiscovery => {
            let discovered = discover_projects_in_folder(source_folder, request.include_tests);
            if discovered.is_empty() {
                push_required_finding(
                    request.warning_only,
                    warnings,
                    failures,
                    format!(
                        "No .csproj files found under {}",
                        to_repo_relative_path(repo_root, source_folder)
                    ),
                );
            } else {
                targets.extend(discovered);
            }
        }
    }

    targets.sort_by(|left, right| left.name.cmp(&right.name));
    targets.dedup_by(|left, right| left.directory == right.directory);
    targets
}

fn resolve_project_input(repo_root: &Path, input: &Path) -> Option<ProjectTarget> {
    let resolved = resolve_full_path(repo_root, input);
    if resolved.is_file()
        && resolved
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
    {
        return Some(ProjectTarget {
            name: resolved
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("project")
                .to_string(),
            directory: resolved.parent()?.to_path_buf(),
        });
    }

    if resolved.is_dir() {
        let direct_project = fs::read_dir(&resolved)
            .ok()?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|path| {
                path.is_file()
                    && path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
            });
        if let Some(project_path) = direct_project {
            return Some(ProjectTarget {
                name: project_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("project")
                    .to_string(),
                directory: resolved,
            });
        }
    }

    None
}

fn discover_projects_in_folder(root: &Path, include_tests: bool) -> Vec<ProjectTarget> {
    if !root.is_dir() {
        return Vec::new();
    }

    let mut projects = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_excluded_build_path(entry.path()))
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
        })
        .filter_map(|entry| {
            let project_path = entry.into_path();
            let project_name = project_path.file_stem()?.to_str()?.to_string();
            if !include_tests && looks_like_test_project(&project_name) {
                return None;
            }

            Some(ProjectTarget {
                name: project_name,
                directory: project_path.parent()?.to_path_buf(),
            })
        })
        .collect::<Vec<_>>();

    projects.sort_by(|left, right| left.name.cmp(&right.name));
    projects
}

fn collect_source_files(project_directory: &Path) -> Vec<PathBuf> {
    let mut files = WalkDir::new(project_directory)
        .into_iter()
        .filter_entry(|entry| !is_excluded_build_path(entry.path()))
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cs"))
        })
        .filter_map(|entry| {
            let file_name = entry.file_name().to_str()?;
            (!matches!(file_name, "GlobalSuppressions.cs" | "AssemblyInfo.cs"))
                .then_some(entry.into_path())
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn analyze_source_file(
    repo_root: &Path,
    project_name: &str,
    file_path: &Path,
) -> FileAnalysisOutcome {
    let Ok(content) = fs::read_to_string(file_path) else {
        return FileAnalysisOutcome {
            documented: false,
            skipped: true,
            missing_entry: None,
        };
    };

    let type_pattern = Regex::new(
        r"(?m)(?:public|internal|private|protected)\s+(?:sealed\s+)?(?:static\s+)?(?:abstract\s+)?(class|interface|enum|record|struct)\s+([A-Za-z0-9_<>,\s]+?)(?:\s*:\s*|\s*where\s*|\s*\{|\s*\()",
    )
    .expect("xml documentation type regex should compile");
    let Some(type_capture) = type_pattern.captures(&content) else {
        return FileAnalysisOutcome {
            documented: false,
            skipped: true,
            missing_entry: None,
        };
    };

    let type_kind = type_capture
        .get(1)
        .map(|capture| capture.as_str().to_string())
        .unwrap_or_else(|| "type".to_string());
    let type_name = type_capture
        .get(2)
        .map(|capture| sanitize_type_name(capture.as_str()))
        .unwrap_or_else(|| "UnknownType".to_string());
    let has_summary = Regex::new(r"(?m)^\s*///\s*<summary>")
        .expect("summary regex should compile")
        .is_match(&content);

    if has_summary {
        return FileAnalysisOutcome {
            documented: true,
            skipped: false,
            missing_entry: None,
        };
    }

    let namespace = Regex::new(r"(?m)^\s*namespace\s+([^\r\n\{;]+)")
        .expect("namespace regex should compile")
        .captures(&content)
        .and_then(|captures| captures.get(1))
        .map(|capture| capture.as_str().trim().to_string());
    let relative_path = to_repo_relative_path(repo_root, file_path);
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown.cs")
        .to_string();

    FileAnalysisOutcome {
        documented: false,
        skipped: false,
        missing_entry: Some(MissingXmlDocumentationEntry {
            project: project_name.to_string(),
            file: file_name,
            path: relative_path,
            full_path: file_path.display().to_string(),
            type_kind,
            type_name,
            namespace,
        }),
    }
}

fn sanitize_type_name(raw_type_name: &str) -> String {
    raw_type_name
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .split('<')
        .next()
        .unwrap_or(raw_type_name)
        .trim()
        .to_string()
}

fn looks_like_test_project(project_name: &str) -> bool {
    project_name.to_ascii_lowercase().contains("test")
}

fn is_excluded_build_path(path: &Path) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|segment| matches!(segment, "bin" | "obj"))
    })
}

fn write_missing_entries_report(
    output_path: &Path,
    total_files: usize,
    documented_files: usize,
    missing_files: usize,
    skipped_files: usize,
    coverage_percent: f64,
    missing_entries: &[MissingXmlDocumentationEntry],
) -> anyhow::Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let payload = serde_json::to_string_pretty(&serde_json::json!({
        "generatedAt": current_timestamp_string(),
        "summary": {
            "totalFiles": total_files,
            "documented": documented_files,
            "missing": missing_files,
            "skipped": skipped_files,
            "coverage": coverage_percent
        },
        "missingDocumentation": missing_entries
    }))?;
    fs::write(output_path, payload)?;
    Ok(())
}

fn current_timestamp_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
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