//! Validation surface contracts for the migration program.

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

/// Validation-owned command surface kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationSurfaceKind {
    /// Repository validation commands.
    ValidationCommands,
    /// Security and supply-chain commands.
    SecurityCommands,
    /// Governance and protection commands.
    GovernanceCommands,
    /// Documentation validation commands.
    DocumentationCommands,
    /// Deploy preflight commands.
    DeployCommands,
}

/// Contract describing one validation-owned migration surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationSurfaceContract {
    /// Stable validation surface identifier.
    pub surface_id: &'static str,
    /// Legacy root that the surface replaces.
    pub legacy_root: &'static str,
    /// Number of locked PowerShell scripts covered by the surface.
    pub legacy_script_count: usize,
    /// Surface classification.
    pub kind: ValidationSurfaceKind,
    /// Migration wave in which the surface is expected to cut over.
    pub wave: MigrationWave,
}

impl ValidationSurfaceContract {
    /// Build one validation surface contract entry.
    pub const fn new(
        surface_id: &'static str,
        legacy_root: &'static str,
        legacy_script_count: usize,
        kind: ValidationSurfaceKind,
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

/// Locked validation migration surfaces.
pub const VALIDATION_SURFACE_CONTRACTS: &[ValidationSurfaceContract] = &[
    ValidationSurfaceContract::new(
        "validation-commands",
        "scripts/validation",
        29,
        ValidationSurfaceKind::ValidationCommands,
        MigrationWave::Wave2,
    ),
    ValidationSurfaceContract::new(
        "security-commands",
        "scripts/security",
        6,
        ValidationSurfaceKind::SecurityCommands,
        MigrationWave::Wave2,
    ),
    ValidationSurfaceContract::new(
        "governance-commands",
        "scripts/governance",
        2,
        ValidationSurfaceKind::GovernanceCommands,
        MigrationWave::Wave2,
    ),
    ValidationSurfaceContract::new(
        "documentation-commands",
        "scripts/doc",
        1,
        ValidationSurfaceKind::DocumentationCommands,
        MigrationWave::Wave2,
    ),
    ValidationSurfaceContract::new(
        "deploy-commands",
        "scripts/deploy",
        1,
        ValidationSurfaceKind::DeployCommands,
        MigrationWave::Wave2,
    ),
];

/// Lookup one validation surface contract by identifier.
#[must_use]
pub fn validation_surface_contract(surface_id: &str) -> Option<&'static ValidationSurfaceContract> {
    VALIDATION_SURFACE_CONTRACTS
        .iter()
        .find(|contract| contract.surface_id == surface_id)
}

/// Sum all validation-owned legacy scripts represented by the contract catalog.
#[must_use]
pub fn validation_surface_script_total() -> usize {
    VALIDATION_SURFACE_CONTRACTS
        .iter()
        .map(|contract| contract.legacy_script_count)
        .sum()
}
