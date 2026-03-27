//! Validation command boundary contracts for the migration program.

pub mod contracts;
pub mod agent_orchestration;
pub mod architecture;
pub mod documentation;
pub mod error;
pub mod evidence;
pub mod governance;
pub mod instruction_graph;
pub mod orchestration;
pub mod operational_hygiene;
pub mod policy;
pub mod release;
pub mod security;
pub mod standards;
pub mod structure;
pub mod workspace;

pub use contracts::{
    validation_surface_contract, validation_surface_script_total, MigrationWave,
    ValidationSurfaceContract, ValidationSurfaceKind, VALIDATION_SURFACE_CONTRACTS,
};
pub use agent_orchestration::{
    invoke_validate_agent_hooks, invoke_validate_agent_orchestration,
    invoke_validate_agent_permissions,
    invoke_validate_agent_skill_alignment,
    ValidateAgentHooksRequest, ValidateAgentHooksResult,
    ValidateAgentOrchestrationRequest, ValidateAgentOrchestrationResult,
    ValidateAgentPermissionsRequest, ValidateAgentPermissionsResult,
    ValidateAgentSkillAlignmentRequest,
    ValidateAgentSkillAlignmentResult,
};
pub use architecture::{
    invoke_validate_architecture_boundaries, ValidateArchitectureBoundariesRequest,
    ValidateArchitectureBoundariesResult,
};
pub use documentation::{
    invoke_validate_instruction_metadata, invoke_validate_readme_standards,
    ValidateInstructionMetadataRequest, ValidateInstructionMetadataResult,
    ValidateReadmeStandardsRequest, ValidateReadmeStandardsResult,
};
pub use error::{
    ValidateAgentHooksCommandError, ValidateAgentPermissionsCommandError,
    ValidateAgentSkillAlignmentCommandError,
    ValidateAgentOrchestrationCommandError,
    ValidateArchitectureBoundariesCommandError,
    ValidateCompatibilityLifecyclePolicyCommandError,
    ValidateDotnetStandardsCommandError,
    ValidatePolicyCommandError,
    ValidateSecurityBaselineCommandError,
    ValidateSharedScriptChecksumsCommandError,
    ValidateSupplyChainCommandError,
    ValidateAllCommandError, ValidateAuditLedgerCommandError,
    ValidateAuthoritativeSourcePolicyCommandError, ValidateInstructionArchitectureCommandError,
    ValidateInstructionMetadataCommandError, ValidateInstructionsCommandError,
    ValidatePlanningStructureCommandError, ValidateReadmeStandardsCommandError,
    ValidateReleaseGovernanceCommandError, ValidateReleaseProvenanceCommandError,
    ValidateRuntimeScriptTestsCommandError, ValidateShellHooksCommandError,
    ValidateRoutingCoverageCommandError, ValidateTemplateStandardsCommandError,
    ValidateWarningBaselineCommandError, ValidateWorkspaceEfficiencyCommandError,
    ValidationSurfaceError,
};
pub use evidence::{
    invoke_validate_audit_ledger, ValidateAuditLedgerRequest, ValidateAuditLedgerResult,
};
pub use governance::{
    invoke_validate_routing_coverage, invoke_validate_template_standards,
    ValidateRoutingCoverageRequest, ValidateRoutingCoverageResult,
    ValidateTemplateStandardsRequest, ValidateTemplateStandardsResult,
};
pub use instruction_graph::{
    invoke_validate_authoritative_source_policy, invoke_validate_instruction_architecture,
    invoke_validate_instructions,
    ValidateAuthoritativeSourcePolicyRequest, ValidateAuthoritativeSourcePolicyResult,
    ValidateInstructionArchitectureRequest, ValidateInstructionArchitectureResult,
    ValidateInstructionsRequest, ValidateInstructionsResult,
};
pub use orchestration::{
    invoke_validate_all, ValidateAllRequest, ValidateAllResult, ValidationCheckResult,
    ValidationCheckStatus,
};
pub use operational_hygiene::{
    invoke_validate_runtime_script_tests, invoke_validate_shell_hooks,
    invoke_validate_warning_baseline,
    ValidateRuntimeScriptTestsRequest, ValidateRuntimeScriptTestsResult,
    ValidateShellHooksRequest, ValidateShellHooksResult, ValidateWarningBaselineRequest,
    ValidateWarningBaselineResult,
};
pub use policy::{
    invoke_validate_compatibility_lifecycle_policy, invoke_validate_policy,
    ValidateCompatibilityLifecyclePolicyRequest, ValidateCompatibilityLifecyclePolicyResult,
    ValidatePolicyRequest, ValidatePolicyResult,
};
pub use release::{
    invoke_validate_release_governance, ValidateReleaseGovernanceRequest,
    ValidateReleaseGovernanceResult, invoke_validate_release_provenance,
    ValidateReleaseProvenanceRequest, ValidateReleaseProvenanceResult,
};
pub use security::{
    invoke_validate_security_baseline, invoke_validate_shared_script_checksums,
    invoke_validate_supply_chain,
    ValidateSecurityBaselineRequest, ValidateSecurityBaselineResult,
    ValidateSharedScriptChecksumsRequest, ValidateSharedScriptChecksumsResult,
    ValidateSupplyChainRequest, ValidateSupplyChainResult,
};
pub use standards::{
    invoke_validate_dotnet_standards, ValidateDotnetStandardsRequest,
    ValidateDotnetStandardsResult,
};
pub use structure::{
    invoke_validate_planning_structure, ValidatePlanningStructureRequest,
    ValidatePlanningStructureResult,
};
pub use workspace::{
    invoke_validate_workspace_efficiency, ValidateWorkspaceEfficiencyRequest,
    ValidateWorkspaceEfficiencyResult,
};

/// Require a registered validation surface contract.
///
/// # Errors
///
/// Returns [`ValidationSurfaceError::UnknownSurface`] when the requested
/// surface is not registered in [`VALIDATION_SURFACE_CONTRACTS`].
pub fn require_validation_surface_contract(
    surface_id: &str,
) -> Result<&'static ValidationSurfaceContract, ValidationSurfaceError> {
    validation_surface_contract(surface_id).ok_or_else(|| ValidationSurfaceError::UnknownSurface {
        surface_id: surface_id.to_string(),
    })
}