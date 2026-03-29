//! Executable AI command surfaces exposed by `ntk`.

use clap::{Args, Subcommand};
use nettoolskit_orchestrator::{
    query_ai_usage_summary, query_weekly_ai_usage_summary, AiUsageSummaryReport,
    AiUsageSummaryReportRequest, AiUsageWeeklyReport, AiUsageWeeklyReportRequest, ExitStatus,
};
use std::path::PathBuf;

/// AI command group.
#[derive(Debug, Subcommand)]
pub enum AiCommand {
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
        AiCommand::Usage { command } => execute_ai_usage_command(command),
    }
}

fn execute_ai_usage_command(command: AiUsageCommand) -> ExitStatus {
    match command {
        AiUsageCommand::Weekly(arguments) => execute_ai_usage_weekly(arguments),
        AiUsageCommand::Summary(arguments) => execute_ai_usage_summary(arguments),
    }
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
    T: serde::Serialize,
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
