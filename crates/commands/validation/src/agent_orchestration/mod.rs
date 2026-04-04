//! Agent orchestration validation commands.

mod agent_hooks;
mod agent_permissions;
mod agent_skill_alignment;
pub(crate) mod common;
mod orchestration_integrity;

pub use agent_hooks::{
    invoke_validate_agent_hooks, ValidateAgentHooksRequest, ValidateAgentHooksResult,
};
pub use agent_permissions::{
    invoke_validate_agent_permissions, ValidateAgentPermissionsRequest,
    ValidateAgentPermissionsResult,
};
pub use agent_skill_alignment::{
    invoke_validate_agent_skill_alignment, ValidateAgentSkillAlignmentRequest,
    ValidateAgentSkillAlignmentResult,
};
pub use orchestration_integrity::{
    invoke_validate_agent_orchestration, ValidateAgentOrchestrationRequest,
    ValidateAgentOrchestrationResult,
};