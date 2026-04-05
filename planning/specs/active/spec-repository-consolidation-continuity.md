# Spec: Repository Consolidation and Continuity

Generated: 2026-03-29

## Status

- LastUpdated: 2026-04-05 11:05
- Objective: define the design intent and safe execution conditions for the six consolidation workstreams identified after the triangulation analysis of `nettoolskit-copilot`, `nettoolskit-cli`, and `copilot-instructions`.
- Planning Readiness: ready-for-plan
- Related Plan: `planning/active/plan-repository-consolidation-continuity.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Source Inputs:
  - `planning/completed/plan-script-retirement-phase-17.md`
  - `planning/completed/plan-script-retirement-phase-18.md`
  - `planning/completed/plan-script-retirement-phase-19.md`
  - `planning/completed/plan-script-retirement-phase-20c-self-heal.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`
  - `.github/workflows/ci.yml`
  - `crates/cli/src/main.rs`
  - `crates/cli/src/runtime_commands.rs`
  - `crates/cli/src/validation_commands.rs`
  - `C:\Users\tguis\copilot-instructions\planning\active\plan-rust-runtime-engine-foundation-phase-8.md`

---

## Problem Statement

The triangulation analysis revealed six distinct consolidation concerns that are not covered by the current Phase 17 tactical plan:

1. **Operating model drift across canonical and projected surfaces**: the repository operating model now belongs to `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`, but historical plans and compatibility mirrors still mention the older `.github/instructions/core/*` and `definitions/shared/instructions/*` paths. If these stale references remain untreated, future work can reopen the wrong authority model even after the Rust workspace topology was fixed.

2. **CLI surface documentation gap**: The `ntk` binary now exposes 12 `runtime` subcommands, 29 `validation` subcommands, `manifest` submenu, `service` mode (Axum HTTP + ChatOps), and `completions` (bash/zsh/fish/powershell). None of these surfaces are documented in the root `README.md` or `crates/cli/README.md`. The existing docs describe either the original interactive TUI model or just 3 bullet features.

3. **CI gap for PowerShell parity tests**: The 23 `scripts/tests/runtime/*.ps1` files are classified as `compatibility wrapper retained intentionally` in the safety matrix and described as the canonical parity harness. However, `ci.yml` contains zero PowerShell test invocations — these scripts run only on developer machines. If they regress silently, the parity evidence base becomes stale.

4. **Post-Phase-20f domain consumer migration**: After Phases 17, 18, the tactical `self-heal` runtime slice, the tactical provider-surface dispatcher slice, the tactical catalog-native renderer slice, and the tactical Codex orchestration renderer slice close, 63 scripts remain in the `retain until consumer migration completes` bucket across five domains (`scripts/common/`, `scripts/runtime/` excl. hooks, `scripts/security/`, `scripts/governance/`, `scripts/orchestration/`). Each domain has confirmed Rust ownership but no exact local-consumer proof. Without planned consumer sweeps, these scripts will remain indefinitely even though deletions are safe once consumer evidence is collected.

5. **`copilot-instructions` Phase 8 awaiting directives**: The `copilot-instructions` repo has a planning-ready spec (`spec-rust-runtime-engine-foundation-phase-8.md`) that is blocked on user-provided Rust directives. The spec defines the compatibility-first migration contract but does not create any Cargo files until directives arrive. Starting this migration would bring the instruction runtime into the same Rust-native model that `nettoolskit-copilot` already operates with.

6. **Canonical vs compatibility surface synchronization**: the repository now authors instructions in `definitions/instructions/*`, keeps `definitions/shared/*` only as compatibility, and renders provider/runtime consumers from `definitions/providers/*`. Historical references must stop treating `definitions/shared/instructions/*` or `.github/instructions/*` as the authored source, or future maintenance will drift back to the wrong root.

7. **Pre-commit EOF hook command-line overflow on Windows**: the managed EOF hygiene hook currently forwards every staged file as its own `--literal-path` argument to `ntk runtime trim-trailing-blank-lines`. Large commits with hundreds of staged files can exceed Windows process invocation limits and fail before the commit completes.

8. **Global-core budget drift in provider root files**: `definitions/providers/github/root/AGENTS.md` and `definitions/providers/github/root/copilot-instructions.md` can easily regrow into duplicated policy hubs because both are always loaded. When that happens, instruction-architecture warnings reappear and the provider root stops behaving like a thin bootstrap layer.

---

## Desired Outcome

- Every AI agent that routes through `ntk-core-repository-operating-model.instructions.md` receives correct Rust workspace commands, correct topology, and correct domain instruction references.
- Root `README.md` and `crates/cli/README.md` document the full `ntk` CLI surface with named subcommands, so feature discoverability matches implementation reality.
- The 23 PowerShell parity tests either run in CI with an explicit gate or are explicitly documented as local-only with a rationale that is not ambiguous about their coverage model.
- Phase 19 is closed with an audit-only result, and the remaining runtime/security/governance/orchestration sweeps still have concrete consumer-sweep plans so the remaining 63 `retain until` scripts can move to confirmed deletion candidates when evidence is collected.
- `copilot-instructions` Phase 8 receives the Rust directives and creates the initial Cargo workspace scaffold with the first migration slice defined.
- `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md` remains the single authored source, while compatibility mirrors and `.github` projections stay renderer-derived.
- Large staged commits remain committable on Windows because the managed EOF hygiene hook trims files in bounded batches instead of issuing one oversized process invocation.
- The provider-root global-core files stay thin enough to remain bootstrap-only, while detailed policy continues to live in canonical `definitions/instructions/*` content.

---

## Design Decisions

### W2: Operating Model Alignment

The authoritative source for the repository operating model is `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`. Provider/runtime copies are consumer surfaces. The correct fix sequence is:

1. Edit `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md` as the single authoritative change.
2. Re-render downstream provider/runtime surfaces using the GitHub instruction surface renderer (`scripts/runtime/render-github-instruction-surfaces.ps1`) or `ntk runtime bootstrap`.
3. Validate with `ntk validation instructions --repo-root . --warning-only false`.

Changing only a projected or compatibility copy is rejected because the next render pass would overwrite the fix.

Current execution checkpoint:
- the authoritative `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md` has now been rewritten to the Rust workspace topology and command model
- provider/runtime projections now derive from canonical `definitions/*` roots instead of treating `.github/*` as authored source
- native validation confirmed the operating-model update without reopening planning-structure drift

### W3: CI PowerShell Test Coverage

Option A — Add a CI job that runs the 23 parity tests via Pester and Windows runner:
- Adds a `pwsh-parity` job to `ci.yml` under `jobs:`.
- Uses `windows-latest` runner.
- Invokes `Invoke-Pester -Path scripts/tests/runtime/ -EnableExit` after installing Pester.
- Selected for parity harness workstreams that explicitly need shell-level contract verification in addition to Rust unit tests.

Option B — Document parity tests as local-only with explicit rationale:
- Adds a note to `scripts/tests/runtime/README.md` (or creates it).
- States that these tests are an operator-compatible harness and are not expected to run in CI.
- Chosen only if adding a second Windows CI job is considered too costly in CI minutes.

Decision: Option A is the correct long-term answer because the parity harness is the evidence foundation for consumer migration decisions. However, if CI minutes are a constraint, Option B is acceptable as a documented interim state — but it must be an explicit decision, not a default silence. This spec designates Option A as the target and allows the plan to defer to Option B only with an explicit rationale recorded in the plan comment.

### W4: CLI Documentation

The root `README.md` already has a correct Features and Build/Tests sections. The fix is additive: insert a `### Command Reference` subsection under an existing section rather than restructuring the entire file. This minimizes diff noise and preserves the current table format.

For `crates/cli/README.md`, the existing 3-feature bullet list is too shallow. Expand to a table of subcommand groups with entry-point forms.

### W5: Post-Phase-20c Consumer Migration Phases

Consumer sweeps are executed in domain order, smallest-first, with parity requirement: each domain must prove zero local non-self consumers before being treated as a deletion candidate. The sequence is:

- Phase 19: `scripts/common/*.ps1` (15) — shared helpers; high risk of implicit consumers in every other domain.
- Phase 20: `scripts/runtime/*.ps1` excluding hooks (30 after Phases 17, 18, the tactical 20c `self-heal` cutover, the tactical 20d provider-surface dispatcher cutover, the tactical 20e catalog-native renderer cutover, and the tactical 20f Codex orchestration renderer cutover) — largest single domain; planned as one grouped sweep, may split into sub-phases.
- Phase 21: `scripts/security/*.ps1` (6) + `scripts/governance/*.ps1` (2) — governance surface; `shared-script-checksums.manifest.json` is the key blocker to repoint.
- Phase 22: `scripts/orchestration/**/*.ps1` (10) — staged execution; depends on orchestrator parity being proven end-to-end.

No domain moves to deletion without the same exact consumer-proof standard used in Phases 1–16. Tactical runtime leaf cutovers may proceed ahead of the broader Phase 20 domain sweep only when a specific leaf already has native CLI parity and same-slice consumer proof; the completed `self-heal` cutover is the precedent and does not eliminate the need for the remaining runtime consumer sweep.

### W6: `copilot-instructions` Phase 8 Directives

The Cargo workspace scaffold for `copilot-instructions` must follow these constraints:
- `Cargo.toml` at repo root with `[workspace]` pointing at `src/`.
- First crate: `ntk-runtime-engine` under `src/runtime/` — wraps the bootstrap, install, sync, and renderer dispatch surfaces that currently live in `scripts/runtime/*.ps1`.
- No Rust code replaces a PowerShell wrapper until the Rust crate has deterministic tests and an operator smoke check proves the native path works.
- `scripts/` stays as the canonical operator entrypoint layer during the migration.
- The projection architecture (`definitions/` → `.github/.codex/.claude/.vscode`) is NOT touched by Phase 8; only the execution engine moves to Rust.

---

## Alternatives Considered

### W2 alternatives

1. Edit only a projected `.github` copy directly
   - Rejected: the rendering pipeline would overwrite the fix on the next sync.
2. Delete the file and inherit only from the global `%USERPROFILE%\.github` mirror
   - Rejected: would break workspace-adapter routing that depends on local override.

### W3 alternatives

1. Remove the 23 parity tests entirely since their surface is proven in Rust
   - Rejected: the cutover map explicitly marks `scripts/tests/runtime/*.ps1` as `compatibility wrapper retained intentionally`. Removing them without completing the domain consumer sweeps removes evidence.
2. Move the 23 tests to `crates/` as Rust integration tests
   - Deferred: this is the right long-term outcome but requires rewriting 23 test files; scope it as a follow-up for Phase 20 when the runtime domain consumer sweep is complete.

### W5 alternatives

1. Run all four consumer sweeps as one mega-phase
   - Rejected: 71-script domain is too large to validate atomically; a failure mid-sweep leaves the safety matrix partially updated.
2. Skip consumer sweeps and keep scripts indefinitely
   - Rejected: the scripts create CI maintenance overhead and confuse new contributors about what the canonical entrypoints are.

### W6 alternatives

1. Do not start Phase 8 until all nettoolskit-copilot phases are complete
   - Rejected: the two repos are independent; `copilot-instructions` Phase 8 can proceed in parallel without creating conflicts.
2. Mirror the full `nettoolskit-copilot` crate layout into `copilot-instructions`
   - Rejected: `copilot-instructions` has a different scope (instruction/provider runtime, not CLI product). It needs a subset of crates, not a full copy.

---

## Risks

| # | Risk | Severity | Mitigation |
|---|---|---|---|
| R1 | Editing the authoritative `definitions/instructions/` file triggers a provider surface re-render that breaks unrelated instruction surfaces | Medium | Run `ntk validation instructions` and `ntk validation agent-hooks` before committing the re-render |
| R2 | Adding a Windows CI job for PowerShell parity tests increases CI cost and may fail due to environment differences | Low | Pin Pester version; use `continue-on-error: false` only after a trial run confirms stability; allow `warning-only` mode for first merge |
| R3 | Phase 19 common-script consumer sweep finds unexpected consumers in test crates that re-lock deletion | Medium | Map all consumers with `rg` before touching any file; document blockers explicitly rather than forcing deletions |
| R4 | Phase 20 runtime-script consumer sweep is too large for one PR; may need to split into 3–4 sub-phases | Low | Define sub-phase boundaries in the Phase 20 plan; each sub-phase closes its own consumer evidence set |
| R5 | `copilot-instructions` Phase 8 diverges from `nettoolskit-copilot` Rust conventions mid-implementation | Medium | Lock a shared Rust edition, MSRV, deny list, and clippy flags between both repos before Phase 8 starts |
| R6 | A legacy consumer still assumes `definitions/shared/instructions/` or `.github/instructions/` is authored source | High | Keep compatibility mirrors available, but enforce the canonical-first rule through validators and provider-surface renderers |

---

## Acceptance Criteria

### W2: Operating Model Alignment
- `ntk-core-repository-operating-model.instructions.md` describes the Rust workspace topology (`crates/`, `scripts/`, `definitions/`, `deployments/`) with correct build and test commands (`cargo build --workspace`, `cargo test --workspace`).
- The domain instruction map reflects Rust-applicable facts; .NET-specific entries are removed or scoped to a separate future workstream.
- `ntk validation instructions --repo-root .` passes after the update.

### W3: CI PowerShell Test Coverage
- Either the 23 parity tests are invoked in CI via a dedicated `pwsh-parity` job, or a decision record explicitly states they are local-only and why.
- No silent coverage gap remains.

### W4: CLI Surface Documentation
- Root `README.md` lists all `ntk` command groups in a structured subsection.
- `crates/cli/README.md` lists all subcommands with their CLI forms.
- `ntk service` and `ntk completions` are documented.
- All changes pass `ntk validation readme-standards --repo-root .`.

### W5: Post-Phase-18 Domain Consumer Migration
- Phases 19–22 each have their own active plan before execution begins.
- Each phase's consumer evidence collected via `rg` proves zero non-self consumers before deletion.
- Safety matrix and parity ledger are updated after each phase.

### W6: `copilot-instructions` Phase 8
- `copilot-instructions` has a `Cargo.toml` workspace root and at least one Rust crate (`ntk-runtime-engine`) with a passing `cargo build`.
- No PowerShell wrapper is deleted until the Rust replacement passes deterministic tests and a smoke check.
- Phase 8 spec and plan are updated to `completed` only when the first migration slice is proven.