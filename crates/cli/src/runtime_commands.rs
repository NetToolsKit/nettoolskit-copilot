//! Executable runtime command surfaces for hook, continuity, maintenance, and diagnostics migration.

use clap::{ArgAction, Args, Subcommand};
use nettoolskit_orchestrator::ExitStatus;
use nettoolskit_runtime::{
    export_planning_summary, invoke_apply_vscode_templates, invoke_export_enterprise_trends,
    invoke_pre_commit_eof_hygiene, invoke_pre_tool_use, invoke_runtime_doctor,
    invoke_render_mcp_runtime_artifacts, invoke_render_vscode_mcp_template,
    invoke_runtime_healthcheck, invoke_runtime_self_heal, invoke_setup_git_hooks,
    invoke_setup_global_git_aliases, invoke_sync_codex_mcp_config,
    invoke_trim_trailing_blank_lines, query_local_context_index, update_local_context_index,
    ExportPlanningSummaryRequest, QueryLocalContextIndexRequest,
    RuntimeApplyVscodeTemplatesRequest, RuntimeDoctorRequest, RuntimeDoctorStatus,
    RuntimeExportEnterpriseTrendsRequest, RuntimeHealthcheckRequest, RuntimeHealthcheckStatus,
    RuntimePreCommitEofHygieneRequest, RuntimePreCommitEofHygieneStatus,
    RuntimePreToolUseRequest, RuntimeRenderMcpRuntimeArtifactsRequest,
    RuntimeRenderVscodeMcpTemplateRequest, RuntimeSelfHealRequest, RuntimeSelfHealStatus,
    RuntimeSetupGitHooksRequest, RuntimeSetupGlobalGitAliasesRequest,
    RuntimeSyncCodexMcpConfigRequest, RuntimeTrimTrailingBlankLinesRequest,
    UpdateLocalContextIndexRequest,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Runtime command group.
#[derive(Debug, Subcommand)]
pub enum RuntimeCommand {
    /// Normalize hook payloads for VS Code `PreToolUse`.
    PreToolUse,
    /// Run the native runtime doctor workflow.
    Doctor(RuntimeDoctorArgs),
    /// Run the native runtime healthcheck workflow.
    Healthcheck(RuntimeHealthcheckArgs),
    /// Run the native runtime self-heal workflow.
    SelfHeal(RuntimeSelfHealArgs),
    /// Build or refresh the repository-owned local context index.
    UpdateLocalContextIndex(RuntimeUpdateLocalContextIndexArgs),
    /// Query the repository-owned local context index.
    QueryLocalContextIndex(RuntimeQueryLocalContextIndexArgs),
    /// Export a context handoff summary from active planning artifacts.
    ExportPlanningSummary(RuntimeExportPlanningSummaryArgs),
    /// Export enterprise validation and vulnerability trends.
    #[command(name = "export-enterprise-trends")]
    ExportEnterpriseTrends(RuntimeExportEnterpriseTrendsArgs),
    /// Apply tracked VS Code template files into active workspace files.
    ApplyVscodeTemplates(RuntimeApplyVscodeTemplatesArgs),
    /// Render the tracked VS Code MCP template from the canonical catalog.
    #[command(name = "render-vscode-mcp-template")]
    RenderVscodeMcpTemplate(RuntimeRenderVscodeMcpTemplateArgs),
    /// Render the tracked VS Code and Codex MCP artifacts from the canonical catalog.
    #[command(name = "render-mcp-runtime-artifacts")]
    RenderMcpRuntimeArtifacts(RuntimeRenderMcpRuntimeArtifactsArgs),
    /// Apply MCP server definitions into the local Codex config.toml.
    #[command(name = "sync-codex-mcp-config")]
    SyncCodexMcpConfig(RuntimeSyncCodexMcpConfigArgs),
    /// Trim trailing whitespace and blank lines from text files.
    TrimTrailingBlankLines(RuntimeTrimTrailingBlankLinesArgs),
    /// Apply staged-file EOF hygiene for repository pre-commit flows.
    PreCommitEofHygiene(RuntimePreCommitEofHygieneArgs),
    /// Configure repository-local or managed-global Git hooks.
    SetupGitHooks(RuntimeSetupGitHooksArgs),
    /// Configure managed global Git aliases.
    SetupGlobalGitAliases(RuntimeSetupGlobalGitAliasesArgs),
}

/// CLI arguments for `runtime doctor`.
#[derive(Debug, Args)]
pub struct RuntimeDoctorArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit GitHub runtime target path.
    #[clap(long)]
    pub target_github_path: Option<PathBuf>,
    /// Optional explicit Codex runtime target path.
    #[clap(long)]
    pub target_codex_path: Option<PathBuf>,
    /// Optional explicit picker-visible agent skills path.
    #[clap(long)]
    pub target_agents_skills_path: Option<PathBuf>,
    /// Optional explicit Copilot native skills path.
    #[clap(long)]
    pub target_copilot_skills_path: Option<PathBuf>,
    /// Optional explicit runtime profile name.
    #[clap(long)]
    pub runtime_profile: Option<String>,
    /// Emit detailed mapping output.
    #[clap(long)]
    pub detailed: bool,
    /// Re-run bootstrap remediation when drift is detected.
    #[clap(long)]
    pub sync_on_drift: bool,
    /// Treat extra runtime files as drift failures.
    #[clap(long)]
    pub strict_extras: bool,
}

