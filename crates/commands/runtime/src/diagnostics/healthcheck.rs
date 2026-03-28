//! Runtime healthcheck orchestration for validation and drift audit flows.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::resolve_full_path;
use nettoolskit_core::runtime_execution::resolve_runtime_execution_context;
use nettoolskit_validation::{
    invoke_validate_all, ValidateAllRequest, ValidationCheckStatus as ValidateAllStatus,
};
use serde_json::json;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::{
    error::RuntimeHealthcheckCommandError, invoke_runtime_bootstrap, invoke_runtime_doctor,
    RuntimeBootstrapRequest, RuntimeDoctorRequest, RuntimeDoctorStatus,
};

/// Request payload for `healthcheck`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHealthcheckRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit GitHub runtime target path.
    pub target_github_path: Option<PathBuf>,
    /// Optional explicit Codex runtime target path.
    pub target_codex_path: Option<PathBuf>,
    /// Optional explicit picker-visible agent skills path.
    pub target_agents_skills_path: Option<PathBuf>,
    /// Optional explicit Copilot native skills path.
    pub target_copilot_skills_path: Option<PathBuf>,
    /// Optional explicit runtime profile name.
    pub runtime_profile: Option<String>,
    /// Optional explicit fallback runtime profile name.
    ///
    /// When omitted, `healthcheck` follows the PowerShell contract and falls
    /// back to `all`.
    pub fallback_runtime_profile: Option<String>,
    /// Run bootstrap sync before the remaining checks.
    pub sync_runtime: bool,
    /// Pass mirror mode when bootstrap sync is enabled.
    pub mirror: bool,
    /// Fail runtime doctor on extra files.
    pub strict_extras: bool,
    /// Validation profile passed to `validate-all`.
    pub validation_profile: String,
    /// Global warning-only mode.
    pub warning_only: bool,
    /// Convert runtime drift failures into warnings.
    pub treat_runtime_drift_as_warning: bool,
    /// Optional explicit report output path.
    pub output_path: Option<PathBuf>,
    /// Optional explicit plain-text log path.
    pub log_path: Option<PathBuf>,
}

impl Default for RuntimeHealthcheckRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            target_github_path: None,
            target_codex_path: None,
            target_agents_skills_path: None,
            target_copilot_skills_path: None,
            runtime_profile: None,
            fallback_runtime_profile: None,
            sync_runtime: false,
            mirror: false,
            strict_extras: false,
            validation_profile: "dev".to_string(),
            warning_only: true,
            treat_runtime_drift_as_warning: true,
            output_path: None,
            log_path: None,
        }
    }
}

/// Healthcheck status used by individual checks and the final summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHealthcheckStatus {
    /// The check or overall run passed.
    Passed,
    /// The check or overall run completed with warnings.
    Warning,
    /// The check or overall run failed.
    Failed,
}

impl RuntimeHealthcheckStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::Failed => "failed",
        }
    }
}

/// One recorded healthcheck step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHealthcheckCheckResult {
    /// Logical check name.
    pub name: String,
    /// Script path or Rust surface identifier.
    pub script: String,
    /// Formatted argument list.
    pub arguments: Vec<String>,
    /// Final check status.
    pub status: RuntimeHealthcheckStatus,
    /// Exit code equivalent used by the check.
    pub exit_code: i32,
    /// Elapsed execution time in milliseconds.
    pub duration_ms: u128,
    /// Start timestamp token.
    pub started_at: String,
    /// End timestamp token.
    pub finished_at: String,
    /// Optional error message recorded by the step.
    pub error: Option<String>,
}

/// Result payload for `healthcheck`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHealthcheckResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective runtime profile name.
    pub runtime_profile_name: String,
    /// Validation profile used by the run.
    pub validation_profile: String,
    /// Resolved JSON report path.
    pub output_path: PathBuf,
    /// Resolved plain-text log path.
    pub log_path: PathBuf,
    /// Ordered checks executed by the run.
    pub checks: Vec<RuntimeHealthcheckCheckResult>,
    /// Number of checks executed.
    pub total_checks: usize,
    /// Number of passed checks.
    pub passed_checks: usize,
    /// Number of warning checks.
    pub warning_checks: usize,
    /// Number of failed checks.
    pub failed_checks: usize,
    /// Overall healthcheck status.
    pub overall_status: RuntimeHealthcheckStatus,
    /// Process exit code equivalent for wrapper/CLI use.
    pub exit_code: i32,
    /// Persisted JSON report payload.
    pub report_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandArgument {
    name: &'static str,
    value: Option<String>,
}

