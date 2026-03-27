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

/// Errors raised by planning-structure validation commands.
#[derive(Debug, Error)]
pub enum ValidatePlanningStructureCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve planning structure workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by audit-ledger validation commands.
#[derive(Debug, Error)]
pub enum ValidateAuditLedgerCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve audit ledger workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by README standards validation commands.
#[derive(Debug, Error)]
pub enum ValidateReadmeStandardsCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve readme standards workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by instruction metadata validation commands.
#[derive(Debug, Error)]
pub enum ValidateInstructionMetadataCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve instruction metadata workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by routing coverage validation commands.
#[derive(Debug, Error)]
pub enum ValidateRoutingCoverageCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve routing coverage workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by template standards validation commands.
#[derive(Debug, Error)]
pub enum ValidateTemplateStandardsCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve template standards workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}