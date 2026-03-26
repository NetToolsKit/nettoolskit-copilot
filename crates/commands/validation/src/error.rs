//! Error types for validation surface lookup.

use thiserror::Error;

/// Errors raised by validation surface resolution.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationSurfaceError {
    /// Requested validation surface is not registered.
    #[error("unknown validation surface contract: {surface_id}")]
    UnknownSurface {
        /// Validation surface identifier requested by the caller.
        surface_id: String,
    },
}