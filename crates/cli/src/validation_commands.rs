//! Executable validation command surfaces exposed by `ntk`.

use clap::{ArgAction, Args, Subcommand};
use nettoolskit_orchestrator::ExitStatus;
use nettoolskit_validation::{
    invoke_validate_agent_orchestration,
    invoke_validate_agent_permissions, invoke_validate_agent_skill_alignment,
    invoke_validate_architecture_boundaries, invoke_validate_audit_ledger,
    invoke_validate_policy,
    invoke_validate_powershell_standards, invoke_validate_routing_coverage,
    invoke_validate_release_governance, invoke_validate_release_provenance,
    invoke_validate_security_baseline, invoke_validate_shared_script_checksums,
    invoke_validate_supply_chain, invoke_validate_warning_baseline,
    ValidateAgentOrchestrationRequest,
    ValidateAgentPermissionsRequest, ValidateAgentSkillAlignmentRequest,
    ValidateArchitectureBoundariesRequest, ValidateAuditLedgerRequest,
    ValidatePolicyRequest,
    ValidatePowerShellStandardsRequest, ValidateRoutingCoverageRequest,
    ValidateReleaseGovernanceRequest, ValidateReleaseProvenanceRequest,
    ValidateSecurityBaselineRequest, ValidateSharedScriptChecksumsRequest,
    ValidateSupplyChainRequest, ValidateWarningBaselineRequest, ValidationCheckStatus,
};
use std::path::PathBuf;

/// Validation command group.
#[derive(Debug, Subcommand)]
pub enum ValidationCommand {
    /// Validate multi-agent orchestration contracts and runtime assets.
    #[command(name = "agent-orchestration")]
    AgentOrchestration(ValidationAgentOrchestrationArgs),
    /// Validate agent permission matrix and stage command contracts.
    #[command(name = "agent-permissions")]
    AgentPermissions(ValidationAgentPermissionsArgs),
    /// Validate agent skill references against manifests, pipeline, and evals.
    #[command(name = "agent-skill-alignment")]
    AgentSkillAlignment(ValidationAgentSkillAlignmentArgs),
    /// Validate the audit ledger hash chain.
    AuditLedger(ValidationAuditLedgerArgs),
    /// Validate repository architecture boundary baselines.
    ArchitectureBoundaries(ValidationArchitectureBoundariesArgs),
    /// Validate repository policy contracts under `.github/policies`.
    Policy(ValidationPolicyArgs),
    /// Validate PowerShell script standards across repository scripts.
    #[command(name = "powershell-standards")]
    PowershellStandards(ValidationPowerShellStandardsArgs),
    /// Validate routing catalog coverage against golden fixtures.
    RoutingCoverage(ValidationRoutingCoverageArgs),
    /// Validate repository security baseline contracts.
    #[command(name = "security-baseline")]
    SecurityBaseline(ValidationSecurityBaselineArgs),
    /// Validate shared script checksum manifest integrity.
    #[command(name = "shared-script-checksums")]
    SharedScriptChecksums(ValidationSharedScriptChecksumsArgs),
    /// Validate local supply-chain baseline and export SBOM evidence.
    #[command(name = "supply-chain")]
    SupplyChain(ValidationSupplyChainArgs),
    /// Validate release governance contracts and release guardrails.
    #[command(name = "release-governance")]
    ReleaseGovernance(ValidationReleaseGovernanceArgs),
    /// Validate release provenance evidence and git traceability.
    #[command(name = "release-provenance")]
    ReleaseProvenance(ValidationReleaseProvenanceArgs),
    /// Validate analyzer warning volume against the warning baseline.
    #[command(name = "warning-baseline")]
    WarningBaseline(ValidationWarningBaselineArgs),
}

/// CLI arguments for `validation agent-orchestration`.
#[derive(Debug, Args)]
pub struct ValidationAgentOrchestrationArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
}

/// CLI arguments for `validation agent-permissions`.
#[derive(Debug, Args)]
pub struct ValidationAgentPermissionsArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit permission matrix path.
    #[clap(long)]
    pub matrix_path: Option<PathBuf>,
    /// Optional explicit agent manifest path.
    #[clap(long)]
    pub agent_manifest_path: Option<PathBuf>,
    /// Optional explicit pipeline path.
    #[clap(long)]
    pub pipeline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `validation agent-skill-alignment`.
