//! Runtime self-heal orchestration for repair and follow-up health validation.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::resolve_full_path;
use nettoolskit_core::runtime_execution::{
    resolve_runtime_execution_context, runtime_target_arguments, RuntimeTargetArguments,
};
use serde_json::json;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::{
    error::RuntimeSelfHealCommandError, invoke_runtime_bootstrap, invoke_runtime_healthcheck,
    RuntimeBootstrapRequest, RuntimeHealthcheckRequest, RuntimeHealthcheckResult,
};

/// Request payload for `self-heal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSelfHealRequest {
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
    pub fallback_runtime_profile: Option<String>,
    /// Use mirror mode for bootstrap sync.
    pub mirror: bool,
    /// Apply MCP server settings during bootstrap.
    pub apply_mcp_config: bool,
    /// Create MCP backup during bootstrap.
    pub backup_config: bool,
    /// Apply VS Code active files from templates.
    pub apply_vscode_templates: bool,
    /// Pass strict extras to the follow-up healthcheck.
    pub strict_extras: bool,
    /// Optional explicit JSON report output path.
    pub output_path: Option<PathBuf>,
    /// Optional explicit plain-text log path.
    pub log_path: Option<PathBuf>,
}

impl Default for RuntimeSelfHealRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            target_github_path: None,
            target_codex_path: None,
            target_agents_skills_path: None,
            target_copilot_skills_path: None,
            runtime_profile: None,
            fallback_runtime_profile: None,
            mirror: false,
            apply_mcp_config: false,
            backup_config: false,
            apply_vscode_templates: false,
            strict_extras: false,
            output_path: None,
            log_path: None,
        }
    }
}

/// Self-heal status for individual steps and the final report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSelfHealStatus {
    /// Step or overall run passed.
    Passed,
    /// Step or overall run failed.
    Failed,
}

impl RuntimeSelfHealStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
        }
    }
}

/// One recorded self-heal step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSelfHealStepResult {
    /// Logical step name.
    pub name: String,
    /// Script path or Rust surface identifier.
    pub script: String,
    /// Formatted argument list.
    pub arguments: Vec<String>,
    /// Final step status.
    pub status: RuntimeSelfHealStatus,
    /// Exit code equivalent used by the step.
    pub exit_code: i32,
    /// Elapsed execution time in milliseconds.
    pub duration_ms: u128,
    /// Start timestamp token.
    pub started_at: String,
    /// End timestamp token.
    pub finished_at: String,
    /// Optional error message.
    pub error: Option<String>,
}

/// Result payload for `self-heal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSelfHealResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective runtime profile name.
    pub runtime_profile_name: String,
    /// Resolved JSON report path.
    pub output_path: PathBuf,
    /// Resolved plain-text log path.
    pub log_path: PathBuf,
    /// Ordered steps executed by the run.
    pub steps: Vec<RuntimeSelfHealStepResult>,
    /// Number of executed steps.
    pub total_steps: usize,
    /// Number of passed steps.
    pub passed_steps: usize,
    /// Number of failed steps.
    pub failed_steps: usize,
    /// Overall self-heal status.
    pub overall_status: RuntimeSelfHealStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
    /// Persisted JSON report payload.
    pub report_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandArgument {
    name: &'static str,
    value: Option<String>,
}

