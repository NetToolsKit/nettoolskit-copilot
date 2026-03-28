//! Workspace-focused validation commands.

mod workspace_efficiency;

pub use workspace_efficiency::{
    invoke_validate_workspace_efficiency, ValidateWorkspaceEfficiencyRequest,
    ValidateWorkspaceEfficiencyResult,
};
