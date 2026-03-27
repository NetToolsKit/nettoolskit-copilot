# nettoolskit-commands

> Command facade that re-exports the NetToolsKit command crates.

---

## Introduction

`nettoolskit-commands` provides a single dependency surface for the command crates used by the workspace. It keeps the `help`, `manifest`, `runtime`, and `validation` crates available through one import point.

---

## Features

- ✅ Re-exports the command crates used by the NetToolsKit workspace
- ✅ Provides a stable facade for CLI and orchestration consumers
- ✅ Keeps command boundary dependencies centralized

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Import the command facade](#example-1-import-the-command-facade)
- [API Reference](#api-reference)
  - [Reexports](#reexports)
- [References](#references)
- [License](#license)

---

## Installation

Add the package as a workspace path dependency:

```toml
[dependencies]
nettoolskit-commands = { path = "../commands" }
```

---

## Quick Start

```rust
use nettoolskit_commands::{
    nettoolskit_help, nettoolskit_manifest, nettoolskit_runtime, nettoolskit_validation,
};

let _ = (
    nettoolskit_help::discover_manifests,
    nettoolskit_manifest::ManifestExecutor::new,
    nettoolskit_runtime::runtime_surface_contract,
    nettoolskit_validation::validation_surface_script_total,
);
```

---

## Usage Examples

### Example 1: Import the command facade

```rust
use nettoolskit_commands::{
    nettoolskit_help, nettoolskit_manifest, nettoolskit_runtime, nettoolskit_validation,
};

let manifest_lookup = nettoolskit_manifest::get_action("check");
let runtime_contract = nettoolskit_runtime::runtime_surface_contract("runtime-hooks");
let validation_total = nettoolskit_validation::validation_surface_script_total();

assert_eq!(manifest_lookup, Some(nettoolskit_manifest::ManifestAction::Check));
assert!(runtime_contract.is_some());
assert_eq!(validation_total, 41);

let _ = nettoolskit_help::ManifestInfo {
    path: std::path::PathBuf::from("sample/ntk-manifest.yml"),
    project_name: "Sample".to_string(),
    language: "rust".to_string(),
    context_count: 2,
};
```

---

## API Reference

### Reexports

- `nettoolskit_help`
- `nettoolskit_manifest`
- `nettoolskit_runtime`
- `nettoolskit_validation`

---

## References

- [crates/commands/help/README.md](help/README.md)
- [crates/commands/manifest/README.md](manifest/README.md)
- [crates/commands/runtime/README.md](runtime/README.md)
- [crates/commands/validation/README.md](validation/README.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---