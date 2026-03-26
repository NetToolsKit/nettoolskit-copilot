//! Runtime command boundary contracts for the migration program.

pub mod bootstrap;
pub mod contracts;
pub mod doctor;
pub mod error;
pub mod healthcheck;
pub mod local_context;
pub mod planning_summary;
pub mod self_heal;

pub use bootstrap::{invoke_runtime_bootstrap, RuntimeBootstrapRequest, RuntimeBootstrapResult};
pub use contracts::{
    runtime_surface_contract, runtime_surface_script_total, MigrationWave, RuntimeSurfaceContract,
    RuntimeSurfaceKind, RUNTIME_SURFACE_CONTRACTS,
};
pub use doctor::{
    invoke_runtime_doctor, RuntimeDoctorMappingReport, RuntimeDoctorRequest, RuntimeDoctorResult,
    RuntimeDoctorStatus,
};
pub use error::{
    LocalContextCommandError, PlanningSummaryCommandError, RuntimeBootstrapCommandError,
    RuntimeDoctorCommandError, RuntimeHealthcheckCommandError, RuntimeSelfHealCommandError,
    RuntimeSurfaceError,
};
pub use healthcheck::{
    invoke_runtime_healthcheck, RuntimeHealthcheckCheckResult, RuntimeHealthcheckRequest,
    RuntimeHealthcheckResult, RuntimeHealthcheckStatus,
};
pub use local_context::{
    query_local_context_index, update_local_context_index, QueryLocalContextIndexRequest,
    QueryLocalContextIndexResult, UpdateLocalContextIndexRequest, UpdateLocalContextIndexResult,
};
pub use planning_summary::{
    export_planning_summary, ExportPlanningSummaryRequest, ExportPlanningSummaryResult,
};
pub use self_heal::{
    invoke_runtime_self_heal, RuntimeSelfHealRequest, RuntimeSelfHealResult, RuntimeSelfHealStatus,
    RuntimeSelfHealStepResult,
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