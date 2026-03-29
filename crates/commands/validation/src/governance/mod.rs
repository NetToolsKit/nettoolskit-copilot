//! Governance and repository-policy validation commands.

mod routing_coverage;
mod template_standards;

pub use routing_coverage::{
    invoke_validate_routing_coverage, ValidateRoutingCoverageRequest, ValidateRoutingCoverageResult,
};
pub use template_standards::{
    invoke_validate_template_standards, ValidateTemplateStandardsRequest,
    ValidateTemplateStandardsResult,
};
