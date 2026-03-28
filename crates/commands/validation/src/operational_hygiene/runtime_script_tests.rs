//! Runtime PowerShell test suite validation.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};

use crate::error::ValidateRuntimeScriptTestsCommandError;
use crate::operational_hygiene::common::{
    derive_status, push_required_finding, resolve_executable,
};
use crate::ValidationCheckStatus;

const DEFAULT_TEST_ROOT: &str = "scripts/tests/runtime";

/// Request payload for `validate-runtime-script-tests`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateRuntimeScriptTestsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit runtime test root.
    pub test_root: Option<PathBuf>,
    /// Optional explicit PowerShell runtime path.
    pub powershell_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateRuntimeScriptTestsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            test_root: None,
            powershell_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-runtime-script-tests`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateRuntimeScriptTestsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved runtime test root.
    pub test_root: PathBuf,
    /// Resolved PowerShell runtime path when available.
    pub powershell_path: Option<PathBuf>,
    /// Number of test scripts discovered.
    pub test_scripts_checked: usize,
    /// Number of passing runtime tests.
    pub passed_tests: usize,
    /// Number of failing runtime tests.
    pub failed_tests: usize,
    /// Number of skipped runtime tests.
    pub skipped_tests: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the runtime PowerShell test suite validation.
///
/// # Errors
///
/// Returns [`ValidateRuntimeScriptTestsCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_runtime_script_tests(
    request: &ValidateRuntimeScriptTestsRequest,
) -> Result<ValidateRuntimeScriptTestsResult, ValidateRuntimeScriptTestsCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateRuntimeScriptTestsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| ValidateRuntimeScriptTestsCommandError::ResolveWorkspaceRoot { source },
        )?;
    let test_root = match request.test_root.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_TEST_ROOT),
    };
    let powershell_path = resolve_executable(
        request.powershell_path.as_deref(),
        &["pwsh", "powershell"],
        &[],
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut test_scripts_checked = 0usize;
    let mut passed_tests = 0usize;
    let mut failed_tests = 0usize;
    let skipped_tests = 0usize;

    if !test_root.is_dir() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!("Runtime test path not found: {}", test_root.display()),
        );
    } else if powershell_path.is_none() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "PowerShell runtime not found; runtime script tests cannot execute.".to_string(),
        );
    } else {
        let mut test_scripts = discover_test_scripts(&test_root);
        if test_scripts.is_empty() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("No runtime test scripts found in: {}", test_root.display()),
            );
        } else {
            let powershell_path = powershell_path.clone().expect("checked above");
            test_scripts.sort();
            for test_script in test_scripts {
                test_scripts_checked += 1;
                if execute_runtime_test(
                    &powershell_path,
                    &test_script,
                    &repo_root,
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                ) {
                    passed_tests += 1;
                } else {
                    failed_tests += 1;
                }
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateRuntimeScriptTestsResult {
        repo_root,
        warning_only: request.warning_only,
        test_root,
        powershell_path,
        test_scripts_checked,
        passed_tests,
        failed_tests,
        skipped_tests,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn discover_test_scripts(test_root: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(test_root) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("ps1"))
        .collect()
}

fn execute_runtime_test(
    powershell_path: &Path,
    test_script_path: &Path,
    repo_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> bool {
    let output = Command::new(powershell_path)
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(test_script_path)
        .arg("-RepoRoot")
        .arg(repo_root)
        .output();

    let script_name = test_script_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("unknown-test");
    let Ok(output) = output else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Runtime test failed: {script_name} (could not launch PowerShell runtime)"),
        );
        return false;
    };

    let exit_code = output.status.code().unwrap_or(1);
    if exit_code == 0 {
        return true;
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let detail = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        "no diagnostic output".to_string()
    };
    push_required_finding(
        warning_only,
        warnings,
        failures,
        format!("Runtime test failed: {script_name} (exit code {exit_code}) :: {detail}"),
    );
    false
}