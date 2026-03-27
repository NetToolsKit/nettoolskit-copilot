//! Instruction-system policy and graph validation commands.

mod authoritative_source_policy;
mod instruction_architecture;

pub use authoritative_source_policy::{
    invoke_validate_authoritative_source_policy, ValidateAuthoritativeSourcePolicyRequest,
    ValidateAuthoritativeSourcePolicyResult,
};
pub use instruction_architecture::{
    invoke_validate_instruction_architecture, ValidateInstructionArchitectureRequest,
    ValidateInstructionArchitectureResult,
};