/// Execute the runtime self-heal flow.
///
/// # Errors
///
/// Returns [`RuntimeSelfHealCommandError`] when workspace resolution, output
/// preparation, or report/log persistence fails.
pub fn invoke_runtime_self_heal(
    request: &RuntimeSelfHealRequest,
) -> Result<RuntimeSelfHealResult, RuntimeSelfHealCommandError> {
    let current_dir =
        env::current_dir().map_err(|source| RuntimeSelfHealCommandError::ResolveWorkspaceRoot {
            source: source.into(),
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
    .map_err(|source| RuntimeSelfHealCommandError::ResolveExecutionContext { source })?;
    let target_arguments = runtime_target_arguments(&context, true, true);

    let output_path =
        resolve_self_heal_output_path(&context.resolved_repo_root, request.output_path.as_deref())
            .map_err(|source| RuntimeSelfHealCommandError::PrepareArtifacts { source })?;
    let log_path =
        resolve_self_heal_log_path(&context.resolved_repo_root, request.log_path.as_deref())
            .map_err(|source| RuntimeSelfHealCommandError::PrepareArtifacts { source })?;
    initialize_output_file_parent(&output_path)
        .map_err(|source| RuntimeSelfHealCommandError::PrepareArtifacts { source })?;
    initialize_output_file_parent(&log_path)
        .map_err(|source| RuntimeSelfHealCommandError::PrepareArtifacts { source })?;
    initialize_log_file(&log_path)
        .map_err(|source| RuntimeSelfHealCommandError::PrepareArtifacts { source })?;

    append_log_line(
        &log_path,
        &format!("repo root: {}", context.resolved_repo_root.display()),
    )
    .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("runtime profile: {}", context.runtime_profile.name),
    )
    .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("output report: {}", output_path.display()),
    )
    .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;
    append_log_line(&log_path, &format!("log file: {}", log_path.display()))
        .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;

    let mut steps = Vec::new();
    steps.push(run_bootstrap_step(
        request,
        &target_arguments,
        &context.resolved_repo_root,
        &log_path,
    ));

    if request.apply_vscode_templates {
        let vscode_arguments = vec![
            CommandArgument {
                name: "RepoRoot",
                value: Some(context.resolved_repo_root.display().to_string()),
            },
            CommandArgument {
                name: "Force",
                value: Some("true".to_string()),
            },
        ];
        steps.push(run_external_step(
            "apply-vscode-templates",
            &context
                .resolved_repo_root
                .join("scripts/runtime/apply-vscode-templates.ps1"),
            "scripts/runtime/apply-vscode-templates.ps1",
            &vscode_arguments,
            &log_path,
        ));
    } else {
        append_log_line(
            &log_path,
            "skipping VS Code templates apply (enable with apply_vscode_templates)",
        )
        .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;
    }

    let healthcheck_report_path = context
        .resolved_repo_root
        .join(".temp/healthcheck-report.json");
    let healthcheck_log_path = context
        .resolved_repo_root
        .join(".temp/logs/healthcheck-from-self-heal.log");
    steps.push(run_healthcheck_step(
        request,
        &target_arguments,
        &healthcheck_report_path,
        &healthcheck_log_path,
        &log_path,
    ));

    let passed_steps = steps
        .iter()
        .filter(|step| step.status == RuntimeSelfHealStatus::Passed)
        .count();
    let failed_steps = steps
        .iter()
        .filter(|step| step.status == RuntimeSelfHealStatus::Failed)
        .count();
    let overall_status = if failed_steps == 0 {
        RuntimeSelfHealStatus::Passed
    } else {
        RuntimeSelfHealStatus::Failed
    };
    let exit_code = if overall_status == RuntimeSelfHealStatus::Passed {
        0
    } else {
        1
    };

    let healthcheck_payload = read_json_payload(&healthcheck_report_path);
    let report_json = serde_json::to_string_pretty(&json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string().map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?,
        "repoRoot": context.resolved_repo_root.display().to_string(),
        "targets": {
            "github": context.targets.github_runtime_root.display().to_string(),
            "codex": context.targets.codex_runtime_root.display().to_string(),
            "agentsSkills": context.targets.agents_skills_root.display().to_string(),
            "copilotSkills": context.targets.copilot_skills_root.display().to_string(),
        },
        "options": {
            "mirror": request.mirror,
            "applyMcpConfig": request.apply_mcp_config,
            "backupConfig": request.backup_config,
            "applyVscodeTemplates": request.apply_vscode_templates,
            "strictExtras": request.strict_extras,
            "runtimeProfile": context.runtime_profile.name,
        },
        "summary": {
            "totalSteps": steps.len(),
            "passedSteps": passed_steps,
            "failedSteps": failed_steps,
            "overallStatus": overall_status.as_str(),
        },
        "issues": serde_json::Value::Null,
        "steps": steps.iter().map(step_to_json).collect::<Vec<_>>(),
        "healthcheck": healthcheck_payload,
        "logPath": log_path.display().to_string(),
    }))
    .map_err(|source| RuntimeSelfHealCommandError::WriteOutput {
        source: source.into(),
    })?;

    fs::write(&output_path, &report_json)
        .with_context(|| format!("failed to write '{}'", output_path.display()))
        .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!(
            "self-heal summary: total={} passed={} failed={}",
            steps.len(),
            passed_steps,
            failed_steps
        ),
    )
    .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;
    append_log_line(
        &log_path,
        &format!("self-heal report generated: {}", output_path.display()),
    )
    .map_err(|source| RuntimeSelfHealCommandError::WriteOutput { source })?;

    Ok(RuntimeSelfHealResult {
        repo_root: context.resolved_repo_root,
        runtime_profile_name: context.runtime_profile.name,
        output_path,
        log_path,
        steps,
        total_steps: passed_steps + failed_steps,
        passed_steps,
        failed_steps,
        overall_status,
        exit_code,
        report_json,
    })
}

