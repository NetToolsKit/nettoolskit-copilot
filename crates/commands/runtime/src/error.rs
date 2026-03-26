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

/// Errors raised by runtime planning-summary commands.
#[derive(Debug, Error)]
pub enum PlanningSummaryCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve planning summary workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Output document rendering failed.
    #[error("failed to render planning summary document")]
    RenderDocument {
        /// Underlying rendering failure.
        #[source]
        source: AnyhowError,
    },
    /// Output path creation or write failed.
    #[error("failed to write planning summary output")]
    WriteOutput {
        /// Underlying I/O failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime doctor commands.
#[derive(Debug, Error)]
pub enum RuntimeDoctorCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime doctor workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Runtime execution context resolution failed.
    #[error("failed to resolve runtime doctor execution context")]
    ResolveExecutionContext {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Drift report construction failed.
    #[error("failed to build runtime doctor report")]
    BuildReport {
        /// Underlying inventory or hashing failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime healthcheck commands.
#[derive(Debug, Error)]
pub enum RuntimeHealthcheckCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime healthcheck workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Runtime execution context resolution failed.
    #[error("failed to resolve runtime healthcheck execution context")]
    ResolveExecutionContext {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Output/log artifact preparation failed.
    #[error("failed to prepare runtime healthcheck artifacts")]
    PrepareArtifacts {
        /// Underlying filesystem or path resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Report or log writing failed.
    #[error("failed to write runtime healthcheck output")]
    WriteOutput {
        /// Underlying serialization or I/O failure.
        #[source]
        source: AnyhowError,
    },
}