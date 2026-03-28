//! Language and template standards validation commands.

mod dotnet_standards;
mod powershell_standards;

pub use dotnet_standards::{
    invoke_validate_dotnet_standards, ValidateDotnetStandardsRequest, ValidateDotnetStandardsResult,
};
pub use powershell_standards::{
    invoke_validate_powershell_standards, ValidatePowerShellStandardsRequest,
    ValidatePowerShellStandardsResult,
};
