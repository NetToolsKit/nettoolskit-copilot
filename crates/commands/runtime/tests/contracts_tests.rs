//! Tests for runtime surface contracts.

use nettoolskit_runtime::{
    runtime_surface_contract, runtime_surface_script_total, MigrationWave, RuntimeSurfaceKind,
    RUNTIME_SURFACE_CONTRACTS,
};
use std::collections::HashSet;

#[test]
fn test_runtime_surface_total_matches_locked_inventory() {
    assert_eq!(runtime_surface_script_total(), 54);
}

#[test]
fn test_runtime_surface_contracts_have_unique_ids_and_roots() {
    let mut ids = HashSet::new();
    let mut roots = HashSet::new();

    for contract in RUNTIME_SURFACE_CONTRACTS {
        assert!(ids.insert(contract.surface_id));
        assert!(roots.insert(contract.legacy_root));
        assert!(contract.legacy_script_count > 0);
    }
}

#[test]
fn test_runtime_surface_lookup_returns_expected_hook_contract() {
    let contract =
        runtime_surface_contract("runtime-hooks").expect("runtime hook surface should exist");

    assert_eq!(contract.kind, RuntimeSurfaceKind::RuntimeHooks);
    assert_eq!(contract.wave, MigrationWave::Wave3);
    assert_eq!(contract.legacy_root, "scripts/runtime/hooks");
    assert_eq!(contract.legacy_script_count, 4);
}

#[test]
fn test_runtime_surface_category_counts_match_matrix() {
    let runtime_commands = RUNTIME_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == RuntimeSurfaceKind::RuntimeCommands)
        .expect("runtime command surface should exist");
    let maintenance_commands = RUNTIME_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == RuntimeSurfaceKind::MaintenanceCommands)
        .expect("maintenance surface should exist");
    let git_hook_commands = RUNTIME_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.kind == RuntimeSurfaceKind::GitHookCommands)
        .expect("git hook surface should exist");

    assert_eq!(runtime_commands.legacy_script_count, 42);
    assert_eq!(maintenance_commands.legacy_script_count, 5);
    assert_eq!(git_hook_commands.legacy_script_count, 3);
}
