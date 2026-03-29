//! Git shell hook validation.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::Regex;

use crate::error::ValidateShellHooksCommandError;
use crate::operational_hygiene::common::{
    derive_status, push_required_finding, resolve_executable,
};
use crate::ValidationCheckStatus;

const DEFAULT_HOOK_ROOT: &str = ".githooks";
const REQUIRED_HOOKS: &[&str] = &["pre-commit", "post-commit", "post-merge", "post-checkout"];
const WINDOWS_SH_CANDIDATES: &[&str] = &[
    "C:\\Program Files\\Git\\usr\\bin\\sh.exe",
    "C:\\Program Files\\Git\\bin\\sh.exe",
];

/// Request payload for `validate-shell-hooks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateShellHooksRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit shell hook root.
    pub hook_root: Option<PathBuf>,
    /// Optional explicit shell runtime path.
    pub shell_path: Option<PathBuf>,
    /// Optional explicit shellcheck path.
    pub shellcheck_path: Option<PathBuf>,
    /// Run shellcheck when available.
    pub enable_shellcheck: bool,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateShellHooksRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            hook_root: None,
            shell_path: None,
            shellcheck_path: None,
            enable_shellcheck: false,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-shell-hooks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateShellHooksResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved shell hook root.
    pub hook_root: PathBuf,
    /// Resolved shell runtime path when available.
    pub shell_path: Option<PathBuf>,
    /// Resolved shellcheck path when available.
    pub shellcheck_path: Option<PathBuf>,
    /// Number of hook files checked.
    pub hook_files_checked: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the shell hook validation.
///
/// # Errors
///
/// Returns [`ValidateShellHooksCommandError`] when the repository root cannot be resolved.
pub fn invoke_validate_shell_hooks(
    request: &ValidateShellHooksRequest,
) -> Result<ValidateShellHooksResult, ValidateShellHooksCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateShellHooksCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateShellHooksCommandError::ResolveWorkspaceRoot { source })?;
    let hook_root = match request.hook_root.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_HOOK_ROOT),
    };
    let shell_path = resolve_executable(
        request.shell_path.as_deref(),
        &["sh"],
        WINDOWS_SH_CANDIDATES,
    );
    let shellcheck_path = if request.enable_shellcheck {
        resolve_executable(request.shellcheck_path.as_deref(), &["shellcheck"], &[])
    } else {
        None
    };

    let invalid_warning_only_pattern =
        Regex::new(r"(?im)-WarningOnly\s+(true|false)\b").expect("regex should compile");
    let invalid_shell_expansion_pattern =
        Regex::new(r"(?im)(^|[ \t])-WarningOnly:\\\$(true|false)\b").expect("regex should compile");

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut hook_files_checked = 0usize;

    if shell_path.is_none() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "Shell runtime not found (`sh`). Install Git Bash (Windows) or POSIX shell."
                .to_string(),
        );
    }

    for required_hook in REQUIRED_HOOKS {
        let hook_path = hook_root.join(required_hook);
        if !hook_path.is_file() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Hook file not found: {}",
                    hook_path
                        .strip_prefix(&repo_root)
                        .unwrap_or(&hook_path)
                        .display()
                        .to_string()
                        .replace('\\', "/")
                ),
            );
            continue;
        }

        hook_files_checked += 1;
        if let Some(shell_path) = shell_path.as_deref() {
            run_shell_syntax_check(
                shell_path,
                &hook_path,
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }

        test_hook_semantic_guards(
            &hook_path,
            &invalid_warning_only_pattern,
            &invalid_shell_expansion_pattern,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        if request.enable_shellcheck {
            run_shellcheck(shellcheck_path.as_deref(), &hook_path, &mut warnings);
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateShellHooksResult {
        repo_root,
        warning_only: request.warning_only,
        hook_root,
        shell_path,
        shellcheck_path,
        hook_files_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn run_shell_syntax_check(
    shell_path: &Path,
    hook_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let output = Command::new(shell_path).arg("-n").arg(hook_path).output();
    let Ok(output) = output else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Shell syntax check failed: {} :: could not launch shell runtime",
                hook_path.display()
            ),
        );
        return;
    };

    if output.status.code().unwrap_or(1) == 0 {
        return;
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
        format!(
            "Shell syntax check failed: {} :: {detail}",
            hook_path.display()
        ),
    );
}

fn test_hook_semantic_guards(
    hook_path: &Path,
    invalid_warning_only_pattern: &Regex,
    invalid_shell_expansion_pattern: &Regex,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Ok(content) = fs::read_to_string(hook_path) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Could not read hook file: {}", hook_path.display()),
        );
        return;
    };

    if invalid_warning_only_pattern.is_match(&content) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Hook uses unsupported boolean argument form for PowerShell bool parameters: {}. Use the single-quoted literal form '-WarningOnly:`$true' or '-WarningOnly:`$false' in shell hooks.",
                hook_path.display()
            ),
        );
    }

    if invalid_shell_expansion_pattern.is_match(&content) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Hook passes a PowerShell boolean literal without shell-safe quoting: {}. Use the single-quoted literal form '-WarningOnly:`$true' or '-WarningOnly:`$false' so POSIX shell does not expand `$true/`$false.",
                hook_path.display()
            ),
        );
    }
}

fn run_shellcheck(shellcheck_path: Option<&Path>, hook_path: &Path, warnings: &mut Vec<String>) {
    let Some(shellcheck_path) = shellcheck_path else {
        warnings.push("shellcheck not found; optional shellcheck pass skipped.".to_string());
        return;
    };

    let output = Command::new(shellcheck_path)
        .arg("-S")
        .arg("warning")
        .arg(hook_path)
        .output();
    let Ok(output) = output else {
        warnings.push(format!(
            "shellcheck: could not launch shellcheck for {}",
            hook_path.display()
        ));
        return;
    };

    if output.status.code().unwrap_or(1) == 0 {
        return;
    }

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    for line in stdout.lines().chain(stderr.lines()) {
        let text = line.trim();
        if text.is_empty() {
            continue;
        }
        warnings.push(format!("shellcheck: {text}"));
    }
}
