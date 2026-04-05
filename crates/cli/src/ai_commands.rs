//! Executable AI command surfaces exposed by `ntk`.

use clap::{Args, Subcommand};
use nettoolskit_orchestrator::{
    invoke_ai_doctor, list_ai_provider_profiles, query_ai_usage_summary,
    query_weekly_ai_usage_summary, render_ai_doctor_report, resolve_ai_provider_profile,
    resolve_ai_provider_profile_from_env, AiDoctorRequest, AiDoctorResult, AiProviderProfile,
    AiUsageSummaryReport, AiUsageSummaryReportRequest, AiUsageWeeklyReport,
    AiUsageWeeklyReportRequest, ExitStatus, NTK_AI_PROFILE_ENV,
};
use std::fs;
use std::path::PathBuf;

/// AI command group.
#[derive(Debug, Subcommand)]
pub enum AiCommand {
    /// Diagnose AI provider/profile readiness without executing a request.
    Doctor(AiDoctorArgs),
    /// Inspect built-in AI provider profiles and presets.
    Profiles {
        /// Profile subcommand.
        #[clap(subcommand)]
        command: AiProfilesCommand,
    },
    /// Inspect persisted AI usage history.
    Usage {
        /// Usage subcommand.
        #[clap(subcommand)]
        command: AiUsageCommand,
    },
}

/// AI usage subcommands.
#[derive(Debug, Subcommand)]
pub enum AiUsageCommand {
    /// Report one ISO week of persisted local AI usage history.
    Weekly(AiUsageWeeklyArgs),
    /// Report a bounded recent multi-week summary of persisted local AI usage history.
    Summary(AiUsageSummaryArgs),
}

/// AI provider profile subcommands.
#[derive(Debug, Subcommand)]
pub enum AiProfilesCommand {
    /// List built-in AI provider profiles.
    List(AiProfilesListArgs),
    /// Show one built-in AI provider profile, or the active profile when no id is provided.
    Show(AiProfilesShowArgs),
}

/// CLI arguments for `ai profiles list`.
#[derive(Debug, Args)]
pub struct AiProfilesListArgs {
    /// Emit JSON instead of the default human-readable summary.
    #[clap(long)]
    pub json_output: bool,
}

/// CLI arguments for `ai profiles show`.
#[derive(Debug, Args)]
pub struct AiProfilesShowArgs {
    /// Optional profile id. When omitted, resolves the active `NTK_AI_PROFILE`.
    pub profile: Option<String>,
    /// Emit JSON instead of the default human-readable summary.
    #[clap(long)]
    pub json_output: bool,
}

/// CLI arguments for `ai doctor`.
#[derive(Debug, Args)]
pub struct AiDoctorArgs {
    /// Emit JSON instead of the default human-readable summary.
    #[clap(long)]
    pub json_output: bool,
    /// Optionally write a Markdown report to disk.
    #[clap(long)]
    pub report_path: Option<PathBuf>,
}

/// Shared report options for AI usage history commands.
#[derive(Debug, Args, Clone)]
pub struct AiUsageReportOptions {
    /// Optional explicit repository root filter.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit AI usage DB path.
    #[clap(long)]
    pub db_path: Option<PathBuf>,
    /// Optional explicit budget config path.
    #[clap(long)]
    pub budget_config_path: Option<PathBuf>,
    /// Optional named budget profile.
    #[clap(long)]
    pub budget_profile: Option<String>,
    /// Emit JSON instead of the default human-readable summary.
    #[clap(long)]
    pub json_output: bool,
}

/// CLI arguments for `ai usage weekly`.
#[derive(Debug, Args)]
pub struct AiUsageWeeklyArgs {
    /// Shared usage report options.
    #[clap(flatten)]
    pub report: AiUsageReportOptions,
    /// Optional ISO year override. Requires `--iso-week`.
    #[clap(long)]
    pub iso_year: Option<i32>,
    /// Optional ISO week override. Requires `--iso-year`.
    #[clap(long)]
    pub iso_week: Option<u32>,
}

