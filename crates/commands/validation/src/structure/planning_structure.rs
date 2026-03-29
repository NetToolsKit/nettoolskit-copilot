//! Planning workspace structure validation.

use std::env;
use std::path::PathBuf;

use nettoolskit_core::path_utils::repository::resolve_repository_root;

use crate::{error::ValidatePlanningStructureCommandError, ValidationCheckStatus};

/// Request payload for `validate-planning-structure`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatePlanningStructureRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Emit structural findings as warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidatePlanningStructureRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-planning-structure`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatePlanningStructureResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Missing required planning files.
    pub missing_required_files: Vec<String>,
    /// Missing required planning directories.
    pub missing_required_directories: Vec<String>,
    /// Optional planning directories that are absent and can be created later.
    pub missing_optional_directories: Vec<String>,
    /// Whether legacy `.temp/planning` drift was detected.
    pub legacy_temp_planning_detected: bool,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the planning workspace structure validation.
///
/// # Errors
///
/// Returns [`ValidatePlanningStructureCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_planning_structure(
    request: &ValidatePlanningStructureRequest,
) -> Result<ValidatePlanningStructureResult, ValidatePlanningStructureCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidatePlanningStructureCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidatePlanningStructureCommandError::ResolveWorkspaceRoot { source })?;

    let required_files = ["planning/README.md", "planning/specs/README.md"];
    let required_directories = ["planning", "planning/specs"];
    let optional_directories = [
        "planning/active",
        "planning/completed",
        "planning/specs/active",
        "planning/specs/completed",
    ];

    let missing_required_files = required_files
        .iter()
        .filter(|path| !repo_root.join(path).is_file())
        .map(|path| (*path).to_string())
        .collect::<Vec<_>>();
    let missing_required_directories = required_directories
        .iter()
        .filter(|path| !repo_root.join(path).is_dir())
        .map(|path| (*path).to_string())
        .collect::<Vec<_>>();
    let missing_optional_directories = optional_directories
        .iter()
        .filter(|path| !repo_root.join(path).is_dir())
        .map(|path| (*path).to_string())
        .collect::<Vec<_>>();
    let legacy_temp_planning_detected = repo_root.join(".temp/planning").exists();

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    for path in &missing_required_files {
        push_message(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!("Missing required planning file: {path}"),
        );
    }
    for path in &missing_required_directories {
        push_message(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!("Missing required planning directory: {path}"),
        );
    }
    if legacy_temp_planning_detected {
        push_message(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "Legacy planning workspace found under .temp/planning. Move versioned planning artifacts to planning/."
                .to_string(),
        );
    }

    let status = if !failures.is_empty() {
        ValidationCheckStatus::Failed
    } else if !warnings.is_empty() {
        ValidationCheckStatus::Warning
    } else {
        ValidationCheckStatus::Passed
    };
    let exit_code = if !failures.is_empty() { 1 } else { 0 };

    Ok(ValidatePlanningStructureResult {
        repo_root,
        warning_only: request.warning_only,
        missing_required_files,
        missing_required_directories,
        missing_optional_directories,
        legacy_temp_planning_detected,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn push_message(
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
