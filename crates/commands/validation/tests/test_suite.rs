//! Validation command contract test suite entry point.

mod contracts_tests;
mod error_tests;

mod evidence {
    pub mod audit_ledger_tests;
}

mod orchestration {
    pub mod validate_all_tests;
}

mod structure {
    pub mod planning_structure_tests;
}