/// Run the runtime healthcheck flow.
///
/// # Errors
///
/// Returns [`RuntimeHealthcheckCommandError`] when workspace resolution, output
/// path preparation, log creation, or report persistence fails.
pub fn invoke_runtime_healthcheck(
    request: &RuntimeHealthcheckRequest,
) -> Result<RuntimeHealthcheckResult, RuntimeHealthcheckCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeHealthcheckCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let fallback_profile = request.fallback_runtime_profile.as_deref().or(Some("all"));
    let context = resolve_runtime_execution_context(
        request.repo_root.as_deref(),
        request.runtime_profile.as_deref(),
        fallback_profile,
        request.target_github_path.as_deref(),
        request.target_codex_path.as_deref(),
        request.target_agents_skills_path.as_deref(),
        request.target_copilot_skills_path.as_deref(),
        None,
        &current_dir,
    )
    .map_err(|source| RuntimeHealthcheckCommandError::ResolveExecutionContext { source })?;

    let output_path = resolve_healthcheck_output_path(
        &context.resolved_repo_root,
        request.output_path.as_deref(),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::PrepareArtifacts { source })?;
    let log_path =
        resolve_healthcheck_log_path(&context.resolved_repo_root, request.log_path.as_deref())
            .map_err(|source| RuntimeHealthcheckCommandError::PrepareArtifacts { source })?;
    initialize_output_file_parent(&output_path)
        .map_err(|source| RuntimeHealthcheckCommandError::PrepareArtifacts { source })?;
    initialize_output_file_parent(&log_path)
        .map_err(|source| RuntimeHealthcheckCommandError::PrepareArtifacts { source })?;
    initialize_log_file(&log_path)
        .map_err(|source| RuntimeHealthcheckCommandError::PrepareArtifacts { source })?;

    append_log_line(
        &log_path,
        &format!("repo root: {}", context.resolved_repo_root.display()),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("validation profile: {}", request.validation_profile),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("runtime profile: {}", context.runtime_profile.name),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("warning-only mode: {}", request.warning_only),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("output report: {}", output_path.display()),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;
    append_log_line(&log_path, &format!("log file: {}", log_path.display()))
        .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;

    let mut checks = Vec::new();

    if request.sync_runtime {
        checks.push(run_bootstrap_check(
            request,
            &context.resolved_repo_root,
            &log_path,
        ));
    }

    let validate_arguments = vec![
        CommandArgument {
            name: "RepoRoot",
            value: Some(context.resolved_repo_root.display().to_string()),
        },
        CommandArgument {
            name: "ValidationProfile",
            value: Some(request.validation_profile.clone()),
        },
        CommandArgument {
            name: "WarningOnly",
            value: Some(request.warning_only.to_string()),
        },
    ];
    checks.push(run_validation_check(
        request,
        &context.resolved_repo_root,
        &validate_arguments,
        &log_path,
    ));

    checks.push(run_doctor_check(
        request,
        &context.resolved_repo_root,
        &log_path,
    ));

    let passed_checks = checks
        .iter()
        .filter(|check| check.status == RuntimeHealthcheckStatus::Passed)
        .count();
    let warning_checks = checks
        .iter()
        .filter(|check| check.status == RuntimeHealthcheckStatus::Warning)
        .count();
    let failed_checks = checks
        .iter()
        .filter(|check| check.status == RuntimeHealthcheckStatus::Failed)
        .count();
    let overall_status = if failed_checks > 0 {
        RuntimeHealthcheckStatus::Failed
    } else if warning_checks > 0 {
        RuntimeHealthcheckStatus::Warning
    } else {
        RuntimeHealthcheckStatus::Passed
    };
    let exit_code = if failed_checks > 0 && !request.warning_only {
        1
    } else {
        0
    };

    append_log_line(
        &log_path,
        &format!(
            "healthcheck summary: total={} passed={} warning={} failed={}",
            checks.len(),
            passed_checks,
            warning_checks,
            failed_checks
        ),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;

    let report_json = serde_json::to_string_pretty(&json!({
        "schemaVersion": 2,
        "generatedAt": current_timestamp_string().map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?,
        "repoRoot": context.resolved_repo_root.display().to_string(),
        "targets": {
            "github": context.targets.github_runtime_root.display().to_string(),
            "codex": context.targets.codex_runtime_root.display().to_string(),
            "agentsSkills": context.targets.agents_skills_root.display().to_string(),
            "copilotSkills": context.targets.copilot_skills_root.display().to_string(),
        },
        "options": {
            "syncRuntime": request.sync_runtime,
            "mirror": request.mirror,
            "strictExtras": request.strict_extras,
            "runtimeProfile": context.runtime_profile.name,
            "validationProfile": request.validation_profile,
            "warningOnly": request.warning_only,
            "treatRuntimeDriftAsWarning": request.treat_runtime_drift_as_warning,
        },
        "summary": {
            "totalChecks": checks.len(),
            "passedChecks": passed_checks,
            "warningChecks": warning_checks,
            "failedChecks": failed_checks,
            "overallStatus": overall_status.as_str(),
        },
        "issues": serde_json::Value::Null,
        "checks": checks.iter().map(check_to_json).collect::<Vec<_>>(),
        "logPath": log_path.display().to_string(),
    }))
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput {
        source: source.into(),
    })?;

    fs::write(&output_path, &report_json)
        .with_context(|| format!("failed to write '{}'", output_path.display()))
        .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("healthcheck report generated: {}", output_path.display()),
    )
    .map_err(|source| RuntimeHealthcheckCommandError::WriteOutput { source })?;

    Ok(RuntimeHealthcheckResult {
        repo_root: context.resolved_repo_root,
        runtime_profile_name: context.runtime_profile.name,
        validation_profile: request.validation_profile.clone(),
        output_path,
        log_path,
        total_checks: checks.len(),
        passed_checks,
        warning_checks,
        failed_checks,
        overall_status,
        exit_code,
        report_json,
        checks,
    })
}

