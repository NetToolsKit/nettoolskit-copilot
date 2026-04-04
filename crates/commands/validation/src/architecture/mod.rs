//! Architecture validation commands.

mod architecture_boundaries;

pub use architecture_boundaries::{
    invoke_validate_architecture_boundaries, ValidateArchitectureBoundariesRequest,
    ValidateArchitectureBoundariesResult,
};