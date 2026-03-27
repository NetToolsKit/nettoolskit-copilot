//! Validation command contract test suite entry point.

mod contracts_tests;
mod documentation {
    pub mod instruction_metadata_tests;
    pub mod readme_standards_tests;
}
mod agent_orchestration {
    pub mod agent_orchestration_tests;
    pub mod agent_hooks_tests;
    pub mod agent_permissions_tests;
    pub mod agent_skill_alignment_tests;
}
mod error_tests;
mod instruction_graph {
    pub mod authoritative_source_policy_tests;
    pub mod instruction_architecture_tests;
    pub mod instructions_tests;
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

mod operational_hygiene {
    pub mod runtime_script_tests_tests;
    pub mod shell_hooks_tests;
    pub mod warning_baseline_tests;
}

mod structure {
    pub mod planning_structure_tests;
}

mod workspace {
    pub mod workspace_efficiency_tests;
}

mod support;