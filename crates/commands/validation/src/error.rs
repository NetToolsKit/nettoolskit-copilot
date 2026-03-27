//! Error types for validation surface lookup.

use anyhow::Error as AnyhowError;
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

/// Errors raised by validation orchestration commands.
#[derive(Debug, Error)]
pub enum ValidateAllCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve validation workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}