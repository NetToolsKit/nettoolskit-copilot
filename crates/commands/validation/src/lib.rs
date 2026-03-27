//! Validation command boundary contracts for the migration program.

pub mod contracts;
pub mod documentation;
pub mod error;
pub mod evidence;
pub mod governance;
pub mod instruction_graph;
pub mod orchestration;
pub mod structure;
pub mod workspace;

pub use contracts::{
    validation_surface_contract, validation_surface_script_total, MigrationWave,
    ValidationSurfaceContract, ValidationSurfaceKind, VALIDATION_SURFACE_CONTRACTS,
};
pub use documentation::{
    invoke_validate_instruction_metadata, invoke_validate_readme_standards,
    ValidateInstructionMetadataRequest, ValidateInstructionMetadataResult,
    ValidateReadmeStandardsRequest, ValidateReadmeStandardsResult,
};
pub use error::{
    ValidateAllCommandError, ValidateAuditLedgerCommandError,
    ValidateAuthoritativeSourcePolicyCommandError, ValidateInstructionMetadataCommandError,
    ValidatePlanningStructureCommandError, ValidateReadmeStandardsCommandError,
    ValidateRoutingCoverageCommandError, ValidateTemplateStandardsCommandError,
    ValidateWorkspaceEfficiencyCommandError, ValidationSurfaceError,
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
    invoke_validate_authoritative_source_policy, ValidateAuthoritativeSourcePolicyRequest,
    ValidateAuthoritativeSourcePolicyResult,
};
pub use orchestration::{
    invoke_validate_all, ValidateAllRequest, ValidateAllResult, ValidationCheckResult,
    ValidationCheckStatus,
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