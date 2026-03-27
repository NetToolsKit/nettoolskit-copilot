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

/// Errors raised by authoritative-source-policy validation commands.
#[derive(Debug, Error)]
pub enum ValidateAuthoritativeSourcePolicyCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve authoritative source policy workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by instruction-architecture validation commands.
#[derive(Debug, Error)]
pub enum ValidateInstructionArchitectureCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve instruction architecture workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by top-level instruction validation commands.
#[derive(Debug, Error)]
pub enum ValidateInstructionsCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve instruction validation workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by warning-baseline validation commands.
#[derive(Debug, Error)]
pub enum ValidateWarningBaselineCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve warning baseline workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime-script-tests validation commands.
#[derive(Debug, Error)]
pub enum ValidateRuntimeScriptTestsCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime script test workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by shell-hooks validation commands.
#[derive(Debug, Error)]
pub enum ValidateShellHooksCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve shell hooks workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by agent-hooks validation commands.
#[derive(Debug, Error)]
pub enum ValidateAgentHooksCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve agent hooks workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by agent-permissions validation commands.
#[derive(Debug, Error)]
pub enum ValidateAgentPermissionsCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve agent permissions workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by workspace-efficiency validation commands.
#[derive(Debug, Error)]
pub enum ValidateWorkspaceEfficiencyCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve workspace efficiency workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
}