/// CLI arguments for `ai usage summary`.
#[derive(Debug, Args)]
pub struct AiUsageSummaryArgs {
    /// Shared usage report options.
    #[clap(flatten)]
    pub report: AiUsageReportOptions,
    /// Number of ISO weeks to include, ending at the selected/current week.
    #[clap(long, default_value_t = 4)]
    pub weeks: usize,
    /// Optional explicit end ISO year override. Requires `--end-iso-week`.
    #[clap(long)]
    pub end_iso_year: Option<i32>,
    /// Optional explicit end ISO week override. Requires `--end-iso-year`.
    #[clap(long)]
    pub end_iso_week: Option<u32>,
}

/// Execute one AI command.
pub async fn execute_ai_command(command: AiCommand) -> ExitStatus {
    match command {
        AiCommand::Doctor(arguments) => execute_ai_doctor(arguments),
        AiCommand::Profiles { command } => execute_ai_profiles_command(command),
        AiCommand::Usage { command } => execute_ai_usage_command(command),
    }
}

fn execute_ai_doctor(arguments: AiDoctorArgs) -> ExitStatus {
    let result = match invoke_ai_doctor(&AiDoctorRequest) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if let Some(report_path) = &arguments.report_path {
        if let Some(parent) = report_path.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                eprintln!("{error}");
                return ExitStatus::Error;
            }
        }
        if let Err(error) = fs::write(report_path, render_ai_doctor_report(&result)) {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    }

    if arguments.json_output {
        return print_json_or_error(&result);
    }

    print_ai_doctor(&result, arguments.report_path.as_deref());
    ExitStatus::Success
}

fn execute_ai_profiles_command(command: AiProfilesCommand) -> ExitStatus {
    match command {
        AiProfilesCommand::List(arguments) => execute_ai_profiles_list(arguments),
        AiProfilesCommand::Show(arguments) => execute_ai_profiles_show(arguments),
    }
}

fn execute_ai_usage_command(command: AiUsageCommand) -> ExitStatus {
    match command {
        AiUsageCommand::Weekly(arguments) => execute_ai_usage_weekly(arguments),
        AiUsageCommand::Summary(arguments) => execute_ai_usage_summary(arguments),
    }
}

fn execute_ai_profiles_list(arguments: AiProfilesListArgs) -> ExitStatus {
    let profiles = list_ai_provider_profiles();

    if arguments.json_output {
        return print_json_or_error(profiles);
    }

    println!("AI provider profiles");
    for profile in profiles {
        println!(
            "- {} ({}) [{} / {}]",
            profile.id, profile.title, profile.provider_mode, profile.support_tier
        );
        println!("  {}", profile.summary);
        println!("  chain: {}", profile.provider_chain.join(" -> "));
    }

    match resolve_ai_provider_profile_from_env() {
        Ok(Some(active_profile)) => {
            println!();
            println!(
                "Active profile: {} (from {})",
                active_profile.id, NTK_AI_PROFILE_ENV
            );
        }
        Ok(None) => {
            println!();
            println!(
                "Active profile: none (set {} to activate a preset)",
                NTK_AI_PROFILE_ENV
            );
        }
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    }

    ExitStatus::Success
}

fn execute_ai_profiles_show(arguments: AiProfilesShowArgs) -> ExitStatus {
    let profile = match arguments.profile.as_deref() {
        Some(profile_id) => match resolve_ai_provider_profile(Some(profile_id)) {
            Ok(Some(profile)) => profile,
            Ok(None) => {
                eprintln!("AI profile id is required");
                return ExitStatus::Error;
            }
            Err(error) => {
                eprintln!("{error}");
                return ExitStatus::Error;
            }
        },
        None => match resolve_ai_provider_profile_from_env() {
            Ok(Some(profile)) => profile,
            Ok(None) => {
                eprintln!(
                    "No active AI profile is set. Pass a profile id or configure {}.",
                    NTK_AI_PROFILE_ENV
                );
                return ExitStatus::Error;
            }
            Err(error) => {
                eprintln!("{error}");
                return ExitStatus::Error;
            }
        },
    };

    if arguments.json_output {
        return print_json_or_error(profile);
    }

    print_ai_profile(profile);
    ExitStatus::Success
}

