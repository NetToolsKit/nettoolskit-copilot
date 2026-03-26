//! Error types for runtime surface lookup.

use anyhow::Error as AnyhowError;
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

/// Errors raised by runtime local-context commands.
#[derive(Debug, Error)]
pub enum LocalContextCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve local context workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Catalog loading failed.
    #[error("failed to read local context catalog")]
    ReadCatalog {
        /// Underlying catalog loading failure.
        #[source]
        source: AnyhowError,
    },
    /// Index build failed.
    #[error("failed to build local context index")]
    BuildIndex {
        /// Underlying build failure.
        #[source]
        source: AnyhowError,
    },
    /// Query text was empty.
    #[error("empty local context query is not allowed")]
    EmptyQuery,
    /// The persisted index document does not exist yet.
    #[error("local context index not found: {index_path}")]
    IndexNotFound {
        /// Resolved index document path expected by the command.
        index_path: String,
    },
    /// Persisted index loading failed.
    #[error("failed to read local context index")]
    ReadIndex {
        /// Underlying document loading failure.
        #[source]
        source: AnyhowError,
    },
}