fn run_doctor_check(
    request: &RuntimeHealthcheckRequest,
    repo_root: &Path,
    log_path: &Path,
) -> RuntimeHealthcheckCheckResult {
    let arguments = doctor_argument_list(request, repo_root);
    let treat_failure_as_warning = request.warning_only || request.treat_runtime_drift_as_warning;
    let started_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let started = Instant::now();

    let doctor_result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: request
            .repo_root
            .clone()
            .or_else(|| Some(repo_root.to_path_buf())),
        target_github_path: request.target_github_path.clone(),
        target_codex_path: request.target_codex_path.clone(),
        target_agents_skills_path: request.target_agents_skills_path.clone(),
        target_copilot_skills_path: request.target_copilot_skills_path.clone(),
        runtime_profile: request.runtime_profile.clone(),
        fallback_runtime_profile: request.fallback_runtime_profile.clone(),
        strict_extras: request.strict_extras,
        sync_on_drift: false,
    });

    let (status, exit_code, error) = match doctor_result {
        Ok(result) => match result.status {
            RuntimeDoctorStatus::Clean | RuntimeDoctorStatus::CleanWithExtras => {
                (RuntimeHealthcheckStatus::Passed, 0, None)
            }
            RuntimeDoctorStatus::Detected if treat_failure_as_warning => {
                (RuntimeHealthcheckStatus::Warning, 1, None)
            }
            RuntimeDoctorStatus::Detected => (RuntimeHealthcheckStatus::Failed, 1, None),
        },
        Err(error) if treat_failure_as_warning => (
            RuntimeHealthcheckStatus::Warning,
            1,
            Some(error.to_string()),
        ),
        Err(error) => (RuntimeHealthcheckStatus::Failed, 1, Some(error.to_string())),
    };

    let finished_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let result = RuntimeHealthcheckCheckResult {
        name: "runtime-doctor".to_string(),
        script: "rust:nettoolskit-runtime::doctor".to_string(),
        arguments,
        status,
        exit_code,
        duration_ms: started.elapsed().as_millis(),
        started_at,
        finished_at,
        error,
    };
    let _ = append_log_line(
        log_path,
        &format!(
            "check {} => {} (exit code {})",
            result.name,
            result.status.as_str(),
            result.exit_code
        ),
    );
    result
}

