//! Validation command boundary contracts for the migration program.

pub mod contracts;
pub mod error;
pub mod orchestration;

pub use contracts::{
    validation_surface_contract, validation_surface_script_total, MigrationWave,
    ValidationSurfaceContract, ValidationSurfaceKind, VALIDATION_SURFACE_CONTRACTS,
};
pub use error::{ValidateAllCommandError, ValidationSurfaceError};
pub use orchestration::{
    invoke_validate_all, ValidateAllRequest, ValidateAllResult, ValidationCheckResult,
    ValidationCheckStatus,
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