//! Language and template standards validation commands.

mod dotnet_standards;

pub use dotnet_standards::{
    invoke_validate_dotnet_standards, ValidateDotnetStandardsRequest,
    ValidateDotnetStandardsResult,
};