fn execute_ai_usage_weekly(arguments: AiUsageWeeklyArgs) -> ExitStatus {
    let request = AiUsageWeeklyReportRequest {
        db_path: arguments.report.db_path,
        repo_root: arguments.report.repo_root,
        iso_year: arguments.iso_year,
        iso_week: arguments.iso_week,
        budget_config_path: arguments.report.budget_config_path,
        budget_profile: arguments.report.budget_profile,
    };

    let report = match query_weekly_ai_usage_summary(&request) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if arguments.report.json_output {
        return print_json_or_error(&report);
    }

    print_weekly_usage_report(&report);
    ExitStatus::Success
}

fn execute_ai_usage_summary(arguments: AiUsageSummaryArgs) -> ExitStatus {
    let request = AiUsageSummaryReportRequest {
        db_path: arguments.report.db_path,
        repo_root: arguments.report.repo_root,
        week_count: arguments.weeks,
        end_iso_year: arguments.end_iso_year,
        end_iso_week: arguments.end_iso_week,
        budget_config_path: arguments.report.budget_config_path,
        budget_profile: arguments.report.budget_profile,
    };

    let report = match query_ai_usage_summary(&request) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if arguments.report.json_output {
        return print_json_or_error(&report);
    }

    print_ai_usage_summary_report(&report);
    ExitStatus::Success
}

fn print_json_or_error<T>(value: &T) -> ExitStatus
where
    T: serde::Serialize + ?Sized,
{
    match serde_json::to_string_pretty(value) {
        Ok(payload) => {
            println!("{payload}");
            ExitStatus::Success
        }
        Err(error) => {
            eprintln!("{error}");
            ExitStatus::Error
        }
    }
}

fn print_ai_profile(profile: &AiProviderProfile) {
    println!("Profile: {}", profile.id);
    println!("Title: {}", profile.title);
    println!("Summary: {}", profile.summary);
    println!("Provider mode: {}", profile.provider_mode);
    println!("Support tier: {}", profile.support_tier);
    println!(
        "Live network required: {}",
        if profile.live_network_required {
            "yes"
        } else {
            "no"
        }
    );
    println!("Provider chain: {}", profile.provider_chain.join(" -> "));
    println!(
        "Timeouts: primary={}ms secondary={}ms",
        profile.primary_timeout_ms, profile.secondary_timeout_ms
    );
    println!(
        "Cheap model: {}",
        profile.cheap_model.unwrap_or("provider-default")
    );
    println!(
        "Reasoning model: {}",
        profile.reasoning_model.unwrap_or("provider-default")
    );
    println!("Cheap intents: {}", profile.cheap_intents.join(", "));
    println!(
        "Reasoning intents: {}",
        profile.reasoning_intents.join(", ")
    );
}

