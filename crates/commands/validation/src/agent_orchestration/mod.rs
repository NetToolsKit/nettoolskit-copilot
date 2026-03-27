//! Agent orchestration validation commands.

pub(crate) mod common;
mod agent_permissions;
mod agent_hooks;

pub use agent_permissions::{
    invoke_validate_agent_permissions, ValidateAgentPermissionsRequest,
    ValidateAgentPermissionsResult,
};
pub use agent_hooks::{
    invoke_validate_agent_hooks, ValidateAgentHooksRequest, ValidateAgentHooksResult,
};