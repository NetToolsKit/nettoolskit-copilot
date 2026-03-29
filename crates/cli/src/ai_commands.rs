//! Executable AI command surfaces exposed by `ntk`.

use clap::{Args, Subcommand};
use nettoolskit_orchestrator::{
    query_weekly_ai_usage_summary, AiUsageWeeklyReport, AiUsageWeeklyReportRequest, ExitStatus,
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
}

/// CLI arguments for `ai usage weekly`.
#[derive(Debug, Args)]
pub struct AiUsageWeeklyArgs {
    /// Optional explicit repository root filter.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit AI usage DB path.
    #[clap(long)]
    pub db_path: Option<PathBuf>,
    /// Optional ISO year override. Requires `--iso-week`.
    #[clap(long)]
    pub iso_year: Option<i32>,
    /// Optional ISO week override. Requires `--iso-year`.
    #[clap(long)]
    pub iso_week: Option<u32>,
    /// Emit JSON instead of the default human-readable summary.
    #[clap(long)]
    pub json_output: bool,
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
    }
}

fn execute_ai_usage_weekly(arguments: AiUsageWeeklyArgs) -> ExitStatus {
    let request = AiUsageWeeklyReportRequest {
        db_path: arguments.db_path,
        repo_root: arguments.repo_root,
        iso_year: arguments.iso_year,
        iso_week: arguments.iso_week,
    };

    let report = match query_weekly_ai_usage_summary(&request) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    if arguments.json_output {
        match serde_json::to_string_pretty(&report) {
            Ok(payload) => {
                println!("{payload}");
                return ExitStatus::Success;
            }
            Err(error) => {
                eprintln!("{error}");
                return ExitStatus::Error;
            }
        }
    }

    print_weekly_usage_report(&report);
    ExitStatus::Success
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
        println!("Configured weekly budget");
        if let Some(token_budget_total) = budget.token_budget_total {
            println!(
                "  tokens: {} used / {} budget (remaining: {}, burn: {}%)",
                report.estimated_billable_tokens_total,
                token_budget_total,
                budget.estimated_tokens_remaining.unwrap_or(0),
                budget.estimated_token_burn_pct.unwrap_or(0.0)
            );
        }
        if let Some(cost_budget_usd_total) = budget.cost_budget_usd_total {
            println!(
                "  cost: {:.4} used / {:.4} budget (remaining: {:.4}, burn: {}%)",
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