#[derive(Debug, Args)]
pub struct ValidationAgentSkillAlignmentArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit agent manifest path.
    #[clap(long)]
    pub agent_manifest_path: Option<PathBuf>,
    /// Optional explicit pipeline path.
    #[clap(long)]
    pub pipeline_path: Option<PathBuf>,
    /// Optional explicit eval fixture path.
    #[clap(long)]
    pub eval_fixture_path: Option<PathBuf>,
    /// Optional explicit skills root path.
    #[clap(long)]
    pub skills_root_path: Option<PathBuf>,
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

/// CLI arguments for `validation security-baseline`.
#[derive(Debug, Args)]
pub struct ValidationSecurityBaselineArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    #[clap(long)]
    pub baseline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `validation policy`.
#[derive(Debug, Args)]
pub struct ValidationPolicyArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit policy directory.
    #[clap(long)]
    pub policy_directory: Option<PathBuf>,
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

/// CLI arguments for `validation supply-chain`.
#[derive(Debug, Args)]
pub struct ValidationSupplyChainArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    #[clap(long)]
    pub baseline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `validation release-governance`.
#[derive(Debug, Args)]
pub struct ValidationReleaseGovernanceArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional changelog path override.
    #[clap(long)]
    pub changelog_path: Option<PathBuf>,
    /// Optional CODEOWNERS path override.
    #[clap(long)]
    pub codeowners_path: Option<PathBuf>,
    /// Optional governance document path override.
    #[clap(long)]
    pub governance_doc_path: Option<PathBuf>,
    /// Optional branch protection baseline path override.
    #[clap(long)]
    pub branch_protection_baseline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
}

/// CLI arguments for `validation release-provenance`.
#[derive(Debug, Args)]
pub struct ValidationReleaseProvenanceArgs {
    /// Optional explicit repository root.
    #[clap(long)]
    pub repo_root: Option<PathBuf>,
    /// Optional baseline path override.
    #[clap(long)]
    pub baseline_path: Option<PathBuf>,
    /// Optional audit report path override.
    #[clap(long)]
    pub audit_report_path: Option<PathBuf>,
    /// Force audit-report validation even if the baseline does not require it.
    #[clap(long, action = ArgAction::SetTrue)]
    pub require_audit_report: bool,
    /// Convert required findings to warnings instead of failures.
    #[clap(long, action = ArgAction::Set, default_value_t = true)]
    pub warning_only: bool,
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
        ValidationCommand::AgentOrchestration(arguments) => {
            execute_agent_orchestration(arguments)
        }
        ValidationCommand::AgentPermissions(arguments) => execute_agent_permissions(arguments),
        ValidationCommand::AgentSkillAlignment(arguments) => {
            execute_agent_skill_alignment(arguments)
        }
        ValidationCommand::AuditLedger(arguments) => execute_audit_ledger(arguments),
        ValidationCommand::ArchitectureBoundaries(arguments) => {
            execute_architecture_boundaries(arguments)
        }
        ValidationCommand::Policy(arguments) => execute_policy(arguments),
        ValidationCommand::PowershellStandards(arguments) => {
            execute_powershell_standards(arguments)
        }
        ValidationCommand::RoutingCoverage(arguments) => execute_routing_coverage(arguments),
        ValidationCommand::SecurityBaseline(arguments) => execute_security_baseline(arguments),
        ValidationCommand::SharedScriptChecksums(arguments) => {
            execute_shared_script_checksums(arguments)
        }
        ValidationCommand::SupplyChain(arguments) => execute_supply_chain(arguments),
        ValidationCommand::ReleaseGovernance(arguments) => {
            execute_release_governance(arguments)
        }
        ValidationCommand::ReleaseProvenance(arguments) => {
            execute_release_provenance(arguments)
        }
        ValidationCommand::WarningBaseline(arguments) => execute_warning_baseline(arguments),
    }
}

