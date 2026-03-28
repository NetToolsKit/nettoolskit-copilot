//! Runtime command boundary contracts for the migration program.

pub mod continuity;
pub mod contracts;
pub mod diagnostics;
pub mod error;
pub mod hooks;
pub mod maintenance;
pub mod sync;

pub use continuity::local_context::{
    query_local_context_index, update_local_context_index, QueryLocalContextIndexRequest,
    QueryLocalContextIndexResult, UpdateLocalContextIndexRequest, UpdateLocalContextIndexResult,
};
pub use continuity::planning_summary::{
    export_planning_summary, ExportPlanningSummaryRequest, ExportPlanningSummaryResult,
};
pub use contracts::{
    runtime_surface_contract, runtime_surface_script_total, MigrationWave, RuntimeSurfaceContract,
    RuntimeSurfaceKind, RUNTIME_SURFACE_CONTRACTS,
};
pub use diagnostics::doctor::{
    invoke_runtime_doctor, RuntimeDoctorMappingReport, RuntimeDoctorRequest, RuntimeDoctorResult,
    RuntimeDoctorStatus,
};
pub use diagnostics::healthcheck::{
    invoke_runtime_healthcheck, RuntimeHealthcheckCheckResult, RuntimeHealthcheckRequest,
    RuntimeHealthcheckResult, RuntimeHealthcheckStatus,
};
pub use diagnostics::self_heal::{
    invoke_runtime_self_heal, RuntimeSelfHealRequest, RuntimeSelfHealResult, RuntimeSelfHealStatus,
    RuntimeSelfHealStepResult,
};
pub use error::{
    LocalContextCommandError, PlanningSummaryCommandError, RuntimeApplyVscodeTemplatesCommandError,
    RuntimeBootstrapCommandError, RuntimeCleanBuildArtifactsCommandError,
    RuntimeDoctorCommandError, RuntimeHealthcheckCommandError,
    RuntimePreCommitEofHygieneCommandError, RuntimeSelfHealCommandError,
    RuntimeSetupGitHooksCommandError, RuntimeSetupGlobalGitAliasesCommandError,
    RuntimeSurfaceError, RuntimeTrimTrailingBlankLinesCommandError,
};
pub use hooks::pre_commit_eof_hygiene::{
    invoke_pre_commit_eof_hygiene, RuntimePreCommitEofHygieneRequest,
    RuntimePreCommitEofHygieneResult, RuntimePreCommitEofHygieneStatus,
};
pub use hooks::setup_git_hooks::{
    invoke_setup_git_hooks, RuntimeSetupGitHooksRequest, RuntimeSetupGitHooksResult,
};
pub use hooks::setup_global_git_aliases::{
    invoke_setup_global_git_aliases, RuntimeSetupGlobalGitAliasesRequest,
    RuntimeSetupGlobalGitAliasesResult,
};
pub use maintenance::clean_build_artifacts::{
    invoke_clean_build_artifacts, RuntimeCleanBuildArtifactsRequest,
    RuntimeCleanBuildArtifactsResult, RuntimeCleanBuildArtifactsStatus,
};
pub use maintenance::trim_trailing_blank_lines::{
    invoke_trim_trailing_blank_lines, RuntimeTrimTrailingBlankLinesRequest,
    RuntimeTrimTrailingBlankLinesResult, RuntimeTrimTrailingBlankLinesStatus,
};
pub use sync::apply_vscode_templates::{
    invoke_apply_vscode_templates, RuntimeApplyVscodeTemplateFileResult,
    RuntimeApplyVscodeTemplatesRequest, RuntimeApplyVscodeTemplatesResult,
};
pub use sync::bootstrap::{
    invoke_runtime_bootstrap, RuntimeBootstrapRequest, RuntimeBootstrapResult,
};

/// Require a registered runtime surface contract.
///
/// # Errors
///
/// Returns [`RuntimeSurfaceError::UnknownSurface`] when the requested surface
/// is not registered in [`RUNTIME_SURFACE_CONTRACTS`].
pub fn require_runtime_surface_contract(
    surface_id: &str,
) -> Result<&'static RuntimeSurfaceContract, RuntimeSurfaceError> {
    runtime_surface_contract(surface_id).ok_or_else(|| RuntimeSurfaceError::UnknownSurface {
        surface_id: surface_id.to_string(),
    })
}
