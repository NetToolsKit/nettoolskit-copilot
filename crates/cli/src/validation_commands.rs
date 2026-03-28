//! Executable validation command surfaces exposed by `ntk`.

use clap::{ArgAction, Args, Subcommand};
use nettoolskit_orchestrator::ExitStatus;
use nettoolskit_validation::{
    invoke_validate_architecture_boundaries, invoke_validate_audit_ledger,
    invoke_validate_routing_coverage, ValidateArchitectureBoundariesRequest,
    ValidateAuditLedgerRequest, ValidateRoutingCoverageRequest, ValidationCheckStatus,
};
use std::path::PathBuf;

/// Validation command group.
#[derive(Debug, Subcommand)]
pub enum ValidationCommand {
    /// Validate the audit ledger hash chain.
    AuditLedger(ValidationAuditLedgerArgs),
    /// Validate repository architecture boundary baselines.
    ArchitectureBoundaries(ValidationArchitectureBoundariesArgs),
    /// Validate routing catalog coverage against golden fixtures.
    RoutingCoverage(ValidationRoutingCoverageArgs),
}

/// CLI arguments for `validation audit-ledger`.
#[derive(Debug, Args)]
pub struct ValidationAuditLedgerArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit ledger path.
    #[clap(long)]
    pub ledger_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `validation architecture-boundaries`.
#[derive(Debug, Args)]
pub struct ValidationArchitectureBoundariesArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    #[clap(long)]
    pub baseline_path: Option<PathBuf>,
}

/// CLI arguments for `validation routing-coverage`.
#[derive(Debug, Args)]
pub struct ValidationRoutingCoverageArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit catalog path.
    #[clap(long)]
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit fixture path.
    #[clap(long)]
    pub fixture_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// Execute one validation command through the `ntk` binary.
pub fn execute_validation_command(command: ValidationCommand) -> ExitStatus {
    match command {
        ValidationCommand::AuditLedger(arguments) => execute_audit_ledger(arguments),
        ValidationCommand::ArchitectureBoundaries(arguments) => {
            execute_architecture_boundaries(arguments)
        }
        ValidationCommand::RoutingCoverage(arguments) => execute_routing_coverage(arguments),
    }
}

fn execute_audit_ledger(arguments: ValidationAuditLedgerArgs) -> ExitStatus {
    let result = match invoke_validate_audit_ledger(&ValidateAuditLedgerRequest {
        repo_root: arguments.repo_root,
        ledger_path: arguments.ledger_path,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Ledger path: {}", result.ledger_path.display());
    println!("Entries checked: {}", result.entries_checked);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_architecture_boundaries(
    arguments: ValidationArchitectureBoundariesArgs,
) -> ExitStatus {
    let result = match invoke_validate_architecture_boundaries(
        &ValidateArchitectureBoundariesRequest {
            repo_root: arguments.repo_root,
            baseline_path: arguments.baseline_path,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Baseline path: {}", result.baseline_path.display());
    println!("Rules checked: {}", result.rules_checked);
    println!("File checks: {}", result.file_checks);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_routing_coverage(arguments: ValidationRoutingCoverageArgs) -> ExitStatus {
    let result = match invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
        repo_root: arguments.repo_root,
        catalog_path: arguments.catalog_path,
        fixture_path: arguments.fixture_path,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Catalog path: {}", result.catalog_path.display());
    println!("Fixture path: {}", result.fixture_path.display());
    println!("Routes checked: {}", result.routes_checked);
    println!("Cases checked: {}", result.cases_checked);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn print_messages(label: &str, messages: &[String]) {
    if messages.is_empty() {
        return;
    }

    println!("{label}:");
    for message in messages {
        println!("- {message}");
    }
}

fn exit_status_from_code(exit_code: i32) -> ExitStatus {
    if exit_code == 0 {
        ExitStatus::Success
    } else {
        ExitStatus::Error
    }
}

fn status_label(status: ValidationCheckStatus) -> &'static str {
    match status {
        ValidationCheckStatus::Passed => "passed",
        ValidationCheckStatus::Warning => "warning",
        ValidationCheckStatus::Failed => "failed",
    }
}
