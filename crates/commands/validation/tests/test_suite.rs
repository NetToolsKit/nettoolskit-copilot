//! Validation command contract test suite entry point.

mod contracts_tests;
mod documentation {
    pub mod instruction_metadata_tests;
    pub mod readme_standards_tests;
}
mod error_tests;
mod instruction_graph {
    pub mod authoritative_source_policy_tests;
}

mod governance {
    pub mod routing_coverage_tests;
    pub mod template_standards_tests;
}

mod evidence {
    pub mod audit_ledger_tests;
}

mod orchestration {
    pub mod validate_all_tests;
}

mod structure {
    pub mod planning_structure_tests;
}

mod workspace {
    pub mod workspace_efficiency_tests;
}