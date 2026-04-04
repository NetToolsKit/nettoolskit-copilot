//! Documentation and authoring validation commands.

mod instruction_metadata;
mod readme_standards;
mod xml_documentation;

pub use instruction_metadata::{
    invoke_validate_instruction_metadata, ValidateInstructionMetadataRequest,
    ValidateInstructionMetadataResult,
};
pub use readme_standards::{
    invoke_validate_readme_standards, ValidateReadmeStandardsRequest, ValidateReadmeStandardsResult,
};
pub use xml_documentation::{
    invoke_validate_xml_documentation, ValidateXmlDocumentationRequest,
    ValidateXmlDocumentationResult,
};