/// CLI arguments for `runtime healthcheck`.
#[derive(Debug, Args)]
pub struct RuntimeHealthcheckArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit GitHub runtime target path.
    #[clap(long)]
    pub target_github_path: Option<PathBuf>,
    /// Optional explicit Codex runtime target path.
    #[clap(long)]
    pub target_codex_path: Option<PathBuf>,
    /// Optional explicit picker-visible agent skills path.
    #[clap(long)]
    pub target_agents_skills_path: Option<PathBuf>,
    /// Optional explicit Copilot native skills path.
    #[clap(long)]
    pub target_copilot_skills_path: Option<PathBuf>,
    /// Optional explicit runtime profile name.
    #[clap(long)]
    pub runtime_profile: Option<String>,
    /// Run bootstrap before the remaining checks.
    #[clap(long)]
    pub sync_runtime: bool,
    /// Use mirror mode when bootstrap sync is enabled.
    #[clap(long)]
    pub mirror: bool,
    /// Fail runtime doctor on extra files.
    #[clap(long)]
    pub strict_extras: bool,
    /// Validation profile passed to `validate-all`.
    #[clap(long, default_value = "dev")]
    pub validation_profile: String,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
    /// Convert runtime drift failures into warnings.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub treat_runtime_drift_as_warning: bool,
    /// Optional explicit report output path.
    #[clap(long)]
    pub output_path: Option<PathBuf>,
    /// Optional explicit plain-text log path.
    #[clap(long)]
    pub log_path: Option<PathBuf>,
}

/// CLI arguments for `runtime self-heal`.
#[derive(Debug, Args)]
pub struct RuntimeSelfHealArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit GitHub runtime target path.
    #[clap(long)]
    pub target_github_path: Option<PathBuf>,
    /// Optional explicit Codex runtime target path.
    #[clap(long)]
    pub target_codex_path: Option<PathBuf>,
    /// Optional explicit picker-visible agent skills path.
    #[clap(long)]
    pub target_agents_skills_path: Option<PathBuf>,
    /// Optional explicit Copilot native skills path.
    #[clap(long)]
    pub target_copilot_skills_path: Option<PathBuf>,
    /// Optional explicit runtime profile name.
    #[clap(long)]
    pub runtime_profile: Option<String>,
    /// Use mirror mode when bootstrap sync is enabled.
    #[clap(long)]
    pub mirror: bool,
    /// Apply MCP config during bootstrap.
    #[clap(long)]
    pub apply_mcp_config: bool,
    /// Create a timestamped MCP config backup when applying MCP config.
    #[clap(long)]
    pub backup_config: bool,
    /// Apply tracked VS Code templates before the follow-up healthcheck.
    #[clap(long)]
    pub apply_vscode_templates: bool,
    /// Treat extra runtime files as failures during the follow-up healthcheck.
    #[clap(long)]
    pub strict_extras: bool,
    /// Optional explicit JSON report output path.
    #[clap(long)]
    pub output_path: Option<PathBuf>,
    /// Optional explicit plain-text log path.
    #[clap(long)]
    pub log_path: Option<PathBuf>,
}

