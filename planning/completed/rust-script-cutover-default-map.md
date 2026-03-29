# Rust Script Cutover Default Map

Generated: 2026-03-28 10:23

## Status

- LastUpdated: 2026-03-28 16:10
- Objective: record the final operator-default decision for each major script domain after the completed migration waves.
- Source Inputs:
  - `planning/completed/plan-rust-migration-closeout-and-cutover.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-transcription-ownership-matrix.md`
  - `planning/completed/plan-repository-operations-hygiene.md`
  - `planning/completed/plan-repository-unification-and-rust-migration.md`
- Archived From Branch: `feature/native-validation-policy`
- Decision Rule:
  - `Rust-default now` means the Rust surface is the canonical operating path and the PowerShell file remains only as a compatibility surface, if it remains at all.
  - `compatibility wrapper retained intentionally` means the PowerShell entrypoint stays in the repository by design, even though Rust owns the underlying behavior.
  - `legacy integration wrapper retained intentionally` means the PowerShell script remains shell-owned by design because it is editor/toolchain/external-runtime glue, and the closeout plan treats it as an explicit exception rather than as unresolved migration debt.

## Domain Decisions

| Major Domain | Decision | Rationale | Closeout Note |
| --- | --- | --- | --- |
| `scripts/common` | `Rust-default now` | Shared primitives, catalogs, and runtime helpers are owned by `crates/core`; no operator-facing fallback is required for the shared helper layer. | Canonical support primitives live in Rust. |
| `scripts/runtime` excluding hooks | `Rust-default now` | Runtime verbs, sync, diagnostics, and continuity flows are implemented in Rust, and the docs/workflows already frame PowerShell as compatibility-only. | Default operator runtime path is Rust-backed. |
| `scripts/runtime/hooks` | `legacy integration wrapper retained intentionally` | `pre-tool-use` now has a native Rust boundary, while `common`, `session-start`, and `subagent-start` remain approved shell-owned startup glue for the VS Code/Codex hook contract. | The domain is no longer treated as blocked migration debt; the retained startup hooks are now explicit exceptions. |
| `scripts/maintenance` | `compatibility wrapper retained intentionally` | Four maintenance mutators are native in Rust, and only `generate-http-from-openapi` remains as an approved generator wrapper around the existing external OpenAPI reader toolchain. | The maintenance domain is closed with one explicit retained wrapper. |
| `scripts/validation` | `Rust-default now` | Wave 2 is complete and `validate-all` plus the individual checks are Rust-owned. | Validation is the canonical quality gate surface. |
| `scripts/security` | `Rust-default now` | Security baseline, checksum, and supply-chain checks are Rust-owned through the validation crate. | Security gates default to Rust-owned execution. |
| `scripts/governance` | `Rust-default now` | Routing, template, and repository governance checks are implemented natively in Rust. | Governance checks default to Rust-owned execution. |
| `scripts/doc` | `Rust-default now` | The doc-only validation script now has an explicit native replacement in `crates/commands/validation/documentation`, and `validate-all` routes it without the PowerShell bridge. | Documentation validation defaults to Rust-owned execution. |
| `scripts/deploy` | `compatibility wrapper retained intentionally` | Deploy preflight is now Rust-native through `crates/commands/validation/deploy`, while the PowerShell entrypoint remains the approved SSH/SCP operational executor by design. | The deploy wrapper stays intentionally for remote execution. |
| `scripts/orchestration` | `Rust-default now` | Stage orchestration, resume/replay, and parity harness coverage are Rust-owned and complete. | Control-plane execution defaults to Rust. |
| `scripts/git-hooks` | `compatibility wrapper retained intentionally` | The hook install/check logic is Rust-owned, but the hook entrypoints remain a deliberate compatibility surface for Git-integrated workflows. | Wrapper remains by design for local hook integration. |
| `scripts/tests` excluding runtime | `legacy integration wrapper retained intentionally` | `check-test-naming` and `refactor_tests_to_aaa` are now Rust-native, while `apply-aaa-pattern` and `run-coverage` remain approved wrappers for frontend/.NET-specific workflows that are not being re-homed in Rust. | The non-runtime test domain is closed with two explicit retained wrappers. |
| `scripts/tests/runtime` | `compatibility wrapper retained intentionally` | The native parity harness is real, and the PowerShell test entrypoints remain only as a compatibility launch surface for runtime/operator workflows. | Compatibility wrapper remains by design while the Rust harness stays canonical. |

## Explicit Retained Exceptions

- keep the compatibility wrapper decisions explicit for:
  - `scripts/deploy`
  - `scripts/git-hooks`
  - `scripts/maintenance` (`generate-http-from-openapi`)
  - `scripts/tests/runtime`
- keep the legacy integration wrapper decisions explicit for:
  - `scripts/runtime/hooks`
  - `scripts/tests` excluding runtime (`apply-aaa-pattern`, `run-coverage`)

## Operating Rule

No retained-wrapper domain should change status without a corresponding planning update in the closeout plan and a matching parity-ledger note.