//! Executable validation command surfaces exposed by `ntk`.

use clap::{ArgAction, Args, Subcommand};
use nettoolskit_orchestrator::ExitStatus;
use nettoolskit_validation::{
    invoke_validate_architecture_boundaries, invoke_validate_audit_ledger,
    invoke_validate_powershell_standards, invoke_validate_routing_coverage,
    invoke_validate_shared_script_checksums, invoke_validate_warning_baseline,
    ValidateArchitectureBoundariesRequest, ValidateAuditLedgerRequest,
    ValidatePowerShellStandardsRequest, ValidateRoutingCoverageRequest,
    ValidateSharedScriptChecksumsRequest, ValidateWarningBaselineRequest, ValidationCheckStatus,
};
use std::path::PathBuf;

/// Validation command group.
#[derive(Debug, Subcommand)]
pub enum ValidationCommand {
    /// Validate the audit ledger hash chain.
    AuditLedger(ValidationAuditLedgerArgs),
    /// Validate repository architecture boundary baselines.
    ArchitectureBoundaries(ValidationArchitectureBoundariesArgs),
    /// Validate PowerShell script standards across repository scripts.
    #[command(name = "powershell-standards")]
    PowershellStandards(ValidationPowerShellStandardsArgs),
    /// Validate routing catalog coverage against golden fixtures.
    RoutingCoverage(ValidationRoutingCoverageArgs),
    /// Validate shared script checksum manifest integrity.
    #[command(name = "shared-script-checksums")]
    SharedScriptChecksums(ValidationSharedScriptChecksumsArgs),
    /// Validate analyzer warning volume against the warning baseline.
    #[command(name = "warning-baseline")]
    WarningBaseline(ValidationWarningBaselineArgs),
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

/// CLI arguments for `validation powershell-standards`.
#[derive(Debug, Args)]
pub struct ValidationPowerShellStandardsArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit scripts root.
    #[clap(long)]
    pub scripts_root: Option<PathBuf>,
    /// Compatibility switch; validate all scripts under the scripts root.
    #[clap(long, action = ArgAction::SetTrue)]
    pub include_all_scripts: bool,
    /// Escalate warning-level style findings into failures.
    #[clap(long, action = ArgAction::SetTrue)]
    pub strict: bool,
    /// Skip optional PSScriptAnalyzer execution.
    #[clap(long, action = ArgAction::SetTrue)]
    pub skip_script_analyzer: bool,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `validation shared-script-checksums`.
#[derive(Debug, Args)]
pub struct ValidationSharedScriptChecksumsArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit manifest path.
    #[clap(long)]
    pub manifest_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
    /// Include per-file mismatch details in the output.
    #[clap(long, action = ArgAction::SetTrue)]
    pub detailed_output: bool,
}

/// CLI arguments for `validation warning-baseline`.
#[derive(Debug, Args)]
pub struct ValidationWarningBaselineArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    #[clap(long)]
    pub baseline_path: Option<PathBuf>,
    /// Optional explicit analyzer warning report path.
    #[clap(long)]
    pub analyzer_report_path: Option<PathBuf>,
    /// Optional explicit output report path.
    #[clap(long)]
    pub report_path: Option<PathBuf>,
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
        ValidationCommand::PowershellStandards(arguments) => {
            execute_powershell_standards(arguments)
        }
        ValidationCommand::RoutingCoverage(arguments) => execute_routing_coverage(arguments),
        ValidationCommand::SharedScriptChecksums(arguments) => {
            execute_shared_script_checksums(arguments)
        }
        ValidationCommand::WarningBaseline(arguments) => execute_warning_baseline(arguments),
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

fn execute_powershell_standards(arguments: ValidationPowerShellStandardsArgs) -> ExitStatus {
    let result = match invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
        repo_root: arguments.repo_root,
        scripts_root: arguments.scripts_root,
        include_all_scripts: arguments.include_all_scripts,
        strict: arguments.strict,
        skip_script_analyzer: arguments.skip_script_analyzer,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Scripts root: {}", result.scripts_root.display());
    println!("Files checked: {}", result.files_checked);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_shared_script_checksums(
    arguments: ValidationSharedScriptChecksumsArgs,
) -> ExitStatus {
    let result = match invoke_validate_shared_script_checksums(
        &ValidateSharedScriptChecksumsRequest {
            repo_root: arguments.repo_root,
            manifest_path: arguments.manifest_path,
            warning_only: arguments.warning_only,
            detailed_output: arguments.detailed_output,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Manifest path: {}", result.manifest_path.display());
    println!("Manifest entries: {}", result.manifest_entries);
    println!("Current entries: {}", result.current_entries);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);
    print_messages("Mismatch details", &result.mismatch_details);

    exit_status_from_code(result.exit_code)
}

fn execute_warning_baseline(arguments: ValidationWarningBaselineArgs) -> ExitStatus {
    let result = match invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
        repo_root: arguments.repo_root,
        baseline_path: arguments.baseline_path,
        analyzer_report_path: arguments.analyzer_report_path,
        report_path: arguments.report_path,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Baseline path: {}", result.baseline_path.display());
    println!("Total warnings: {}", result.total_warnings);
    println!("Report path: {}", result.report_path.display());
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