fn run_bootstrap_check(
    request: &RuntimeHealthcheckRequest,
    repo_root: &Path,
    log_path: &Path,
) -> RuntimeHealthcheckCheckResult {
    let mut arguments = Vec::new();
    arguments.push(format!("-RepoRoot={}", repo_root.display()));
    if let Some(path) = &request.target_github_path {
        arguments.push(format!("-TargetGithubPath={}", path.display()));
    }
    if let Some(path) = &request.target_codex_path {
        arguments.push(format!("-TargetCodexPath={}", path.display()));
    }
    if let Some(path) = &request.target_agents_skills_path {
        arguments.push(format!("-TargetAgentsSkillsPath={}", path.display()));
    }
    if let Some(path) = &request.target_copilot_skills_path {
        arguments.push(format!("-TargetCopilotSkillsPath={}", path.display()));
    }
    if let Some(profile) = &request.runtime_profile {
        arguments.push(format!("-RuntimeProfile={profile}"));
    }
    if request.mirror {
        arguments.push("-Mirror".to_string());
    }

    let started_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let started = Instant::now();
    let bootstrap_result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: request
            .repo_root
            .clone()
            .or_else(|| Some(repo_root.to_path_buf())),
        target_github_path: request.target_github_path.clone(),
        target_codex_path: request.target_codex_path.clone(),
        target_agents_skills_path: request.target_agents_skills_path.clone(),
        target_copilot_skills_path: request.target_copilot_skills_path.clone(),
        runtime_profile: request.runtime_profile.clone(),
        fallback_runtime_profile: request.fallback_runtime_profile.clone(),
        mirror: request.mirror,
        apply_mcp_config: false,
        backup_config: false,
    });

    let (status, exit_code, error) = match bootstrap_result {
        Ok(_) => (RuntimeHealthcheckStatus::Passed, 0, None),
        Err(error) if request.warning_only => (
            RuntimeHealthcheckStatus::Warning,
            1,
            Some(error.to_string()),
        ),
        Err(error) => (RuntimeHealthcheckStatus::Failed, 1, Some(error.to_string())),
    };

    let finished_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let result = RuntimeHealthcheckCheckResult {
        name: "runtime-bootstrap".to_string(),
        script: "rust:nettoolskit-runtime::bootstrap".to_string(),
        arguments,
        status,
        exit_code,
        duration_ms: started.elapsed().as_millis(),
        started_at,
        finished_at,
        error,
    };
    let _ = append_log_line(
        log_path,
        &format!(
            "check {} => {} (exit code {})",
            result.name,
            result.status.as_str(),
            result.exit_code
        ),
    );
    result
}

fn run_validation_check(
    request: &RuntimeHealthcheckRequest,
    repo_root: &Path,
    arguments: &[CommandArgument],
    log_path: &Path,
) -> RuntimeHealthcheckCheckResult {
    let started_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let started = Instant::now();
    let formatted_arguments = format_argument_list(arguments);
    let result = match invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo_root.to_path_buf()),
        validation_profile: Some(request.validation_profile.clone()),
        warning_only: request.warning_only,
        ..ValidateAllRequest::default()
    }) {
        Ok(validation_result) => {
            let (status, exit_code) = match validation_result.overall_status {
                ValidateAllStatus::Passed => (RuntimeHealthcheckStatus::Passed, 0),
                ValidateAllStatus::Warning => (RuntimeHealthcheckStatus::Warning, 0),
                ValidateAllStatus::Failed => (
                    RuntimeHealthcheckStatus::Failed,
                    validation_result.exit_code,
                ),
            };

            RuntimeHealthcheckCheckResult {
                name: "validate-all".to_string(),
                script: "rust:nettoolskit-validation::validate-all".to_string(),
                arguments: formatted_arguments,
                status,
                exit_code,
                duration_ms: started.elapsed().as_millis(),
                started_at,
                finished_at: current_timestamp_string().unwrap_or_else(|_| "0".to_string()),
                error: validation_result
                    .suite_warning_messages
                    .iter()
                    .find(|message| message.starts_with("Could not"))
                    .cloned(),
            }
        }
        Err(error) if request.warning_only => build_check_result(
            "validate-all",
            "rust:nettoolskit-validation::validate-all",
            formatted_arguments,
            true,
            1,
            Some(error.to_string()),
            started_at,
            started.elapsed().as_millis(),
        ),
        Err(error) => build_check_result(
            "validate-all",
            "rust:nettoolskit-validation::validate-all",
            formatted_arguments,
            false,
            1,
            Some(error.to_string()),
            started_at,
            started.elapsed().as_millis(),
        ),
    };

    let _ = append_log_line(
        log_path,
        &format!(
            "check {} => {} (exit code {})",
            result.name,
            result.status.as_str(),
            result.exit_code
        ),
    );
    result
}

