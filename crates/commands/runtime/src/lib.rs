//! Runtime command boundary contracts for the migration program.

pub mod contracts;
pub mod error;
pub mod local_context;

pub use contracts::{
    runtime_surface_contract, runtime_surface_script_total, MigrationWave, RuntimeSurfaceContract,
    RuntimeSurfaceKind, RUNTIME_SURFACE_CONTRACTS,
};
pub use error::{LocalContextCommandError, RuntimeSurfaceError};
pub use local_context::{
    query_local_context_index, update_local_context_index, QueryLocalContextIndexRequest,
    QueryLocalContextIndexResult, UpdateLocalContextIndexRequest, UpdateLocalContextIndexResult,
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