/// CLI arguments for `runtime trim-trailing-blank-lines`.
#[derive(Debug, Args)]
pub struct RuntimeTrimTrailingBlankLinesArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional directory or single-file path to scan.
    #[clap(long)]
    pub path: Option<PathBuf>,
    /// Explicit file paths to trim.
    #[clap(long = "literal-path")]
    pub literal_paths: Vec<PathBuf>,
    /// Do not modify files; only report the files that would change.
    #[clap(long)]
    pub check_only: bool,
    /// Limit discovery to files currently reported by git status.
    #[clap(long)]
    pub git_changed_only: bool,
}

/// CLI arguments for `runtime update-local-context-index`.
#[derive(Debug, Args)]
pub struct RuntimeUpdateLocalContextIndexArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output directory.
    #[clap(long)]
    pub output_root: Option<PathBuf>,
    /// Rebuild every file even when a persisted index already exists.
    #[clap(long)]
    pub force_full_rebuild: bool,
}

/// CLI arguments for `runtime query-local-context-index`.
#[derive(Debug, Args)]
pub struct RuntimeQueryLocalContextIndexArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Search query executed against the local context index.
    #[clap(long)]
    pub query_text: String,
    /// Optional explicit catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output directory.
    #[clap(long)]
    pub output_root: Option<PathBuf>,
    /// Maximum number of hits to return.
    #[clap(long)]
    pub top: Option<usize>,
    /// Emit JSON instead of the default human-readable summary.
    #[clap(long)]
    pub json_output: bool,
}

/// CLI arguments for `runtime export-planning-summary`.
#[derive(Debug, Args)]
pub struct RuntimeExportPlanningSummaryArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit output file path.
    #[clap(long)]
    pub output_path: Option<PathBuf>,
    /// Render to stdout only without creating a file.
    #[clap(long)]
    pub print_only: bool,
}

/// CLI arguments for `runtime export-enterprise-trends`.
#[derive(Debug, Args)]
pub struct RuntimeExportEnterpriseTrendsArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit validation ledger path.
    #[clap(long)]
    pub ledger_path: Option<PathBuf>,
    /// Optional explicit validation report path.
    #[clap(long)]
    pub validation_report_path: Option<PathBuf>,
    /// Optional explicit vulnerability summary path.
    #[clap(long)]
    pub vulnerability_summary_path: Option<PathBuf>,
    /// Optional explicit JSON output path.
    #[clap(long)]
    pub output_path: Option<PathBuf>,
    /// Optional explicit Markdown summary path.
    #[clap(long)]
    pub summary_path: Option<PathBuf>,
    /// Maximum number of historical entries included in the trend history.
    #[clap(long, default_value_t = 30)]
    pub max_entries: usize,
    /// Preserve warning-only compatibility semantics.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `runtime apply-vscode-templates`.
#[derive(Debug, Args)]
pub struct RuntimeApplyVscodeTemplatesArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit VS Code workspace path.
    #[clap(long)]
    pub vscode_path: Option<PathBuf>,
    /// Overwrite existing target files.
    #[clap(long)]
    pub force: bool,
    /// Skip applying the settings template.
    #[clap(long)]
    pub skip_settings: bool,
    /// Skip applying the MCP template.
    #[clap(long)]
    pub skip_mcp: bool,
}

/// CLI arguments for `runtime render-vscode-mcp-template`.
#[derive(Debug, Args)]
pub struct RuntimeRenderVscodeMcpTemplateArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit MCP runtime catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output path.
    #[clap(long)]
    pub output_path: Option<PathBuf>,
}

/// CLI arguments for `runtime render-mcp-runtime-artifacts`.
#[derive(Debug, Args)]
pub struct RuntimeRenderMcpRuntimeArtifactsArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit MCP runtime catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit VS Code output path.
    #[clap(long)]
    pub vscode_output_path: Option<PathBuf>,
    /// Optional explicit Codex manifest output path.
    #[clap(long)]
    pub codex_output_path: Option<PathBuf>,
}

/// CLI arguments for `runtime sync-codex-mcp-config`.
#[derive(Debug, Args)]
pub struct RuntimeSyncCodexMcpConfigArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit MCP runtime catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit generated Codex manifest path.
    #[clap(long)]
    pub manifest_path: Option<PathBuf>,
    /// Optional explicit target Codex config path.
    #[clap(long)]
    pub target_config_path: Option<PathBuf>,
    /// Create a timestamped backup before writing.
    #[clap(long)]
    pub create_backup: bool,
    /// Print the rendered document without writing it.
    #[clap(long)]
    pub dry_run: bool,
}

