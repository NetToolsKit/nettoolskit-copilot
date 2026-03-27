//! Documentation and authoring validation commands.

mod instruction_metadata;
mod readme_standards;

pub use instruction_metadata::{
    invoke_validate_instruction_metadata, ValidateInstructionMetadataRequest,
    ValidateInstructionMetadataResult,
};
pub use readme_standards::{
    invoke_validate_readme_standards, ValidateReadmeStandardsRequest, ValidateReadmeStandardsResult,
};