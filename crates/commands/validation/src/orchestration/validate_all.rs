//! Rust-owned `validate-all` orchestration.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::error::ValidateAllCommandError;
use crate::orchestration::profiles::{load_profiles_document, select_profile, ValidationProfile};
use crate::{
    invoke_validate_agent_hooks, invoke_validate_agent_orchestration,
    invoke_validate_agent_permissions, invoke_validate_agent_skill_alignment,
    invoke_validate_architecture_boundaries, invoke_validate_audit_ledger,
    invoke_validate_authoritative_source_policy, invoke_validate_compatibility_lifecycle_policy,
    invoke_validate_dotnet_standards, invoke_validate_instruction_architecture,
    invoke_validate_instruction_metadata, invoke_validate_instructions,
    invoke_validate_planning_structure, invoke_validate_policy,
    invoke_validate_powershell_standards, invoke_validate_readme_standards,
    invoke_validate_release_governance, invoke_validate_release_provenance,
    invoke_validate_routing_coverage, invoke_validate_runtime_script_tests,
    invoke_validate_security_baseline, invoke_validate_shared_script_checksums,
    invoke_validate_shell_hooks, invoke_validate_supply_chain, invoke_validate_template_standards,
    invoke_validate_warning_baseline, invoke_validate_workspace_efficiency,
    ValidateAgentHooksRequest, ValidateAgentOrchestrationRequest, ValidateAgentPermissionsRequest,
    ValidateAgentSkillAlignmentRequest, ValidateArchitectureBoundariesRequest,
    ValidateAuditLedgerRequest, ValidateAuthoritativeSourcePolicyRequest,
    ValidateCompatibilityLifecyclePolicyRequest, ValidateDotnetStandardsRequest,
    ValidateInstructionArchitectureRequest, ValidateInstructionMetadataRequest,
    ValidateInstructionsRequest, ValidatePlanningStructureRequest, ValidatePolicyRequest,
    ValidatePowerShellStandardsRequest, ValidateReadmeStandardsRequest,
    ValidateReleaseGovernanceRequest, ValidateReleaseProvenanceRequest,
    ValidateRoutingCoverageRequest, ValidateRuntimeScriptTestsRequest,
    ValidateSecurityBaselineRequest, ValidateSharedScriptChecksumsRequest,
    ValidateShellHooksRequest, ValidateSupplyChainRequest, ValidateTemplateStandardsRequest,
    ValidateWarningBaselineRequest, ValidateWorkspaceEfficiencyRequest,
};

const DEFAULT_VALIDATION_PROFILES_PATH: &str = ".github/governance/validation-profiles.json";
const DEFAULT_LEDGER_PATH: &str = ".temp/audit/validation-ledger.jsonl";
const DEFAULT_OUTPUT_PATH: &str = ".temp/audit/validate-all.latest.json";
const ZERO_LEDGER_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const DEFAULT_CHECK_ORDER: &[&str] = &[
    "validate-instructions",
    "validate-policy",
    "validate-security-baseline",
    "validate-shared-script-checksums",
    "validate-agent-orchestration",
    "validate-agent-skill-alignment",
    "validate-agent-permissions",
    "validate-planning-structure",
    "validate-routing-coverage",
    "validate-authoritative-source-policy",
    "validate-instruction-architecture",
    "validate-readme-standards",
    "validate-template-standards",
    "validate-workspace-efficiency",
    "validate-compatibility-lifecycle-policy",
    "validate-powershell-standards",
    "validate-agent-hooks",
    "validate-shell-hooks",
    "validate-runtime-script-tests",
    "validate-warning-baseline",
    "validate-dotnet-standards",
    "validate-architecture-boundaries",
    "validate-instruction-metadata",
    "validate-supply-chain",
    "validate-release-governance",
    "validate-release-provenance",
    "validate-audit-ledger",
];

/// Request payload for `validate-all`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAllRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit validation profile id.
    pub validation_profile: Option<String>,
    /// Optional explicit validation profile catalog path.
    pub validation_profiles_path: Option<PathBuf>,
    /// Pass `-IncludeAllScripts` to PowerShell standards validation.
    pub include_all_powershell_scripts: bool,
    /// Pass `-Strict` to PowerShell standards validation.
    pub strict_powershell_standards: bool,
    /// Pass `-SkipScriptAnalyzer` to PowerShell standards validation.
    pub skip_ps_script_analyzer: bool,
    /// Global warning-only mode.
    pub warning_only: bool,
    /// Append ledger evidence for the run.
    pub write_ledger: bool,
    /// Optional explicit validation ledger path.
    pub ledger_path: Option<PathBuf>,
    /// Optional explicit JSON report output path.
    pub output_path: Option<PathBuf>,
}

impl Default for ValidateAllRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            validation_profile: None,
            validation_profiles_path: None,
            include_all_powershell_scripts: false,
            strict_powershell_standards: false,
            skip_ps_script_analyzer: false,
            warning_only: true,
            write_ledger: true,
            ledger_path: None,
            output_path: None,
        }
    }
}

/// Validation check status used for individual checks and the overall result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationCheckStatus {
    /// The check or the overall run passed.
    Passed,
    /// The check or the overall run completed with warnings.
    Warning,
    /// The check or the overall run failed.
    Failed,
}

impl ValidationCheckStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::Failed => "failed",
        }
    }
}

/// One executed validation check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationCheckResult {
    /// Logical check name.
    pub name: String,
    /// Legacy script path executed by the check.
    pub script: String,
    /// Formatted argument list.
    pub arguments: Vec<String>,
    /// Final check status.
    pub status: ValidationCheckStatus,
    /// Exit code equivalent for the check.
    pub exit_code: i32,
    /// Elapsed execution time in milliseconds.
    pub duration_ms: u128,
    /// Optional error emitted by the check.
    pub error: Option<String>,
}