fn run_bootstrap_step(
    request: &RuntimeSelfHealRequest,
    target_arguments: &RuntimeTargetArguments,
    repo_root: &Path,
    log_path: &Path,
) -> RuntimeSelfHealStepResult {
    let mut arguments = runtime_target_argument_list(target_arguments);
    if request.mirror {
        arguments.push("-Mirror".to_string());
    }
    if request.apply_mcp_config {
        arguments.push("-ApplyMcpConfig".to_string());
    }
    if request.backup_config {
        arguments.push("-BackupConfig".to_string());
    }

    let started_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let started = Instant::now();
    let bootstrap_result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: Some(repo_root.to_path_buf()),
        target_github_path: Some(target_arguments.target_github_path.clone()),
        target_codex_path: Some(target_arguments.target_codex_path.clone()),
        target_agents_skills_path: Some(target_arguments.target_agents_skills_path.clone()),
        target_copilot_skills_path: Some(target_arguments.target_copilot_skills_path.clone()),
        runtime_profile: target_arguments.runtime_profile.clone(),
        fallback_runtime_profile: request.fallback_runtime_profile.clone(),
        mirror: request.mirror,
        apply_mcp_config: request.apply_mcp_config,
        backup_config: request.backup_config,
    });

    let result = match bootstrap_result {
        Ok(_) => build_step_result(
            "runtime-bootstrap",
            "rust:nettoolskit-runtime::bootstrap",
            arguments,
            0,
            None,
            started_at,
            started.elapsed().as_millis(),
        ),
        Err(error) => build_step_result(
            "runtime-bootstrap",
            "rust:nettoolskit-runtime::bootstrap",
            arguments,
            1,
            Some(error.to_string()),
            started_at,
            started.elapsed().as_millis(),
        ),
    };
    let _ = append_log_line(
        log_path,
        &format!(
            "step {} => {} (exit code {})",
            result.name,
            result.status.as_str(),
            result.exit_code
        ),
    );
    result
}

fn run_healthcheck_step(
    request: &RuntimeSelfHealRequest,
    target_arguments: &RuntimeTargetArguments,
    healthcheck_report_path: &Path,
    healthcheck_log_path: &Path,
    log_path: &Path,
) -> RuntimeSelfHealStepResult {
    let mut arguments = runtime_target_argument_list(target_arguments);
    arguments.push(format!("-OutputPath={}", healthcheck_report_path.display()));
    arguments.push(format!("-LogPath={}", healthcheck_log_path.display()));
    if request.strict_extras {
        arguments.push("-StrictExtras".to_string());
    }

    let started_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let started = Instant::now();
    let healthcheck_result = invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        repo_root: target_arguments.repo_root.clone(),
        target_github_path: Some(target_arguments.target_github_path.clone()),
        target_codex_path: Some(target_arguments.target_codex_path.clone()),
        target_agents_skills_path: Some(target_arguments.target_agents_skills_path.clone()),
        target_copilot_skills_path: Some(target_arguments.target_copilot_skills_path.clone()),
        runtime_profile: target_arguments.runtime_profile.clone(),
        output_path: Some(healthcheck_report_path.to_path_buf()),
        log_path: Some(healthcheck_log_path.to_path_buf()),
        strict_extras: request.strict_extras,
        ..RuntimeHealthcheckRequest::default()
    });

    let result = match healthcheck_result {
        Ok(result) => map_healthcheck_step_result(
            result,
            arguments,
            started_at,
            started.elapsed().as_millis(),
        ),
        Err(error) => build_step_result(
            "healthcheck",
            "rust:nettoolskit-runtime::healthcheck",
            arguments,
            1,
            Some(error.to_string()),
            started_at,
            started.elapsed().as_millis(),
        ),
    };
    let _ = append_log_line(
        log_path,
        &format!(
            "step {} => {} (exit code {})",
            result.name,
            result.status.as_str(),
            result.exit_code
        ),
    );
    result
}

fn map_healthcheck_step_result(
    result: RuntimeHealthcheckResult,
    arguments: Vec<String>,
    started_at: String,
    duration_ms: u128,
) -> RuntimeSelfHealStepResult {
    if result.exit_code == 0 {
        build_step_result(
            "healthcheck",
            "rust:nettoolskit-runtime::healthcheck",
            arguments,
            result.exit_code,
            None,
            started_at,
            duration_ms,
        )
    } else {
        build_step_result(
            "healthcheck",
            "rust:nettoolskit-runtime::healthcheck",
            arguments,
            result.exit_code,
            Some(format!("healthcheck exit code: {}", result.exit_code)),
            started_at,
            duration_ms,
        )
    }
}