/// CLI arguments for `runtime pre-commit-eof-hygiene`.
#[derive(Debug, Args)]
pub struct RuntimePreCommitEofHygieneArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit EOF catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit global settings path.
    #[clap(long)]
    pub global_settings_path: Option<PathBuf>,
}

/// CLI arguments for `runtime setup-git-hooks`.
#[derive(Debug, Args)]
pub struct RuntimeSetupGitHooksArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit EOF mode to persist.
    #[clap(long)]
    pub eof_hygiene_mode: Option<String>,
    /// Optional explicit EOF scope to persist.
    #[clap(long)]
    pub eof_hygiene_scope: Option<String>,
    /// Remove managed hook ownership instead of configuring it.
    #[clap(long)]
    pub uninstall: bool,
    /// Optional explicit EOF catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit global settings path.
    #[clap(long)]
    pub global_settings_path: Option<PathBuf>,
    /// Optional explicit managed global hooks path.
    #[clap(long)]
    pub git_hooks_path: Option<PathBuf>,
    /// Optional isolated global Git config path.
    #[clap(long)]
    pub git_config_global_path: Option<PathBuf>,
}

/// CLI arguments for `runtime setup-global-git-aliases`.
#[derive(Debug, Args)]
pub struct RuntimeSetupGlobalGitAliasesArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit Codex runtime root.
    #[clap(long)]
    pub target_codex_path: Option<PathBuf>,
    /// Remove managed aliases instead of configuring them.
    #[clap(long)]
    pub uninstall: bool,
    /// Optional isolated global Git config path.
    #[clap(long)]
    pub git_config_global_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Default)]
struct PreToolUsePayload {
    cwd: Option<PathBuf>,
    tool_name: Option<String>,
    tool_input: Option<Value>,
}

/// Execute one runtime command through the `ntk` binary.
pub fn execute_runtime_command(command: RuntimeCommand) -> ExitStatus {
    match command {
        RuntimeCommand::PreToolUse => execute_pre_tool_use(),
        RuntimeCommand::Doctor(arguments) => execute_runtime_doctor(arguments),
        RuntimeCommand::Healthcheck(arguments) => execute_runtime_healthcheck(arguments),
        RuntimeCommand::SelfHeal(arguments) => execute_runtime_self_heal(arguments),
        RuntimeCommand::UpdateLocalContextIndex(arguments) => {
            execute_update_local_context_index(arguments)
        }
        RuntimeCommand::QueryLocalContextIndex(arguments) => {
            execute_query_local_context_index(arguments)
        }
        RuntimeCommand::ExportPlanningSummary(arguments) => {
            execute_export_planning_summary(arguments)
        }
        RuntimeCommand::ExportEnterpriseTrends(arguments) => {
            execute_export_enterprise_trends(arguments)
        }
        RuntimeCommand::ApplyVscodeTemplates(arguments) => {
            execute_apply_vscode_templates(arguments)
        }
        RuntimeCommand::RenderVscodeMcpTemplate(arguments) => {
            execute_render_vscode_mcp_template(arguments)
        }
        RuntimeCommand::RenderMcpRuntimeArtifacts(arguments) => {
            execute_render_mcp_runtime_artifacts(arguments)
        }
        RuntimeCommand::SyncCodexMcpConfig(arguments) => {
            execute_sync_codex_mcp_config(arguments)
        }
        RuntimeCommand::TrimTrailingBlankLines(arguments) => {
            execute_trim_trailing_blank_lines(arguments)
        }
        RuntimeCommand::PreCommitEofHygiene(arguments) => execute_pre_commit_eof_hygiene(arguments),
        RuntimeCommand::SetupGitHooks(arguments) => execute_setup_git_hooks(arguments),
        RuntimeCommand::SetupGlobalGitAliases(arguments) => {
            execute_setup_global_git_aliases(arguments)
        }
    }
}

