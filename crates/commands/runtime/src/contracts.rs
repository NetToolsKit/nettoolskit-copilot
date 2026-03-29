//! Runtime surface contracts for the migration program.

/// Migration wave allocation for a command surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MigrationWave {
    /// Foundation migration wave.
    Wave1,
    /// Quality and policy migration wave.
    Wave2,
    /// Control-plane and parity migration wave.
    Wave3,
}

/// Runtime-owned command surface kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeSurfaceKind {
    /// Runtime operator command surface.
    RuntimeCommands,
    /// Runtime lifecycle hook surface.
    RuntimeHooks,
    /// Maintenance command surface.
    MaintenanceCommands,
    /// Git hook installation and hygiene surface.
    GitHookCommands,
}

/// Contract describing one runtime-owned migration surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSurfaceContract {
    /// Stable runtime surface identifier.
    pub surface_id: &'static str,
    /// Legacy root that the surface replaces.
    pub legacy_root: &'static str,
    /// Number of locked PowerShell scripts covered by the surface.
    pub legacy_script_count: usize,
    /// Surface classification.
    pub kind: RuntimeSurfaceKind,
    /// Migration wave in which the surface is expected to cut over.
    pub wave: MigrationWave,
}

impl RuntimeSurfaceContract {
    /// Build one runtime surface contract entry.
    pub const fn new(
        surface_id: &'static str,
        legacy_root: &'static str,
        legacy_script_count: usize,
        kind: RuntimeSurfaceKind,
        wave: MigrationWave,
    ) -> Self {
        Self {
            surface_id,
            legacy_root,
            legacy_script_count,
            kind,
            wave,
        }
    }
}

/// Locked runtime migration surfaces.
pub const RUNTIME_SURFACE_CONTRACTS: &[RuntimeSurfaceContract] = &[
    RuntimeSurfaceContract::new(
        "runtime-commands",
        "scripts/runtime",
        42,
        RuntimeSurfaceKind::RuntimeCommands,
        MigrationWave::Wave1,
    ),
    RuntimeSurfaceContract::new(
        "runtime-hooks",
        "scripts/runtime/hooks",
        4,
        RuntimeSurfaceKind::RuntimeHooks,
        MigrationWave::Wave3,
    ),
    RuntimeSurfaceContract::new(
        "maintenance-commands",
        "scripts/maintenance",
        5,
        RuntimeSurfaceKind::MaintenanceCommands,
        MigrationWave::Wave2,
    ),
    RuntimeSurfaceContract::new(
        "git-hook-commands",
        "scripts/git-hooks",
        3,
        RuntimeSurfaceKind::GitHookCommands,
        MigrationWave::Wave3,
    ),
];

/// Lookup one runtime surface contract by identifier.
#[must_use]
pub fn runtime_surface_contract(surface_id: &str) -> Option<&'static RuntimeSurfaceContract> {
    RUNTIME_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.surface_id == surface_id)
}

/// Sum all runtime-owned legacy scripts represented by the contract catalog.
#[must_use]
pub fn runtime_surface_script_total() -> usize {
    RUNTIME_SURFACE_CONTRACTS
        .iter()
        .map(|contract| contract.legacy_script_count)
        .sum()
}
