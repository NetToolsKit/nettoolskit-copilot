//! Tests for validation surface contracts.

use nettoolskit_validation::{
    validation_surface_contract, validation_surface_script_total, MigrationWave,
    ValidationSurfaceKind, VALIDATION_SURFACE_CONTRACTS,
};
use std::collections::HashSet;

#[test]
fn test_validation_surface_total_matches_locked_inventory() {
    assert_eq!(validation_surface_script_total(), 41);
}

#[test]
fn test_validation_surface_contracts_have_unique_ids_and_roots() {
    let mut ids = HashSet::new();
    let mut roots = HashSet::new();

    for contract in VALIDATION_SURFACE_CONTRACTS {
        assert!(ids.insert(contract.surface_id));
        assert!(roots.insert(contract.legacy_root));
        assert!(contract.legacy_script_count > 0);
        assert_eq!(contract.wave, MigrationWave::Wave2);
    }
}

#[test]
fn test_validation_surface_lookup_returns_expected_security_contract() {
    let contract =
        validation_surface_contract("security-commands").expect("security surface should exist");

    assert_eq!(contract.kind, ValidationSurfaceKind::SecurityCommands);
    assert_eq!(contract.legacy_root, "scripts/security");
    assert_eq!(contract.legacy_script_count, 6);
}

#[test]
fn test_validation_surface_category_counts_match_matrix() {
    let validation_commands = VALIDATION_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == ValidationSurfaceKind::ValidationCommands)
        .expect("validation surface should exist");
    let security_commands = VALIDATION_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == ValidationSurfaceKind::SecurityCommands)
        .expect("security surface should exist");
    let governance_commands = VALIDATION_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == ValidationSurfaceKind::GovernanceCommands)
        .expect("governance surface should exist");
    let documentation_commands = VALIDATION_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == ValidationSurfaceKind::DocumentationCommands)
        .expect("documentation surface should exist");
    let deploy_commands = VALIDATION_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == ValidationSurfaceKind::DeployCommands)
        .expect("deploy surface should exist");

    assert_eq!(validation_commands.legacy_script_count, 31);
    assert_eq!(security_commands.legacy_script_count, 6);
    assert_eq!(governance_commands.legacy_script_count, 2);
    assert_eq!(documentation_commands.legacy_script_count, 1);
    assert_eq!(documentation_commands.legacy_root, "scripts/doc");
    assert_eq!(deploy_commands.legacy_script_count, 1);
}