fn build_check_result(
    name: &str,
    script: &str,
    arguments: Vec<String>,
    treat_failure_as_warning: bool,
    exit_code: i32,
    error: Option<String>,
    started_at: String,
    duration_ms: u128,
) -> RuntimeHealthcheckCheckResult {
    let status = if exit_code == 0 {
        RuntimeHealthcheckStatus::Passed
    } else if treat_failure_as_warning {
        RuntimeHealthcheckStatus::Warning
    } else {
        RuntimeHealthcheckStatus::Failed
    };

    RuntimeHealthcheckCheckResult {
        name: name.to_string(),
        script: script.to_string(),
        arguments,
        status,
        exit_code,
        duration_ms,
        started_at,
        finished_at: current_timestamp_string().unwrap_or_else(|_| "0".to_string()),
        error,
    }
}

fn doctor_argument_list(request: &RuntimeHealthcheckRequest, repo_root: &Path) -> Vec<String> {
    let mut arguments = Vec::new();
    arguments.push(format!("-RepoRoot={}", repo_root.display()));
    if let Some(path) = &request.target_github_path {
        arguments.push(format!("-TargetGithubPath={}", path.display()));
    }
    if let Some(path) = &request.target_codex_path {
        arguments.push(format!("-TargetCodexPath={}", path.display()));
    }
    if let Some(path) = &request.target_agents_skills_path {
        arguments.push(format!("-TargetAgentsSkillsPath={}", path.display()));
    }
    if let Some(path) = &request.target_copilot_skills_path {
        arguments.push(format!("-TargetCopilotSkillsPath={}", path.display()));
    }
    if let Some(profile) = &request.runtime_profile {
        arguments.push(format!("-RuntimeProfile={profile}"));
    }
    if request.strict_extras {
        arguments.push("-StrictExtras".to_string());
    }
    arguments
}

fn format_argument_list(arguments: &[CommandArgument]) -> Vec<String> {
    arguments
        .iter()
        .map(|argument| match &argument.value {
            Some(value) => format!("-{}={value}", argument.name),
            None => format!("-{}", argument.name),
        })
        .collect()
}

fn check_to_json(check: &RuntimeHealthcheckCheckResult) -> serde_json::Value {
    json!({
        "name": check.name,
        "script": check.script,
        "arguments": check.arguments,
        "status": check.status.as_str(),
        "exitCode": check.exit_code,
        "durationMs": check.duration_ms,
        "startedAt": check.started_at,
        "finishedAt": check.finished_at,
        "error": check.error,
    })
}

fn resolve_healthcheck_output_path(
    repo_root: &Path,
    output_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    match output_path {
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(repo_root.join(".temp/healthcheck-report.json")),
    }
}

fn resolve_healthcheck_log_path(
    repo_root: &Path,
    log_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    match log_path {
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(repo_root
            .join(".temp/logs")
            .join(format!("healthcheck-{}.log", current_timestamp_token()?))),
    }
}

fn initialize_output_file_parent(path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    Ok(())
}

fn initialize_log_file(log_path: &Path) -> anyhow::Result<()> {
    fs::write(
        log_path,
        format!(
            "# healthcheck log\n# generatedAt={}",
            current_timestamp_string()?
        ),
    )
    .with_context(|| format!("failed to initialize '{}'", log_path.display()))
}

fn append_log_line(log_path: &Path, message: &str) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(log_path)
        .with_context(|| format!("failed to open '{}'", log_path.display()))?;
    writeln!(file, "\n[{}] {message}", current_timestamp_string()?)
        .with_context(|| format!("failed to append '{}'", log_path.display()))
}

fn current_timestamp_string() -> anyhow::Result<String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to compute current timestamp")?;
    Ok(duration.as_secs().to_string())
}

fn current_timestamp_token() -> anyhow::Result<String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to compute current timestamp token")?;
    Ok(duration.as_secs().to_string())
}