/// Result payload for `validate-all`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAllResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective validation profile id.
    pub profile_id: String,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved JSON report output path.
    pub output_path: PathBuf,
    /// Resolved ledger path when ledger writing is enabled.
    pub ledger_path: Option<PathBuf>,
    /// Archived broken ledger path when chain repair was needed.
    pub archived_broken_ledger_path: Option<PathBuf>,
    /// Ordered check results.
    pub checks: Vec<ValidationCheckResult>,
    /// Number of checks executed.
    pub total_checks: usize,
    /// Number of passed checks.
    pub passed_checks: usize,
    /// Number of warning checks.
    pub warning_checks: usize,
    /// Number of failed checks.
    pub failed_checks: usize,
    /// Number of non-check failures such as ledger/report persistence in enforcing mode.
    pub supplemental_failures: usize,
    /// Overall result status.
    pub overall_status: ValidationCheckStatus,
    /// Process exit code equivalent for wrapper and runtime callers.
    pub exit_code: i32,
    /// Suite warning messages.
    pub suite_warning_messages: Vec<String>,
    /// Persisted JSON report payload.
    pub report_json: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationArgumentKind {
    String,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValidationArgumentValue {
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationCommandArgument {
    name: String,
    value: ValidationArgumentValue,
    kind: ValidationArgumentKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationCheckDefinition {
    name: &'static str,
    executor: ValidationCheckExecutor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationCheckExecutor {
    Native(NativeValidationCheck),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeValidationCheck {
    AgentHooks,
    AgentOrchestration,
    AgentPermissions,
    AgentSkillAlignment,
    ArchitectureBoundaries,
    CompatibilityLifecyclePolicy,
    DotnetStandards,
    PowerShellStandards,
    Policy,
    SecurityBaseline,
    SharedScriptChecksums,
    SupplyChain,
    ReleaseGovernance,
    ReleaseProvenance,
    Instructions,
    PlanningStructure,
    AuditLedger,
    ReadmeStandards,
    InstructionMetadata,
    InstructionArchitecture,
    RoutingCoverage,
    TemplateStandards,
    AuthoritativeSourcePolicy,
    RuntimeScriptTests,
    ShellHooks,
    WarningBaseline,
    WorkspaceEfficiency,
}

impl ValidationCheckDefinition {
    fn script_label(&self) -> &'static str {
        match self.executor {
            ValidationCheckExecutor::Native(NativeValidationCheck::Instructions) => {
                "rust:nettoolskit-validation::validate-instructions"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentHooks) => {
                "rust:nettoolskit-validation::validate-agent-hooks"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentOrchestration) => {
                "rust:nettoolskit-validation::validate-agent-orchestration"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentPermissions) => {
                "rust:nettoolskit-validation::validate-agent-permissions"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentSkillAlignment) => {
                "rust:nettoolskit-validation::validate-agent-skill-alignment"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::ArchitectureBoundaries) => {
                "rust:nettoolskit-validation::validate-architecture-boundaries"
            }
            ValidationCheckExecutor::Native(
                NativeValidationCheck::CompatibilityLifecyclePolicy,
            ) => "rust:nettoolskit-validation::validate-compatibility-lifecycle-policy",
            ValidationCheckExecutor::Native(NativeValidationCheck::DotnetStandards) => {
                "rust:nettoolskit-validation::validate-dotnet-standards"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::PowerShellStandards) => {
                "rust:nettoolskit-validation::validate-powershell-standards"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::Policy) => {
                "rust:nettoolskit-validation::validate-policy"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::SecurityBaseline) => {
                "rust:nettoolskit-validation::validate-security-baseline"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::SharedScriptChecksums) => {
                "rust:nettoolskit-validation::validate-shared-script-checksums"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::SupplyChain) => {
                "rust:nettoolskit-validation::validate-supply-chain"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::ReleaseGovernance) => {
                "rust:nettoolskit-validation::validate-release-governance"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::ReleaseProvenance) => {
                "rust:nettoolskit-validation::validate-release-provenance"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::PlanningStructure) => {
                "rust:nettoolskit-validation::validate-planning-structure"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::AuditLedger) => {
                "rust:nettoolskit-validation::validate-audit-ledger"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::ReadmeStandards) => {
                "rust:nettoolskit-validation::validate-readme-standards"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::InstructionMetadata) => {
                "rust:nettoolskit-validation::validate-instruction-metadata"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::InstructionArchitecture) => {
                "rust:nettoolskit-validation::validate-instruction-architecture"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::RoutingCoverage) => {
                "rust:nettoolskit-validation::validate-routing-coverage"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::TemplateStandards) => {
                "rust:nettoolskit-validation::validate-template-standards"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::AuthoritativeSourcePolicy) => {
                "rust:nettoolskit-validation::validate-authoritative-source-policy"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::RuntimeScriptTests) => {
                "rust:nettoolskit-validation::validate-runtime-script-tests"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::ShellHooks) => {
                "rust:nettoolskit-validation::validate-shell-hooks"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::WarningBaseline) => {
                "rust:nettoolskit-validation::validate-warning-baseline"
            }
            ValidationCheckExecutor::Native(NativeValidationCheck::WorkspaceEfficiency) => {
                "rust:nettoolskit-validation::validate-workspace-efficiency"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitState {
    available: bool,
    branch: Option<String>,
    commit: Option<String>,
    is_dirty: Option<bool>,
}

/// Run the validation suite orchestrator.
///
/// # Errors
///
/// Returns [`ValidateAllCommandError`] when the repository root cannot be
/// resolved.
pub fn invoke_validate_all(
    request: &ValidateAllRequest,
) -> Result<ValidateAllResult, ValidateAllCommandError> {
    let current_dir =
        env::current_dir().map_err(|source| ValidateAllCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateAllCommandError::ResolveWorkspaceRoot { source })?;
    let output_path = resolve_output_path(&repo_root, request.output_path.as_deref());
    let ledger_path = request
        .write_ledger
        .then(|| resolve_ledger_path(&repo_root, request.ledger_path.as_deref()));

    let mut suite_warnings = Vec::new();
    let archived_broken_ledger_path = if let Some(ledger_path) = ledger_path.as_deref() {
        repair_validation_ledger_if_needed(&repo_root, ledger_path, &mut suite_warnings)
    } else {
        None
    };

    let profiles_path =
        resolve_profiles_path(&repo_root, request.validation_profiles_path.as_deref());
    let profiles_document = load_profiles_document(&profiles_path, &mut suite_warnings);
    let selected_profile = select_profile(
        profiles_document.as_ref(),
        request.validation_profile.as_deref(),
        &mut suite_warnings,
    );
    let profile_id = selected_profile
        .as_ref()
        .map(|profile| profile.id.clone())
        .unwrap_or_else(|| "custom".to_string());
    let effective_warning_only = request.warning_only
        || selected_profile
            .as_ref()
            .is_some_and(|profile| profile.warning_only);

    let selected_check_order = resolve_check_order(selected_profile.as_ref());
    let check_option_map = selected_profile
        .as_ref()
        .map(|profile| profile.check_options.clone())
        .unwrap_or_default();
    let check_definitions = validation_check_catalog();
    let mut checks = Vec::new();

    for check_name in selected_check_order {
        let Some(definition) = check_definitions.get(check_name.as_str()) else {
            suite_warnings.push(format!(
                "Profile references unknown check '{check_name}' and it will be skipped."
            ));
            continue;
        };

        let mut check_arguments = build_base_check_arguments(definition, &repo_root, request);
        let mut check_warning_only = effective_warning_only;
        if let Some(check_options) = check_option_map.get(check_name.as_str()) {
            apply_check_options(check_options, &mut check_arguments, &mut check_warning_only);
        }

        if definition_supports_parameter(definition, "WarningOnly") {
            check_arguments.insert(
                "WarningOnly".to_string(),
                ValidationCommandArgument {
                    name: "WarningOnly".to_string(),
                    value: ValidationArgumentValue::Bool(check_warning_only),
                    kind: ValidationArgumentKind::Bool,
                },
            );
        }

        checks.push(run_validation_script(
            &repo_root,
            definition,
            check_arguments,
            check_warning_only,
        ));
    }

    let passed_checks = checks
        .iter()
        .filter(|check| check.status == ValidationCheckStatus::Passed)
        .count();
    let warning_checks = checks
        .iter()
        .filter(|check| check.status == ValidationCheckStatus::Warning)
        .count();
    let failed_checks = checks
        .iter()
        .filter(|check| check.status == ValidationCheckStatus::Failed)
        .count();
    let mut supplemental_failures = 0usize;

    if let Some(ledger_path) = ledger_path.as_deref() {
        if let Err(error) = write_validation_ledger(
            &repo_root,
            ledger_path,
            &profile_id,
            effective_warning_only,
            &checks,
        ) {
            suite_warnings.push(format!("Could not write validation ledger: {error}"));
            if !effective_warning_only {
                supplemental_failures += 1;
            }
        }
    }

    let report_json = render_validation_report(
        &repo_root,
        &profile_id,
        effective_warning_only,
        &checks,
        &suite_warnings,
        failed_checks + supplemental_failures,
    );
    if let Err(error) = write_validation_report(&output_path, &report_json) {
        suite_warnings.push(format!("Could not write validation report: {error}"));
        if !effective_warning_only {
            supplemental_failures += 1;
        }
    }

    let total_failures = failed_checks + supplemental_failures;
    let overall_status = if total_failures > 0 {
        ValidationCheckStatus::Failed
    } else if warning_checks > 0 || !suite_warnings.is_empty() {
        ValidationCheckStatus::Warning
    } else {
        ValidationCheckStatus::Passed
    };
    let exit_code = if total_failures > 0 && !effective_warning_only {
        1
    } else {
        0
    };

    Ok(ValidateAllResult {
        repo_root,
        profile_id,
        warning_only: effective_warning_only,
        output_path,
        ledger_path,
        archived_broken_ledger_path,
        total_checks: checks.len(),
        passed_checks,
        warning_checks,
        failed_checks,
        supplemental_failures,
        overall_status,
        exit_code,
        suite_warning_messages: suite_warnings,
        report_json,
        checks,
    })
}

fn resolve_profiles_path(repo_root: &Path, requested_path: Option<&Path>) -> PathBuf {
    match requested_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(DEFAULT_VALIDATION_PROFILES_PATH),
    }
}

fn resolve_ledger_path(repo_root: &Path, requested_path: Option<&Path>) -> PathBuf {
    match requested_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(DEFAULT_LEDGER_PATH),
    }
}

fn resolve_output_path(repo_root: &Path, requested_path: Option<&Path>) -> PathBuf {
    match requested_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(DEFAULT_OUTPUT_PATH),
    }
}

fn resolve_check_order(selected_profile: Option<&ValidationProfile>) -> Vec<String> {
    if let Some(profile) = selected_profile {
        if !profile.check_order.is_empty() {
            return profile.check_order.clone();
        }
    }

    DEFAULT_CHECK_ORDER
        .iter()
        .map(|entry| (*entry).to_string())
        .collect()
}

fn validation_check_catalog() -> HashMap<&'static str, ValidationCheckDefinition> {
    let mut catalog = HashMap::new();
    for (name, script) in [
        (
            "validate-instructions",
            ValidationCheckExecutor::Native(NativeValidationCheck::Instructions),
        ),
        (
            "validate-policy",
            ValidationCheckExecutor::Native(NativeValidationCheck::Policy),
        ),
        (
            "validate-security-baseline",
            ValidationCheckExecutor::Native(NativeValidationCheck::SecurityBaseline),
        ),
        (
            "validate-shared-script-checksums",
            ValidationCheckExecutor::Native(NativeValidationCheck::SharedScriptChecksums),
        ),
        (
            "validate-agent-orchestration",
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentOrchestration),
        ),
        (
            "validate-agent-skill-alignment",
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentSkillAlignment),
        ),
        (
            "validate-agent-permissions",
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentPermissions),
        ),
        (
            "validate-planning-structure",
            ValidationCheckExecutor::Native(NativeValidationCheck::PlanningStructure),
        ),
        (
            "validate-routing-coverage",
            ValidationCheckExecutor::Native(NativeValidationCheck::RoutingCoverage),
        ),
        (
            "validate-authoritative-source-policy",
            ValidationCheckExecutor::Native(NativeValidationCheck::AuthoritativeSourcePolicy),
        ),
        (
            "validate-instruction-architecture",
            ValidationCheckExecutor::Native(NativeValidationCheck::InstructionArchitecture),
        ),
        (
            "validate-readme-standards",
            ValidationCheckExecutor::Native(NativeValidationCheck::ReadmeStandards),
        ),
        (
            "validate-template-standards",
            ValidationCheckExecutor::Native(NativeValidationCheck::TemplateStandards),
        ),
        (
            "validate-workspace-efficiency",
            ValidationCheckExecutor::Native(NativeValidationCheck::WorkspaceEfficiency),
        ),
        (
            "validate-compatibility-lifecycle-policy",
            ValidationCheckExecutor::Native(NativeValidationCheck::CompatibilityLifecyclePolicy),
        ),
        (
            "validate-powershell-standards",
            ValidationCheckExecutor::Native(NativeValidationCheck::PowerShellStandards),
        ),
        (
            "validate-agent-hooks",
            ValidationCheckExecutor::Native(NativeValidationCheck::AgentHooks),
        ),
        (
            "validate-shell-hooks",
            ValidationCheckExecutor::Native(NativeValidationCheck::ShellHooks),
        ),
        (
            "validate-runtime-script-tests",
            ValidationCheckExecutor::Native(NativeValidationCheck::RuntimeScriptTests),
        ),
        (
            "validate-warning-baseline",
            ValidationCheckExecutor::Native(NativeValidationCheck::WarningBaseline),
        ),
        (
            "validate-dotnet-standards",
            ValidationCheckExecutor::Native(NativeValidationCheck::DotnetStandards),
        ),
        (
            "validate-architecture-boundaries",
            ValidationCheckExecutor::Native(NativeValidationCheck::ArchitectureBoundaries),
        ),
        (
            "validate-instruction-metadata",
            ValidationCheckExecutor::Native(NativeValidationCheck::InstructionMetadata),
        ),
        (
            "validate-supply-chain",
            ValidationCheckExecutor::Native(NativeValidationCheck::SupplyChain),
        ),
        (
            "validate-release-governance",
            ValidationCheckExecutor::Native(NativeValidationCheck::ReleaseGovernance),
        ),
        (
            "validate-release-provenance",
            ValidationCheckExecutor::Native(NativeValidationCheck::ReleaseProvenance),
        ),
        (
            "validate-audit-ledger",
            ValidationCheckExecutor::Native(NativeValidationCheck::AuditLedger),
        ),
    ] {
        catalog.insert(
            name,
            ValidationCheckDefinition {
                name,
                executor: script,
            },
        );
    }

    catalog
}

fn build_base_check_arguments(
    definition: &ValidationCheckDefinition,
    repo_root: &Path,
    request: &ValidateAllRequest,
) -> BTreeMap<String, ValidationCommandArgument> {
    let mut arguments = BTreeMap::new();
    arguments.insert(
        "RepoRoot".to_string(),
        ValidationCommandArgument {
            name: "RepoRoot".to_string(),
            value: ValidationArgumentValue::String(repo_root.display().to_string()),
            kind: ValidationArgumentKind::String,
        },
    );

    if definition.name == "validate-powershell-standards" {
        if request.include_all_powershell_scripts {
            arguments.insert(
                "IncludeAllScripts".to_string(),
                ValidationCommandArgument {
                    name: "IncludeAllScripts".to_string(),
                    value: ValidationArgumentValue::Bool(true),
                    kind: ValidationArgumentKind::Bool,
                },
            );
        }
        if request.strict_powershell_standards {
            arguments.insert(
                "Strict".to_string(),
                ValidationCommandArgument {
                    name: "Strict".to_string(),
                    value: ValidationArgumentValue::Bool(true),
                    kind: ValidationArgumentKind::Bool,
                },
            );
        }
        if request.skip_ps_script_analyzer {
            arguments.insert(
                "SkipScriptAnalyzer".to_string(),
                ValidationCommandArgument {
                    name: "SkipScriptAnalyzer".to_string(),
                    value: ValidationArgumentValue::Bool(true),
                    kind: ValidationArgumentKind::Bool,
                },
            );
        }
    }

    arguments
}

fn apply_check_options(
    options: &Map<String, Value>,
    arguments: &mut BTreeMap<String, ValidationCommandArgument>,
    check_warning_only: &mut bool,
) {
    for (key, value) in options {
        if key == "WarningOnly" {
            *check_warning_only |= value.as_bool().unwrap_or(false);
            continue;
        }

        if let Some(argument) = option_to_command_argument(key, value) {
            arguments.insert(key.clone(), argument);
        }
    }
}

fn option_to_command_argument(key: &str, value: &Value) -> Option<ValidationCommandArgument> {
    match value {
        Value::Bool(boolean) => Some(ValidationCommandArgument {
            name: key.to_string(),
            value: ValidationArgumentValue::Bool(*boolean),
            kind: ValidationArgumentKind::Bool,
        }),
        Value::Number(number) => Some(ValidationCommandArgument {
            name: key.to_string(),
            value: ValidationArgumentValue::String(number.to_string()),
            kind: ValidationArgumentKind::String,
        }),
        Value::String(string) => Some(ValidationCommandArgument {
            name: key.to_string(),
            value: ValidationArgumentValue::String(string.clone()),
            kind: ValidationArgumentKind::String,
        }),
        _ => None,
    }
}

fn definition_supports_parameter(
    definition: &ValidationCheckDefinition,
    parameter_name: &str,
) -> bool {
    match definition.executor {
        ValidationCheckExecutor::Native(NativeValidationCheck::AgentHooks)
        | ValidationCheckExecutor::Native(NativeValidationCheck::AgentPermissions)
        | ValidationCheckExecutor::Native(NativeValidationCheck::Instructions)
        | ValidationCheckExecutor::Native(NativeValidationCheck::PlanningStructure)
        | ValidationCheckExecutor::Native(NativeValidationCheck::AuditLedger)
        | ValidationCheckExecutor::Native(NativeValidationCheck::ReadmeStandards)
        | ValidationCheckExecutor::Native(NativeValidationCheck::InstructionMetadata)
        | ValidationCheckExecutor::Native(NativeValidationCheck::RoutingCoverage)
        | ValidationCheckExecutor::Native(NativeValidationCheck::TemplateStandards) => {
            parameter_name == "WarningOnly"
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::AgentOrchestration) => false,
        ValidationCheckExecutor::Native(NativeValidationCheck::ArchitectureBoundaries) => {
            parameter_name == "BaselinePath"
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::CompatibilityLifecyclePolicy) => {
            matches!(
                parameter_name,
                "WarningOnly" | "CompatibilityPath" | "DetailedOutput"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::DotnetStandards) => {
            parameter_name == "TemplateDirectory"
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::PowerShellStandards) => {
            matches!(
                parameter_name,
                "WarningOnly"
                    | "ScriptsRoot"
                    | "IncludeAllScripts"
                    | "Strict"
                    | "SkipScriptAnalyzer"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::Policy) => false,
        ValidationCheckExecutor::Native(NativeValidationCheck::SecurityBaseline) => {
            matches!(parameter_name, "WarningOnly" | "BaselinePath")
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::SharedScriptChecksums) => {
            matches!(
                parameter_name,
                "WarningOnly" | "ManifestPath" | "DetailedOutput"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::SupplyChain) => {
            matches!(parameter_name, "WarningOnly" | "BaselinePath")
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::ReleaseGovernance) => {
            matches!(
                parameter_name,
                "WarningOnly"
                    | "ChangelogPath"
                    | "CodeownersPath"
                    | "GovernanceDocPath"
                    | "BranchProtectionBaselinePath"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::ReleaseProvenance) => {
            matches!(
                parameter_name,
                "WarningOnly" | "BaselinePath" | "AuditReportPath" | "RequireAuditReport"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::InstructionArchitecture) => {
            matches!(
                parameter_name,
                "WarningOnly"
                    | "ManifestPath"
                    | "AgentsPath"
                    | "GlobalInstructionsPath"
                    | "RoutingCatalogPath"
                    | "RoutePromptPath"
                    | "PromptRoot"
                    | "TemplateRoot"
                    | "SkillRoot"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::AgentSkillAlignment) => {
            matches!(
                parameter_name,
                "AgentManifestPath" | "PipelinePath" | "EvalFixturePath" | "SkillsRootPath"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::AuthoritativeSourcePolicy) => {
            matches!(
                parameter_name,
                "WarningOnly"
                    | "SourceMapPath"
                    | "InstructionPath"
                    | "AgentsPath"
                    | "GlobalInstructionsPath"
                    | "RoutingCatalogPath"
                    | "InstructionSearchRoot"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::WarningBaseline) => {
            matches!(
                parameter_name,
                "WarningOnly" | "BaselinePath" | "AnalyzerReportPath" | "ReportPath"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::RuntimeScriptTests) => {
            matches!(
                parameter_name,
                "WarningOnly" | "TestRoot" | "PowerShellPath"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::ShellHooks) => {
            matches!(
                parameter_name,
                "WarningOnly" | "HookRoot" | "ShellPath" | "EnableShellcheck" | "ShellcheckPath"
            )
        }
        ValidationCheckExecutor::Native(NativeValidationCheck::WorkspaceEfficiency) => {
            matches!(
                parameter_name,
                "WarningOnly" | "BaselinePath" | "SettingsTemplatePath" | "WorkspaceSearchRoot"
            )
        }
    }
}

fn run_validation_script(
    repo_root: &Path,
    definition: &ValidationCheckDefinition,
    arguments: BTreeMap<String, ValidationCommandArgument>,
    treat_failure_as_warning: bool,
) -> ValidationCheckResult {
    match definition.executor {
        ValidationCheckExecutor::Native(native_check) => run_native_validation_check(
            repo_root,
            definition.name,
            definition.script_label(),
            native_check,
            arguments,
            treat_failure_as_warning,
        ),
    }
}

fn run_native_validation_check(
    repo_root: &Path,
    name: &str,
    script_label: &str,
    native_check: NativeValidationCheck,
    arguments: BTreeMap<String, ValidationCommandArgument>,
    treat_failure_as_warning: bool,
) -> ValidationCheckResult {
    let started = Instant::now();
    let formatted_arguments = format_argument_list(&arguments);

    let (native_status, raw_exit_code, error) = match native_check {
        NativeValidationCheck::AgentHooks => {
            invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
                ..ValidateAgentHooksRequest::default()
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::AgentOrchestration => {
            invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
                repo_root: Some(repo_root.to_path_buf()),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::AgentPermissions => {
            invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
                ..ValidateAgentPermissionsRequest::default()
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::AgentSkillAlignment => {
            invoke_validate_agent_skill_alignment(&ValidateAgentSkillAlignmentRequest {
                repo_root: Some(repo_root.to_path_buf()),
                agent_manifest_path: string_argument_path(&arguments, "AgentManifestPath"),
                pipeline_path: string_argument_path(&arguments, "PipelinePath"),
                eval_fixture_path: string_argument_path(&arguments, "EvalFixturePath"),
                skills_root_path: string_argument_path(&arguments, "SkillsRootPath"),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::ArchitectureBoundaries => {
            invoke_validate_architecture_boundaries(&ValidateArchitectureBoundariesRequest {
                repo_root: Some(repo_root.to_path_buf()),
                baseline_path: string_argument_path(&arguments, "BaselinePath"),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::CompatibilityLifecyclePolicy => {
            invoke_validate_compatibility_lifecycle_policy(
                &ValidateCompatibilityLifecyclePolicyRequest {
                    repo_root: Some(repo_root.to_path_buf()),
                    compatibility_path: string_argument_path(&arguments, "CompatibilityPath"),
                    warning_only: bool_argument(&arguments, "WarningOnly")
                        .unwrap_or(treat_failure_as_warning),
                    detailed_output: bool_argument(&arguments, "DetailedOutput").unwrap_or(false),
                },
            )
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::DotnetStandards => {
            invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                template_directory: string_argument_path(&arguments, "TemplateDirectory"),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::PowerShellStandards => {
            invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                scripts_root: string_argument_path(&arguments, "ScriptsRoot"),
                include_all_scripts: bool_argument(&arguments, "IncludeAllScripts")
                    .unwrap_or(false),
                strict: bool_argument(&arguments, "Strict").unwrap_or(false),
                skip_script_analyzer: bool_argument(&arguments, "SkipScriptAnalyzer")
                    .unwrap_or(false),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::Policy => invoke_validate_policy(&ValidatePolicyRequest {
            repo_root: Some(repo_root.to_path_buf()),
            ..ValidatePolicyRequest::default()
        })
        .map(|result| {
            (
                result.status,
                result.exit_code,
                combine_native_messages(&result.failures, &result.warnings),
            )
        })
        .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string()))),
        NativeValidationCheck::SecurityBaseline => {
            invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
                repo_root: Some(repo_root.to_path_buf()),
                baseline_path: string_argument_path(&arguments, "BaselinePath"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::SharedScriptChecksums => {
            invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                manifest_path: string_argument_path(&arguments, "ManifestPath"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
                detailed_output: bool_argument(&arguments, "DetailedOutput").unwrap_or(false),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::SupplyChain => {
            invoke_validate_supply_chain(&ValidateSupplyChainRequest {
                repo_root: Some(repo_root.to_path_buf()),
                baseline_path: string_argument_path(&arguments, "BaselinePath"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::ReleaseGovernance => {
            invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
                repo_root: Some(repo_root.to_path_buf()),
                changelog_path: string_argument_path(&arguments, "ChangelogPath"),
                codeowners_path: string_argument_path(&arguments, "CodeownersPath"),
                governance_doc_path: string_argument_path(&arguments, "GovernanceDocPath"),
                branch_protection_baseline_path: string_argument_path(
                    &arguments,
                    "BranchProtectionBaselinePath",
                ),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::ReleaseProvenance => {
            invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
                repo_root: Some(repo_root.to_path_buf()),
                baseline_path: string_argument_path(&arguments, "BaselinePath"),
                audit_report_path: string_argument_path(&arguments, "AuditReportPath"),
                require_audit_report: bool_argument(&arguments, "RequireAuditReport")
                    .unwrap_or(false),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::Instructions => {
            invoke_validate_instructions(&ValidateInstructionsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::PlanningStructure => {
            invoke_validate_planning_structure(&ValidatePlanningStructureRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: treat_failure_as_warning,
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::AuditLedger => {
            invoke_validate_audit_ledger(&ValidateAuditLedgerRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: treat_failure_as_warning,
                ..ValidateAuditLedgerRequest::default()
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::ReadmeStandards => {
            invoke_validate_readme_standards(&ValidateReadmeStandardsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: treat_failure_as_warning,
                ..ValidateReadmeStandardsRequest::default()
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::InstructionMetadata => {
            invoke_validate_instruction_metadata(&ValidateInstructionMetadataRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: treat_failure_as_warning,
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::InstructionArchitecture => {
            invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
                repo_root: Some(repo_root.to_path_buf()),
                manifest_path: string_argument_path(&arguments, "ManifestPath"),
                agents_path: string_argument_path(&arguments, "AgentsPath"),
                global_instructions_path: string_argument_path(
                    &arguments,
                    "GlobalInstructionsPath",
                ),
                routing_catalog_path: string_argument_path(&arguments, "RoutingCatalogPath"),
                route_prompt_path: string_argument_path(&arguments, "RoutePromptPath"),
                prompt_root: string_argument_path(&arguments, "PromptRoot"),
                template_root: string_argument_path(&arguments, "TemplateRoot"),
                skill_root: string_argument_path(&arguments, "SkillRoot"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::RoutingCoverage => {
            invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: treat_failure_as_warning,
                ..ValidateRoutingCoverageRequest::default()
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::TemplateStandards => {
            invoke_validate_template_standards(&ValidateTemplateStandardsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                warning_only: treat_failure_as_warning,
                ..ValidateTemplateStandardsRequest::default()
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::AuthoritativeSourcePolicy => {
            invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
                repo_root: Some(repo_root.to_path_buf()),
                source_map_path: string_argument_path(&arguments, "SourceMapPath"),
                instruction_path: string_argument_path(&arguments, "InstructionPath"),
                agents_path: string_argument_path(&arguments, "AgentsPath"),
                global_instructions_path: string_argument_path(
                    &arguments,
                    "GlobalInstructionsPath",
                ),
                routing_catalog_path: string_argument_path(&arguments, "RoutingCatalogPath"),
                instruction_search_root: string_argument_path(&arguments, "InstructionSearchRoot"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::RuntimeScriptTests => {
            invoke_validate_runtime_script_tests(&ValidateRuntimeScriptTestsRequest {
                repo_root: Some(repo_root.to_path_buf()),
                test_root: string_argument_path(&arguments, "TestRoot"),
                powershell_path: string_argument_path(&arguments, "PowerShellPath"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::ShellHooks => {
            invoke_validate_shell_hooks(&ValidateShellHooksRequest {
                repo_root: Some(repo_root.to_path_buf()),
                hook_root: string_argument_path(&arguments, "HookRoot"),
                shell_path: string_argument_path(&arguments, "ShellPath"),
                shellcheck_path: string_argument_path(&arguments, "ShellcheckPath"),
                enable_shellcheck: bool_argument(&arguments, "EnableShellcheck").unwrap_or(false),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::WarningBaseline => {
            invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
                repo_root: Some(repo_root.to_path_buf()),
                baseline_path: string_argument_path(&arguments, "BaselinePath"),
                analyzer_report_path: string_argument_path(&arguments, "AnalyzerReportPath"),
                report_path: string_argument_path(&arguments, "ReportPath"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
        NativeValidationCheck::WorkspaceEfficiency => {
            invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
                repo_root: Some(repo_root.to_path_buf()),
                baseline_path: string_argument_path(&arguments, "BaselinePath"),
                settings_template_path: string_argument_path(&arguments, "SettingsTemplatePath"),
                workspace_search_root: string_argument_path(&arguments, "WorkspaceSearchRoot"),
                warning_only: bool_argument(&arguments, "WarningOnly")
                    .unwrap_or(treat_failure_as_warning),
            })
            .map(|result| {
                (
                    result.status,
                    result.exit_code,
                    combine_native_messages(&result.failures, &result.warnings),
                )
            })
            .unwrap_or_else(|error| (ValidationCheckStatus::Failed, 1, Some(error.to_string())))
        }
    };

    build_native_check_result(
        name,
        script_label,
        formatted_arguments,
        NativeCheckOutcome {
            native_status,
            treat_failure_as_warning,
            raw_exit_code,
            error,
            duration_ms: started.elapsed().as_millis(),
        },
    )
}

fn string_argument_path(
    arguments: &BTreeMap<String, ValidationCommandArgument>,
    name: &str,
) -> Option<PathBuf> {
    arguments
        .get(name)
        .and_then(|argument| match &argument.value {
            ValidationArgumentValue::String(value) => Some(PathBuf::from(value)),
            ValidationArgumentValue::Bool(_) => None,
        })
}

fn bool_argument(
    arguments: &BTreeMap<String, ValidationCommandArgument>,
    name: &str,
) -> Option<bool> {
    arguments
        .get(name)
        .and_then(|argument| match argument.value {
            ValidationArgumentValue::Bool(value) => Some(value),
            ValidationArgumentValue::String(_) => None,
        })
}

fn combine_native_messages(failures: &[String], warnings: &[String]) -> Option<String> {
    if !failures.is_empty() {
        Some(failures.join(" | "))
    } else if !warnings.is_empty() {
        Some(warnings.join(" | "))
    } else {
        None
    }
}

fn format_argument_list(arguments: &BTreeMap<String, ValidationCommandArgument>) -> Vec<String> {
    arguments
        .values()
        .map(|argument| match &argument.value {
            ValidationArgumentValue::String(value) => format!("-{}={value}", argument.name),
            ValidationArgumentValue::Bool(value) => format!("-{}={value}", argument.name),
        })
        .collect()
}

struct NativeCheckOutcome {
    native_status: ValidationCheckStatus,
    treat_failure_as_warning: bool,
    raw_exit_code: i32,
    error: Option<String>,
    duration_ms: u128,
}

fn build_native_check_result(
    name: &str,
    script: &str,
    arguments: Vec<String>,
    outcome: NativeCheckOutcome,
) -> ValidationCheckResult {
    let (status, exit_code) = match outcome.native_status {
        ValidationCheckStatus::Passed => (ValidationCheckStatus::Passed, 0),
        ValidationCheckStatus::Warning => (ValidationCheckStatus::Warning, 0),
        ValidationCheckStatus::Failed if outcome.treat_failure_as_warning => {
            (ValidationCheckStatus::Warning, 0)
        }
        ValidationCheckStatus::Failed => (
            ValidationCheckStatus::Failed,
            if outcome.raw_exit_code == 0 {
                1
            } else {
                outcome.raw_exit_code
            },
        ),
    };

    ValidationCheckResult {
        name: name.to_string(),
        script: script.to_string(),
        arguments,
        status,
        exit_code,
        duration_ms: outcome.duration_ms,
        error: outcome.error,
    }
}

fn render_validation_report(
    repo_root: &Path,
    profile_id: &str,
    warning_only: bool,
    checks: &[ValidationCheckResult],
    suite_warnings: &[String],
    failed_total: usize,
) -> String {
    let duration_sum = checks.iter().map(|check| check.duration_ms).sum::<u128>();
    let average_check_duration_ms = if checks.is_empty() {
        0.0
    } else {
        duration_sum as f64 / checks.len() as f64
    };
    let mut slowest_checks = checks.to_vec();
    slowest_checks.sort_by(|left, right| right.duration_ms.cmp(&left.duration_ms));
    let slowest_checks = slowest_checks
        .into_iter()
        .take(5)
        .map(|check| {
            json!({
                "name": check.name,
                "durationMs": check.duration_ms,
                "status": check.status.as_str(),
            })
        })
        .collect::<Vec<_>>();

    serde_json::to_string_pretty(&json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string(),
        "profile": profile_id,
        "warningOnly": warning_only,
        "repoRoot": repo_root.display().to_string(),
        "summary": {
            "totalChecks": checks.len(),
            "passed": checks.iter().filter(|check| check.status == ValidationCheckStatus::Passed).count(),
            "warnings": checks.iter().filter(|check| check.status == ValidationCheckStatus::Warning).count(),
            "failed": failed_total,
            "suiteWarnings": suite_warnings.len(),
        },
        "performance": {
            "totalDurationMs": duration_sum,
            "averageCheckDurationMs": average_check_duration_ms,
            "slowestChecks": slowest_checks,
        },
        "checks": checks.iter().map(|check| {
            json!({
                "name": check.name,
                "script": check.script,
                "status": check.status.as_str(),
                "exitCode": check.exit_code,
                "durationMs": check.duration_ms,
                "error": check.error,
            })
        }).collect::<Vec<_>>(),
        "suiteWarningMessages": suite_warnings,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

fn write_validation_report(output_path: &Path, report_json: &str) -> anyhow::Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    fs::write(output_path, report_json)
        .with_context(|| format!("failed to write '{}'", output_path.display()))
}

fn repair_validation_ledger_if_needed(
    repo_root: &Path,
    ledger_path: &Path,
    suite_warnings: &mut Vec<String>,
) -> Option<PathBuf> {
    let reason = validate_ledger_chain(ledger_path)?;
    let timestamp_token = current_timestamp_string();
    let archive_path = ledger_path.with_file_name(format!(
        "{}.broken-{}{}",
        ledger_path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("validation-ledger"),
        timestamp_token,
        ledger_path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| format!(".{extension}"))
            .unwrap_or_default()
    ));

    if let Some(parent) = archive_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::rename(ledger_path, &archive_path);

    let latest_path = repo_root.join(".temp/audit/validation-ledger.latest.json");
    if latest_path.is_file() {
        let latest_archive_path = repo_root.join(format!(
            ".temp/audit/validation-ledger.latest.broken-{timestamp_token}.json"
        ));
        if let Some(parent) = latest_archive_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::rename(&latest_path, latest_archive_path);
    }

    suite_warnings.push(format!(
        "Archived broken validation ledger and started a new chain: {} ({reason})",
        archive_path
            .strip_prefix(repo_root)
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| archive_path.display().to_string())
    ));

    Some(archive_path)
}

fn validate_ledger_chain(ledger_path: &Path) -> Option<String> {
    if !ledger_path.is_file() {
        return None;
    }

    let document = match fs::read_to_string(ledger_path) {
        Ok(document) => document,
        Err(error) => {
            return Some(format!("could not read ledger: {error}"));
        }
    };
    let mut previous_hash = ZERO_LEDGER_HASH.to_string();
    for (index, line) in document.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let entry: Value = match serde_json::from_str(line) {
            Ok(entry) => entry,
            Err(_) => return Some(format!("line {} is not valid JSON", index + 1)),
        };
        let payload_json = match entry.get("payloadJson").and_then(Value::as_str) {
            Some(value) => value.to_string(),
            None => return Some(format!("line {} is missing payloadJson", index + 1)),
        };
        let payload_hash = match entry.get("payloadHash").and_then(Value::as_str) {
            Some(value) => value.to_string(),
            None => return Some(format!("line {} is missing payloadHash", index + 1)),
        };
        let prev_hash = match entry.get("prevHash").and_then(Value::as_str) {
            Some(value) => value.to_string(),
            None => return Some(format!("line {} is missing prevHash", index + 1)),
        };
        let entry_hash = match entry.get("entryHash").and_then(Value::as_str) {
            Some(value) => value.to_string(),
            None => return Some(format!("line {} is missing entryHash", index + 1)),
        };

        if prev_hash != previous_hash {
            return Some(format!("chain break at line {}", index + 1));
        }

        let computed_payload_hash = sha256_hex(&payload_json);
        if computed_payload_hash != payload_hash {
            return Some(format!("payload hash mismatch at line {}", index + 1));
        }

        let computed_entry_hash = sha256_hex(&format!("{prev_hash}|{payload_hash}"));
        if computed_entry_hash != entry_hash {
            return Some(format!("entry hash mismatch at line {}", index + 1));
        }

        previous_hash = entry_hash;
    }

    None
}

fn write_validation_ledger(
    repo_root: &Path,
    ledger_path: &Path,
    profile_id: &str,
    warning_only: bool,
    checks: &[ValidationCheckResult],
) -> anyhow::Result<()> {
    if let Some(parent) = ledger_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    let previous_hash =
        last_ledger_hash(ledger_path).unwrap_or_else(|| ZERO_LEDGER_HASH.to_string());
    let payload_json = serde_json::to_string(&json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string(),
        "profile": profile_id,
        "warningOnly": warning_only,
        "git": git_state_to_json(&get_git_state(repo_root)),
        "summary": {
            "totalChecks": checks.len(),
            "passed": checks.iter().filter(|check| check.status == ValidationCheckStatus::Passed).count(),
            "warnings": checks.iter().filter(|check| check.status == ValidationCheckStatus::Warning).count(),
            "failed": checks.iter().filter(|check| check.status == ValidationCheckStatus::Failed).count(),
        },
        "checks": checks.iter().map(|check| {
            json!({
                "name": check.name,
                "status": check.status.as_str(),
                "exitCode": check.exit_code,
                "durationMs": check.duration_ms,
            })
        }).collect::<Vec<_>>(),
    }))?;
    let payload_hash = sha256_hex(&payload_json);
    let entry_hash = sha256_hex(&format!("{previous_hash}|{payload_hash}"));
    let ledger_entry = serde_json::to_string(&json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string(),
        "profile": profile_id,
        "warningOnly": warning_only,
        "prevHash": previous_hash,
        "payloadHash": payload_hash,
        "entryHash": entry_hash,
        "payloadJson": payload_json,
    }))?;

    let mut existing_lines = if ledger_path.is_file() {
        fs::read_to_string(ledger_path)
            .with_context(|| format!("failed to read '{}'", ledger_path.display()))?
    } else {
        String::new()
    };
    if !existing_lines.is_empty() && !existing_lines.ends_with('\n') {
        existing_lines.push('\n');
    }
    existing_lines.push_str(&ledger_entry);
    existing_lines.push('\n');
    fs::write(ledger_path, existing_lines)
        .with_context(|| format!("failed to write '{}'", ledger_path.display()))?;

    let latest_path = repo_root.join(".temp/audit/validation-ledger.latest.json");
    if let Some(parent) = latest_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    let pretty_entry =
        serde_json::to_string_pretty(&serde_json::from_str::<Value>(&ledger_entry)?)?;
    fs::write(&latest_path, pretty_entry)
        .with_context(|| format!("failed to write '{}'", latest_path.display()))?;

    Ok(())
}

fn last_ledger_hash(ledger_path: &Path) -> Option<String> {
    let document = fs::read_to_string(ledger_path).ok()?;
    for line in document.lines().rev() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: Value = serde_json::from_str(line).ok()?;
        let hash = entry.get("entryHash")?.as_str()?.to_string();
        if !hash.trim().is_empty() {
            return Some(hash);
        }
    }

    None
}

fn get_git_state(repo_root: &Path) -> GitState {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();
    let Ok(branch_output) = output else {
        return GitState {
            available: false,
            branch: None,
            commit: None,
            is_dirty: None,
        };
    };
    if !branch_output.status.success() {
        return GitState {
            available: false,
            branch: None,
            commit: None,
            is_dirty: None,
        };
    }

    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();
    let commit_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok();
    let status_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("status")
        .arg("--porcelain")
        .output()
        .ok();

    GitState {
        available: true,
        branch: (!branch.is_empty()).then_some(branch),
        commit: commit_output.and_then(|output| {
            let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
            (!commit.is_empty()).then_some(commit)
        }),
        is_dirty: status_output
            .map(|output| !String::from_utf8_lossy(&output.stdout).trim().is_empty()),
    }
}

fn git_state_to_json(git_state: &GitState) -> Value {
    json!({
        "available": git_state.available,
        "branch": git_state.branch,
        "commit": git_state.commit,
        "isDirty": git_state.is_dirty,
    })
}

fn sha256_hex(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(&mut hex, "{byte:02x}");
    }
    hex
}

fn current_timestamp_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}