fn execute_pre_tool_use() -> ExitStatus {
    let payload = match read_json_stdin::<PreToolUsePayload>() {
        Ok(payload) => payload,
        Err(message) => {
            eprintln!("{message}");
            return ExitStatus::Error;
        }
    };

    let result = match invoke_pre_tool_use(&RuntimePreToolUseRequest {
        workspace_path: payload.cwd,
        tool_name: payload.tool_name,
        tool_input: payload.tool_input,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    let mut hook_specific_output = serde_json::Map::new();
    hook_specific_output.insert(
        "hookEventName".to_string(),
        Value::String(result.hook_event_name),
    );
    if let Some(additional_context) = result.additional_context {
        hook_specific_output.insert(
            "additionalContext".to_string(),
            Value::String(additional_context),
        );
    }
    if let Some(updated_input) = result.updated_input {
        hook_specific_output.insert("updatedInput".to_string(), updated_input);
    }

    println!("{}", json!({ "hookSpecificOutput": hook_specific_output }));
    ExitStatus::Success
}

fn execute_runtime_doctor(arguments: RuntimeDoctorArgs) -> ExitStatus {
    let result = match invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: arguments.repo_root,
        target_github_path: arguments.target_github_path,
        target_codex_path: arguments.target_codex_path,
        target_agents_skills_path: arguments.target_agents_skills_path,
        target_copilot_skills_path: arguments.target_copilot_skills_path,
        runtime_profile: arguments.runtime_profile,
        sync_on_drift: arguments.sync_on_drift,
        strict_extras: arguments.strict_extras,
        ..RuntimeDoctorRequest::default()
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", doctor_status_label(result.status));
    println!("Runtime profile: {}", result.runtime_profile_name);
    println!("Mappings checked: {}", result.mappings_checked);
    println!("Drift detected: {}", result.has_drift);
    println!("Extras detected: {}", result.has_extras);

    if arguments.detailed {
        for report in &result.reports {
            println!();
            println!("Mapping: {}", report.name);
            println!("  Source: {}", report.source_path.display());
            println!("  Target: {}", report.target_path.display());
            println!("  Source count: {}", report.source_count);
            println!("  Target count: {}", report.target_count);
            println!("  Missing in runtime: {}", report.missing_in_runtime.len());
            println!("  Extra in runtime: {}", report.extra_in_runtime.len());
            println!("  Drifted files: {}", report.drifted_files.len());

            for path in &report.missing_in_runtime {
                println!("    [missing] {path}");
            }
            for path in &report.extra_in_runtime {
                println!("    [extra] {path}");
            }
            for path in &report.drifted_files {
                println!("    [drift] {path}");
            }
        }
    }

    if result.has_drift {
        ExitStatus::Error
    } else {
        ExitStatus::Success
    }
}

fn execute_runtime_healthcheck(arguments: RuntimeHealthcheckArgs) -> ExitStatus {
    let result = match invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        repo_root: arguments.repo_root,
        target_github_path: arguments.target_github_path,
        target_codex_path: arguments.target_codex_path,
        target_agents_skills_path: arguments.target_agents_skills_path,
        target_copilot_skills_path: arguments.target_copilot_skills_path,
        runtime_profile: arguments.runtime_profile,
        sync_runtime: arguments.sync_runtime,
        mirror: arguments.mirror,
        strict_extras: arguments.strict_extras,
        validation_profile: arguments.validation_profile,
        warning_only: arguments.warning_only,
        treat_runtime_drift_as_warning: arguments.treat_runtime_drift_as_warning,
        output_path: arguments.output_path,
        log_path: arguments.log_path,
        ..RuntimeHealthcheckRequest::default()
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", healthcheck_status_label(result.overall_status));
    println!("Runtime profile: {}", result.runtime_profile_name);
    println!("Validation profile: {}", result.validation_profile);
    println!("Output path: {}", result.output_path.display());
    println!("Log path: {}", result.log_path.display());
    println!("Total checks: {}", result.total_checks);
    println!("Passed checks: {}", result.passed_checks);
    println!("Warning checks: {}", result.warning_checks);
    println!("Failed checks: {}", result.failed_checks);

    if result.exit_code == 0 {
        ExitStatus::Success
    } else {
        ExitStatus::Error
    }
}

fn execute_runtime_self_heal(arguments: RuntimeSelfHealArgs) -> ExitStatus {
    let result = match invoke_runtime_self_heal(&RuntimeSelfHealRequest {
        repo_root: arguments.repo_root,
        target_github_path: arguments.target_github_path,
        target_codex_path: arguments.target_codex_path,
        target_agents_skills_path: arguments.target_agents_skills_path,
        target_copilot_skills_path: arguments.target_copilot_skills_path,
        runtime_profile: arguments.runtime_profile,
        mirror: arguments.mirror,
        apply_mcp_config: arguments.apply_mcp_config,
        backup_config: arguments.backup_config,
        apply_vscode_templates: arguments.apply_vscode_templates,
        strict_extras: arguments.strict_extras,
        output_path: arguments.output_path,
        log_path: arguments.log_path,
        ..RuntimeSelfHealRequest::default()
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", self_heal_status_label(result.overall_status));
    println!("Runtime profile: {}", result.runtime_profile_name);
    println!("Output path: {}", result.output_path.display());
    println!("Log path: {}", result.log_path.display());
    println!("Total steps: {}", result.total_steps);
    println!("Passed steps: {}", result.passed_steps);
    println!("Failed steps: {}", result.failed_steps);

    if result.exit_code == 0 {
        ExitStatus::Success
    } else {
        ExitStatus::Error
    }
}

fn execute_update_local_context_index(arguments: RuntimeUpdateLocalContextIndexArgs) -> ExitStatus {
    let result = match update_local_context_index(&UpdateLocalContextIndexRequest {
        repo_root: arguments.repo_root,
        catalog_path: arguments.catalog_path,
        output_root: arguments.output_root,
        force_full_rebuild: arguments.force_full_rebuild,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!(
        "Local context index updated: {}",
        result.index_path.to_string_lossy()
    );
    println!("Files indexed: {}", result.indexed_file_count);
    println!("Files rebuilt: {}", result.rebuilt_file_count);
    println!("Files reused: {}", result.reused_file_count);
    println!("Chunks total: {}", result.chunk_count);
    ExitStatus::Success
}

fn execute_query_local_context_index(arguments: RuntimeQueryLocalContextIndexArgs) -> ExitStatus {
    let result = match query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: arguments.repo_root,
        query_text: arguments.query_text,
        catalog_path: arguments.catalog_path,
        output_root: arguments.output_root,
        top: arguments.top,
        exclude_paths: Vec::new(),
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if arguments.json_output {
        let payload = json!({
            "query": result.query,
            "top": result.top,
            "indexPath": result.index_path,
            "resultCount": result.result_count,
            "hits": result.hits.iter().map(|hit| {
                json!({
                    "id": hit.id,
                    "path": hit.path,
                    "heading": hit.heading,
                    "score": hit.score,
                    "excerpt": hit.excerpt,
                })
            }).collect::<Vec<_>>()
        });
        println!("{payload}");
        return ExitStatus::Success;
    }

    println!("Local context index query: {}", result.query);
    println!("Index: {}", result.index_path.to_string_lossy());
    println!("Hits: {}", result.result_count);
    for hit in &result.hits {
        println!();
        println!("- [{}] {}", hit.score, hit.path);
        if let Some(heading) = &hit.heading {
            if !heading.trim().is_empty() {
                println!("  heading: {heading}");
            }
        }
        println!("  excerpt: {}", hit.excerpt);
    }

    ExitStatus::Success
}

fn execute_export_planning_summary(arguments: RuntimeExportPlanningSummaryArgs) -> ExitStatus {
    let result = match export_planning_summary(&ExportPlanningSummaryRequest {
        repo_root: arguments.repo_root,
        output_path: arguments.output_path,
        print_only: arguments.print_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if arguments.print_only {
        println!("{}", result.document);
        return ExitStatus::Success;
    }

    if let Some(output_path) = result.output_path {
        println!(
            "Context handoff summary written to: {}",
            output_path.to_string_lossy()
        );
        ExitStatus::Success
    } else {
        eprintln!("planning summary did not report an output path");
        ExitStatus::Error
    }
}

fn execute_export_enterprise_trends(arguments: RuntimeExportEnterpriseTrendsArgs) -> ExitStatus {
    let result = match invoke_export_enterprise_trends(&RuntimeExportEnterpriseTrendsRequest {
        repo_root: arguments.repo_root,
        ledger_path: arguments.ledger_path,
        validation_report_path: arguments.validation_report_path,
        vulnerability_summary_path: arguments.vulnerability_summary_path,
        output_path: arguments.output_path,
        summary_path: arguments.summary_path,
        max_entries: arguments.max_entries,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!(
        "Enterprise trends JSON written: {}",
        result.output_path.display()
    );
    println!(
        "Enterprise trends summary written: {}",
        result.summary_path.display()
    );
    println!("Trend history entries: {}", result.history_entries);
    println!("Warnings: {}", result.warnings.len());
    ExitStatus::Success
}

fn execute_apply_vscode_templates(arguments: RuntimeApplyVscodeTemplatesArgs) -> ExitStatus {
    let result = match invoke_apply_vscode_templates(&RuntimeApplyVscodeTemplatesRequest {
        repo_root: arguments.repo_root,
        vscode_path: arguments.vscode_path,
        force: arguments.force,
        skip_settings: arguments.skip_settings,
        skip_mcp: arguments.skip_mcp,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    for file in &result.files {
        if file.applied {
            println!(
                "[OK] Applied template: {} -> {}",
                file.source_path.to_string_lossy(),
                file.target_path.to_string_lossy()
            );
        } else if file.skipped {
            println!(
                "[SKIP] Target exists (use --force): {}",
                file.target_path.to_string_lossy()
            );
        }
    }

    println!();
    println!("VS Code template apply summary");
    println!("  applied: {}", result.applied_count);
    println!("  skipped: {}", result.skipped_count);
    ExitStatus::Success
}

fn execute_render_vscode_mcp_template(
    arguments: RuntimeRenderVscodeMcpTemplateArgs,
) -> ExitStatus {
    let result = match invoke_render_vscode_mcp_template(&RuntimeRenderVscodeMcpTemplateRequest {
        repo_root: arguments.repo_root,
        catalog_path: arguments.catalog_path,
        output_path: arguments.output_path,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Generated: {}", result.output_path.display());
    println!("Catalog: {}", result.catalog_path.display());
    println!("Servers rendered: {}", result.server_count);
    ExitStatus::Success
}

fn execute_render_mcp_runtime_artifacts(
    arguments: RuntimeRenderMcpRuntimeArtifactsArgs,
) -> ExitStatus {
    let result = match invoke_render_mcp_runtime_artifacts(
        &RuntimeRenderMcpRuntimeArtifactsRequest {
            repo_root: arguments.repo_root,
            catalog_path: arguments.catalog_path,
            vscode_output_path: arguments.vscode_output_path,
            codex_output_path: arguments.codex_output_path,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!();
    println!("MCP runtime render summary");
    println!("  Catalog: {}", result.catalog_path.display());
    println!("  VS Code template: {}", result.vscode_output_path.display());
    println!("  Codex manifest: {}", result.codex_output_path.display());
    println!("  VS Code servers: {}", result.vscode_server_count);
    println!("  Codex servers: {}", result.codex_server_count);
    ExitStatus::Success
}

fn execute_sync_codex_mcp_config(arguments: RuntimeSyncCodexMcpConfigArgs) -> ExitStatus {
    let result = match invoke_sync_codex_mcp_config(&RuntimeSyncCodexMcpConfigRequest {
        repo_root: arguments.repo_root,
        catalog_path: arguments.catalog_path,
        manifest_path: arguments.manifest_path,
        target_config_path: arguments.target_config_path,
        create_backup: arguments.create_backup,
        dry_run: arguments.dry_run,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if let Some(backup_path) = &result.backup_path {
        println!("Backup: {}", backup_path.display());
    }

    if result.dry_run {
        println!("{}", result.rendered_document);
        println!();
        println!("Dry-run only. No file changes were written.");
    } else {
        println!("Updated: {}", result.target_config_path.display());
        println!("Servers applied: {}", result.servers_applied);
    }

    ExitStatus::Success
}

fn execute_trim_trailing_blank_lines(arguments: RuntimeTrimTrailingBlankLinesArgs) -> ExitStatus {
    let result = match invoke_trim_trailing_blank_lines(&RuntimeTrimTrailingBlankLinesRequest {
        repo_root: arguments.repo_root,
        path: arguments.path,
        literal_paths: arguments.literal_paths,
        check_only: arguments.check_only,
        git_changed_only: arguments.git_changed_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if arguments.git_changed_only {
        println!("Git changed files mode: enabled");
    }
    println!("Files found: {}", result.discovered_files.len());
    for file_path in &result.discovered_files {
        println!(
            "{}",
            display_repo_relative_path(&result.repo_root, file_path)
        );
    }

    if result.exit_code == 0 {
        ExitStatus::Success
    } else {
        ExitStatus::Error
    }
}

fn execute_pre_commit_eof_hygiene(arguments: RuntimePreCommitEofHygieneArgs) -> ExitStatus {
    let result = match invoke_pre_commit_eof_hygiene(&RuntimePreCommitEofHygieneRequest {
        repo_root: arguments.repo_root,
        catalog_path: arguments.catalog_path,
        global_settings_path: arguments.global_settings_path,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    match result.status {
        RuntimePreCommitEofHygieneStatus::Passed => {
            println!(
                "[pre-commit] EOF autofix mode active. Checking {} staged file(s)...",
                result.staged_file_count
            );
            println!("[pre-commit] EOF autofix completed.");
            ExitStatus::Success
        }
        RuntimePreCommitEofHygieneStatus::Skipped => {
            if let Some(reason) = result.skipped_reason {
                println!("[pre-commit] EOF autofix skipped: {reason}.");
            }
            ExitStatus::Success
        }
        RuntimePreCommitEofHygieneStatus::Failed => {
            eprintln!(
                "[pre-commit] EOF autofix blocked: these files have both staged and unstaged changes."
            );
            for file_path in result.blocked_files {
                eprintln!("  - {file_path}");
            }
            eprintln!(
                "[pre-commit] Run `git trim-eof` manually or stage the full file before committing."
            );
            ExitStatus::Error
        }
    }
}

fn execute_setup_git_hooks(arguments: RuntimeSetupGitHooksArgs) -> ExitStatus {
    match invoke_setup_git_hooks(&RuntimeSetupGitHooksRequest {
        repo_root: arguments.repo_root,
        eof_hygiene_mode: arguments.eof_hygiene_mode,
        eof_hygiene_scope: arguments.eof_hygiene_scope,
        uninstall: arguments.uninstall,
        catalog_path: arguments.catalog_path,
        global_settings_path: arguments.global_settings_path,
        git_hooks_path: arguments.git_hooks_path,
        git_config_global_path: arguments.git_config_global_path,
    }) {
        Ok(_) => ExitStatus::Success,
        Err(error) => {
            eprintln!("{error}");
            ExitStatus::Error
        }
    }
}

fn execute_setup_global_git_aliases(arguments: RuntimeSetupGlobalGitAliasesArgs) -> ExitStatus {
    match invoke_setup_global_git_aliases(&RuntimeSetupGlobalGitAliasesRequest {
        repo_root: arguments.repo_root,
        target_codex_path: arguments.target_codex_path,
        uninstall: arguments.uninstall,
        git_config_global_path: arguments.git_config_global_path,
    }) {
        Ok(_) => ExitStatus::Success,
        Err(error) => {
            eprintln!("{error}");
            ExitStatus::Error
        }
    }
}

fn read_json_stdin<T>() -> Result<T, String>
where
    T: for<'de> Deserialize<'de> + Default,
{
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|error| format!("failed to read stdin: {error}"))?;

    if buffer.trim().is_empty() {
        return Ok(T::default());
    }

    serde_json::from_str(&buffer).map_err(|error| format!("failed to parse stdin JSON: {error}"))
}

fn display_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn healthcheck_status_label(status: RuntimeHealthcheckStatus) -> &'static str {
    match status {
        RuntimeHealthcheckStatus::Passed => "passed",
        RuntimeHealthcheckStatus::Warning => "warning",
        RuntimeHealthcheckStatus::Failed => "failed",
    }
}

fn doctor_status_label(status: RuntimeDoctorStatus) -> &'static str {
    match status {
        RuntimeDoctorStatus::Clean => "clean",
        RuntimeDoctorStatus::CleanWithExtras => "clean-with-extras",
        RuntimeDoctorStatus::Detected => "detected",
    }
}

fn self_heal_status_label(status: RuntimeSelfHealStatus) -> &'static str {
    match status {
        RuntimeSelfHealStatus::Passed => "passed",
        RuntimeSelfHealStatus::Failed => "failed",
    }
}
