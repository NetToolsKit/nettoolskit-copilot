//! Runtime command boundary contracts for the migration program.

pub mod contracts;
pub mod doctor;
pub mod error;
pub mod local_context;
pub mod planning_summary;

pub use contracts::{
    runtime_surface_contract, runtime_surface_script_total, MigrationWave, RuntimeSurfaceContract,
    RuntimeSurfaceKind, RUNTIME_SURFACE_CONTRACTS,
};
pub use doctor::{
    invoke_runtime_doctor, RuntimeDoctorMappingReport, RuntimeDoctorRequest, RuntimeDoctorResult,
    RuntimeDoctorStatus,
};
pub use error::{
    LocalContextCommandError, PlanningSummaryCommandError, RuntimeDoctorCommandError,
    RuntimeSurfaceError,
};
pub use local_context::{
    query_local_context_index, update_local_context_index, QueryLocalContextIndexRequest,
    QueryLocalContextIndexResult, UpdateLocalContextIndexRequest, UpdateLocalContextIndexResult,
};
pub use planning_summary::{
    export_planning_summary, ExportPlanningSummaryRequest, ExportPlanningSummaryResult,
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