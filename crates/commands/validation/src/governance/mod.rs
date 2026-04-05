//! Governance and repository-policy validation commands.

mod routing_coverage;
mod template_standards;

pub use routing_coverage::{
    invoke_validate_routing_coverage, ValidateRoutingCoverageRequest, ValidateRoutingCoverageResult,
};
pub(crate) use routing_coverage::{resolve_catalog_reference_path, resolve_default_catalog_path};
pub use template_standards::{
    invoke_validate_template_standards, ValidateTemplateStandardsRequest,
    ValidateTemplateStandardsResult,
};