fn print_ai_doctor(result: &AiDoctorResult, report_path: Option<&std::path::Path>) {
    println!("AI doctor");
    println!("Status: {:?}", result.status);
    println!("Active profile source: {}", result.active_profile_source);
    if let Some(profile) = &result.active_profile {
        println!("Active profile: {} ({})", profile.id, profile.title);
    } else {
        println!("Active profile: none");
    }
    println!(
        "Provider chain: {} ({})",
        result.provider_chain.join(" -> "),
        result.provider_chain_source
    );
    println!(
        "Routing strategy: {} ({})",
        result.routing_plan.strategy.as_str(),
        result.routing_plan.strategy_source
    );
    println!(
        "Routed order: {}",
        result.routing_plan.ordered_provider_ids.join(" -> ")
    );
    println!(
        "Timeouts: primary={}ms secondary={}ms",
        result.primary_timeout_ms, result.secondary_timeout_ms
    );
    println!("Live provider ready: {}", result.live_provider_ready);
    println!("Fallback ready: {}", result.fallback_ready);

    if let Some(endpoint) = &result.endpoint {
        println!(
            "Endpoint: {} ({})",
            endpoint,
            result.endpoint_source.as_deref().unwrap_or("n/a")
        );
    }
    if let Some(model) = &result.provider_default_model {
        println!(
            "Provider default model: {} ({})",
            model,
            result
                .provider_default_model_source
                .as_deref()
                .unwrap_or("n/a")
        );
    }
    println!("API key present: {}", result.api_key_present);
    println!(
        "Model selection enabled: {}",
        result.model_selection.enabled
    );
    println!(
        "Cheap model: {}",
        result
            .model_selection
            .cheap_model
            .as_deref()
            .unwrap_or("provider-default")
    );
    println!(
        "Reasoning model: {}",
        result
            .model_selection
            .reasoning_model
            .as_deref()
            .unwrap_or("provider-default")
    );
    if !result.routing_plan.provider_scores.is_empty() {
        println!();
        println!("Routing scores");
        for candidate in &result.routing_plan.provider_scores {
            println!(
                "- {}: total={:.3} latency={:.2} cost={:.2} reliability={:.2} policy_fit={:.2}",
                candidate.provider_id,
                candidate.total_score,
                candidate.latency_score,
                candidate.cost_score,
                candidate.reliability_score,
                candidate.policy_fit_score
            );
        }
    }

    if let Some(report_path) = report_path {
        println!("Report path: {}", report_path.display());
    }

    if !result.warnings.is_empty() {
        println!();
        println!("Warnings");
        for warning in &result.warnings {
            println!("- {warning}");
        }
    }
}

fn print_weekly_usage_report(report: &AiUsageWeeklyReport) {
    println!("Week: {}", report.week_label);
    println!("DB path: {}", report.db_path.display());
    if let Some(repo_root_filter) = &report.repo_root_filter {
        println!("Repo root filter: {}", repo_root_filter.display());
    }
    println!("Total events: {}", report.total_events);
    println!("Billable events: {}", report.billable_events);
    println!("Cache-hit events: {}", report.cache_hit_events);
    println!(
        "Estimated tokens: {} (billable: {}, cached: {})",
        report.estimated_tokens_total,
        report.estimated_billable_tokens_total,
        report.estimated_cached_tokens_total
    );
    println!(
        "Estimated cost USD: {:.4} (billable: {:.4}, cached: {:.4})",
        report.estimated_cost_usd_total,
        report.estimated_billable_cost_usd_total,
        report.estimated_cached_cost_usd_total
    );
    if let Some(actual_tokens_total) = report.actual_tokens_total {
        println!("Actual tokens: {actual_tokens_total}");
    } else {
        println!("Actual tokens: n/a");
    }
    if let Some(actual_cost_usd_total) = report.actual_cost_usd_total {
        println!("Actual cost USD: {:.4}", actual_cost_usd_total);
    } else {
        println!("Actual cost USD: n/a");
    }

    if let Some(budget) = &report.budget_status {
        println!();
        println!("Configured weekly budget ({})", budget.profile_name);
        if let Some(token_budget_total) = budget.token_budget_total {
            println!(
                "  tokens: {} used / {} budget (remaining: {}, burn: {:.1}%)",
                report.estimated_billable_tokens_total,
                token_budget_total,
                budget.estimated_tokens_remaining.unwrap_or(0),
                budget.estimated_token_burn_pct.unwrap_or(0.0)
            );
        }
        if let Some(cost_budget_usd_total) = budget.cost_budget_usd_total {
            println!(
                "  cost: {:.4} used / {:.4} budget (remaining: {:.4}, burn: {:.1}%)",
                report.estimated_billable_cost_usd_total,
                cost_budget_usd_total,
                budget.estimated_cost_usd_remaining.unwrap_or(0.0),
                budget.estimated_cost_burn_pct.unwrap_or(0.0)
            );
        }
    }

    println!();
    println!("Providers/models");
    for provider_total in &report.provider_totals {
        let model_label = provider_total.model.as_deref().unwrap_or("n/a");
        println!(
            "- {} / {}: events={} billable={} cache_hits={} estimated_tokens={} estimated_cost_usd={:.4}",
            provider_total.provider,
            model_label,
            provider_total.total_events,
            provider_total.billable_events,
            provider_total.cache_hit_events,
            provider_total.estimated_tokens_total,
            provider_total.estimated_cost_usd_total
        );
    }
}

