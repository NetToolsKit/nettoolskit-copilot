# nettoolskit-manifest

> Manifest parsing, execution, and menu actions for NetToolsKit.

---

## Introduction

`nettoolskit-manifest` provides the manifest-driven engine used by the CLI to parse `ntk/v1` manifests, validate them, render templates, and apply file changes to a target output root.

---

## Features

- ✅ Parse and validate `ntk/v1` manifest documents
- ✅ Execute manifests in dry-run or apply mode
- ✅ Render templates and apply file changes with collision policies
- ✅ Expose manifest menu actions and handlers for CLI orchestration

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Parse and validate a manifest](#example-1-parse-and-validate-a-manifest)
  - [Example 2: Execute a dry-run apply](#example-2-execute-a-dry-run-apply)
- [API Reference](#api-reference)
  - [Parsing](#parsing)
  - [Execution](#execution)
  - [Handlers](#handlers)
  - [Enums](#enums)
  - [Errors](#errors)
  - [Data Shapes](#data-shapes)
- [References](#references)
- [License](#license)

---

## Installation

Add the package as a workspace path dependency:

```toml
[dependencies]
nettoolskit-manifest = { path = "../manifest" }
```

---

## Quick Start

```rust
use nettoolskit_manifest::ManifestParser;
use std::path::Path;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let manifest = ManifestParser::from_file(Path::new("ntk-manifest.yml"))?;
ManifestParser::validate(&manifest)?;
# Ok(())
# }
```

---

## Usage Examples

### Example 1: Parse and validate a manifest

```rust
use nettoolskit_manifest::ManifestParser;
use std::path::Path;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let manifest = ManifestParser::from_file(Path::new("ntk-manifest.yml"))?;
ManifestParser::validate(&manifest)?;
# Ok(())
# }
```

### Example 2: Execute a dry-run apply

```rust
use nettoolskit_manifest::{ExecutionConfig, ManifestExecutor};
use std::path::PathBuf;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let executor = ManifestExecutor::new();
let summary = executor
    .execute(ExecutionConfig {
        manifest_path: PathBuf::from("ntk-manifest.yml"),
        output_root: PathBuf::from("target/out"),
        dry_run: true,
    })
    .await?;

println!("Created: {}", summary.created.len());
# Ok(())
# }
```

---

## API Reference

### Parsing

`ManifestParser::from_file(path: &Path) -> ManifestResult<ManifestDocument>`

Loads a manifest from disk and validates the `ntk/v1` API version.

`ManifestParser::validate(manifest: &ManifestDocument) -> ManifestResult<()>`

Validates the manifest structure, required sections, and apply-mode-specific fields.

### Execution

`ExecutionConfig`

Configuration passed to `ManifestExecutor::execute`.

`ManifestExecutor::new() -> Self`

Creates a new executor instance.

`ManifestExecutor::execute(&self, config: ExecutionConfig) -> ManifestResult<ExecutionSummary>`

Parses, validates, and applies a manifest.

### Handlers

`execute_apply(manifest_path: PathBuf, output_root: Option<PathBuf>, dry_run: bool) -> ExitStatus`

Executes the application flow used by the CLI surface.

`show_menu() -> ExitStatus`

Displays the manifest menu.

`show_apply_menu() -> ExitStatus`

Displays the apply-specific manifest menu.

### Enums

`ManifestAction`

Use `get_action(name: &str) -> Option<ManifestAction>` to map user input into an action, then use `description()` and `full_command()` to render menu text.

| Variant | Command | Description |
| --- | --- | --- |
| `Check` | `check` | Validate manifest structure and dependencies |
| `Render` | `render` | Preview generated files without creating them |
| `Apply` | `apply` | Apply manifest to generate or update project files |
| `Back` | `back` | Return to main menu |

### Errors

`ManifestError`

Represents manifest parsing, validation, rendering, and file-system failures.

`ManifestResult<T>`

Result alias returned by manifest operations.

### Data Shapes

| Field | Description | Example |
| --- | --- | --- |
| `manifest_path` | Path to the manifest YAML file. | `ntk-manifest.yml` |
| `output_root` | Output directory for generated files. | `target/out` |
| `dry_run` | When `true`, validates without writing files. | `true` |

---

## References

- [crates/commands/README.md](../README.md)
- [crates/commands/help/README.md](../help/README.md)
- [crates/commands/templating/README.md](../templating/README.md)
- [serde_yaml](https://docs.rs/serde_yaml)
- [strum](https://docs.rs/strum)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---