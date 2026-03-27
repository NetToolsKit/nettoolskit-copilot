# NetToolsKit Workspace

> Rust workspace, runtime scripts, and versioned projection assets for the NetToolsKit toolchain.

---

## Introduction

NetToolsKit is a multi-crate workspace that combines Rust command boundaries, repository-owned runtime scripts, and versioned definitions that project into provider and editor surfaces.

It is organized to keep implementation, orchestration, planning, and reference documentation separate while still making the full workspace easy to navigate.

---

## Features

- ✅ Rust crates for CLI entry points, orchestration, commands, telemetry, runtime validation, and UI boundaries
- ✅ Versioned projection model for `.github/`, `.codex/`, `.claude/`, and `.vscode/`
- ✅ Deterministic planning, specification, and reference docs under `planning/`
- ✅ Operational scripts for bootstrap, sync, validation, health, and maintenance flows
- ✅ Workspace documentation that keeps crate and support surfaces discoverable

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Build and Tests](#build-and-tests)
- [Contributing](#contributing)
- [Dependencies](#dependencies)
- [References](#references)
- [License](#license)

---

## Build and Tests

- `cargo build --workspace`
- `cargo test --workspace`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace -- -D warnings`
- `pwsh -File .\scripts\validation\validate-readme-standards.ps1`

---

## Contributing

- Keep README content in English.
- Preserve the allowed root section order.
- Update crate and support README links when workspace structure changes.
- Run workspace validation before committing documentation updates.

---

## Dependencies

- Rust toolchain and Cargo for workspace builds and tests.
- PowerShell 7+ for repository scripts and validation entrypoints.
- GitHub Copilot / Codex runtime assets when working on projected surfaces.

---

## References

| Area | README |
| --- | --- |
| Workspace crate | [crates/cli/README.md](crates/cli/README.md) |
| Workspace crate | [crates/core/README.md](crates/core/README.md) |
| Workspace crate | [crates/ui/README.md](crates/ui/README.md) |
| Workspace crate | [crates/otel/README.md](crates/otel/README.md) |
| Workspace crate | [crates/orchestrator/README.md](crates/orchestrator/README.md) |
| Command boundary | [crates/commands/README.md](crates/commands/README.md) |
| Command package | [crates/commands/help/README.md](crates/commands/help/README.md) |
| Command package | [crates/commands/manifest/README.md](crates/commands/manifest/README.md) |
| Command package | [crates/commands/runtime/README.md](crates/commands/runtime/README.md) |
| Command package | [crates/commands/templating/README.md](crates/commands/templating/README.md) |
| Command package | [crates/commands/validation/README.md](crates/commands/validation/README.md) |
| Workspace crate | [crates/task-worker/README.md](crates/task-worker/README.md) |

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---