fn execute_agent_orchestration(arguments: ValidationAgentOrchestrationArgs) -> ExitStatus {
    let result = match invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
        repo_root: arguments.repo_root,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!(
        "Required directories checked: {}",
        result.required_directories_checked
    );
    println!("Required files checked: {}", result.required_files_checked);
    println!("Agents checked: {}", result.agents_checked);
    println!("Stage checks: {}", result.stage_checks);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_agent_permissions(arguments: ValidationAgentPermissionsArgs) -> ExitStatus {
    let result = match invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
        repo_root: arguments.repo_root,
        matrix_path: arguments.matrix_path,
        agent_manifest_path: arguments.agent_manifest_path,
        pipeline_path: arguments.pipeline_path,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Matrix path: {}", result.matrix_path.display());
    println!(
        "Agent manifest path: {}",
        result.agent_manifest_path.display()
    );
    println!("Pipeline path: {}", result.pipeline_path.display());
    println!("Agents checked: {}", result.agents_checked);
    println!("Stage checks: {}", result.stage_checks);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_agent_skill_alignment(arguments: ValidationAgentSkillAlignmentArgs) -> ExitStatus {
    let result = match invoke_validate_agent_skill_alignment(
        &ValidateAgentSkillAlignmentRequest {
            repo_root: arguments.repo_root,
            agent_manifest_path: arguments.agent_manifest_path,
            pipeline_path: arguments.pipeline_path,
            eval_fixture_path: arguments.eval_fixture_path,
            skills_root_path: arguments.skills_root_path,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!(
        "Agent manifest path: {}",
        result.agent_manifest_path.display()
    );
    println!("Pipeline path: {}", result.pipeline_path.display());
    println!("Eval fixture path: {}", result.eval_fixture_path.display());
    println!("Skills root path: {}", result.skills_root_path.display());
    println!("Agents checked: {}", result.agents_checked);
    println!("Stage checks: {}", result.stage_checks);
    println!("Eval case checks: {}", result.eval_case_checks);
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
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

fn execute_policy(arguments: ValidationPolicyArgs) -> ExitStatus {
    let result = match invoke_validate_policy(&ValidatePolicyRequest {
        repo_root: arguments.repo_root,
        policy_directory: arguments.policy_directory,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Policy directory: {}", result.policy_directory.display());
    println!("Policies checked: {}", result.policies_checked);
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

fn execute_security_baseline(arguments: ValidationSecurityBaselineArgs) -> ExitStatus {
    let result = match invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: arguments.repo_root,
        baseline_path: arguments.baseline_path,
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
    println!(
        "Repository files evaluated: {}",
        result.repository_files_evaluated
    );
    println!("Files scanned: {}", result.files_scanned);
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

fn execute_supply_chain(arguments: ValidationSupplyChainArgs) -> ExitStatus {
    let result = match invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: arguments.repo_root,
        baseline_path: arguments.baseline_path,
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
    println!("Dependency manifests: {}", result.dependency_manifests);
    println!("Packages discovered: {}", result.packages_discovered);
    if let Some(sbom_path) = result.sbom_path.as_ref() {
        println!("SBOM path: {}", sbom_path.display());
    }
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_release_governance(arguments: ValidationReleaseGovernanceArgs) -> ExitStatus {
    let result = match invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: arguments.repo_root,
        changelog_path: arguments.changelog_path,
        codeowners_path: arguments.codeowners_path,
        governance_doc_path: arguments.governance_doc_path,
        branch_protection_baseline_path: arguments.branch_protection_baseline_path,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Warning only: {}", result.warning_only);
    println!("Changelog path: {}", result.changelog_path.display());
    println!("CODEOWNERS path: {}", result.codeowners_path.display());
    println!(
        "Governance doc path: {}",
        result.governance_doc_path.display()
    );
    println!(
        "Branch protection baseline path: {}",
        result.branch_protection_baseline_path.display()
    );
    if let Some(version) = result.latest_version.as_ref() {
        println!("Latest changelog version: {version}");
    }
    print_messages("Warnings", &result.warnings);
    print_messages("Failures", &result.failures);

    exit_status_from_code(result.exit_code)
}

fn execute_release_provenance(arguments: ValidationReleaseProvenanceArgs) -> ExitStatus {
    let result = match invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: arguments.repo_root,
        baseline_path: arguments.baseline_path,
        audit_report_path: arguments.audit_report_path,
        require_audit_report: arguments.require_audit_report,
        warning_only: arguments.warning_only,
    }) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return ExitStatus::Error;
        }
    };

    println!("Status: {}", status_label(result.status));
    println!("Warning only: {}", result.warning_only);
    println!("Require audit report: {}", result.require_audit_report);
    println!("Baseline path: {}", result.baseline_path.display());
    println!("Audit report path: {}", result.audit_report_path.display());
    println!("Checks declared: {}", result.checks_declared);
    println!(
        "Checks found in validate-all: {}",
        result.checks_found_in_validate_all
    );
    println!("Evidence files: {}", result.evidence_files);
    println!("Git available: {}", result.git_available);
    if let Some(version) = result.latest_version.as_ref() {
        println!("Latest changelog version: {version}");
    }
    if let Some(branch) = result.current_branch.as_ref() {
        println!("Current branch: {branch}");
    }
    if let Some(commit) = result.head_commit.as_ref() {
        println!("HEAD commit: {commit}");
    }
    if let Some(is_dirty) = result.is_dirty {
        println!("Worktree dirty: {is_dirty}");
    }
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
