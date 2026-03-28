//! Agent orchestration validation commands.

mod agent_hooks;
mod agent_permissions;
mod agent_skill_alignment;
mod orchestration_integrity;
pub(crate) mod common;

pub use agent_hooks::{
    invoke_validate_agent_hooks, ValidateAgentHooksRequest, ValidateAgentHooksResult,
};
pub use orchestration_integrity::{
    invoke_validate_agent_orchestration, ValidateAgentOrchestrationRequest,
    ValidateAgentOrchestrationResult,
};
pub use agent_permissions::{
    invoke_validate_agent_permissions, ValidateAgentPermissionsRequest,
    ValidateAgentPermissionsResult,
};
pub use agent_skill_alignment::{
    invoke_validate_agent_skill_alignment, ValidateAgentSkillAlignmentRequest,
    ValidateAgentSkillAlignmentResult,
};