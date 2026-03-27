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
    /// Drift remediation sync failed.
    #[error("failed to synchronize runtime doctor drift remediation")]
    SynchronizeRuntime {
        /// Underlying runtime bootstrap failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime bootstrap commands.
#[derive(Debug, Error)]
pub enum RuntimeBootstrapCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime bootstrap workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Runtime execution context resolution failed.
    #[error("failed to resolve runtime bootstrap execution context")]
    ResolveExecutionContext {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Provider surface rendering failed.
    #[error("failed to render runtime bootstrap provider surfaces")]
    RenderProviderSurfaces {
        /// Underlying render failure.
        #[source]
        source: AnyhowError,
    },
    /// Runtime file synchronization failed.
    #[error("failed to synchronize runtime bootstrap assets")]
    SyncAssets {
        /// Underlying filesystem failure.
        #[source]
        source: AnyhowError,
    },
    /// MCP configuration application failed.
    #[error("failed to apply runtime bootstrap MCP configuration")]
    ApplyMcpConfig {
        /// Underlying delegate or validation failure.
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

/// Errors raised by runtime VS Code template apply commands.
#[derive(Debug, Error)]
pub enum RuntimeApplyVscodeTemplatesCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime vscode template workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// VS Code workspace path resolution failed.
    #[error("failed to resolve runtime vscode template path")]
    ResolveVscodePath {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Target VS Code path does not exist.
    #[error("VS Code path not found: {vscode_path}")]
    VscodePathNotFound {
        /// Resolved VS Code path expected by the command.
        vscode_path: String,
    },
    /// Template application failed.
    #[error("failed to apply runtime vscode templates")]
    ApplyTemplates {
        /// Underlying filesystem failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime global Git alias commands.
#[derive(Debug, Error)]
pub enum RuntimeSetupGlobalGitAliasesCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime global git alias workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Target Codex runtime path resolution failed.
    #[error("failed to resolve runtime global git alias codex path")]
    ResolveTargetCodexPath {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Required runtime-synced trim script was missing.
    #[error("runtime global git alias trim script not found: {trim_script_path}")]
    TrimScriptNotFound {
        /// Resolved trim script path expected by the command.
        trim_script_path: String,
    },
    /// The isolated global Git config path could not be prepared.
    #[error("failed to prepare runtime global git alias config path")]
    PrepareGitConfigPath {
        /// Underlying filesystem failure.
        #[source]
        source: AnyhowError,
    },
    /// Git alias configuration failed.
    #[error("failed to configure runtime global git aliases")]
    ConfigureAliases {
        /// Underlying command failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime pre-commit EOF hygiene commands.
#[derive(Debug, Error)]
pub enum RuntimePreCommitEofHygieneCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime pre-commit eof workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Effective EOF mode resolution failed.
    #[error("failed to resolve runtime pre-commit eof mode")]
    ResolveMode {
        /// Underlying catalog or settings failure.
        #[source]
        source: AnyhowError,
    },
    /// Staged file discovery or git status inspection failed.
    #[error("failed to inspect runtime pre-commit eof staged files")]
    InspectGitState {
        /// Underlying git command failure.
        #[source]
        source: AnyhowError,
    },
    /// File normalization failed.
    #[error("failed to normalize runtime pre-commit eof file contents")]
    NormalizeFiles {
        /// Underlying filesystem failure.
        #[source]
        source: AnyhowError,
    },
    /// Restaging normalized files failed.
    #[error("failed to restage runtime pre-commit eof files")]
    RestageFiles {
        /// Underlying git command failure.
        #[source]
        source: AnyhowError,
    },
}

/// Errors raised by runtime self-heal commands.
#[derive(Debug, Error)]
pub enum RuntimeSelfHealCommandError {
    /// Workspace root resolution failed.
    #[error("failed to resolve runtime self-heal workspace root")]
    ResolveWorkspaceRoot {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Runtime execution context resolution failed.
    #[error("failed to resolve runtime self-heal execution context")]
    ResolveExecutionContext {
        /// Underlying resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Output/log artifact preparation failed.
    #[error("failed to prepare runtime self-heal artifacts")]
    PrepareArtifacts {
        /// Underlying filesystem or path resolution failure.
        #[source]
        source: AnyhowError,
    },
    /// Report or log writing failed.
    #[error("failed to write runtime self-heal output")]
    WriteOutput {
        /// Underlying serialization or I/O failure.
        #[source]
        source: AnyhowError,
    },
}