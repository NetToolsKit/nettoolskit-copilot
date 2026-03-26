# Repository Unification And Rust Migration

## Objective

Create `nettoolskit-copilot` as the unified successor workspace that combines the Rust product lineage from `nettoolskit-cli` with the automation/runtime lineage from `copilot-instructions`, while preserving both histories and enabling a phased migration of legacy PowerShell runtime scripts into Rust without carrying an embedded legacy subtree in the active worktree.

## Problem Statement

The current automation and runtime behavior that powers the Copilot-oriented repository lives primarily in PowerShell scripts and projected runtime surfaces inside `C:\Users\tguis\copilot-instructions`, while `nettoolskit-cli` already provides the Rust-first repository structure, Cargo workspace, and production-oriented execution model that should become the long-term home.

If the repositories stay separate, shared runtime capabilities will continue to drift. If the PowerShell logic is rewritten before the repositories are unified, commit history, rationale, and migration traceability will be lost. The work therefore needs a compatibility-first unification step before code-porting begins.

## Design Summary

- Use `nettoolskit-cli` as the root repository and primary lineage.
- Create a new local repository named `nettoolskit-copilot` from `nettoolskit-cli`.
- Keep `C:\Users\tguis\copilot-instructions` as the external legacy reference repository during the migration instead of carrying `legacy/copilot-instructions/` inside the active worktree.
- Bring the organized AI/runtime folders into the new repository root early so the future Rust implementation has the correct destination topology before code-porting begins.
- Preserve the `copilot-instructions` lineage through the dedicated source remote and reference repository while keeping the new workspace tree focused on the new product layout.
- Port script families to Rust in slices, keeping compatibility wrappers until parity is validated.
- Keep the Cargo workspace centered on `crates/`, not a monolithic root `src/`, so the new implementation stays aligned with normal multi-crate Rust repository structure.

## Key Decisions

1. `nettoolskit-cli` remains the root/base repository for the unified workspace.
2. `copilot-instructions` remains an external reference repository during migration instead of living under `legacy/copilot-instructions/` in the active tree.
3. The organized AI/runtime folders (`definitions/`, `scripts/`, `.codex/`, `.claude/`, `.vscode/`, and the AI/runtime `.github` surfaces) should live directly in the new repository before the Rust port starts.
4. `C:\Users\tguis\copilot-instructions` remains a reference repository during migration, but implementation work should target the hydrated root folders in `nettoolskit-copilot`.
5. The Rust implementation will stay organized as a Cargo workspace under `crates/`, with new migration slices landing as dedicated crates rather than collapsing everything into a root `src/`.
6. Rust migration starts only after unification, planning registration, legacy asset classification, and root-folder organization are complete.
7. Legacy PowerShell entrypoints remain available until Rust replacements are validated and explicitly approved for cutover.
8. The migration is compatibility-first: preserve working operator flows before optimizing or deleting legacy surfaces.

## Constraints

- Preserve commit history from both repositories.
- Avoid breaking the current `nettoolskit-cli` Rust workspace layout.
- Do not modify or delete `C:\Users\tguis\copilot-instructions` during the early phases.
- Keep the migration incremental so the user can supply Rust-specific directives before implementation slices begin.

## Alternatives Considered

### Alternative 1: Flatten Both Repositories Into The Root Immediately

Rejected. This would create path conflicts, obscure provenance, and make parity validation harder before the new Rust paths exist.

### Alternative 2: Keep An Embedded `legacy/copilot-instructions/` Tree In The Unified Workspace

Rejected. This keeps the new workspace noisy, duplicates the external source repo that will remain available anyway, and makes the final product layout harder to stabilize before Rust migration begins.

### Alternative 3: Rewrite PowerShell Logic Into Rust Before Unification

Rejected. This would sever the migration from its source history and make it much harder to prove behavior parity.

### Alternative 4: Keep The Repositories Separate And Copy Files Manually

Rejected. This would preserve neither unified history nor a single long-term execution surface.

## Risks

- The external legacy repository contains runtime assets, projected provider surfaces, and PowerShell utilities that may not map one-to-one onto the Rust workspace boundaries.
- A premature cutover to Rust could regress local runtime sync, projected surface rendering, or MCP-related behavior.
- The unified repository could become difficult to navigate if the external legacy assets are not classified before Rust slices start landing.

## Acceptance Criteria

1. `nettoolskit-copilot` exists locally as a standalone git repository.
2. `nettoolskit-cli` history is preserved as the root lineage.
3. `copilot-instructions` remains traceable through the dedicated source remote and migration planning without requiring an embedded `legacy/copilot-instructions/` working tree.
4. The new repository root already contains the organized AI/runtime folders needed for the migration, without relying on `legacy/copilot-instructions/`.
5. `C:\Users\tguis\copilot-instructions` remains available as an external reference repository while the new root folders are migrated.
6. The Rust migration target structure is based on `crates/` rather than collapsing the workspace into a root `src/`.
7. A phased plan exists before any large-scale PowerShell-to-Rust conversion begins.
8. Early migration slices preserve compatibility instead of deleting legacy behavior prematurely.

## Planning Readiness

- `ready-for-plan`

## Recommended Specialist Focus

- `dev-rust-engineer` for the future implementation slices
- `plan-active-work-planner` for phased migration planning
- `docs-release-engineer` for operator-path and migration documentation once the first slice lands