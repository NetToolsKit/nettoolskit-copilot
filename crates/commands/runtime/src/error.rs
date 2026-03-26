//! Error types for runtime surface lookup.

use thiserror::Error;

/// Errors raised by runtime surface resolution.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RuntimeSurfaceError {
    /// Requested runtime surface is not registered.
    #[error("unknown runtime surface contract: {surface_id}")]
    UnknownSurface {
        /// Runtime surface identifier requested by the caller.
        surface_id: String,
    },
}