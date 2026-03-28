# Rust Script Cutover Default Map

Generated: 2026-03-28 10:23

## Status

- LastUpdated: 2026-03-28 11:22
- Objective: record the final operator-default decision for each major script domain after the completed migration waves.
- Source Inputs:
  - `planning/active/plan-rust-migration-closeout-and-cutover.md`
  - `planning/active/rust-script-parity-ledger.md`
  - `planning/active/rust-script-transcription-ownership-matrix.md`
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
- Active Branch: `feature/native-validation-policy`
- Decision Rule:
  - `Rust-default now` means the Rust surface is the canonical operating path and the PowerShell file remains only as a compatibility surface, if it remains at all.
  - `compatibility wrapper retained intentionally` means the PowerShell entrypoint stays in the repository by design, even though Rust owns the underlying behavior.
  - `still blocked` means the domain does not yet have enough closeout evidence for a final default decision.

## Domain Decisions

| Major Domain | Decision | Rationale | Closeout Note |
| --- | --- | --- | --- |
| `scripts/common` | `Rust-default now` | Shared primitives, catalogs, and runtime helpers are owned by `crates/core`; no operator-facing fallback is required for the shared helper layer. | Canonical support primitives live in Rust. |
| `scripts/runtime` excluding hooks | `Rust-default now` | Runtime verbs, sync, diagnostics, and continuity flows are implemented in Rust, and the docs/workflows already frame PowerShell as compatibility-only. | Default operator runtime path is Rust-backed. |
| `scripts/runtime/hooks` | `still blocked` | Hook dispatch and local hook ownership still lack a final Rust-default closeout decision. | Mixed EOF/hook behavior is aligned, but wrapper cutover is not approved yet. |
| `scripts/maintenance` | `still blocked` | The cluster has partial Rust coverage, but not a complete native replacement record. | Mutation-heavy helpers still need explicit closeout evidence. |
| `scripts/validation` | `Rust-default now` | Wave 2 is complete and `validate-all` plus the individual checks are Rust-owned. | Validation is the canonical quality gate surface. |
| `scripts/security` | `Rust-default now` | Security baseline, checksum, and supply-chain checks are Rust-owned through the validation crate. | Security gates default to Rust-owned execution. |
| `scripts/governance` | `Rust-default now` | Routing, template, and repository governance checks are implemented natively in Rust. | Governance checks default to Rust-owned execution. |
| `scripts/doc` | `Rust-default now` | The doc-only validation script now has an explicit native replacement in `crates/commands/validation/documentation`, and `validate-all` routes it without the PowerShell bridge. | Documentation validation defaults to Rust-owned execution. |
| `scripts/deploy` | `compatibility wrapper retained intentionally` | Deploy preflight is now Rust-native through `crates/commands/validation/deploy`, while the PowerShell entrypoint remains the approved SSH/SCP operational executor by design. | The deploy wrapper stays intentionally for remote execution. |
| `scripts/orchestration` | `Rust-default now` | Stage orchestration, resume/replay, and parity harness coverage are Rust-owned and complete. | Control-plane execution defaults to Rust. |
| `scripts/git-hooks` | `compatibility wrapper retained intentionally` | The hook install/check logic is Rust-owned, but the hook entrypoints remain a deliberate compatibility surface for Git-integrated workflows. | Wrapper remains by design for local hook integration. |
| `scripts/tests` excluding runtime | `still blocked` | `check-test-naming` is now Rust-native, but the remaining non-runtime test automation scripts still lack explicit Rust-native replacement evidence. | The blocked tail is limited to three scripts. |
| `scripts/tests/runtime` | `compatibility wrapper retained intentionally` | The native parity harness is real, and the PowerShell test entrypoints remain only as a compatibility launch surface for runtime/operator workflows. | Compatibility wrapper remains by design while the Rust harness stays canonical. |

## Remaining Closeout Backlog

- finalize the blocked domains:
  - `scripts/runtime/hooks`
  - `scripts/maintenance`
  - `scripts/tests` excluding runtime
- keep the compatibility wrapper decisions explicit for:
  - `scripts/git-hooks`
  - `scripts/tests/runtime`

## Operating Rule

No domain should move from `still blocked` to a default state without a corresponding planning update in the closeout plan and a matching parity-ledger note.