fn run_external_step(
    name: &str,
    script_path: &Path,
    relative_script_path: &str,
    arguments: &[CommandArgument],
    log_path: &Path,
) -> RuntimeSelfHealStepResult {
    let started_at = current_timestamp_string().unwrap_or_else(|_| "0".to_string());
    let started = Instant::now();
    let formatted_arguments = format_argument_list(arguments);

    let result = if !script_path.is_file() {
        build_step_result(
            name,
            relative_script_path,
            formatted_arguments,
            1,
            Some(format!("script not found: {}", script_path.display())),
            started_at,
            started.elapsed().as_millis(),
        )
    } else {
        let mut command = Command::new("pwsh");
        command
            .arg("-NoLogo")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(script_path);
        for argument in arguments {
            command.arg(format!("-{}", argument.name));
            if let Some(value) = &argument.value {
                command.arg(value);
            }
        }

        match command.output() {
            Ok(output) => {
                let exit_code = output.status.code().unwrap_or(1);
                let error = if exit_code == 0 {
                    None
                } else {
                    command_output_error_message(&output.stdout, &output.stderr)
                };
                build_step_result(
                    name,
                    relative_script_path,
                    formatted_arguments,
                    exit_code,
                    error,
                    started_at,
                    started.elapsed().as_millis(),
                )
            }
            Err(error) => build_step_result(
                name,
                relative_script_path,
                formatted_arguments,
                1,
                Some(error.to_string()),
                started_at,
                started.elapsed().as_millis(),
            ),
        }
    };

    let _ = append_log_line(
        log_path,
        &format!(
            "step {} => {} (exit code {})",
            result.name,
            result.status.as_str(),
            result.exit_code
        ),
    );
    result
}

fn build_step_result(
    name: &str,
    script: &str,
    arguments: Vec<String>,
    exit_code: i32,
    error: Option<String>,
    started_at: String,
    duration_ms: u128,
) -> RuntimeSelfHealStepResult {
    RuntimeSelfHealStepResult {
        name: name.to_string(),
        script: script.to_string(),
        arguments,
        status: if exit_code == 0 {
            RuntimeSelfHealStatus::Passed
        } else {
            RuntimeSelfHealStatus::Failed
        },
        exit_code,
        duration_ms,
        started_at,
        finished_at: current_timestamp_string().unwrap_or_else(|_| "0".to_string()),
        error,
    }
}

fn runtime_target_argument_list(target_arguments: &RuntimeTargetArguments) -> Vec<String> {
    let mut arguments = Vec::new();
    if let Some(repo_root) = &target_arguments.repo_root {
        arguments.push(format!("-RepoRoot={}", repo_root.display()));
    }
    arguments.push(format!(
        "-TargetGithubPath={}",
        target_arguments.target_github_path.display()
    ));
    arguments.push(format!(
        "-TargetCodexPath={}",
        target_arguments.target_codex_path.display()
    ));
    arguments.push(format!(
        "-TargetAgentsSkillsPath={}",
        target_arguments.target_agents_skills_path.display()
    ));
    arguments.push(format!(
        "-TargetCopilotSkillsPath={}",
        target_arguments.target_copilot_skills_path.display()
    ));
    if let Some(runtime_profile) = &target_arguments.runtime_profile {
        arguments.push(format!("-RuntimeProfile={runtime_profile}"));
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

fn command_output_error_message(stdout: &[u8], stderr: &[u8]) -> Option<String> {
    let stderr = String::from_utf8_lossy(stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(stdout).trim().to_string();
    if !stderr.is_empty() {
        Some(stderr)
    } else if !stdout.is_empty() {
        Some(stdout)
    } else {
        None
    }
}

fn read_json_payload(path: &Path) -> serde_json::Value {
    if !path.is_file() {
        return serde_json::Value::Null;
    }

    fs::read_to_string(path)
        .ok()
        .and_then(|payload| serde_json::from_str::<serde_json::Value>(&payload).ok())
        .unwrap_or(serde_json::Value::Null)
}

fn step_to_json(step: &RuntimeSelfHealStepResult) -> serde_json::Value {
    json!({
        "name": step.name,
        "script": step.script,
        "arguments": step.arguments,
        "status": step.status.as_str(),
        "exitCode": step.exit_code,
        "durationMs": step.duration_ms,
        "startedAt": step.started_at,
        "finishedAt": step.finished_at,
        "error": step.error,
    })
}

fn resolve_self_heal_output_path(
    repo_root: &Path,
    output_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    match output_path {
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(repo_root.join(".temp/self-heal-report.json")),
    }
}

fn resolve_self_heal_log_path(
    repo_root: &Path,
    log_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    match log_path {
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(repo_root
            .join(".temp/logs")
            .join(format!("self-heal-{}.log", current_timestamp_token()?))),
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
            "# self-heal log\n# generatedAt={}",
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