fn print_ai_usage_summary_report(report: &AiUsageSummaryReport) {
    println!(
        "Summary range: {} week(s) ending at {}",
        report.week_count_returned, report.end_week_label
    );
    println!("DB path: {}", report.db_path.display());
    if let Some(repo_root_filter) = &report.repo_root_filter {
        println!("Repo root filter: {}", repo_root_filter.display());
    }
    println!("Total events: {}", report.total_events);
    println!("Billable events: {}", report.billable_events);
    println!("Cache-hit events: {}", report.cache_hit_events);
    println!(
        "Estimated billable tokens: {}",
        report.estimated_billable_tokens_total
    );
    println!(
        "Estimated billable cost USD: {:.4}",
        report.estimated_billable_cost_usd_total
    );
    if let Some(actual_tokens_total) = report.actual_tokens_total {
        println!("Actual tokens: {actual_tokens_total}");
    } else {
        println!("Actual tokens: n/a");
    }
    if let Some(actual_cost_usd_total) = report.actual_cost_usd_total {
        println!("Actual cost USD: {:.4}", actual_cost_usd_total);
    } else {
        println!("Actual cost USD: n/a");
    }

    if let Some(budget_status) = &report.current_week_budget_status {
        println!();
        println!("Current week budget ({})", budget_status.profile_name);
        if let Some(token_budget_total) = budget_status.token_budget_total {
            println!(
                "  tokens: {} used / {} budget (remaining: {}, burn: {:.1}%)",
                report
                    .weekly_totals
                    .first()
                    .map(|week| week.estimated_billable_tokens_total)
                    .unwrap_or(0),
                token_budget_total,
                budget_status.estimated_tokens_remaining.unwrap_or(0),
                budget_status.estimated_token_burn_pct.unwrap_or(0.0)
            );
        }
        if let Some(cost_budget_total) = budget_status.cost_budget_usd_total {
            println!(
                "  cost: {:.4} used / {:.4} budget (remaining: {:.4}, burn: {:.1}%)",
                report
                    .weekly_totals
                    .first()
                    .map(|week| week.estimated_billable_cost_usd_total)
                    .unwrap_or(0.0),
                cost_budget_total,
                budget_status.estimated_cost_usd_remaining.unwrap_or(0.0),
                budget_status.estimated_cost_burn_pct.unwrap_or(0.0)
            );
        }
    }

    println!();
    println!("Recent weeks");
    for week_total in &report.weekly_totals {
        println!(
            "- {}: events={} billable={} cache_hits={} estimated_billable_tokens={} estimated_billable_cost_usd={:.4}",
            week_total.week_label,
            week_total.total_events,
            week_total.billable_events,
            week_total.cache_hit_events,
            week_total.estimated_billable_tokens_total,
            week_total.estimated_billable_cost_usd_total
        );
    }

    println!();
    println!("Providers/models in range");
    for provider_total in &report.provider_totals {
        let model_label = provider_total.model.as_deref().unwrap_or("n/a");
        println!(
            "- {} / {}: events={} billable={} cache_hits={} estimated_billable_tokens={} estimated_billable_cost_usd={:.4}",
            provider_total.provider,
            model_label,
            provider_total.total_events,
            provider_total.billable_events,
            provider_total.cache_hit_events,
            provider_total.estimated_billable_tokens_total,
            provider_total.estimated_billable_cost_usd_total
        );
    }
}